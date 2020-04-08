use crate::config;
use crate::mcc::agent::mcc_agent::MCCAgent;
use crate::neatns::agent::Agent;

pub struct AgentQueue {
    agents: Vec<MCCAgent>,
    current_agent_index: usize,
    max_items_limit: usize,
}

impl AgentQueue {
    pub fn new(agents: Vec<Agent>) -> AgentQueue {
        let mut mcc_agents: Vec<MCCAgent> = vec![];

        for agent in agents {
            let mcc_agent = MCCAgent::new(agent);
            mcc_agents.push(mcc_agent);
        }

        AgentQueue {
            agents: mcc_agents,
            current_agent_index: 0,
            max_items_limit: config::MCC.agent_population_capacity,
        }
    }

    pub fn len(&self) -> usize {
        self.agents.len()
    }

    pub fn push(&mut self, agent: MCCAgent) {
        if self.agents.len() >= self.max_items_limit {
            self.remove_oldest(self.agents.len() - self.max_items_limit);
        }

        self.agents.push(agent);
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

    pub fn get_children(&mut self) -> Vec<MCCAgent> {
        let mut children: Vec<MCCAgent> = vec![];

        for _ in 0..config::MCC.agent_selection_limit {
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
