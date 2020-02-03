use crate::maze::{MazeGenome, PathGene, WallGene, Orientation};

#[macro_use]
extern crate envconfig_derive;
extern crate envconfig;
extern crate lazy_static;

use std::env;
use envconfig::Envconfig;
use lazy_static::*;
use std::borrow::Borrow;

mod maze;
mod navigator;
mod testing;
mod evolution;
mod simulator;
mod visualization;

#[derive(Envconfig)]
pub struct MazeMutationOptions {
    #[envconfig(from = "mutate_structure", default = "0.5")]
    mutate_structure: f32,

    #[envconfig(from = "add_wall", default = "0.1")]
    add_wall: f32,

    #[envconfig(from = "delete_wall", default = "0.001")]
    delete_wall: f32,

    #[envconfig(from = "add_waypoint", default = "0.1")]
    add_waypoint: f32,

    #[envconfig(from = "delete_waypoint", default = "0.001")]
    delete_waypoint: f32,
}

#[derive(Envconfig)]
pub struct NavigatorMutationOptions {
    #[envconfig(from = "mutate_weight", default = "0.001")]
    mutate_weight: f32,

    #[envconfig(from = "add_connection", default = "0.001")]
    add_connection: f32,

    #[envconfig(from = "add_neuron", default = "0.001")]
    add_neuron: f32,

    #[envconfig(from = "delete_neuron", default = "0.001")]
    delete_neuron: f32,
}

#[derive(Envconfig)]
pub struct MCCOptions {
    #[envconfig(from = "generations", default = "10")]
    pub generations: i32,

    #[envconfig(from = "maze_population_capacity", default = "250")]
    maze_population_capacity: i32,

    #[envconfig(from = "maze_seed_amount", default = "20")]
    pub maze_seed_amount: i32,

    #[envconfig(from = "navigator_population_capacity", default = "250")]
    navigator_population_capacity: usize,

    #[envconfig(from = "navigator_seed_amount", default = "20")]
    pub navigator_seed_amount: i32,
}

lazy_static! {
    pub static ref mcc_options: MCCOptions = MCCOptions::init().unwrap();
    pub static ref maze_options: MazeMutationOptions = MazeMutationOptions::init().unwrap();
    pub static ref navigator_options: NavigatorMutationOptions = NavigatorMutationOptions::init().unwrap();
}

fn run_mcc_plain() {
    /*let mazes = maze::generate_random_mazes(options.maze_population_capacity);
    let viable_navigators = evolution::evolve_seed_navigators(&mazes, options.navigator_seed_amount);

    for x in 0..options.generations {
        println!("Generation {}", x);

        let parents = evolution::dequeue(&viable_navigators, 10);
        let children = evolution::reproduce_navigators(parents);
        evolution::enqueue(&viable_navigators, parents);

        //

        let survivors = simulator::evaluate_navigators(&children, &mazes);
        evolution::enqueue(&viable_navigators, &survivors);

        if viable_navigators.len() > options.navigator_population_capacity {
            let amount_to_remove = viable_navigators.len() - options.navigator_population_capacity;
            evolution::remove_oldest(&viable_navigators, amount_to_remove)
        }
    }*/
}


fn main() {
    let a = &mcc_options;
    let b = &maze_options;
    let c = &navigator_options;

    test();
}

fn test() {
    let p1 = PathGene::new(2, 3, Orientation::Vertical);
    let p2 = PathGene::new(5, 6, Orientation::Vertical);

    let w1 = WallGene::new(0.278, 0.855);
    let w2 = WallGene::new(400.0, 0.808);

    let mazey_boi = MazeGenome::new(10, 10, vec![p1, p2], vec![w1, w2]);

    let pheno = mazey_boi.to_phenotype();
    pheno.visualize();
}
