use std::collections::HashMap;

use rand::seq::SliceRandom;
use rand::Rng;

use crate::config;
use crate::neatns::network::genome::Genome;
use crate::neatns::network::link::Link;
use crate::neatns::network::node::{Node, NodeRef};
use crate::neatns::network::{activation, connection, order};

#[derive(Clone)]
pub struct AgentGenome {
    pub inputs: HashMap<NodeRef, Node>,
    pub hidden_nodes: HashMap<NodeRef, Node>,
    pub outputs: HashMap<NodeRef, Node>,
    pub links: HashMap<(NodeRef, NodeRef), Link>, // Links between nodes

    pub order: order::Order<NodeRef>,
    // Actions to perform when evaluating
    pub connections: connection::Connections<NodeRef>, // Fast connection lookup
}

impl AgentGenome {
    pub fn new(genome: Genome) -> AgentGenome {
        AgentGenome {
            inputs: genome.inputs.clone(),
            hidden_nodes: genome.hidden_nodes.clone(),
            outputs: genome.outputs.clone(),
            links: genome.links.clone(),
            order: genome.order.clone(),
            connections: genome.connections.clone(),
        }
    }

    fn split_link(&mut self, link: Link, new_node_id: u64) {
        {
            // Disable link
            let link = self
                .links
                .get_mut(&(link.from, link.to))
                .expect("Unable to split nonexistent link");

            assert!(link.enabled);
            link.enabled = false;
            link.split = true;
        }

        let new_node_ref = NodeRef::Hidden(new_node_id);

        // Might have inherited that the connection is not split, but also the nodes splitting it
        if self.hidden_nodes.contains_key(&new_node_ref) {
            return;
        }

        // Disable connection
        self.connections.disable(link.from, link.to);

        // Add and remove actions
        self.order.split_link(link.from, link.to, new_node_ref);

        self.hidden_nodes
            .insert(new_node_ref, Node::new(NodeRef::Hidden(new_node_id)));

        let link1 = Link::new(link.from, new_node_ref, 1.0, 0);
        let link2 = Link::new(new_node_ref, link.to, link.weight, 0);

        assert!(!self.links.contains_key(&(link1.from, link1.to)));
        self.insert_link(link1, false);

        assert!(!self.links.contains_key(&(link2.from, link2.to)));
        self.insert_link(link2, false);
    }

    fn insert_link(&mut self, link: Link, add_action: bool) {
        // Add link
        self.links.insert((link.from, link.to), link);

        // Add connections
        self.connections.add(link.from, link.to, link.enabled);

        // Add action
        if link.enabled && add_action {
            // When adding many links at the same time, it is faster to sort
            // topologically at the end than adding every connection independently
            // When 'add_action' is false, 'sort_topologically' must be called on
            // self.actions when all links are inserted.
            // Except when the link is added by split, then self.action should
            // perform the split internally.
            self.order.add_link(link.from, link.to, &self.connections);
        }
    }

    pub fn mutate(&mut self) {
        let mut rng = rand::thread_rng();

        if rng.gen::<f64>() < config::NEAT.add_node_probability {
            self.mutation_add_node();
        }

        if rng.gen::<f64>() < config::NEAT.add_connection_probability {
            self.mutation_add_connection();
        }

        if rng.gen::<f64>() < config::NEAT.disable_connection_probability {
            self.mutation_disable_connection();
        }

        if rng.gen::<f64>() < config::NEAT.mutate_link_weight_probability {
            self.mutate_link_weight();
        }
    }

    fn mutate_link_weight(&mut self) {
        let mut rng = rand::thread_rng();

        // Mutate single link
        if !self.links.is_empty() {
            let link_index = rng.gen_range(0, self.links.len());
            if let Some(link) = self.links.values_mut().skip(link_index).next() {
                link.weight +=
                    (rng.gen::<f64>() - 0.5) * 2.0 * config::NEAT.mutate_link_weight_size;
            }
        }

        /*for link in self.links.values_mut() {
            link.weight += (rng.gen::<f64>() - 0.5) * 2.0 * config::NEAT.mutate_link_weight_size;
        }*/
    }

