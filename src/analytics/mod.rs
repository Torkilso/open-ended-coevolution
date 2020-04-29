use std::fs;
use std::io::prelude::*;

use crate::analytics::text::write_text_to_file;
use crate::maze::maze_genotype::MazeGenome;
use crate::mcc::agent::agent_queue::AgentQueue;
use crate::mcc::agent::mcc_agent::MCCAgent;
use crate::mcc::maze::maze_queue::MazeQueue;
use crate::neatns::agent::Agent;
use crate::neatns::Seeds;
use crate::simulator::{simulate_single_mcc, simulate_single_neatns};
use crate::visualization::maze::visualize_maze;
use crate::visualization::simulation::visualize_agent_path;
use crate::visualization::VisualizationOptions;
use std::borrow::Borrow;
use std::fs::OpenOptions;
use std::path::Path;

mod image;
mod text;

#[derive(Debug, Clone)]
pub struct GenerationStatistics {
    generation: u32,
    average_maze_size: f64,
    largest_maze_size: u32,
    smallest_maze_size: u32,
    average_maze_path_size: f64,
    largest_maze_path_size: u32,
    smallest_maze_path_size: u32,
    average_agent_size: f64,
    largest_agent_size: u32,
    smallest_agent_size: u32,
}

impl GenerationStatistics {
    pub fn new(
        generation: u32,
        average_maze_size: f64,
        largest_maze_size: u32,
        smallest_maze_size: u32,
        average_maze_path_size: f64,
        largest_maze_path_size: u32,
        smallest_maze_path_size: u32,
        average_agent_size: f64,
        largest_agent_size: u32,
        smallest_agent_size: u32,
    ) -> GenerationStatistics {
        GenerationStatistics {
            generation,
            average_maze_size,
            largest_maze_size,
            smallest_maze_size,
            average_maze_path_size,
            largest_maze_path_size,
            smallest_maze_path_size,
            average_agent_size,
            largest_agent_size,
            smallest_agent_size,
        }
    }

    pub fn to_whitespace_separated_string(&self) -> String {
        let s = format!(
            "{} {} {} {} {} {} {} {} {} {}",
            self.generation,
            self.average_maze_size,
            self.largest_maze_size,
            self.smallest_maze_size,
            self.average_maze_path_size,
            self.largest_maze_path_size,
            self.smallest_maze_path_size,
            self.average_agent_size,
            self.largest_agent_size,
            self.smallest_agent_size,
        );
        s
    }
}

#[derive(Debug, Clone)]
pub struct Analyzer {
    results_path: String,
    generation_stats: Vec<GenerationStatistics>,
}

impl Analyzer {
    pub fn new(results_path: String) -> Analyzer {
        let result = create_directory(results_path.clone());

        if result.is_err() {
            panic!("Could not create base result directory!");
        }

        let file = OpenOptions::new()
            .create(true)
            .open(format!("{}/result.txt", results_path.clone()));

        Analyzer {
            results_path,
            generation_stats: vec![],
        }
    }

    /*pub fn visualize_seeds(&self, seeds: &Seeds) {
        let seeds_folder_path = format!("{}/seeds", self.write_base_path);
        let result = create_directory(seeds_folder_path.clone());
        if result.is_err() {
            panic!("Could not create seeds directory!");
        }

        self.visualise_mazes(&seeds.mazes, &seeds_folder_path);
        self.visualise_mazes_with_agent_path(&seeds.mazes, &seeds.agents, &seeds_folder_path);
    }*/

    pub fn add_generation_stats(&mut self, generation_statistics: &GenerationStatistics) {
        self.generation_stats.push(generation_statistics.clone())
    }

    pub fn generate_results_files(&self) {
        let path_string = format!("{}/result.txt", self.results_path);
        let path = Path::new(&path_string);

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .create(true)
            .open(path)
            .unwrap();

        for g in self.generation_stats.iter() {
            let text = g.to_whitespace_separated_string();
            if let Err(e) = writeln!(file, "{}", text) {
                eprintln!("Couldn't write to file: {}", e);
            }
        }
    }
}

fn create_directory(path: String) -> std::io::Result<()> {
    fs::create_dir_all(path)?;
    Ok(())
}
