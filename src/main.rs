extern crate elapsed;
//extern crate envconfig;
//#[macro_use]
//extern crate envconfig_derive;
//extern crate lazy_static;

//use std::{env, thread};
use std::path::Path;

//use elapsed::measure_time;
//use envconfig::Envconfig;
//use lazy_static::*;

use crate::general::{Orientation, OpeningLocation};
use crate::maze_genotype::{MazeGenome, PathGene, WallGene};

mod evolution;
mod general;
mod maze_genotype;
mod maze_phenotype;
mod navigator;
mod simulator;
mod testing;
mod visualization;

/*#[derive(Envconfig)]
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
    pub static ref MCC: MCCOptions = MCCOptions::init().unwrap();
    pub static ref MAZE: MazeMutationOptions = MazeMutationOptions::init().unwrap();
    pub static ref NAVIGATOR: NavigatorMutationOptions = NavigatorMutationOptions::init().unwrap();
}*/

/*fn run_mcc_plain() {
    let mazes = maze::generate_random_mazes(options.maze_population_capacity);
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
    }
}*/

fn main() {
    //&MCC;
    //&MAZE;
    //&NAVIGATOR;

    test();
    /*let amount = 10000;
    let mut total_time_without_threading = 0;
    let mut total_time_with_threading = 0;
    let start = std::time::Instant::now();

    for i in 0..amount {
        for i in 0..100 {
            test();
        }
    }
    let stop = start.elapsed().as_micros();

    println!("without threading total elapsed {}", stop);
    println!("average {}", stop / amount);


    let mut children = vec![];

    let start_threading = std::time::Instant::now();

    for i in 0..amount {
        children.push(thread::spawn(move || {
            for i in 0..100 {
                test();
            }
        }));
    }

    for child in children {
        let _ = child.join();
    }

    let stop_threading = start_threading.elapsed().as_micros();

    println!("with threading total elapsed {}", stop_threading);
    println!("average {}", stop_threading / amount);

    println!("ratio: {}", (stop as f64 / stop_threading as f64));*/
}

fn test() {
    let p1 = PathGene::new(2, 3, Orientation::Vertical);
    let p2 = PathGene::new(4, 1, Orientation::Vertical);
    let p3 = PathGene::new(5, 5, Orientation::Vertical);
    let p4 = PathGene::new(6, 14, Orientation::Vertical);
    let p5 = PathGene::new(10, 18, Orientation::Vertical);
    let p6 = PathGene::new(24, 8, Orientation::Vertical);
    let p7 = PathGene::new(34, 22, Orientation::Vertical);
    let p8 = PathGene::new(10, 26, Orientation::Vertical);
    let p9 = PathGene::new(20, 36, Orientation::Vertical);

    let w1 = WallGene::new(0.278, 0.469, Orientation::Horizontal, OpeningLocation::East);
    let w2 = WallGene::new(0.400, 0.6, Orientation::Vertical, OpeningLocation::North);
    let w3 = WallGene::new(0.245, 0.6, Orientation::Horizontal, OpeningLocation::North);
    let w4 = WallGene::new(0.400, 0.5, Orientation::Vertical, OpeningLocation::East);

    let mazey_boi = MazeGenome::new(100, 100, vec![p1, p2, p3, p4, p5, p6, p7, p8, p9], vec![w1]);
    //let mazey_boi2 = MazeGenome::new(10, 10, vec![p1, p2, p3, p4, p5], vec![w1, w2]);
    //let mazey_boi = MazeGenome::new(4, 4, vec![p1], vec![w2]);

    let start = std::time::Instant::now();
    let phenome = mazey_boi.to_phenotype();

    phenome.visualize(Path::new("testing/test.png"));

    println!("single used {:?}", start.elapsed());
}
