pub fn visualise_mazes(mazes: &Vec<MazeGenome>, path: &String) {
    for (i, maze) in mazes.iter().enumerate() {
        let maze_seed_path = format!("{}/maze_{}.png", path, i);
        let maze_phenotype = maze.to_phenotype();

        visualize_maze(&maze_phenotype, maze_seed_path, false);
    }
}

pub fn visualise_maze(maze: &MazeGenome, path: &String) {
    let maze_seed_path = format!("{}", path);
    let maze_phenotype = maze.to_phenotype();

    visualize_maze(&maze_phenotype, maze_seed_path, false);
}

pub fn visualise_mazes_with_agent_path(
    mazes: &Vec<MazeGenome>,
    agents: &Vec<Agent>,
    folder_path: &String,
) {
    for (i, maze) in mazes.iter().enumerate() {
        let file_name = format!("maze_{}_solution.png", i);
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

            visualize_agent_path(
                &maze_phenotype,
                &simulator_result,
                VisualizationOptions {
                    file_name,
                    folder_path: folder_path.clone(),
                    save_all_steps: false,
                },
            );
        }
    }
}

pub fn visualise_maze_with_agent_path(
    maze: &MazeGenome,
    agent: &MCCAgent,
    folder_path: String,
    file_name: String,
    save_all_steps: bool,
) {
    let maze_phenotype = maze.to_phenotype();
    let simulator_result = simulate_single_mcc(
        agent,
        &maze_phenotype,
        maze.get_solution_path_cell_length(),
        true,
    );
    visualize_agent_path(
        &maze_phenotype,
        &simulator_result,
        VisualizationOptions {
            file_name,
            folder_path,
            save_all_steps,
        },
    );
}