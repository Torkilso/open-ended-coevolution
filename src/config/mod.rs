use envconfig::Envconfig;
use lazy_static::lazy_static;

mod agent;
mod maze;
mod mcc;

lazy_static! {
    pub static ref MCC: mcc::MCCConfig = mcc::MCCConfig::init().unwrap();
    pub static ref MAZE: maze::MazeConfig = maze::MazeConfig::init().unwrap();
    pub static ref AGENT: agent::AgentConfig = agent::AgentConfig::init().unwrap();
}
