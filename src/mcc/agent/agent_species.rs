use crate::mcc::agent::mcc_agent::MCCAgent;
use crate::config;

extern crate queues;

use queues::*;

pub struct AgentSpecies {
    id: u32,
    current_agent_index: usize,
    agents: Vec<MCCAgent>,
    centroid: MCCAgent,
    agent_counter: u32,
}

impl AgentSpecies {
    pub fn new(id: u32, mut agent: MCCAgent) -> AgentSpecies {
        agent.mcc_species_id = Option::Some(0);

        AgentSpecies {
            id,
            current_agent_index: 0,
            agents: vec!(agent.clone()),
            centroid: agent.clone(),
            agent_counter: 0
        }
    }

    // removes oldest agent from species
    pub fn remove_oldest(&mut self) {

    }

    // return next agent in line and updates pointer to start of queue
    pub fn get_next_agent(&mut self) -> MCCAgent {
        let agent = self.agents[self.current_agent_index].clone();
        self.current_agent_index += 1;
        agent
    }

    pub fn get_next_batch_in_line(&mut self) -> Vec<MCCAgent> {
        let mut agents: Vec<MCCAgent> = vec![];

        let amount_to_use = if self.agents.len() < config::MCC.agent_selection_limit {
            self.agents.len()
        } else {
            config::MCC.agent_selection_limit
        };

        for _ in 0..amount_to_use {
            //let agent = self.agents.remove().into_ok();
            //agents.push(agent);
        }
        agents
    }

    pub fn add_agent(&mut self, mut agent: MCCAgent) {
        agent.mcc_species_id = Option::Some(self.agent_counter);

        self.agents.push(agent);
    }

    pub fn is_compatible(&mut self, other: &MCCAgent) -> bool {
        self.centroid.distance(other) < config::MCC.speciation_threshold
    }
}
