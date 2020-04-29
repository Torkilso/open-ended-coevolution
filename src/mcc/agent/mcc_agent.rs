use core::fmt;

use crate::mcc::agent::agent_genome::AgentGenome;
use crate::mcc::agent::neural_network::NeuralNetwork;
use crate::neatns::agent::Agent;

#[derive(Clone)]
pub struct MCCAgent {
    pub genome: AgentGenome,
    pub viable: bool,
    pub id: u32,
    pub completed_maze_id: Option<u32>,
}

impl MCCAgent {
    pub fn new(agent: Agent) -> MCCAgent {
        MCCAgent {
            genome: AgentGenome::new(agent.genome),
            viable: true,
            id: agent.id,
            completed_maze_id: Option::None,
        }
    }

    pub fn to_phenotype(&self) -> NeuralNetwork {
        NeuralNetwork::new(&self.genome)
    }

    /// Mutate organism
    pub fn mutate(&mut self) {
        self.genome.mutate();
    }

    pub fn distance(&self, other: &Self) -> f64 {
        self.genome.distance(&other.genome)
    }
}

impl fmt::Display for MCCAgent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Agent")
    }
}
