use crate::config;
use crate::maze::maze_genotype::{generate_random_maze, MazeGenome};
use crate::neatns::agent::Agent;
use crate::neatns::population::Population;
use crate::simulator::simulate_single_neatns;
use crate::visualization::maze::visualize_maze;
use crate::visualization::simulation::{draw_novelty_archive, visualize_agent_path};
use rand::seq::IteratorRandom;
use std::path::Path;

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
pub fn generate_seeds() -> Seeds {
    let mut mazes_fulfilling_mc: Vec<MazeGenome> = vec![];
    let mut agents_fulfilling_mc: Vec<Agent> = vec![];

    while mazes_fulfilling_mc.len() < config::MCC.maze_seed_amount {
        let mut generations = 0;

        let maze = generate_random_maze(10, 10);
        let maze_phenotype = maze.to_phenotype();

        let mut population = Population::new(config::NEATNS.population_size, 10, 2);

        while generations < config::MCC.find_seed_generation_limit {
            population.evolve();
            let result = population.run_simulation_and_update_fitness(&maze_phenotype);

            if result.is_some() {
                mazes_fulfilling_mc.push(maze.clone());
                agents_fulfilling_mc.push(result.unwrap().clone());
                break;
            }

            //println!("Fining mazes and agents, generation: {}", generations);
            generations += 1;
        }

        println!(
            "Mazes found: {}, agents found: {}",
            mazes_fulfilling_mc.len(),
            agents_fulfilling_mc.len()
        );
    }

    while agents_fulfilling_mc.len() < config::MCC.agent_seed_amount {
        let mut generations = 0;

        let maze = mazes_fulfilling_mc
            .iter()
            .choose(&mut rand::thread_rng())
            .unwrap();
        let maze_phenotype = maze.to_phenotype();

        let mut population = Population::new(config::NEATNS.population_size, 10, 2);

        while generations < config::MCC.find_seed_generation_limit {
            population.evolve();
            let result = population.run_simulation_and_update_fitness(&maze_phenotype);

            if result.is_some() {
                agents_fulfilling_mc.push(result.unwrap().clone());
                break;
            }

            //println!("Finding agents, generation: {}", generations);
            generations += 1;
        }
        println!(
            "Mazes found: {}, agents found: {}",
            mazes_fulfilling_mc.len(),
            agents_fulfilling_mc.len()
        );
    }

    Seeds::new(mazes_fulfilling_mc, agents_fulfilling_mc)
}

#[allow(dead_code)]
pub fn test_single_agent() {
    let mut population = Population::new(config::NEAT.population_size, 10, 2);
    for _ in 0..500 {
        population.evolve();
    }
    let agent = population.random_agent();
    let maze = generate_random_maze(5, 5);
    let maze_phenotype = maze.to_phenotype();
    visualize_maze(&maze_phenotype, Path::new("./test.png"), true);
    if agent.is_some() {
        println!("agent: {}", agent.unwrap());
        //let result = simulate_single_neatns(agent.unwrap(), &maze_phenotype, true);
        //visualize_agent_path(&maze_phenotype, &result, Path::new("./test.png"));
    }
}

#[allow(dead_code)]
pub fn test_with_population() {
    let mut generations = 0;

    let maze = generate_random_maze(10, 10);
    let maze_phenotype = maze.to_phenotype();

    let mut population = Population::new(config::NEATNS.population_size, 10, 2);

    while generations < config::MCC.find_seed_generation_limit {
        population.evolve();
        let result = population.run_simulation_and_update_fitness(&maze_phenotype);

        if result.is_some() {
            println!("Found solution!!!!");

            let simulation = simulate_single_neatns(&result.unwrap(), &maze_phenotype, true);
            visualize_agent_path(&maze_phenotype, &simulation);

            break;
        }

        println!("Generation {}", generations);
        generations += 1;
    }
    draw_novelty_archive(&maze_phenotype, &population.novelty_archive);
}
