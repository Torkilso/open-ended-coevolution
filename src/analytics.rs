use crate::neatns::Seeds;
use std::fs;
use crate::visualization::maze::visualize_maze;
use crate::maze::maze_genotype::MazeGenome;
use crate::neatns::agent::Agent;
use crate::visualization::simulation::visualize_agent_path;
use crate::simulator::simulate_single_neatns;

#[derive(Debug, Clone)]
pub struct Analyzer {
    write_base_path: String
}

impl Analyzer {
    pub fn new(write_base_path: String) -> Analyzer {
        let result = create_directory(write_base_path.clone());

        if result.is_err() {
            panic!("Could not create base result directory!");
        }

        Analyzer {
            write_base_path
        }
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

    pub fn visualise_mazes(&self, mazes: &Vec<MazeGenome>, seeds_folder_path: &String) {
        for (i, maze) in mazes.iter().enumerate() {
            let maze_seed_path = format!("{}/maze_{}.png", seeds_folder_path, i);
            let maze_phenotype = maze.to_phenotype();

            visualize_maze(&maze_phenotype, maze_seed_path, false);
        }
    }

    pub fn visualise_mazes_with_agent_path(&self, mazes: &Vec<MazeGenome>, agents: &Vec<Agent>, seeds_folder_path: &String) {
        for (i, maze) in mazes.iter().enumerate() {
            let maze_seed_solution_path = format!("{}/maze_{}_solution.png", seeds_folder_path, i);
            let maze_phenotype = maze.to_phenotype();

            let mut agent_index: Option<u32> = None;

            for (j, agent) in agents.iter().enumerate() {
                if maze.successful_agent_id.is_some() && agent.id == maze.successful_agent_id.unwrap() {
                    agent_index = Some(j as u32);
                    break;
                }
            }

            if agent_index.is_some() {
                let agent = &agents[agent_index.unwrap() as usize];
                let simulator_result = simulate_single_neatns(&agent, &maze_phenotype, true);

                visualize_agent_path(&maze_phenotype, &simulator_result, maze_seed_solution_path);
            }
        }
    }
}

fn create_directory(path: String) -> std::io::Result<()> {
    fs::create_dir_all(path)?;
    Ok(())
}