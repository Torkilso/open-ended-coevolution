use crate::analytics::{Analyzer, GenerationStatistics};
use crate::config;
use crate::mcc::agent::agent_queue::AgentQueue;
use crate::mcc::agent::mcc_agent::MCCAgent;
use crate::mcc::agent::speciated_agent_queue::SpeciatedAgentQueue;
use crate::mcc::agent::ReplacementStrategy;
use crate::mcc::maze::maze_queue::MazeQueue;
use crate::mcc::maze::speciated_maze_queue::SpeciatedMazeQueue;
use crate::neatns;
use crate::simulator::simulate_many;

pub(crate) mod agent;
pub mod experiments;
pub mod maze;

pub fn run_regular_mcc(analyzer: &mut Analyzer) {
    println!("Running regular MCC with no speciation");

    let seeds = neatns::generate_seeds();

    let mcc_agents: Vec<MCCAgent> = seeds
        .agents
        .iter()
        .map(|a| MCCAgent::new(a.clone()))
        .collect();

    let mut agents = AgentQueue::new(mcc_agents, config::MCC.agent_population_capacity);
    let mut mazes = MazeQueue::new(seeds.mazes, config::MCC.maze_population_capacity);

    //let global_start = Instant::now();

    for generation in 0..config::MCC.generations {
        //let start = Instant::now();

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

        let generation_stats = generate_generation_stats(generation as u32, &agents, &mazes);
        analyzer.add_generation_stats(&generation_stats);

        if generation % 20 == 0 {
            println!(
                "Generation: {}",
                generation_stats.to_whitespace_separated_string(),
            );
        }
    }
}

pub fn run_regular_speciated_mcc(analyzer: &mut Analyzer) {
    println!("Running regular MCC with speciation");

    let seeds = neatns::generate_seeds();

    let mut agents = SpeciatedAgentQueue::new(seeds.agents, false, ReplacementStrategy::None);
    let mut mazes = SpeciatedMazeQueue::new(seeds.mazes);

    for generation in 0..config::MCC.generations {
        let mut agent_children = agents.get_children();
        let mut maze_children = mazes.get_children();

        simulate_many(&mut agent_children, &mut maze_children);

        // update children with viable based on simulations

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

        let generation_stats = generate_generation_stats_s(generation as u32, &agents, &mazes);
        analyzer.add_generation_stats(&generation_stats);
        if generation % 10 == 0 {
            println!(
                "Generation: {} | agent amount: {} | maze amount: {}",
                generation_stats.to_whitespace_separated_string(),
                agents.len(),
                mazes.len()
            );
        }
    }
}

fn generate_generation_stats(
    generation: u32,
    agents: &AgentQueue,
    mazes: &MazeQueue,
) -> GenerationStatistics {
    GenerationStatistics::new(
        generation,
        mazes.get_average_size(),
        mazes.get_largest_size(),
        mazes.get_smallest_size(),
        mazes.get_average_path_size(),
        mazes.get_largest_path_size(),
        mazes.get_smallest_path_size(),
        agents.get_average_size(),
        agents.get_largest_size(),
        agents.get_smallest_size(),
    )
}

pub fn generate_generation_stats_s(
    generation: u32,
    agents: &SpeciatedAgentQueue,
    mazes: &SpeciatedMazeQueue,
) -> GenerationStatistics {
    GenerationStatistics::new(
        generation,
        mazes.get_average_size(),
        mazes.get_largest_size(),
        mazes.get_smallest_size(),
        mazes.get_average_path_size(),
        mazes.get_largest_path_size(),
        mazes.get_smallest_path_size(),
        agents.get_average_size(),
        agents.get_largest_size(),
        agents.get_smallest_size(),
    )
}
