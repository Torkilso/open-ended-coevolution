mod agent_genome;
pub mod agent_queue;
mod agent_species;
pub mod mcc_agent;
mod neural_network;
pub mod speciated_agent_queue;

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub enum ReplacementStrategy {
    Gradual,
    Sudden,
    None,
}