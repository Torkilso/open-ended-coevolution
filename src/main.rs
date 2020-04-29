extern crate chrono;
extern crate elapsed;
extern crate envconfig;
#[macro_use]
extern crate envconfig_derive;
extern crate lazy_static;

use chrono::Utc;

use crate::analytics::Analyzer;
use crate::maze::maze_genotype::{generate_random_maze, PathGene, MazeGenome, get_random_orientation, WallGene, get_random_opening};
use crate::visualization::maze::visualize_maze;
use rand::Rng;
use crate::maze::Orientation;

mod analytics;
mod config;
mod maze;
mod mcc;
mod neatns;
mod simulator;
mod visualization;

fn main() {
    let now = Utc::now().format("%m%d%H%M%S%f").to_string();
    let results_base_path = format!("./results/{}", now);

    /*let path: String = format!("./maze.png");

    let mut rng = rand::thread_rng();

    let initial_orientation = Orientation::Horizontal;
    let wall_gene = WallGene::new(
        rng.gen::<f64>(),
        rng.gen::<f64>(),
        get_random_orientation(rng.gen::<f32>()),
        get_random_opening(rng.gen::<f32>()),
    );

    let path_gene = PathGene::new(
        2, 3,
    );

    let path_gene_2 = PathGene::new(
        8, 0,
    );


    let path_genes = vec![path_gene, path_gene_2];
    let wall_genes = vec![wall_gene];


    let mut new_maze = MazeGenome::new(
        10,
        10,
        initial_orientation,
        path_genes,
        wall_genes,
        1,
    );

    visualize_maze(&new_maze.to_phenotype(), path, false);

    println!("juntures: {}", new_maze.get_amount_of_junctures());*/

    for i in 1..config::EXPERIMENTS.batches + 1 {
        println!("Running batch {}", i);

        if config::EXPERIMENTS.run_regular_mcc {
            let results_path = format!("{}/regular_mcc", results_base_path);
            let mut analyzer = Analyzer::new(results_path, i);
            mcc::run_regular_mcc(&mut analyzer);
            analyzer.generate_results_files()
        }

        if config::EXPERIMENTS.run_regular_speciated_mcc {
            let results_path = format!("{}/regular_speciated_mcc", results_base_path);
            let mut analyzer = Analyzer::new(results_path, i);
            mcc::run_regular_speciated_mcc(&mut analyzer);
            analyzer.generate_results_files()
        }

        if config::EXPERIMENTS.run_varied_size_experiment {
            let results_path = format!("{}/varied_size_experiment", results_base_path);
            let mut analyzer = Analyzer::new(results_path, i);
            mcc::experiments::run_varied_size_experiment(&mut analyzer);
            analyzer.generate_results_files();
        }

        if config::EXPERIMENTS.run_gradual_replacement_experiment {
            let results_path = format!("{}/gradual_replacement_experiment", results_base_path);
            let mut analyzer = Analyzer::new(results_path, i);
            mcc::experiments::run_gradual_replacement_experiment(&mut analyzer);
            analyzer.generate_results_files();
        }

        if config::EXPERIMENTS.run_sudden_replacement_experiment {
            let results_path = format!("{}/sudden_replacement_experiment", results_base_path);
            let mut analyzer = Analyzer::new(results_path, i);
            mcc::experiments::run_sudden_replacement_experiment(&mut analyzer);
            analyzer.generate_results_files();
        }
    }
}
