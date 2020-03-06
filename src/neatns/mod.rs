use std::borrow::{Borrow, BorrowMut};
use std::path::Path;

use crate::config;
use crate::maze::maze_genotype::{generate_random_maze, MazeGenome};
use crate::neatns::agent::Agent;

mod population;
pub mod agent;
mod species;

pub struct Seeds {
    mazes: Vec<MazeGenome>,
    agents: Vec<Agent>,
}

impl Seeds {
    pub fn new(mazes: Vec<MazeGenome>, agents: Vec<Agent>) -> Seeds {
        Seeds {
            mazes,
            agents,
        }
    }
}

// generate seeds for mcc with neatns.
// outputs a set of agents and a set of mazes that fulfill the mc.
pub fn generate_seeds() -> Seeds {
    let mut mazes_fulfilling_mc: Vec<MazeGenome> = vec![];
    let agents_fulfilling_mc: Vec<Agent> = vec![];


    /*while mazes_fulfilling_mc.len() < config::MCC.maze_seed_amount {
        // generate random maze
        let maze = generate_random_maze(5, 5);


        // initialise population with agents
        let agent_population = generate_initial_population();

        let maze_completed = false;

        while !maze_completed {
            // evolve population
            // evaluate agents in maze


            // check if completed
            // ...


            if maze_completed {
                mazes_fulfilling_mc.push(maze.clone());
            } else {
                // find novelty metric for each agent
                // set fitness to novelty metric
                // population.update_fitness()
            }
        }
    }

    while agents_fulfilling_mc.len() < config::MCC.agent_seed_amount {}*/

    Seeds::new(mazes_fulfilling_mc, agents_fulfilling_mc)
}