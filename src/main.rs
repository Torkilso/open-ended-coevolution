extern crate chrono;
extern crate elapsed;
extern crate envconfig;
#[macro_use]
extern crate envconfig_derive;
extern crate lazy_static;

use chrono::Utc;

use crate::analytics::Analyzer;
use crate::visualization::maze::visualize_maze;
use crate::maze::maze_genotype::{MazeGenome, PathGene, WallGene, get_random_orientation, get_random_opening};
use crate::maze::{Orientation, OpeningLocation};
use rand::Rng;

mod analytics;
mod config;
mod maze;
mod mcc;
mod neatns;
mod simulator;
mod visualization;

fn main() {
    //create_images();
    //create_geno_to_pheno_steps();
    //return;

    let now = Utc::now().format("%m%d%H%M%S%f").to_string();
    let results_base_path = format!("./results/{}", now);

    for i in 1..config::EXPERIMENTS.batches + 1 {
        println!("Running batch {}", i);

        if config::EXPERIMENTS.run_regular_mcc {
            let results_path = format!("{}/regular_mcc", results_base_path);
            let mut analyzer = Analyzer::new(results_path, i);
            mcc::run_regular_mcc(&mut analyzer);
            //analyzer.generate_results_files();
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
            mcc::experiments::varied_size::run_varied_size_experiment(&mut analyzer);
            analyzer.generate_results_files();
        }

        if config::EXPERIMENTS.run_replacement_experiment {
            let results_path = format!("{}/replacement_experiment", results_base_path);
            let mut analyzer = Analyzer::new(results_path, i);
            mcc::experiments::species_replacement::run_replacement_experiment(&mut analyzer);
            analyzer.generate_results_files();
        }
    }
}

fn create_geno_to_pheno_steps() {
    let initial_orientation = Orientation::Horizontal;
    let wall_gene = WallGene::new(
        0.32,
        0.4,
        Orientation::Vertical,
        OpeningLocation::East,
    );

    let wall_gene_2 = WallGene::new(
        0.6,
        0.8,
        Orientation::Horizontal,
        OpeningLocation::South,
    );

    let path_gene = PathGene::new(
        2, 3,
    );

    let path_gene_2 = PathGene::new(
        6, 6,
    );

    let mut maze_base = MazeGenome::new(
        10,
        10,
        initial_orientation,
        vec![path_gene, path_gene_2],
        vec![wall_gene, wall_gene_2],
        1,
    );
    //let path_base: String = format!("./maze_base_c_path.png");
    //let path_base: String = format!("./maze_base_c_subdivisions.png");
    let path_base: String = format!("./maze_base_c_walls.png");
    visualize_maze(&maze_base.to_phenotype(), path_base, true);

}

fn create_images() {
    let initial_orientation = Orientation::Horizontal;
    let wall_gene = WallGene::new(
        0.32,
        0.4,
        Orientation::Vertical,
        OpeningLocation::East,
    );

    let wall_gene_2 = WallGene::new(
        0.6,
        0.8,
        Orientation::Horizontal,
        OpeningLocation::South,
    );

    let wall_gene_3 = WallGene::new(
        0.1,
        0.5,
        Orientation::Vertical,
        OpeningLocation::North,
    );

    let wall_gene_passage = WallGene::new(
        0.1,
        0.9,
        Orientation::Vertical,
        OpeningLocation::North,
    );

    let wall_gene_wall = WallGene::new(
        0.7,
        0.5,
        Orientation::Vertical,
        OpeningLocation::North,
    );

    let path_gene = PathGene::new(
        2, 3,
    );

    let path_gene_2 = PathGene::new(
        6, 6,
    );

    let mut path_gene_3 = PathGene::new(
        8, 2,
    );

    let path_gene_4 = PathGene::new(
        8, 3,
    );

    let mut maze_base = MazeGenome::new(
        10,
        10,
        initial_orientation,
        vec![path_gene, path_gene_2],
        vec![wall_gene, wall_gene_2],
        1,
    );
    let path_base: String = format!("./maze_base.png");
    visualize_maze(&maze_base.to_phenotype(), path_base, false);

    let mut maze_base_marked = MazeGenome::new(
        10,
        10,
        initial_orientation,
        vec![path_gene, path_gene_2],
        vec![wall_gene, wall_gene_2],
        1,
    );
    let path_base_marked: String = format!("./maze_base_marked.png");
    visualize_maze(&maze_base_marked.to_phenotype(), path_base_marked, true);




    let mut maze_structure = MazeGenome::new(
        11,
        11,
        initial_orientation,
        vec![path_gene, path_gene_2],
        vec![wall_gene, wall_gene_2],
        1,
    );
    let path_structure: String = format!("./maze_structure.png");
    visualize_maze(&maze_structure.to_phenotype(), path_structure, false);



    let mut maze_path_gene = MazeGenome::new(
        10,
        10,
        initial_orientation,
        vec![path_gene, path_gene_2, path_gene_3],
        vec![wall_gene, wall_gene_2],
        1,
    );
    let path_path_gene: String = format!("./maze_path_gene.png");
    visualize_maze(&maze_path_gene.to_phenotype(), path_path_gene, true);

    let mut maze_path_gene_update = MazeGenome::new(
        10,
        10,
        initial_orientation,
        vec![path_gene, path_gene_2, path_gene_4],
        vec![wall_gene, wall_gene_2],
        1,
    );
    let path_path_gene_update: String = format!("./maze_path_gene_update.png");
    visualize_maze(&maze_path_gene_update.to_phenotype(), path_path_gene_update, true);



    let mut maze_wall_gene_add = MazeGenome::new(
        10,
        10,
        initial_orientation,
        vec![path_gene, path_gene_2],
        vec![wall_gene, wall_gene_2, wall_gene_3],
        1,
    );
    let path_wall_gene: String = format!("./maze_wall_gene_add.png");
    visualize_maze(&maze_wall_gene_add.to_phenotype(), path_wall_gene, false);

    let mut maze_wall_gene_wall = MazeGenome::new(
        10,
        10,
        initial_orientation,
        vec![path_gene, path_gene_2],
        vec![wall_gene, wall_gene_2, wall_gene_wall],
        1,
    );
    let path_wall_gene_wall: String = format!("./maze_wall_gene_wall.png");
    visualize_maze(&maze_wall_gene_wall.to_phenotype(), path_wall_gene_wall, false);

    let mut maze_wall_gene_passage = MazeGenome::new(
        10,
        10,
        initial_orientation,
        vec![path_gene, path_gene_2],
        vec![wall_gene, wall_gene_2, wall_gene_passage],
        1,
    );
    let path_wall_gene_passage: String = format!("./maze_wall_gene_passage.png");
    visualize_maze(&maze_wall_gene_passage.to_phenotype(), path_wall_gene_passage, false);

    let mut maze_wall_gene_delete = MazeGenome::new(
        10,
        10,
        initial_orientation,
        vec![path_gene, path_gene_2],
        vec![wall_gene],
        1,
    );
    let path_wall_gene_delete: String = format!("./maze_wall_gene_delete.png");
    visualize_maze(&maze_wall_gene_delete.to_phenotype(), path_wall_gene_delete, false);


}