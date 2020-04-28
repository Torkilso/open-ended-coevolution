use crate::mcc::agent::agent_queue::AgentQueue;
use crate::mcc::agent::mcc_agent::MCCAgent;

pub struct AgentSpecies {
    centroid: MCCAgent,
    pub agent_queue: AgentQueue,
}

impl AgentSpecies {
    pub fn new(mut agent: MCCAgent, max_items_limit: usize) -> AgentSpecies {
        AgentSpecies {
            agent_queue: AgentQueue::new(vec![agent.clone()], max_items_limit),
            centroid: agent.clone(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &MCCAgent> {
        self.agent_queue.iter()
    }

    pub fn push(&mut self, agent: MCCAgent) {
        self.agent_queue.push(agent);
    }

    pub fn len(&self) -> usize {
        self.agent_queue.len()
    }

    pub fn get_children(&mut self, amount: usize) -> Vec<MCCAgent> {
        self.agent_queue.get_children(amount)
    }

    pub fn distance(&self, other: &MCCAgent) -> f64 {
        self.centroid.distance(other)
    }
}
