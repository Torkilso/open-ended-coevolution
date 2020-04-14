use crate::mcc::agent::mcc_agent::MCCAgent;
use crate::mcc::agent::agent_queue::AgentQueue;

pub struct AgentSpecies {
    centroid: MCCAgent,
    agent_queue: AgentQueue
}

impl AgentSpecies {
    pub fn new(mut agent: MCCAgent, max_items_limit: usize) -> AgentSpecies {
        agent.mcc_species_id = Option::Some(0);


        AgentSpecies {
            agent_queue: AgentQueue::new(vec!(agent.clone()), max_items_limit),
            centroid: agent.clone(),
        }
    }

    pub fn push(&mut self, agent: MCCAgent) {
        self.agent_queue.push(agent);
    }

    pub fn get_children(&mut self, amount: usize) -> Vec<MCCAgent> {
        self.agent_queue.get_children(amount)
    }

    pub fn distance(&self, other: &MCCAgent) -> f64 {
        self.centroid.distance(other)
    }
}
