use crate::config;
use crate::mcc::agent::speciated_agent_queue::SpeciatedAgentQueue;
use crate::mcc::maze::speciated_maze_queue::SpeciatedMazeQueue;

pub struct VariedSizeController {
    //pub(crate) agent_entries: Vec<VariedSizeEntry>,
//pub(crate) maze_entries: Vec<VariedSizeEntry>,
}

impl VariedSizeController {
    fn update_agents(&mut self, agents: &mut SpeciatedAgentQueue) {
        let mut clone = agents.species.clone();

        clone.sort_by(|a, b| {
            (a.statistics.get_overall_average_increase())
                .partial_cmp(&b.statistics.get_overall_average_increase())
                .unwrap()
        });

        let length = agents.species.len();
        let (worst, best) = clone.split_at(length / 2);

        for (w, b) in worst.iter().zip(best) {
            let mut valid_to = true;
            let mut valid_from = true;

            let from_species_check = agents.iter_species_mut().find(|m| m.id == w.id);

            if from_species_check.is_some() {
                if from_species_check.unwrap().agent_queue.max_items_limit <= config::MCC.varied_size_agent_default_borrow_amount {
                    continue;
                }
            }

            {
                let to_species = agents.iter_species_mut().find(|m| m.id == b.id);

                if to_species.is_some() {
                    to_species.unwrap().agent_queue.max_items_limit +=
                        config::MCC.varied_size_agent_default_borrow_amount;
                } else {
                    valid_to = false;
                }
            }

            {
                let from_species = agents.iter_species_mut().find(|m| m.id == w.id);

                if from_species.is_some() && valid_to {
                    from_species.unwrap().agent_queue.max_items_limit -=
                        config::MCC.varied_size_agent_default_borrow_amount;
                } else {
                    valid_from = false;
                }
            }

            if valid_to && !valid_from {
                let to_species = agents.iter_species_mut().find(|m| m.id == b.id);

                if to_species.is_some() {
                    to_species.unwrap().agent_queue.max_items_limit -=
                        config::MCC.varied_size_agent_default_borrow_amount;
                }
            }
        }
    }

    fn update_mazes(&mut self, mazes: &mut SpeciatedMazeQueue) {
        let mut clone = mazes.species.clone();

        clone.sort_by(|a, b| {
            (a.statistics.get_overall_score())
                .partial_cmp(&b.statistics.get_overall_score())
                .unwrap()
        });

        let length = mazes.species.len();
        let (worst, best) = clone.split_at(length / 2);

        for (w, b) in worst.iter().zip(best) {
            let mut valid_to = true;
            let mut valid_from = true;

            let from_species_check = mazes.iter_species_mut().find(|m| m.id == w.id);

            if from_species_check.is_some() {
                if from_species_check.unwrap().maze_queue.max_items_limit <= config::MCC.varied_size_maze_default_borrow_amount {
                    continue;
                }
            }

            {
                let to_species = mazes.iter_species_mut().find(|m| m.id == b.id);

                if to_species.is_some() {
                    to_species.unwrap().maze_queue.max_items_limit +=
                        config::MCC.varied_size_maze_default_borrow_amount;
                } else {
                    valid_to = false;
                }
            }

            {
                let from_species = mazes.iter_species_mut().find(|m| m.id == w.id);

                if from_species.is_some() && valid_to {
                    from_species.unwrap().maze_queue.max_items_limit -=
                        config::MCC.varied_size_maze_default_borrow_amount;
                } else {
                    valid_from = false;
                }
            }

            if valid_to && !valid_from {
                let to_species = mazes.iter_species_mut().find(|m| m.id == b.id);

                if to_species.is_some() {
                    to_species.unwrap().maze_queue.max_items_limit -=
                        config::MCC.varied_size_maze_default_borrow_amount;
                }
            }
        }
    }

    pub fn update_population_properties(
        &mut self,
        agents: &mut SpeciatedAgentQueue,
        mazes: &mut SpeciatedMazeQueue,
    ) {
        self.update_agents(agents);
        self.update_mazes(mazes);
    }
}
