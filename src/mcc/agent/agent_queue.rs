use crate::mcc::agent::mcc_agent::MCCAgent;

pub struct AgentQueue {
    agents: Vec<MCCAgent>,
    current_agent_index: usize,
    max_items_limit: usize,
    total_individuals_added: u32,
}

impl AgentQueue {
    pub fn new(mcc_agents: Vec<MCCAgent>, max_items_limit: usize) -> AgentQueue {
        let total_individuals_added = mcc_agents.len() as u32;
        AgentQueue {
            agents: mcc_agents,
            current_agent_index: 0,
            max_items_limit,
            total_individuals_added,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item=&MCCAgent> {
        self.agents.iter()
    }

    pub fn len(&self) -> usize {
        self.agents.len()
    }

    pub fn push(&mut self, mut agent: MCCAgent) {
        agent.id = self.total_individuals_added;
        self.agents.push(agent);
        self.total_individuals_added += 1;

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
            child.id = self.total_individuals_added;
            self.total_individuals_added += 1;

            child.mutate();
            child.viable = false;
        }

        children
    }
}
