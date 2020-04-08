extern crate elapsed;
extern crate envconfig;
#[macro_use]
extern crate envconfig_derive;
extern crate lazy_static;

use std::path::Path;

use crate::maze::maze_genotype::generate_random_maze;
use crate::visualization::maze::visualize_maze;

mod config;
mod maze;
mod mcc;
mod neatns;
mod simulator;
mod visualization;

fn main() {
    let mut maze = generate_random_maze(10, 10);

    visualize_maze(&maze.to_phenotype(), Path::new("before.png"), false);
    maze.add_waypoint();
    maze.add_waypoint();
    maze.add_waypoint();
    maze.add_waypoint();
    maze.add_waypoint();
    maze.add_waypoint();
    maze.add_waypoint();
    maze.add_waypoint();
    maze.add_waypoint();
    maze.add_waypoint();
    maze.add_wall();
    maze.add_wall();
    maze.add_waypoint();
    maze.add_wall();
    maze.add_waypoint();
    visualize_maze(&maze.to_phenotype(), Path::new("after.png"), false);

    mcc::run_without_speciation();
}
