use crate::analytics::Analyzer;
use crate::config;
use crate::mcc::agent::agent_queue::AgentQueue;
use crate::mcc::agent::mcc_agent::MCCAgent;
use crate::mcc::agent::speciated_agent_queue::SpeciatedAgentQueue;
use crate::mcc::agent::ReplacementStrategy;
use crate::mcc::maze::maze_queue::MazeQueue;
use crate::mcc::maze::speciated_maze_queue::SpeciatedMazeQueue;
use crate::neatns;
use crate::simulator::simulate_many;
use crate::visualization::maze::visualize_maze;
use std::path::Path;

pub(crate) mod agent;
pub mod maze;

pub fn run_regular_mcc(analyzer: Analyzer) {
    let seeds = neatns::generate_seeds();

    analyzer.visualize_seeds(&seeds);

    let mcc_agents: Vec<MCCAgent> = seeds
        .agents
        .iter()
        .map(|a| MCCAgent::new(a.clone()))
        .collect();

    let mut agents = AgentQueue::new(mcc_agents, config::MCC.agent_population_capacity);
    let mut mazes = MazeQueue::new(seeds.mazes, config::MCC.maze_population_capacity);

    for generation in 0..config::MCC.generations {
        let mut agent_children = agents.get_children(config::MCC.agent_selection_limit);
        let mut maze_children = mazes.get_children(config::MCC.maze_selection_limit);

        simulate_many(&mut agent_children, &mut maze_children);

        for child in agent_children.iter() {
            if child.viable {
                agents.push(child.clone())
            }
        }

        for child in maze_children.iter() {
            if child.viable {
                mazes.push(child.clone())
            }
        }

        println!(
            "Generation: {}\tAgents: {}\tMazes: {} \tAverage size {}",
            generation,
            agents.len(),
            mazes.len(),
            mazes.get_average_size()
        );
    }

    println!(
        "Generating visualisations"
    );
    analyzer.visualise_regular_mcc_results(&mazes, &agents);


    /*for maze in mazes.iter() {
        println!("Maze dimensions: {}x{}", maze.width, maze.height)
    }

    let max = mazes.get_largest();*/
    //println!("Maze dimensions: {}x{}", max.width, max.height);
}

pub fn run_regular_speciated_mcc(analyzer: Analyzer) {
    let seeds = neatns::generate_seeds();

    analyzer.visualize_seeds(&seeds);

    return;

    let mut agents = SpeciatedAgentQueue::new(seeds.agents, false, ReplacementStrategy::None);
    let mut mazes = SpeciatedMazeQueue::new(seeds.mazes);

    for generation in 0..config::MCC.generations {
        let mut agent_children = agents.get_children();
        let mut maze_children = mazes.get_children();

        simulate_many(&mut agent_children, &mut maze_children);

        for child in agent_children.iter() {
            if child.viable {
                agents.push(child.clone())
            }
        }

        for child in maze_children.iter() {
            if child.viable {
                mazes.push(child.clone())
            }
        }

        println!(
            "Generation: {}\tAgents: {}\tMazes: {}",
            generation,
            agents.len(),
            mazes.len(),
        );
    }

    for maze in mazes.iter() {
        println!("Maze dimensions: {}x{}", maze.width, maze.height)
    }

    //let max = mazes.get_largest();
    //println!("Maze dimensions: {}x{}", max.width, max.height);
}
