use crate::analytics::Analyzer;
use crate::config;
use crate::mcc::agent::speciated_agent_queue::SpeciatedAgentQueue;
use crate::mcc::generate_generation_stats_s;
use crate::mcc::maze::speciated_maze_queue::SpeciatedMazeQueue;
use crate::neatns;
use crate::simulator::simulate_many;

pub enum ReplacementStrategy {
    Gradual,
    Sudden,
}

pub struct ReplacementEntryController {
    species_to_replace_id: u32,
    new_species_id: u32,
    spanning_over_generations: u32,
}

pub struct ReplacementController {
    entries: Vec<ReplacementEntryController>,
    replace_type: ReplacementStrategy,
}

impl ReplacementController {
    pub fn find_species_to_replace(&self, agents: SpeciatedAgentQueue, mazes: SpeciatedMazeQueue) {
        // look for species that perform very bad compared to others
        // also look at history to see how long they have performed bad
        // replace with new seeds if they are bad for 50 generations?
        //
    }

    pub fn update_population_properties(&self, agents: SpeciatedAgentQueue, mazes: SpeciatedMazeQueue) {
        // go through all replacement entries and update capacity for species
    }
}


pub fn run_gradual_replacement_experiment(analyzer: &mut Analyzer) {
    println!("Running gradual replacement experiment");

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

        let generation_stats = generate_generation_stats_s(generation as u32, &agents, &mazes);
        analyzer.add_generation_stats(&generation_stats);

        // remove bad species
        // find new species pair

        println!(
            "Generation: {}",
            generation_stats.to_whitespace_separated_string(),
        );
    }
}

pub fn run_sudden_replacement_experiment(analyzer: &mut Analyzer) {
    println!("Running sudden replacement experiment");

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

        let generation_stats = generate_generation_stats_s(generation as u32, &agents, &mazes);
        analyzer.add_generation_stats(&generation_stats);

        println!(
            "Generation: {}",
            generation_stats.to_whitespace_separated_string(),
        );
    }
}