    fn mutation_add_node(&mut self) {
        // Select random enabled link
        if let Some(index) = self
            .links
            .iter()
            .filter(|(_, link)| !link.split && link.enabled)
            .map(|(i, _)| *i)
            .collect::<Vec<(NodeRef, NodeRef)>>()
            .choose(&mut rand::thread_rng())
        {
            if self.order.contains(&order::Action::Link(index.0, index.1)) {
                if let Some(&link) = self.links.get(index) {
                    self.split_link(link, self.hidden_nodes.len() as u64);
                }
            }
            /*assert!(self.order.contains(&order::Action::Link(index.0, index.1)));

            if let Some(&link) = self.links.get(index) {
                self.split_link(link, self.hidden_nodes.len() as u64);
            }*/
        }
    }

    // TODO: avoid retries
    fn mutation_add_connection(&mut self) {
        let mut rng = rand::thread_rng();

        // Retry 50 times
        for _ in 0..50 {
            // Select random source and target nodes for new link
            let from_index = rng.gen_range(0, self.inputs.len() + self.hidden_nodes.len());
            let to_index = rng.gen_range(0, self.hidden_nodes.len() + self.outputs.len());

            let from_option = if from_index < self.inputs.len() {
                self.inputs.keys().skip(from_index).next()
            } else {
                self.hidden_nodes
                    .keys()
                    .skip(from_index - self.inputs.len())
                    .next()
            };
            let to_option = if to_index < self.outputs.len() {
                self.outputs.keys().skip(to_index).next()
            } else {
                self.hidden_nodes
                    .keys()
                    .skip(to_index - self.outputs.len())
                    .next()
            };

            if let (Some(&from), Some(&to)) = (from_option, to_option) {
                // If connection does not exist and its addition does not create cycle
                if !self.links.contains_key(&(from, to))
                    && !self.connections.creates_cycle(from, to)
                {
                    self.insert_link(
                        Link::new(
                            from,
                            to,
                            (rng.gen::<f64>() - 0.5) * 2.0 * config::NEAT.initial_link_weight_size,
                            0,
                        ),
                        true,
                    );
                    break;
                }
            } else {
                break;
            }
        }
    }

    fn mutation_disable_connection(&mut self) {
        if let Some(&connection_ref) = self
            .links
            .iter()
            .filter(|(_, link)| link.enabled)
            .map(|(i, _)| i)
            .collect::<Vec<&(NodeRef, NodeRef)>>()
            .choose(&mut rand::thread_rng())
        {
            let connection_ref = *connection_ref;

            self.connections.disable(connection_ref.0, connection_ref.1);
            self.order.remove_link(connection_ref.0, connection_ref.1);

            if let Some(link) = self.links.get_mut(&connection_ref) {
                link.enabled = false;
            }
        }
    }

    // Genetic distance between two genomes
    pub fn distance(&self, other: &Self) -> f64 {
        let mut link_differences: u64 = 0; // Number of links present in only one of the genomes
        let mut link_distance: f64 = 0.0; // Total distance between links present in both genomes
        let mut link_count = self.links.len() as u64; // Number of unique links between the two genomes

        for link_ref in other.links.keys() {
            if !self.links.contains_key(link_ref) {
                link_differences += 1;
            }
        }
        link_count += link_differences; // Count is number of links in A + links in B that are not in A

        for (link_ref, link) in self.links.iter() {
            if let Some(link2) = other.links.get(link_ref) {
                link_distance += link.distance(link2); // Distance normalized between 0 and 1
            } else {
                link_differences += 1;
            }
        }

        return if link_count == 0 {
            0.0
        } else {
            ((link_differences as f64) + link_distance) / (link_count as f64)
        };
    }

    pub fn get_activation(&self, node_ref: &NodeRef) -> activation::Activation {
        match node_ref {
            NodeRef::Input(_) => self.inputs.get(node_ref).unwrap().activation,
            NodeRef::Hidden(_) => self.hidden_nodes.get(node_ref).unwrap().activation,
            NodeRef::Output(_) => self.outputs.get(node_ref).unwrap().activation,
        }
    }

    pub fn get_bias(&self, node_ref: &NodeRef) -> f64 {
        match node_ref {
            NodeRef::Input(_) => self.inputs.get(node_ref).unwrap().bias,
            NodeRef::Hidden(_) => self.hidden_nodes.get(node_ref).unwrap().bias,
            NodeRef::Output(_) => self.outputs.get(node_ref).unwrap().bias,
        }
    }
}
