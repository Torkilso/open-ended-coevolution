use crate::config;
use crate::mcc::agent::agent_species::AgentSpecies;
use crate::mcc::agent::mcc_agent::MCCAgent;
use crate::neatns::agent::Agent;

pub struct SpeciatedAgentQueue {
    species: Vec<AgentSpecies>,
}

impl SpeciatedAgentQueue {
    pub fn new(agents: Vec<Agent>) -> SpeciatedAgentQueue {
        let mut mcc_agents: Vec<MCCAgent> = vec![];

        for agent in agents {
            let mcc_agent = MCCAgent::new(agent);
            mcc_agents.push(mcc_agent);
        }

        let mut queue = SpeciatedAgentQueue { species: vec![] };

        let base_amount: usize = config::MCC.agent_population_capacity / mcc_agents.len();
        let mut rest = config::MCC.agent_population_capacity % base_amount;

        println!(
            "config::MCC.agent_population_capacity {}",
            config::MCC.agent_population_capacity
        );
        println!("base amount {}", base_amount);
        println!("rest {}", rest);

        for (i, agent) in mcc_agents.iter().enumerate() {
            let amount = if i < rest {
                base_amount + 1
            } else {
                base_amount
            };
            println!("amount {}", amount);

            let species = AgentSpecies::new(agent.clone(), amount, i as u32);
            queue.species.push(species);
        }

        queue
    }

    pub fn iter_species(&self) -> impl Iterator<Item = &AgentSpecies> {
        self.species.iter()
    }

    /*pub fn iter_individuals(&self) -> impl Iterator<Item = &MCCAgent> {
        self.species.iter().map(|species| species.iter()).flatten()
    }*/

    pub fn len(&self) -> usize {
        let mut length = 0;

        for species in self.species.iter() {
            length += species.len();
        }

        length
    }

    // Looks for suitable species to put new agent in
    // If none are found, a new species is made
    pub fn push(&mut self, agent: MCCAgent) {
        let mut distances: Vec<f64> = vec![];

        for species in self.species.iter() {
            distances.push(species.distance(&agent));
        }

        let mut highest = 0.0;
        let mut index: usize = 0;

        for (i, value) in distances.iter().enumerate() {
            if *value > highest {
                highest = *value;
                index = i;
            }
        }

        self.species[index].push(agent);
    }

    // Generates children from the next parents in line
    // Picks parents from all species
    // The parents are only mutated, no crossover is performed
    pub fn get_children(&mut self) -> Vec<MCCAgent> {
        let mut children: Vec<MCCAgent> = vec![];

        let base_amount: usize = config::MCC.agent_selection_limit / self.species.len();
        let mut rest = config::MCC.agent_selection_limit - base_amount * self.species.len();

        for species in self.species.iter_mut() {
            let amount = if rest > 0 {
                base_amount + 1
            } else {
                base_amount
            };
            rest -= 1;
            for child in species.get_children(amount) {
                children.push(child);
            }
        }

        for child in children.iter_mut() {
            child.mutate();
            child.viable = false;
        }

        children
    }

    pub fn get_largest_size(&self) -> u32 {
        let max = self
            .species
            .iter()
            .max_by_key(|s| s.agent_queue.get_largest_size());
        max.unwrap().agent_queue.get_largest_size()
    }

    pub fn get_smallest_size(&self) -> u32 {
        let min = self
            .species
            .iter()
            .min_by_key(|s| s.agent_queue.get_smallest_size());
        min.unwrap().agent_queue.get_smallest_size()
    }

    pub fn get_average_size(&self) -> f64 {
        let mut sum = 0;
        for s in self.species.iter() {
            for a in s.agent_queue.iter() {
                sum += a.genome.links.len();
            }
        }

        sum as f64 / self.len() as f64
    }

    pub fn get_average_size_increase(&self) -> f64 {
        let mut sum = 0.0;
        for s in self.species.iter() {
            sum += s.statistics.get_overall_average_increase();
        }

        sum as f64 / self.species.len() as f64
    }

    pub fn save_state(&mut self) {
        for s in self.species.iter_mut() {
            s.save_state();
        }
    }
}
