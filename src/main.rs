extern crate elapsed;
extern crate envconfig;
#[macro_use]
extern crate envconfig_derive;
extern crate lazy_static;

use std::{env, thread};
use std::path::Path;

use elapsed::measure_time;
use lazy_static::*;

use crate::maze::maze_genotype::{generate_random_maze, MazeGenome, PathGene, WallGene};
use crate::maze::maze_phenotype::MazePhenotype;
use crate::simulator::radar::get_radar_values;
use crate::visualization::maze::visualize_maze;

mod config;
mod maze;
mod mcc;
mod simulator;
mod neatns;
mod network;
mod generic_neat;
mod visualization;

fn main() {
    let maze = generate_random_maze(5, 5);
    println!("{}", maze);
    let maze_phenotype = maze.to_phenotype();
    visualize_maze(&maze_phenotype, Path::new("./test.png"), true);

    //let val = get_radar_values(&run_state, &maze_phenotype);

    //let seeds = neatns::generate_seeds();
}
