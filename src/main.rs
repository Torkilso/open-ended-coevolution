extern crate elapsed;
extern crate envconfig;
#[macro_use]
extern crate envconfig_derive;
extern crate lazy_static;

use std::path::Path;
use std::{env, thread};

use elapsed::measure_time;
use envconfig::Envconfig;
use lazy_static::*;

use crate::agent::agent::Agent;
use crate::common::{OpeningLocation, Orientation};
use crate::maze::maze_genotype::{MazeGenome, PathGene, WallGene};
use crate::maze::maze_phenotype::MazePhenotype;
use crate::simulator::{get_sensor_value, simulate_navigator_in_maze};

mod agent;
mod common;
mod config;
mod maze;
mod mcc;
mod simulator;

fn main() {
    let maze = generate_test_maze();
    let agent = Agent::new();

    simulate_navigator_in_maze(&agent, &maze);
    maze.visualize(Path::new("testing/test.png"));
}

fn generate_test_maze() -> MazePhenotype {
    let p1 = PathGene::new(4, 3);
    let p2 = PathGene::new(6, 1);
    let p3 = PathGene::new(9, 6);
    let p4 = PathGene::new(1, 8);
    let p5 = PathGene::new(8, 8);
    let p6 = PathGene::new(24, 8);
    let p7 = PathGene::new(34, 22);
    let p8 = PathGene::new(10, 26);
    let p9 = PathGene::new(20, 36);

    let w1 = WallGene::new(
        0.278,
        0.469,
        Orientation::Horizontal,
        OpeningLocation::South,
    );
    let w2 = WallGene::new(0.6, 0.6, Orientation::Vertical, OpeningLocation::North);
    let w3 = WallGene::new(0.245, 0.6, Orientation::Horizontal, OpeningLocation::South);
    let w4 = WallGene::new(0.400, 0.5, Orientation::Vertical, OpeningLocation::East);

    let mazey_boi = MazeGenome::new(
        10,
        10,
        Orientation::Horizontal,
        vec![p1, p2, p3, p4],
        vec![w1, w2, w3, w4],
    );
    //let mazey_boi2 = MazeGenome::new(10, 10, vec![p1, p2, p3, p4, p5], vec![w1, w2]);
    //let mazey_boi = MazeGenome::new(4, 4, vec![p1], vec![w2]);

    let start = std::time::Instant::now();
    let phenotype = mazey_boi.to_phenotype();

    phenotype
}

/*fn test_threading() {
    let amount = 10000;
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

    println!("ratio: {}", (stop as f64 / stop_threading as f64));
}*/
