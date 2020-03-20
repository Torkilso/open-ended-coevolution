use std::borrow::{Borrow, BorrowMut};
use std::path::Path;

use crate::config;
use crate::maze::maze_genotype::{generate_random_maze, MazeGenome};
use crate::neatns::agent::Agent;
use crate::neatns::population::Population;
use crate::simulator::simulate_run;
use crate::visualization::maze::visualize_maze;
use crate::visualization::simulation::visualize_agent_path;

pub mod agent;
mod novelty_archive;
mod novelty_item;
pub mod population;
mod species;

pub struct Seeds {
    mazes: Vec<MazeGenome>,
    agents: Vec<Agent>,
}

impl Seeds {
    pub fn new(mazes: Vec<MazeGenome>, agents: Vec<Agent>) -> Seeds {
        Seeds { mazes, agents }
    }
}

// generate seeds for mcc with neatns.
// outputs a set of agents and a set of mazes that fulfill the mc.
pub fn generate_seeds() -> Seeds {
    let mut mazes_fulfilling_mc: Vec<MazeGenome> = vec![];
    let mut agents_fulfilling_mc: Vec<Agent> = vec![];

    let mut population = Population::new(config::NEAT.population_size, 10, 2);
    for _ in 0..500 {
        population.evolve();
    }
    let agent = population.random_agent();
    let maze = generate_random_maze(5, 5);
    let maze_phenotype = maze.to_phenotype();
    //visualize_maze(&maze_phenotype, Path::new("./test.png"), true);
    if agent.is_some() {
        println!("agent: {}", agent.unwrap());
        let result = simulate_run(agent.unwrap(), &maze_phenotype, true);
        visualize_agent_path(&maze_phenotype, &result, Path::new("./test.png"));
    }

    /*while mazes_fulfilling_mc.len() < config::MCC.maze_seed_amount {
        let mut generations = 0;

        let maze = generate_random_maze(5, 5);
        let maze_phenotype = maze.to_phenotype();
        visualize_maze(&maze_phenotype, Path::new("./test.png"), true);

        let mut population = Population::new(config::NEATNS.population_size, 10, 2);

        //mazes_fulfilling_mc.push(maze.clone());

        while generations < config::MCC.find_seed_generation_limit {
            population.evolve();
            let result = population.run_simulation_and_update_fitness(&maze_phenotype);

            if result.is_some() {
                mazes_fulfilling_mc.push(maze.clone());
                agents_fulfilling_mc.push(result.unwrap());
                break;
            }

            println!("Generation {}", generations);
            generations += 1;
        }
    }*/

    //while agents_fulfilling_mc.len() < config::MCC.agent_seed_amount {}

    Seeds::new(mazes_fulfilling_mc, agents_fulfilling_mc)
}
