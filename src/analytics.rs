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

    pub fn visualize_seeds(&self, seeds: &Seeds) {
        let seeds_folder_path = format!("{}/seeds", self.write_base_path);

        let result = create_directory(seeds_folder_path.clone());

        if result.is_err() {
            panic!("Could not create seeds directory!");
        }

        self.visualise_mazes(&seeds.mazes, &seeds_folder_path);
        self.visualise_mazes_with_agent_path(&seeds.mazes, &seeds.agents, &seeds_folder_path);
    }

    pub fn visualise_regular_mcc_results(&self, mazes: &MazeQueue, agents: &AgentQueue) {
        let text_path = format!("{}/end_result.txt", self.write_base_path);
        let text: String = format!(
            "End results\n\nAverage maze size: {}\nLargest maze: {}",
            mazes.get_average_size(),
            mazes.get_largest()
        );

        write_text_to_file(text_path, text);

        let end_result_folder_path = format!("{}/end_result", self.write_base_path);
        let result = create_directory(end_result_folder_path.clone());

        if result.is_err() {
            panic!("Could not create end results directory!");
        }

        for (i, maze) in mazes.iter().enumerate().take(10) {
            let maze_path = format!("{}/maze_{}.png", end_result_folder_path, i);
            let maze_phenotype = maze.to_phenotype();

            visualize_maze(&maze_phenotype, maze_path, false);

            let maze_solution_path = format!("{}/maze_solution_{}.png", end_result_folder_path, i);

            self.visualise_maze_with_agent_path(
                maze,
                agents,
                maze_solution_path,
            )
        }

        let largest_maze_path = format!("{}/largest_maze.png", end_result_folder_path);

        self.visualise_maze(&mazes.get_largest(), &largest_maze_path);

        let largest_maze_solution_path =
            format!("{}/largest_maze_solution.png", end_result_folder_path);
        self.visualise_maze_with_agent_path(
            &mazes.get_largest(),
            agents,
            largest_maze_solution_path,
        )
    }

    pub fn visualise_mazes(&self, mazes: &Vec<MazeGenome>, path: &String) {
        for (i, maze) in mazes.iter().enumerate() {
            let maze_seed_path = format!("{}/maze_{}.png", path, i);
            let maze_phenotype = maze.to_phenotype();

            visualize_maze(&maze_phenotype, maze_seed_path, false);
        }
    }

    pub fn visualise_maze(&self, maze: &MazeGenome, path: &String) {
        let maze_seed_path = format!("{}", path);
        let maze_phenotype = maze.to_phenotype();

        visualize_maze(&maze_phenotype, maze_seed_path, false);
    }

    pub fn visualise_mazes_with_agent_path(
        &self,
        mazes: &Vec<MazeGenome>,
        agents: &Vec<Agent>,
        seeds_folder_path: &String,
    ) {
        for (i, maze) in mazes.iter().enumerate() {
            let maze_seed_solution_path = format!("{}/maze_{}_solution.png", seeds_folder_path, i);
            let maze_phenotype = maze.to_phenotype();

            let mut agent_index: Option<u32> = None;

            for (j, agent) in agents.iter().enumerate() {
                if maze.successful_agent_id.is_some()
                    && agent.id == maze.successful_agent_id.unwrap()
                {
                    agent_index = Some(j as u32);
                    break;
                }
            }

            if agent_index.is_some() {
                let agent = &agents[agent_index.unwrap() as usize];
                let simulator_result = simulate_single_neatns(
                    &agent,
                    &maze_phenotype,
                    maze.get_solution_path_cell_length(),
                    true,
                );

                visualize_agent_path(&maze_phenotype, &simulator_result, maze_seed_solution_path);
            }
        }
    }

    pub fn visualise_maze_with_agent_path(
        &self,
        maze: &MazeGenome,
        agents: &AgentQueue,
        path: String,
    ) {
        let maze_phenotype = maze.to_phenotype();

        let mut successful_agent: Option<&MCCAgent> = None;

        for agent in agents.iter() {
            if maze.successful_agent_id.is_some() && agent.id == maze.successful_agent_id.unwrap() {
                successful_agent = Some(agent);
                break;
            }
        }

        if successful_agent.is_some() {
            let simulator_result = simulate_single_mcc(
                successful_agent.unwrap(),
                &maze_phenotype,
                maze.get_solution_path_cell_length(),
                true,
            );
            visualize_agent_path(&maze_phenotype, &simulator_result, path);
        } else {
            println!("Could not generate visualisation: {}", path)
        }
    }
}

fn write_text_to_file(path: String, text: String) -> std::io::Result<()> {
    let mut file = fs::File::create(path)?;
    file.write_all(text.as_bytes())?;
    Ok(())
}

fn create_directory(path: String) -> std::io::Result<()> {
    fs::create_dir_all(path)?;
    Ok(())
}
