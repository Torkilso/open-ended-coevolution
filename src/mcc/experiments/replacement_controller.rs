use crate::mcc::agent::speciated_agent_queue::SpeciatedAgentQueue;
use crate::mcc::maze::speciated_maze_queue::SpeciatedMazeQueue;

pub enum ReplacementStrategy {
    Gradual,
    Sudden,
}

pub struct ReplacementEntryController {
    species_to_replace_id: u32,
    new_species_id: u32,
    spanning_over_generations: u32,
}

pub struct ReplacementController {
    pub(crate) agent_entries: Vec<ReplacementEntryController>,
    pub(crate) maze_entries: Vec<ReplacementEntryController>,
    pub(crate) replace_type: ReplacementStrategy,
}

impl ReplacementController {
    pub fn find_species_to_replace(
        &self,
        _agents: &mut SpeciatedAgentQueue,
        _mazes: &mut SpeciatedMazeQueue,
    ) {
        // look for species that perform very bad compared to others
        // also look at history to see how long they have performed bad
        // replace with new seeds if they are bad for 50 generations?
        //
    }

    pub fn update_population_properties(
        &self,
        _agents: &mut SpeciatedAgentQueue,
        _mazes: &mut SpeciatedMazeQueue,
        _current_generation: u32,
    ) {
        // go through all replacement entries and update capacity for species
    }
}
