use crate::analytics::Analyzer;
use crate::config;
use crate::mcc::agent::speciated_agent_queue::SpeciatedAgentQueue;
use crate::mcc::maze::maze_species::MazeSpecies;
use crate::mcc::maze::speciated_maze_queue::SpeciatedMazeQueue;
use crate::mcc::{generate_generation_stats_s, print_stats};
use crate::neatns;
use crate::simulator::simulate_many;
use std::borrow::{Borrow, BorrowMut};

pub struct VariedSizeEntry {
    active: bool,
    is_initiated: bool,
    from_species_id: u32,
    to_species_id: u32,
    population_amount_borrowed: u32,
    pub initiated_at_generation: u32,
}

pub struct VariedSizeController {
    agent_entries: Vec<VariedSizeEntry>,
    maze_entries: Vec<VariedSizeEntry>,
}

impl VariedSizeController {
    fn add_new_agent_entries(&self, agents: &SpeciatedAgentQueue, current_generation: u32) {}

    fn add_new_maze_entries(&mut self, mazes: &SpeciatedMazeQueue, current_generation: u32) {
        let empty_species: Vec<&MazeSpecies> = mazes
            .iter_species()
            .filter(|s| s.maze_queue.len() == 1)
            .collect();

        for s in empty_species {
            let existing_entry = self
                .maze_entries
                .iter()
                .find(|&e| e.active && e.from_species_id == s.id);

            if existing_entry.is_none() {
                // checks 5 last entries
                if s.statistics
                    .stagnant_for_generations(50 / config::MCC.varied_size_generation_jumps as u32)
                {
                    // find non-stagnant species
                    let good_species: Vec<&MazeSpecies> = mazes
                        .iter_species()
                        .filter(|&s| {
                            !s.statistics.stagnant_for_generations(
                                50 / config::MCC.varied_size_generation_jumps as u32,
                            )
                        })
                        .collect();

                    let mut best_found: Option<&MazeSpecies> = Option::None;

                    for g in good_species {
                        let existing = self
                            .maze_entries
                            .iter()
                            .find(|e| e.active && e.from_species_id == g.id);

                        if existing.is_none() {
                            if best_found.is_none() {
                                best_found = Some(g);
                            } else if best_found.unwrap().statistics.get_overall_score()
                                < g.statistics.get_overall_score()
                            {
                                best_found = Some(g);
                            }
                        }
                    }

                    if best_found.is_some() {
                        let new_entry = VariedSizeEntry {
                            active: false,
                            is_initiated: false,
                            from_species_id: s.id,
                            to_species_id: best_found.unwrap().id,
                            population_amount_borrowed: 10,
                            initiated_at_generation: current_generation,
                        };

                        self.maze_entries.push(new_entry);
                    }
                }
            }
        }
    }

    fn update_agents(&mut self, agents: &SpeciatedAgentQueue, current_generation: u32) {
        for agent_entry in self.agent_entries.iter_mut() {
            if !agent_entry.is_initiated && !agent_entry.active {
                // update species

                agent_entry.active = true;
                agent_entry.is_initiated = true;
            } else if agent_entry.active
                && agent_entry.initiated_at_generation + 50 < current_generation
            {
                agent_entry.active = false;

                // update limit of to species
                // update limit of from species
            }
        }
    }

