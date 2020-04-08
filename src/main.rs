extern crate elapsed;
extern crate envconfig;
#[macro_use]
extern crate envconfig_derive;
extern crate lazy_static;

use std::path::Path;
use std::{env, thread};

use elapsed::measure_time;
use lazy_static::*;

use crate::maze::maze_genotype::{generate_random_maze, MazeGenome, PathGene, WallGene};
use crate::maze::maze_phenotype::MazePhenotype;
use crate::simulator::radar::get_radar_values;

use crate::visualization::maze::visualize_maze;

mod config;
mod maze;
mod mcc;
mod neatns;
mod simulator;
mod visualization;

fn main() {
    let mut maze = generate_random_maze(5, 5);

    visualize_maze(&maze.to_phenotype(), Path::new("before.png"), false);
    maze.increase_size();
    visualize_maze(&maze.to_phenotype(), Path::new("after.png"), false);

    //mcc::run_without_speciation();
}
