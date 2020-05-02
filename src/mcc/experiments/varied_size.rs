use crate::analytics::Analyzer;
use crate::config;
use crate::mcc::agent::speciated_agent_queue::SpeciatedAgentQueue;
use crate::mcc::generate_generation_stats_s;
use crate::mcc::maze::speciated_maze_queue::SpeciatedMazeQueue;
use crate::neatns;
use crate::simulator::simulate_many;


pub struct VariedSizeEntry {
    from_species_id: u32,
    to_species_id: u32,
    selection_amount_borrowed: u32,
    population_amount_borrowed: u32,
    for_amount_of_generations: u32,
}

pub struct VariedSizeController {
    entries: Vec<VariedSizeEntry>
}

impl VariedSizeController {
    pub fn find_species_to_adjust(&self, agents: SpeciatedAgentQueue, mazes: SpeciatedMazeQueue) {
        // rank species based on performance
        // if species continue to perform bad, create varied size entry
    }

    pub fn update_population_properties(&self, agents: SpeciatedAgentQueue, mazes: SpeciatedMazeQueue) {
        // go through all replacement entries and update capacity for species
    }
}

pub fn run_varied_size_experiment(analyzer: &mut Analyzer) {
    println!("Running varied size experiment");

    let seeds = neatns::generate_seeds();

    let mut agents = SpeciatedAgentQueue::new(seeds.agents);
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

        //

        // change max limit of species

        let generation_stats = generate_generation_stats_s(generation as u32, &agents, &mazes);
        analyzer.add_generation_stats(&generation_stats);

        println!(
            "Generation: {}",
            generation_stats.to_whitespace_separated_string(),
        );
    }
}