    fn update_mazes(&mut self, mut mazes: &mut SpeciatedMazeQueue, current_generation: u32) {
        for maze_entry in self.maze_entries.iter_mut() {
            if !maze_entry.is_initiated && !maze_entry.active {
                maze_entry.active = true;
                maze_entry.is_initiated = true;

                let mut valid_to = true;
                let mut valid_from = true;

                {
                    let mut to_species = mazes
                        .iter_species_mut()
                        .find(|m| m.id == maze_entry.to_species_id);

                    if to_species.is_some() {
                        to_species.unwrap().maze_queue.max_items_limit +=
                            maze_entry.population_amount_borrowed;
                    } else {
                        valid_to = false;
                    }
                }

                {
                    let mut from_species = mazes
                        .iter_species_mut()
                        .find(|m| m.id == maze_entry.from_species_id);

                    if from_species.is_some() && valid_to {
                        from_species.unwrap().maze_queue.max_items_limit -=
                            maze_entry.population_amount_borrowed;
                    } else {
                        valid_from = false;
                    }
                }

                if valid_to && !valid_from {
                    let mut to_species = mazes
                        .iter_species_mut()
                        .find(|m| m.id == maze_entry.to_species_id);

                    if to_species.is_some() {
                        to_species.unwrap().maze_queue.max_items_limit -=
                            maze_entry.population_amount_borrowed;
                    }
                }
            } else if maze_entry.active
                && maze_entry.initiated_at_generation + 50 < current_generation
            {
                maze_entry.active = false;

                let mut valid_to = true;
                let mut valid_from = true;

                {
                    let mut to_species = mazes
                        .iter_species_mut()
                        .find(|m| m.id == maze_entry.to_species_id);

                    if to_species.is_some() {
                        to_species.unwrap().maze_queue.max_items_limit -=
                            maze_entry.population_amount_borrowed;
                    } else {
                        valid_to = false;
                    }
                }

                {
                    let mut from_species = mazes
                        .iter_species_mut()
                        .find(|m| m.id == maze_entry.from_species_id);

                    if from_species.is_some() && valid_to {
                        from_species.unwrap().maze_queue.max_items_limit +=
                            maze_entry.population_amount_borrowed;
                    } else {
                        valid_from = false;
                    }
                }

                if valid_to && !valid_from {
                    let mut to_species = mazes
                        .iter_species_mut()
                        .find(|m| m.id == maze_entry.to_species_id);

                    if to_species.is_some() {
                        to_species.unwrap().maze_queue.max_items_limit +=
                            maze_entry.population_amount_borrowed;
                    }
                }
            }
        }
    }

    pub fn update_population_properties(
        &mut self,
        agents: &mut SpeciatedAgentQueue,
        mazes: &mut SpeciatedMazeQueue,
        current_generation: u32,
    ) {
        self.add_new_agent_entries(agents, current_generation);
        self.add_new_maze_entries(mazes, current_generation);

        self.update_agents(agents, current_generation);
        self.update_mazes(mazes, current_generation);
    }
}

pub fn run_varied_size_experiment(analyzer: &mut Analyzer) {
    println!("Running varied size experiment");

    let seeds = neatns::generate_seeds();

    let mut agents = SpeciatedAgentQueue::new(seeds.agents);
    let mut mazes = SpeciatedMazeQueue::new(seeds.mazes);

    let mut varied_size_controller = VariedSizeController {
        agent_entries: vec![],
        maze_entries: vec![],
    };

    for generation in 0..config::MCC.generations {
        let mut agent_children = agents.get_children();
        let mut maze_children = mazes.get_children();

        simulate_many(&mut agent_children, &mut maze_children);

        for child in agent_children.iter() {
            if child.viable {
                agents.push(child.clone())
            }
        }

        for child in maze_children.iter() {
            if child.viable {
                mazes.push(child.clone())
            }
        }

        let generation_stats = generate_generation_stats_s(generation as u32, &agents, &mazes);
        analyzer.add_generation_stats(&generation_stats);

        if generation % config::MCC.varied_size_generation_jumps == 0 && generation != 0 {
            agents.save_state();
            mazes.save_state();

            if generation > config::MCC.varied_size_minimum_generation {
                varied_size_controller.update_population_properties(
                    &mut agents,
                    &mut mazes,
                    generation as u32,
                );
            }

            println!(
                "Generation: {} ",
                generation_stats.to_whitespace_separated_string(),
            );
            print_stats(&agents, &mazes);
        }
    }
}
