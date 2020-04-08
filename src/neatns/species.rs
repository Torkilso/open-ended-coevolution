use crate::config;
use crate::neatns::agent::Agent;
use rand::Rng;

/// Collection of similar agents
// The lock is used to add new agents without affecting the reproduction of the previous generation.
// It is unlocked after reproduction, which will remove the previous generation and keep the new.
pub struct Species {
    age: u64,
    pub best_fitness: f64,
    lifetime_best_fitness: f64,
    last_improvement: u64,
    pub offsprings: f64,
    pub agents: Vec<Agent>,
    locked: bool,
    // When locked new agents may be added, but the len() and iter() functions will remain unchanged after addition
    locked_agents: usize, // The number of locked agents, this is the length and number of iterated agents when species is locked
}

impl Species {
    pub fn new() -> Species {
        Species {
            age: 0,
            best_fitness: 0.0,
            lifetime_best_fitness: 0.0,
            last_improvement: 0,
            offsprings: 0.0,
            locked: false,
            locked_agents: 0,
            agents: Vec::new(),
        }
    }

    /// Determine wether a new agent is compatible
    pub fn is_compatible(&mut self, other: &Agent) -> bool {
        if let Some(agent) = self.agents.first() {
            agent.distance(other) < config::NEAT.speciation_threshold
        } else {
            true // All agents are compatible if the species is empty
        }
    }

    /// Add an agent
    pub fn push(&mut self, individual: Agent) {
        self.agents.push(individual);
    }

    /// Number of agents. Adheres to lock.
    pub fn len(&self) -> usize {
        if self.locked {
            self.locked_agents
        } else {
            self.agents.len()
        }
    }

    /// Iterate agents. Adheres to lock.
    pub fn iter(&self) -> impl Iterator<Item = &Agent> {
        self.agents.iter().take(self.len())
    }

    /// Iterate mutable agents. Adheres to lock.
    /*pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Agent> {
        let len = self.len(); // Must read len before iter_mut
        self.agents.iter_mut().take(len)
    }*/

    /// Get a random agent. Adheres to lock.
    pub fn random_agent(&self) -> Option<&Agent> {
        self.iter()
            .skip(rand::thread_rng().gen_range(0, self.len()))
            .next()
    }

    /// Adjust fitnesses of all agents
    pub fn adjust_fitness(&mut self) {
        assert!(!self.locked);

        let is_stagnent = self.age - self.last_improvement > config::NEAT.dropoff_age;
        let is_young = self.age < config::NEAT.young_age_limit;
        let size: f64 = self.agents.len() as f64;

        for agent in self.agents.iter_mut() {
            agent.adjusted_fitness = agent.fitness;

            // Greatly penalize stagnant species
            if is_stagnent {
                agent.adjusted_fitness *= config::NEAT.stagnent_species_fitness_multiplier;
            }

            // Boost young species
            if is_young {
                agent.adjusted_fitness *= config::NEAT.young_species_fitness_multiplier;
            }

            // Share fitness within species
            agent.adjusted_fitness /= size;

            // Avoid zero fitness
            if agent.adjusted_fitness <= 0.0 || !agent.adjusted_fitness.is_finite() {
                agent.adjusted_fitness = 0.0001;
            }
        }

        // Sort agents descendingly by adjusted fitness
        self.agents
            .sort_by(|a, b| b.adjusted_fitness.partial_cmp(&a.adjusted_fitness).unwrap());

        // Update best fitness and last improvement if currently best in lifetime
        self.best_fitness = self
            .agents
            .first()
            .map(|agent| agent.fitness)
            .unwrap_or(0.0);
        if self.best_fitness > self.lifetime_best_fitness {
            self.lifetime_best_fitness = self.best_fitness;
            self.last_improvement = self.age;
        }
    }

    /// Retain only the best individuals
    pub fn retain_best(&mut self) {
        assert!(!self.locked);

        // Assumes the individuals are sorted in descending fitness order
        self.agents.truncate(std::cmp::max(
            (self.agents.len() as f64 * config::NEAT.survival_ratio).floor() as usize,
            2, // Keep a minimum of two individuals for sexual reproduction
        ));
    }

    /// Lock the species, so that next generation agents are not used for reproduction
    pub fn lock(&mut self) {
        assert!(!self.locked);

        self.locked_agents = self.agents.len();
        self.locked = true;
    }

    /// Increase age and prepare for addition of new agents
    pub fn age(&mut self) {
        self.lock();
        self.age += 1;
    }

    /// Remove all the locked agents (the old generation), and retain the agents pushed after lock (next generation)
    pub fn remove_old(&mut self) {
        assert!(self.locked);
        self.agents = self.agents.split_off(self.locked_agents);
        self.locked = false;
    }

    /// Calculate number of offsprings based on adjusted fitness of agents
    pub fn calculate_offsprings(&mut self, avg_fitness: f64) {
        assert!(!self.locked);

        self.offsprings = self
            .agents
            .iter()
            .map(|agent| agent.adjusted_fitness / avg_fitness)
            .sum();
    }
}
