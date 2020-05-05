use crate::mcc::agent::agent_queue::AgentQueue;
use crate::mcc::agent::mcc_agent::MCCAgent;

#[derive(Clone)]
pub struct AgentSpeciesStatistics {
    average_sizes: Vec<f64>,
    maximum_sizes: Vec<u32>,
    minimum_sizes: Vec<u32>,
    average_size_increases: Vec<f64>,
}

impl AgentSpeciesStatistics {
    pub fn get_overall_average_increase(&self) -> f64 {
        self.average_size_increases.iter().sum::<f64>() as f64
            / self.average_size_increases.len() as f64
    }

    pub fn get_current_average_size_increase(&self) -> f64 {
        let last = self.average_size_increases.last();
        if last.is_some() {
            *last.unwrap()
        } else {
            0.0
        }
    }
}

#[derive(Clone)]
pub struct AgentSpecies {
    centroid: MCCAgent,
    pub agent_queue: AgentQueue,
    pub(crate) id: u32,
    pub statistics: AgentSpeciesStatistics,
}

impl AgentSpecies {
    pub fn new(agent: MCCAgent, max_items_limit: u32, id: u32) -> AgentSpecies {
        AgentSpecies {
            agent_queue: AgentQueue::new(vec![agent.clone()], max_items_limit),
            centroid: agent.clone(),
            id,
            statistics: AgentSpeciesStatistics {
                average_sizes: vec![],
                maximum_sizes: vec![],
                minimum_sizes: vec![],
                average_size_increases: vec![],
            },
        }
    }

    /*pub fn iter(&self) -> impl Iterator<Item = &MCCAgent> {
        self.agent_queue.iter()
    }*/

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

    pub fn save_state(&mut self) {
        let last_average_size = self.statistics.average_sizes.last();
        if last_average_size.is_some() {
            let new_average_increase =
                self.agent_queue.get_average_size() - *last_average_size.unwrap();
            self.statistics
                .average_size_increases
                .push(new_average_increase);
        }

        self.statistics
            .average_sizes
            .push(self.agent_queue.get_average_size());
        self.statistics
            .maximum_sizes
            .push(self.agent_queue.get_largest_size());
        self.statistics
            .minimum_sizes
            .push(self.agent_queue.get_smallest_size());
    }
}
