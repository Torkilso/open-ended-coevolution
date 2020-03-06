use crate::generic_neat::genome::Genome;
use crate::generic_neat::innovation::InnovationLog;
use crate::generic_neat::innovation::InnovationTime;
use std::cmp;
use crate::network::neural_network::NeuralNetwork;

#[derive(Clone)]
pub struct Agent {
    pub genome: Genome,
    pub fitness: f64,
    pub adjusted_fitness: f64,
    pub generation: u64,
}

impl Agent {
    pub fn new(generation: u64, inputs: usize, outputs: usize) -> Agent {
        Agent {
            genome: Genome::new(inputs, outputs),
            fitness: 0.0,
            adjusted_fitness: 0.0,
            generation,
        }
    }

    /// Breed organism with other organism
    pub fn crossover(&self, other: &Self) -> Self {
        Agent {
            genome: self
                .genome
                .crossover(&other.genome, self.fitness > other.fitness),
            fitness: 0.0,
            adjusted_fitness: 0.0,
            generation: self.generation + 1,
        }
    }

    /// Compare to other organism based on non-adjusted fitness
    pub fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.fitness.partial_cmp(&other.fitness).unwrap()
    }

    pub fn to_phenotype(
        &self
    ) -> NeuralNetwork {
        NeuralNetwork::new(&self.genome)
    }

    /// Mutate organism
    pub fn mutate(&mut self, log: &mut InnovationLog, global_innovation: &mut InnovationTime) {
        self.genome.mutate(log, global_innovation);
    }

    /// Genetic distance to other organism
    pub fn distance(&self, other: &Self) -> f64 {
        self.genome.distance(&other.genome)
    }
}
