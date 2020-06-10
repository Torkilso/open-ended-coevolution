use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

use crate::mcc::agent::agent_queue::AgentQueue;
use crate::mcc::agent::speciated_agent_queue::SpeciatedAgentQueue;
use crate::mcc::maze::maze_queue::MazeQueue;
use crate::mcc::maze::speciated_maze_queue::SpeciatedMazeQueue;
use crate::neatns::novelty_archive::euclidean_distance;
use crate::simulator::{simulate_single_mcc, SimulatorResult};
use crate::analytics::image::visualise_mazes_with_agent_path;
use crate::maze::maze_genotype::MazeGenome;
use crate::mcc::agent::mcc_agent::MCCAgent;

mod image;
mod text;

#[derive(Debug, Clone)]
pub struct GenerationStatistics {
    generation: u32,
    agent_amount: u32,
    maze_amount: u32,
    average_maze_size: f64,
    largest_maze_size: u32,
    smallest_maze_size: u32,
    average_maze_path_size: f64,
    largest_maze_path_size: u32,
    smallest_maze_path_size: u32,
    average_agent_size: f64,
    largest_agent_size: u32,
    smallest_agent_size: u32,
    average_agent_size_increase: f64,
    average_maze_size_increase: f64,
    average_maze_complexity_increase: f64,
    overall_average_agent_size_increase: f64,
    overall_average_maze_size_increase: f64,
    overall_average_maze_complexity_increase: f64,
}

impl GenerationStatistics {
    pub fn new(
        generation: u32,
        agent_amount: u32,
        maze_amount: u32,
        average_maze_size: f64,
        largest_maze_size: u32,
        smallest_maze_size: u32,
        average_maze_path_size: f64,
        largest_maze_path_size: u32,
        smallest_maze_path_size: u32,
        average_agent_size: f64,
        largest_agent_size: u32,
        smallest_agent_size: u32,
        average_agent_size_increase: f64,
        average_maze_size_increase: f64,
        average_maze_complexity_increase: f64,
        overall_average_agent_size_increase: f64,
        overall_average_maze_size_increase: f64,
        overall_average_maze_complexity_increase: f64,
    ) -> GenerationStatistics {
        GenerationStatistics {
            generation,
            agent_amount,
            maze_amount,
            average_maze_size,
            largest_maze_size,
            smallest_maze_size,
            average_maze_path_size,
            largest_maze_path_size,
            smallest_maze_path_size,
            average_agent_size,
            largest_agent_size,
            smallest_agent_size,
            average_agent_size_increase,
            average_maze_size_increase,
            average_maze_complexity_increase,
            overall_average_agent_size_increase,
            overall_average_maze_size_increase,
            overall_average_maze_complexity_increase,
        }
    }

    pub fn to_whitespace_separated_string(&self) -> String {
        let s = format!(
            "{} {} {} {:.2} {} {} {:.2} {} {} {:.2} {} {} {:.5} {:.5} {:.5} {:.5} {:.5} {:.5}",
            self.generation,
            self.agent_amount,
            self.maze_amount,
            self.average_maze_size,
            self.largest_maze_size,
            self.smallest_maze_size,
            self.average_maze_path_size,
            self.largest_maze_path_size,
            self.smallest_maze_path_size,
            self.average_agent_size,
            self.largest_agent_size,
            self.smallest_agent_size,
            self.average_agent_size_increase,
            self.average_maze_size_increase,
            self.average_maze_complexity_increase,
            self.overall_average_agent_size_increase,
            self.overall_average_maze_size_increase,
            self.overall_average_maze_complexity_increase,
        );
        s
    }
}

#[derive(Debug, Clone)]
pub struct Analyzer {
    results_path: String,
    generation_stats: Vec<GenerationStatistics>,
    batch_number: u32,
}

