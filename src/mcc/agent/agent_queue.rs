use crate::mcc::agent::mcc_agent::MCCAgent;

#[derive(Clone)]
pub struct AgentQueue {
    agents: Vec<MCCAgent>,
    current_agent_index: usize,
    pub max_items_limit: u32,
    total_individuals_added: u32,
}

impl AgentQueue {
    pub fn new(mcc_agents: Vec<MCCAgent>, max_items_limit: u32) -> AgentQueue {
        let total_individuals_added = mcc_agents.len() as u32;
        AgentQueue {
            agents: mcc_agents,
            current_agent_index: 0,
            max_items_limit,
            total_individuals_added,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &MCCAgent> {
        self.agents.iter()
    }

    pub fn len(&self) -> usize {
        self.agents.len()
    }

    pub fn push(&mut self, agent: MCCAgent) {
        self.agents.push(agent);

        if self.agents.len() as u32 >= self.max_items_limit {
            self.remove_oldest(self.agents.len() - self.max_items_limit as usize);
        }
    }

    fn remove_oldest(&mut self, amount: usize) {
        for _ in 0..amount {
            self.agents.remove(0);
        }
        if amount > self.current_agent_index {
            self.current_agent_index = 0;
        } else {
            self.current_agent_index -= amount;
        }
    }

    pub fn get_children(&mut self, amount: usize) -> Vec<MCCAgent> {
        let mut children: Vec<MCCAgent> = vec![];

        for _ in 0..amount {
            if self.current_agent_index >= self.agents.len() {
                self.current_agent_index = 0;
            }

            children.push(self.agents.get(self.current_agent_index).unwrap().clone());
            self.current_agent_index =
                (self.current_agent_index + 1) % self.max_items_limit as usize;
        }

        for child in children.iter_mut() {
            child.id = self.total_individuals_added;
            self.total_individuals_added += 1;

            child.mutate();
            child.viable = false;
        }

        children
    }

    pub fn get_largest_size(&self) -> u32 {
        let max = self.agents.iter().max_by_key(|a| a.genome.links.len());
        return max.unwrap().genome.links.len() as u32;
    }

    pub fn get_smallest_size(&self) -> u32 {
        let min = self.agents.iter().min_by_key(|a| a.genome.links.len());
        return min.unwrap().genome.links.len() as u32;
    }

    pub fn get_average_size(&self) -> f64 {
        let mut sum = 0;
        for a in self.agents.iter() {
            sum += a.genome.links.len();
        }

        sum as f64 / self.agents.len() as f64
    }
}
