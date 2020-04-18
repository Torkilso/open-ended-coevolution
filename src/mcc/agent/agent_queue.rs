use crate::mcc::agent::mcc_agent::MCCAgent;

pub struct AgentQueue {
    agents: Vec<MCCAgent>,
    current_agent_index: usize,
    max_items_limit: usize,
}

impl AgentQueue {
    pub fn new(mcc_agents: Vec<MCCAgent>, max_items_limit: usize) -> AgentQueue {
        AgentQueue {
            agents: mcc_agents,
            current_agent_index: 0,
            max_items_limit,
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

        if self.agents.len() >= self.max_items_limit {
            self.remove_oldest(self.agents.len() - self.max_items_limit);
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
            self.current_agent_index = (self.current_agent_index + 1) % self.max_items_limit;
        }

        for child in children.iter_mut() {
            child.mutate();
            child.viable = false;
        }

        children
    }
}
