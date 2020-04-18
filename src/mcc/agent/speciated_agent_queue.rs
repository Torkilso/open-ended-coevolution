use crate::config;
use crate::mcc::agent::agent_species::AgentSpecies;
use crate::mcc::agent::mcc_agent::MCCAgent;
use crate::neatns::agent::Agent;
use crate::mcc::agent::ReplacementStrategy;

pub struct SpeciatedAgentQueue {
    species: Vec<AgentSpecies>,
    varied_size_in_species: bool,
    replacement_strategy: ReplacementStrategy,
}

impl SpeciatedAgentQueue {
    pub fn new(agents: Vec<Agent>, varied_size_in_species: bool,
               replacement_strategy: ReplacementStrategy, ) -> SpeciatedAgentQueue {
        let mut mcc_agents: Vec<MCCAgent> = vec![];

        for agent in agents {
            let mcc_agent = MCCAgent::new(agent);
            mcc_agents.push(mcc_agent);
        }

        let mut queue = SpeciatedAgentQueue { species: vec![], varied_size_in_species, replacement_strategy };

        let species_max_agents_limit: usize =
            config::MCC.agent_population_capacity / mcc_agents.len();

        for agent in mcc_agents {
            let species = AgentSpecies::new(agent, species_max_agents_limit);
            queue.species.push(species);
        }

        queue
    }

    pub fn iter(&self) -> impl Iterator<Item=&MCCAgent> {
        self.species.iter().map(|species| species.iter()).flatten()
    }

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

        let amount: usize = config::MCC.agent_selection_limit / self.species.len();

        for species in self.species.iter_mut() {
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
}
