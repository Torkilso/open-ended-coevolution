use crate::config;
use crate::generic_neat::innovation::InnovationLog;
use crate::generic_neat::innovation::InnovationTime;
use rand::Rng;
use std::fmt;
use crate::neatns::agent::Agent;
use crate::neatns::species::Species;
use crate::maze::maze_genotype::MazeGenome;
use crate::simulator::{simulate_run, SimulatorResult};
use crate::maze::maze_phenotype::MazePhenotype;
use std::ptr::null;

pub struct Population {
    population_size: usize,
    species: Vec<Species>,
    pub innovation_log: InnovationLog,
    pub global_innovation: InnovationTime,
    // todo add novelty metric archive
}

impl Population {
    pub fn new(population_size: usize, inputs: usize, outputs: usize) -> Population {
        let mut population = Population {
            population_size,
            species: Vec::new(),
            innovation_log: InnovationLog::new(),
            global_innovation: InnovationTime::new(),
        };

        for _ in 0..population_size {
            population.push(Agent::new(0, inputs, outputs), false);
        }

        return population;
    }

    /// Add agent to population
    pub fn push(&mut self, agent: Agent, lock_new: bool) {
        if let Some(species) = self.compatible_species(&agent) {
            species.push(agent);
        } else {
            let mut species = Species::new();
            if lock_new {
                species.lock();
            }
            species.push(agent);
            self.species.push(species);
        }
    }

    /// Find first species compatible with agent
    fn compatible_species(&mut self, agent: &Agent) -> Option<&mut Species> {
        for species in self.species.iter_mut() {
            if species.is_compatible(&agent) {
                return Some(species);
            }
        }

        None
    }

    /// Evolve the population
    pub fn evolve(&mut self) {
        // Adjust fitnesses based on age, stagnation and apply fitness sharing
        for species in self.species.iter_mut() {
            species.adjust_fitness();
        }

        // Average fitness of all agents
        // Subtract 1 from pop size to make allow for potential increase of (up to) 1.0 in best fit species.
        // If not increased by (up to) 1.0, the extra individual will be added to the spicies closest to
        // reproducing an additional child.
        let avg_fitness: f64 = self
            .iter()
            .map(|agent| agent.adjusted_fitness)
            .sum::<f64>()
            / (config::NEAT.population_size - 1) as f64;

        // Calculate number of new offsprings to produce within each new species
        for species in self.species.iter_mut() {
            species.calculate_offsprings(avg_fitness);
        }

        // Make sure best species reproduces (or survives through elitism, if enabled)
        let best_specie = self
            .species
            .iter_mut()
            .max_by(|a, b| a.best_fitness.partial_cmp(&b.best_fitness).unwrap())
            .unwrap();
        if best_specie.offsprings < 1.0 {
            best_specie.offsprings = 1.0;
        }

        // The total size of the next population before making up for floting point precicsion
        let mut new_population_size: usize = self
            .species
            .iter()
            .map(|species| species.offsprings.floor() as usize)
            .sum();

        // Sort species based on closeness to additional offspring
        let mut sorted_species: Vec<(f64, &mut Species)> = self
            .species
            .iter_mut()
            .map(|species| (species.offsprings % 1.0, species))
            .collect();
        // Reversed sort (highest first)
        sorted_species.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        // Distribute missing offsprings amongs species
        // in order of floating distance from additional offspring
        while new_population_size < self.population_size {
            for (_, species) in sorted_species.iter_mut() {
                species.offsprings += 1.0;
                new_population_size += 1;

                if new_population_size == self.population_size {
                    break;
                }
            }
        }

        // Verify correct amount of offsprings
        assert_eq!(
            self.species
                .iter()
                .map(|species| species.offsprings.floor() as usize)
                .sum::<usize>(),
            self.population_size
        );

        // Kill individuals of low performance, not allowed to reproduce
        for species in self.species.iter_mut() {
            species.retain_best();
        }

        // Increase the age of the species, making current agents old
        for species in self.species.iter_mut() {
            species.age();
        }

        // Evolve species
        let mut rng = rand::thread_rng();
        for i in 0..self.species.len() {
            let elites = std::cmp::min(
                config::NEAT.elitism,
                std::cmp::min(
                    self.species[i].len(),
                    self.species[i].offsprings.floor() as usize,
                ),
            );
            let reproductions = self.species[i].offsprings.floor() as usize - elites;

            // Directly copy elites, without crossover or mutation
            for j in 0..elites {
                self.push(self.species[i].agents[j].clone(), true);
            }

            // Breed new agents
            for _ in 0..reproductions {
                let error = "Unable to gather agent";
                let father = if rng.gen::<f64>() < config::NEAT.interspecies_reproduction_chance {
                    // Interspecies breeding
                    self.tournament_select(config::NEAT.interspecies_tournament_size)
                        .expect(error)
                } else {
                    // Breeding within species
                    self.species[i].random_agent().expect(error)
                };
                let mother = self.species[i].random_agent().expect(error);

                let mut child = mother.crossover(father);
                child.mutate(&mut self.innovation_log, &mut self.global_innovation);
                self.push(child, true);
            }
        }

        // Kill old population
        for species in self.species.iter_mut() {
            species.remove_old();
        }

        // Remove empty species
        for i in (0..self.species.len()).rev() {
            if self.species[i].len() == 0 {
                self.species.swap_remove(i);
            }
        }

        // Verify correct number of individuals in new population
        assert_eq!(self.iter().count(), config::NEAT.population_size);
    }

