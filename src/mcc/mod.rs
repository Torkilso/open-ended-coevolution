use crate::analytics::{Analyzer, GenerationStatistics};
use crate::config;
use crate::mcc::agent::agent_queue::AgentQueue;
use crate::mcc::agent::mcc_agent::MCCAgent;
use crate::mcc::agent::speciated_agent_queue::SpeciatedAgentQueue;
use crate::mcc::maze::maze_queue::MazeQueue;
use crate::mcc::maze::speciated_maze_queue::SpeciatedMazeQueue;
use crate::neatns;
use crate::simulator::simulate_many;

pub(crate) mod agent;
pub mod experiments;
pub mod maze;

pub fn run_regular_mcc(analyzer: &mut Analyzer) {
    println!("Running regular MCC with no speciation");

    let seeds = neatns::generate_seeds(config::MCC.maze_seed_amount, true);

    let mcc_agents: Vec<MCCAgent> = seeds
        .agents
        .iter()
        .map(|a| MCCAgent::new(a.clone()))
        .collect();

    let mut agents = AgentQueue::new(mcc_agents, config::MCC.agent_population_capacity);
    let mut mazes = MazeQueue::new(seeds.mazes, config::MCC.maze_population_capacity as u32);

    for _generation in 0..config::MCC.generations {
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

        //let generation_stats = generate_generation_stats(generation as u32, &agents, &mazes);
        //analyzer.add_generation_stats(&generation_stats);

        /*if generation % 20 == 0 {
            println!(
                "Generation: {}",
                generation_stats.to_whitespace_separated_string(),
            );
        }*/
    }
    analyzer.generate_diversity_score_no_species(&agents, &mazes);
}

pub fn run_regular_speciated_mcc(analyzer: &mut Analyzer) {
    println!("Running regular MCC with speciation");

    let seeds = neatns::generate_seeds(config::MCC.maze_seed_amount, true);

    let mut agents = SpeciatedAgentQueue::new(seeds.agents);
    let mut mazes = SpeciatedMazeQueue::new(seeds.mazes);

    agents.save_state();
    mazes.save_state();

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

        let generation_stats = generate_generation_stats_s(generation as u32, &agents, &mazes);
        analyzer.add_generation_stats(&generation_stats);

        agents.save_state();
        mazes.save_state();

        if generation % 100 == 0 && generation != 0 {
            println!(
                "Generation: {} ",
                generation_stats.to_whitespace_separated_string(),
            );
            //print_stats(&agents, &mazes);
        }
    }

    analyzer.generate_diversity_score(&agents, &mazes);
}

#[allow(dead_code)]
pub fn print_stats(agents: &SpeciatedAgentQueue, mazes: &SpeciatedMazeQueue) {
    println!(
        "agent amount: {} | maze amount: {}",
        agents.len(),
        mazes.len()
    );

    println!(
        "Overall avg size increase: {} | Overall avg complexity increase: {}",
        mazes.get_overall_average_size_increase(),
        mazes.get_overall_average_complexity_increase()
    );

    println!(
        "Last avg size increase: {} | Last avg complexity increase: {}",
        mazes.get_last_average_size_increase(),
        mazes.get_last_average_complexity_increase()
    );

    println!("Species populations");

    for (_, s) in agents.iter_species().enumerate() {
        println!(
            "Agent species {}: {}/{}\t| Avg size: {:.2} | Avg size increase: {:.2}",
            s.id,
            s.agent_queue.len(),
            s.agent_queue.max_items_limit,
            s.agent_queue.get_average_size(),
            s.statistics.get_current_average_size_increase()
        );
    }
    for (_, m) in mazes.iter_species().enumerate() {
        println!(
            "Maze species {}: {}/{}\t| Avg size: {:.2} | Avg junctures: {:.2} | Avg size increase: {:.2} | Avg complexity increase: {:.2}",
            m.id,
            m.maze_queue.len(),
            m.maze_queue.max_items_limit,
            m.maze_queue.get_average_size(),
            m.maze_queue.get_average_path_size(),
            m.statistics.get_current_average_size_increase(),
            m.statistics.get_current_average_complexity_increase()
        );
    }
}

/*fn generate_generation_stats(
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
}*/

pub fn generate_generation_stats_s(
    generation: u32,
    agents: &SpeciatedAgentQueue,
    mazes: &SpeciatedMazeQueue,
) -> GenerationStatistics {
    GenerationStatistics::new(
        generation,
        agents.len() as u32,
        mazes.len() as u32,
        mazes.get_average_size(),
        mazes.get_largest_size(),
        mazes.get_smallest_size(),
        mazes.get_average_path_size(),
        mazes.get_largest_path_size(),
        mazes.get_smallest_path_size(),
        agents.get_average_size(),
        agents.get_largest_size(),
        agents.get_smallest_size(),
        agents.get_latest_average_size_increase(),
        mazes.get_last_average_size_increase(),
        mazes.get_last_average_complexity_increase(),
        agents.get_overall_average_size_increase(),
        mazes.get_overall_average_size_increase(),
        mazes.get_overall_average_complexity_increase(),
    )
}