impl Analyzer {
    pub fn new(results_path: String, batch_number: u32) -> Analyzer {
        let result = create_directory(results_path.clone());

        if result.is_err() {
            panic!("Could not create base result directory!");
        }

        let _file = OpenOptions::new().create(true).open(format!(
            "{}/result_{}.txt",
            results_path.clone(),
            batch_number.clone()
        ));

        Analyzer {
            results_path,
            generation_stats: vec![],
            batch_number,
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

    pub fn visualize_trajectories(&self, mazes_queue: &SpeciatedMazeQueue,
                                  agents_queue: &SpeciatedAgentQueue, ) {
        let path_string = format!("{}/trajectories_{}/", self.results_path, self.batch_number);

        let result = create_directory(path_string.clone());

        if result.is_err() {
            panic!("Could not create directory in visualise_mazes_with_agent_path!");
        }


        let mut mazes = vec![];
        let mut agents = vec![];

        for m in mazes_queue.iter_individuals() {
            mazes.push(m.clone());
        }

        for a in agents_queue.iter_individuals() {
            agents.push(a.clone());
        }

        visualise_mazes_with_agent_path(&mazes, &agents, path_string);
    }

    pub fn add_generation_stats(&mut self, generation_statistics: &GenerationStatistics) {
        self.generation_stats.push(generation_statistics.clone())
    }

    pub fn generate_diversity_score_no_species(&self, agents: &AgentQueue, mazes: &MazeQueue) {
        let path_string = format!("{}/diversity_{}.txt", self.results_path, self.batch_number);
        let path = Path::new(&path_string);

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .create(true)
            .open(path)
            .unwrap();

        let text = format!("{}", calculate_diversity_score_no_species(agents, mazes));
        if let Err(e) = writeln!(file, "{}", text) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }

    pub fn generate_diversity_score(
        &self,
        agents: &SpeciatedAgentQueue,
        mazes: &SpeciatedMazeQueue,
    ) {
        let path_string = format!("{}/diversity_{}.txt", self.results_path, self.batch_number);
        let path = Path::new(&path_string);

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .create(true)
            .open(path)
            .unwrap();

        let text = format!("{}", calculate_diversity_score(agents, mazes));
        if let Err(e) = writeln!(file, "{}", text) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }

    pub fn generate_results_files(&self) {
        let path_string = format!("{}/result_{}.txt", self.results_path, self.batch_number);
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

fn calculate_diversity_score(agents: &SpeciatedAgentQueue, mazes: &SpeciatedMazeQueue) -> f64 {
    let mut results: Vec<SimulatorResult> = vec![];
    for maze in mazes.iter_individuals() {
        if maze.successful_agent_id.is_some() {
            let agent = agents
                .iter_individuals()
                .find(|agent| agent.id == maze.successful_agent_id.unwrap());

            if agent.is_some() {
                let maze_phenotype = maze.to_phenotype();

                let simulator_result = simulate_single_mcc(
                    &agent.unwrap(),
                    &maze_phenotype,
                    maze.get_solution_path_cell_length(),
                    true,
                );

                results.push(simulator_result)
            }
        }
    }

    let mut diversity = 0.0;

    for (i, result_a) in results.iter().enumerate() {
        let mut agent_diversity = 0.0;

        for (j, result_b) in results.iter().enumerate() {
            if i == j {
                continue;
            }

            let mut result_diversity = 0.0;

            for (k, point_a) in result_a.agent_path.iter().enumerate() {
                if k > result_b.agent_path.len() - 1 {
                    break;
                }

                result_diversity += euclidean_distance(point_a, &result_b.agent_path[k]);
            }

            result_diversity /=
                (result_a.agent_path.len() as f64).min(result_b.agent_path.len() as f64);

            agent_diversity += result_diversity;
        }

        agent_diversity /= (results.len() - 1) as f64;

        diversity += agent_diversity;
    }

    diversity / results.len() as f64
}

fn calculate_diversity_score_no_species(agents: &AgentQueue, mazes: &MazeQueue) -> f64 {
    let mut results: Vec<SimulatorResult> = vec![];
    for maze in mazes.iter() {
        if maze.successful_agent_id.is_some() {
            let agent = agents
                .iter()
                .find(|agent| agent.id == maze.successful_agent_id.unwrap());

            if agent.is_some() {
                let maze_phenotype = maze.to_phenotype();

                let simulator_result = simulate_single_mcc(
                    &agent.unwrap(),
                    &maze_phenotype,
                    maze.get_solution_path_cell_length(),
                    true,
                );

                results.push(simulator_result)
            }
        }
    }

    let mut diversity = 0.0;

    for (i, result_a) in results.iter().enumerate() {
        let mut agent_diversity = 0.0;

        for (j, result_b) in results.iter().enumerate() {
            if i == j {
                continue;
            }

            let mut result_diversity = 0.0;

            for (k, point_a) in result_a.agent_path.iter().enumerate() {
                if k > result_b.agent_path.len() - 1 {
                    break;
                }

                result_diversity += euclidean_distance(point_a, &result_b.agent_path[k]);
            }

            result_diversity /=
                (result_a.agent_path.len() as f64).min(result_b.agent_path.len() as f64);

            agent_diversity += result_diversity;
        }

        agent_diversity /= (results.len() - 1) as f64;

        diversity += agent_diversity;
    }

    diversity / results.len() as f64
}