    /// Get random agent from population
    fn random_agent(&self) -> Option<&Agent> {
        let len = self.iter().count();

        if len == 0 {
            None
        } else {
            self.iter()
                .skip(rand::thread_rng().gen_range(0, len))
                .next()
        }
    }

    /// Use tournament selection to select an agent
    fn tournament_select(&self, k: usize) -> Option<&Agent> {
        let mut best: Option<&Agent> = None;
        let mut best_fitness = -1.0;

        for _ in 0..k {
            if let Some(agent) = self.random_agent() {
                if agent.fitness > best_fitness {
                    best = Some(agent);
                    best_fitness = agent.fitness;
                }
            }
        }

        return best;
    }

    /// Update fitness of all agents
    /*pub fn evaluate(&mut self, evaluator: &impl evaluate::Evaluate) {
        for (species_index, agent_index, fitness) in evaluator
            .evaluate(
                self.enumerate()
                    .map(|(species_index, agent_index, agent)| {
                        (species_index, agent_index, agent.genome.clone())
                    }),
            )
            .iter()
        {
            self.species[*species_index].agents[*agent_index].fitness = *fitness;
        }
    }*/

    /// Iterate agents
    fn iter(&self) -> impl Iterator<Item=&Agent> {
        self.species.iter().map(|species| species.iter()).flatten()
    }

    /// Iterate agents
    fn iter_mut(&mut self) -> impl Iterator<Item=&mut Agent> {
        self.species
            .iter_mut()
            .map(|species| species.iter_mut())
            .flatten()
    }

    /// Enumerate agents
    fn enumerate(&self) -> impl Iterator<Item=(usize, usize, &Agent)> {
        self.species
            .iter()
            .enumerate()
            .map(|(species_index, species)| {
                species
                    .iter()
                    .enumerate()
                    .map(move |(genome_index, genome)| (species_index, genome_index, genome))
            })
            .flatten()
    }

    /// Gather best agent
    pub fn best(&self) -> Option<&Agent> {
        self.iter().max_by(|a, b| a.cmp(&b))
    }

    pub fn run_simulation_and_update_fitness(&self, maze: &MazePhenotype) -> Option<Agent> {
        let mut successful_agent: Option<Agent> = None;

        for agent in self.iter() {
            let result = simulate_run(agent, &maze);

            if result.agent_reached_end() {
                successful_agent = Some(agent.clone());
            }
        }
        successful_agent
    }
}

impl fmt::Display for Population {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Population(species: {}): ", self.species.len())?;
        for species in self.species.iter() {
            write!(f, "{} ", species.agents.len())?;
        }
        Ok(())
    }
}