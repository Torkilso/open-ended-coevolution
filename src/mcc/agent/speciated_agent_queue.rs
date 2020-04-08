/*use crate::mcc::agent::mcc_agent::MCCAgent;
use crate::mcc::agent::agent_species::AgentSpecies;

pub struct SpeciatedAgentQueue {
    species: Vec<AgentSpecies>,
    species_counter: u32,
}

impl SpeciatedAgentQueue {
    pub fn new(agents: Vec<MCCAgent>) -> SpeciatedAgentQueue {
        let mut queue = SpeciatedAgentQueue {
            species: vec![],
            species_counter: 0,
        };

        for agent in agents {
            let mut species = AgentSpecies::new(queue.species_counter, agent);
            queue.species.push(species);
            queue.species_counter += 1;
        }

        queue
    }

    pub fn add_agent(&mut self, agent: MCCAgent) {
        if let Some(species) = self.compatible_species(&agent) {
            species.add_agent(agent);
        } else {
            let mut species = AgentSpecies::new(self.species_counter, agent);
            self.species.push(species);
            self.species_counter += 1;
        }
    }

    fn compatible_species(&mut self, agent: &MCCAgent) -> Option<&mut AgentSpecies> {
        for species in self.species.iter_mut() {
            if species.is_compatible(&agent) {
                return Some(species);
            }
        }

        None
    }

    pub fn select_next_batch_in_line(&mut self) -> Vec<MCCAgent> {
        let mut batch: Vec<MCCAgent> = vec!();

        for species in self.species.iter_mut() {
            let species_agents = species.get_next_agent();
            //batch.append(species_agents)
        }

        batch

        /*let test = self.species
            .iter_mut()
            .map(|species| species.get_next_batch_in_line())
            .flatten();*/
    }
}
*/
