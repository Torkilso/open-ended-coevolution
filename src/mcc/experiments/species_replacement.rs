use crate::analytics::Analyzer;
use crate::config;
use crate::mcc::agent::speciated_agent_queue::SpeciatedAgentQueue;
use crate::mcc::experiments::replacement_controller::{ReplacementController, ReplacementStrategy};
use crate::mcc::maze::speciated_maze_queue::SpeciatedMazeQueue;
use crate::mcc::{generate_generation_stats_s, print_stats};
use crate::neatns;
use crate::simulator::simulate_many;

pub fn run_gradual_replacement_experiment(analyzer: &mut Analyzer) {
    println!("Running gradual replacement experiment");

    let seeds = neatns::generate_seeds();

    let mut agents = SpeciatedAgentQueue::new(seeds.agents);
    let mut mazes = SpeciatedMazeQueue::new(seeds.mazes);

    let replacement_controller = ReplacementController {
        agent_entries: vec![],
        maze_entries: vec![],
        replace_type: ReplacementStrategy::Gradual,
    };

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

        if generation % config::MCC.generations_between_save == 0 && generation != 0 {
            agents.save_state();
            mazes.save_state();

            if generation % config::MCC.replacement_generations_between_search == 0 && generation != 0 {
                replacement_controller.update_population_properties(
                    &mut agents,
                    &mut mazes,
                    generation as u32,
                );
            }

            println!(
                "Generation: {} ",
                generation_stats.to_whitespace_separated_string(),
            );
            print_stats(&agents, &mazes);
        }
    }
}

pub fn run_sudden_replacement_experiment(analyzer: &mut Analyzer) {
    println!("Running sudden replacement experiment");

    let seeds = neatns::generate_seeds();

    let mut agents = SpeciatedAgentQueue::new(seeds.agents);
    let mut mazes = SpeciatedMazeQueue::new(seeds.mazes);

    let replacement_controller = ReplacementController {
        agent_entries: vec![],
        maze_entries: vec![],
        replace_type: ReplacementStrategy::Sudden,
    };

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

        if generation % config::MCC.generations_between_save == 0 && generation != 0 {
            agents.save_state();
            mazes.save_state();

            if generation % config::MCC.replacement_generations_between_search == 0 && generation != 0 {
                replacement_controller.update_population_properties(
                    &mut agents,
                    &mut mazes,
                    generation as u32,
                );
            }

            println!(
                "Generation: {} ",
                generation_stats.to_whitespace_separated_string(),
            );
            print_stats(&agents, &mazes);
        }
    }
}
