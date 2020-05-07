use crate::config;
use crate::maze::maze_genotype::{generate_random_maze};
use crate::mcc::agent::agent_species::AgentSpecies;
use crate::mcc::agent::mcc_agent::MCCAgent;
use crate::mcc::agent::speciated_agent_queue::SpeciatedAgentQueue;
use crate::mcc::maze::maze_species::MazeSpecies;
use crate::mcc::maze::speciated_maze_queue::SpeciatedMazeQueue;
use crate::neatns;

pub struct ReplacementController {}

impl ReplacementController {
    fn replace_empty_species_pairs(
        &self,
        agents: &mut SpeciatedAgentQueue,
        mazes: &mut SpeciatedMazeQueue,
    ) {
        println!("Finding species to replace");

        let mazes_clone = mazes.species.clone();
        let agent_clone = agents.species.clone();

        let empty_mazes: Vec<MazeSpecies> = mazes_clone
            .into_iter()
            .filter(|m| m.maze_queue.len() == 1)
            .collect();
        let empty_agents: Vec<AgentSpecies> = agent_clone
            .into_iter()
            .filter(|a| a.agent_queue.len() == 1)
            .collect();

        if empty_mazes.len() == 0 && empty_agents.len() == 0 {
            return;
        }

        let seed_pair_amount = if empty_agents.len() > empty_mazes.len() {
            empty_mazes.len()
        } else {
            empty_agents.len()
        };

        let seeds = neatns::generate_seeds(seed_pair_amount as u32, false);

        for i in 0..seed_pair_amount {
            let maze_index = mazes.species.iter().position(|m| m.id == empty_mazes[i].id);
            let agent_index = agents
                .species
                .iter()
                .position(|a| a.id == empty_agents[i].id);

            if maze_index.is_some() && agent_index.is_some() {
                let maze_amount = empty_mazes[i].maze_queue.max_items_limit;
                let agent_amount = empty_agents[i].agent_queue.max_items_limit;

                mazes.species.remove(maze_index.unwrap());
                agents.species.remove(agent_index.unwrap());

                let maze_genome = seeds.mazes[i].clone();
                let agent_genome = seeds.agents[i].clone();

                let maze_species = MazeSpecies::new(maze_genome, maze_amount, mazes.species_added);
                let agent_species = AgentSpecies::new(
                    MCCAgent::new(agent_genome),
                    agent_amount,
                    agents.species_added,
                );

                mazes.species.push(maze_species);
                agents.species.push(agent_species);

                mazes.species_added += 1;
                agents.species_added += 1;
            }
        }
    }

    fn replace_empty_agent(
        &self,
        agents: &mut SpeciatedAgentQueue,
        mazes: &mut SpeciatedMazeQueue,
    ) {
        let agent_clone = agents.species.clone();
        let empty_agents: Vec<AgentSpecies> = agent_clone
            .into_iter()
            .filter(|a| a.agent_queue.len() == 1)
            .collect();

        if empty_agents.len() == 0 {
            return;
        }

        let empty_agent = empty_agents.first();

        let small_maze = mazes.get_smallest_maze();
        let agent_amount = empty_agent.unwrap().agent_queue.max_items_limit;

        if small_maze.is_some() {
            let agent = neatns::find_agent_seed_for_maze(small_maze.unwrap());

            let agent_index = agents
                .species
                .iter()
                .position(|a| a.id == empty_agent.unwrap().id);

            agents.species.remove(agent_index.unwrap());

            let agent_species =
                AgentSpecies::new(MCCAgent::new(agent), agent_amount, agents.species_added);
            agents.species.push(agent_species);
            agents.species_added += 1;
        }
    }

    fn replace_worst_maze_species(&self, mazes: &mut SpeciatedMazeQueue) {
        let mut maze_clone = mazes.species.clone();

        maze_clone.sort_by(|a, b| {
            a.statistics
                .get_overall_score()
                .partial_cmp(&b.statistics.get_overall_score())
                .unwrap()
        });

        let worst_maze = maze_clone.first();

        if worst_maze.is_some() {
            let maze_index = mazes
                .species
                .iter()
                .position(|a| a.id == worst_maze.unwrap().id);

            let maze = generate_random_maze(
                config::MCC.default_maze_size as u32,
                config::MCC.default_maze_size as u32,
                mazes.species_added as u32,
            );

            mazes.species.remove(maze_index.unwrap());

            let maze_amount = worst_maze.unwrap().maze_queue.max_items_limit;

            let maze_species = MazeSpecies::new(maze, maze_amount, mazes.species_added);

            mazes.species.push(maze_species);
            mazes.species_added += 1;
        }
    }

    pub fn update_population_properties(
        &self,
        agents: &mut SpeciatedAgentQueue,
        mazes: &mut SpeciatedMazeQueue,
    ) {
        self.replace_empty_species_pairs(agents, mazes);

        self.replace_empty_agent(agents, mazes);
        self.replace_worst_maze_species(mazes);
    }
}
