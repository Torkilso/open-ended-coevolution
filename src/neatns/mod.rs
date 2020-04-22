use crate::config;
use crate::maze::maze_genotype::{generate_random_maze, MazeGenome};
use crate::neatns::agent::Agent;
use crate::neatns::population::Population;
use crate::simulator::simulate_single_neatns;
use crate::visualization::maze::visualize_maze;
use crate::visualization::simulation::{draw_novelty_archive, visualize_agent_path};
use rand::seq::IteratorRandom;
use std::any::Any;
use std::borrow::Borrow;
use std::path::Path;
use std::thread;
use std::time::Duration;

pub(crate) mod agent;
pub(crate) mod network;
pub(crate) mod novelty_archive;
mod population;
mod species;

pub struct Seeds {
    pub mazes: Vec<MazeGenome>,
    pub agents: Vec<Agent>,
}

impl Seeds {
    pub fn new(mazes: Vec<MazeGenome>, agents: Vec<Agent>) -> Seeds {
        Seeds { mazes, agents }
    }
}

// generate seeds for mcc with neatns.
// outputs a set of agents and a set of mazes that fulfill the mc.
// TODO add threading
pub fn generate_seeds() -> Seeds {
    let mut mazes_fulfilling_mc: Vec<MazeGenome> = vec![];
    let mut agents_fulfilling_mc: Vec<Agent> = vec![];

    let mut threads = vec![];

    for i in 0..config::MCC.maze_seed_amount {
        threads.push(thread::spawn(move || {
            let mut generations = 0;

            let mut maze = generate_random_maze(10, 10, i as u32);
            let maze_phenotype = maze.to_phenotype();

            let mut population = Population::new(config::NEATNS.population_size, 10, 2);

            while generations < config::MCC.find_seed_generation_limit {
                population.evolve();
                let result = population.run_simulation_and_update_fitness(
                    &maze_phenotype,
                    maze.get_solution_path_cell_length(),
                );

                if result.is_some() {
                    let successful_agent = result.unwrap();
                    maze.successful_agent_id = Some(successful_agent.id);

                    println!("Found pair!",);

                    return Some((maze, successful_agent));
                }

                generations += 1;
            }
            None
        }));
    }

    for child in threads {
        let thread_result = child.join();
        let option = thread_result.unwrap();

        if option.is_some() {
            let result = option.unwrap();
            mazes_fulfilling_mc.push(result.0.clone());
            agents_fulfilling_mc.push(result.1.clone());
        }
    }

    let mut agent_threads = vec![];

    for maze in mazes_fulfilling_mc.clone() {
        agent_threads.push(thread::spawn(move || -> Option<Agent> {
            let mut generations = 0;

            let maze_phenotype = maze.to_phenotype();

            let mut population = Population::new(config::NEATNS.population_size, 10, 2);

            while generations < config::MCC.find_seed_generation_limit {
                population.evolve();
                let result = population.run_simulation_and_update_fitness(
                    &maze_phenotype,
                    maze.get_solution_path_cell_length(),
                );

                if result.is_some() {
                    let successful_agent = result.unwrap();
                    println!("Found agent!",);
                    return Some(successful_agent);
                }

                generations += 1;
            }
            None
        }));
    }

    for agent_thread in agent_threads {
        let thread_result = agent_thread.join();
        let option = thread_result.unwrap();

        if option.is_some() {
            let agent = option.unwrap();
            agents_fulfilling_mc.push(agent.clone());
        }
    }

    Seeds::new(mazes_fulfilling_mc, agents_fulfilling_mc)
}
