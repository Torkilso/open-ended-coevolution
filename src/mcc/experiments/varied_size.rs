use crate::analytics::Analyzer;
use crate::config;
use crate::mcc::agent::speciated_agent_queue::SpeciatedAgentQueue;

use crate::mcc::generate_generation_stats_s;
use crate::mcc::maze::speciated_maze_queue::SpeciatedMazeQueue;
use crate::neatns;
use crate::simulator::simulate_many;

use crate::mcc::experiments::varied_size_controller::VariedSizeController;

pub fn run_varied_size_experiment(analyzer: &mut Analyzer) {
    println!("Running varied size experiment");

    let seeds = neatns::generate_seeds(config::MCC.maze_seed_amount, true);

    let mut agents = SpeciatedAgentQueue::new(seeds.agents);
    let mut mazes = SpeciatedMazeQueue::new(seeds.mazes);

    let mut varied_size_controller = VariedSizeController {
        //agent_entries: vec![],
        //maze_entries: vec![],
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

        agents.save_state();
        mazes.save_state();

        if generation % config::MCC.generations_between_save == 0 && generation != 0 {
            if generation % config::MCC.varied_size_generations_between_search == 0
                && generation != 0
            {
                println!(
                    "Generation: {} ",
                    generation_stats.to_whitespace_separated_string(),
                );
                varied_size_controller.update_population_properties(&mut agents, &mut mazes);
            }

            //print_stats(&agents, &mazes);
        }
    }
    analyzer.generate_diversity_score(&agents, &mazes);
}
