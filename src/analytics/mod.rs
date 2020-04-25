use std::fs;
use std::io::prelude::*;

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
use crate::analytics::text::write_text_to_file;

mod image;
mod text;

#[derive(Debug, Clone)]
pub struct Analyzer {
    write_base_path: String,
}

impl Analyzer {
    pub fn new(write_base_path: String) -> Analyzer {
        let result = create_directory(write_base_path.clone());

        if result.is_err() {
            panic!("Could not create base result directory!");
        }

        Analyzer { write_base_path }
    }

    pub fn visualize_seeds(&self, seeds: &Seeds, experiment_name: &str) {
        let seeds_folder_path = format!("{}/{}/seeds", self.write_base_path, experiment_name);

        let result = create_directory(seeds_folder_path.clone());

        if result.is_err() {
            panic!("Could not create seeds directory!");
        }

        self.visualise_mazes(&seeds.mazes, &seeds_folder_path);
        self.visualise_mazes_with_agent_path(&seeds.mazes, &seeds.agents, &seeds_folder_path);
    }

    pub fn visualise_regular_mcc_results(
        &self,
        mazes: &MazeQueue,
        agents: &AgentQueue,
        experiment_name: &str,
    ) {
        let text_path = format!(
            "{}/{}/end_result.txt",
            self.write_base_path, experiment_name
        );

        let text = format!(
            "End results\n\nAverage maze size: {}\nLargest maze: {}x{}",
            mazes.get_average_size(),
            mazes.get_largest().width,
            mazes.get_largest().width
        ).as_bytes();

        write_text_to_file(text_path, text);

        let end_result_folder_path =
            format!("{}/{}/end_result", self.write_base_path, experiment_name);
        let result = create_directory(end_result_folder_path.clone());

        if result.is_err() {
            panic!("Could not create end results directory!");
        }

        /*for (i, maze) in mazes.iter().enumerate() {
            if maze.successful_agent_id.is_some() {
                let agent = agents
                    .iter()
                    .find(|agent| agent.id == maze.successful_agent_id.unwrap());

                if agent.is_some() {
                    let maze_file_name = format!("maze_solution_{}.png", i);
                    self.visualise_maze_with_agent_path(
                        maze,
                        &agent.unwrap(),
                        end_result_folder_path.clone(),
                        maze_file_name,
                        false
                    )
                }
            }
        }*/

        let complex_maze = mazes.get_maze_with_most_wall_genes();

        if complex_maze.successful_agent_id.is_some() {
            let agent = agents
                .iter()
                .find(|agent| agent.id == complex_maze.successful_agent_id.unwrap());

            if agent.is_some() {
                println!("generating video series");

                let maze_file_name = format!("");
                let folder_path = format!("{}/video", end_result_folder_path);
                let result = create_directory(folder_path.clone());
                if result.is_err() {
                    panic!("Could not create video directory!");
                }

                self.visualise_maze_with_agent_path(
                    &complex_maze,
                    &agent.unwrap(),
                    folder_path,
                    maze_file_name,
                    true,
                )
            }
        }
    }
}

fn create_directory(path: String) -> std::io::Result<()> {
    fs::create_dir_all(path)?;
    Ok(())
}
