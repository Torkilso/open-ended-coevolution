use crate::agent::agent::Agent;

#[derive(Debug, Clone)]
pub struct AgentSpecies {
    id: u32,
    pub(crate) agents: Vec<Agent>,
    pub(crate) complexity_growth: f64,
}


#[derive(Debug, Clone)]
pub struct AgentQueue {
    pub(crate) species: Vec<AgentSpecies>,
}


impl AgentQueue {
    pub fn new(agents: Vec<Agent>) -> AgentQueue {

        let queue = AgentQueue { species: vec!() };

        for agent in agents.iter(){
            queue.add_agent(agent.clone())
        }

        queue
    }

    pub fn speciate(&self) {

    }

    pub fn add_agent(&self, agent: Agent) {
        // check if agent fits into any species
        // add to species if yes, create new species if no
    }
}