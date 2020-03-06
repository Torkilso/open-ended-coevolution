use envconfig::Envconfig;
use lazy_static::lazy_static;

mod agent;
mod maze;
mod mcc;
mod neatns;
mod neat;

lazy_static! {
    pub static ref MCC: mcc::Config = mcc::Config::init().unwrap();
    pub static ref MAZE: maze::Config = maze::Config::init().unwrap();
    pub static ref AGENT: agent::Config = agent::Config::init().unwrap();
    pub static ref NEAT: neat::Config = neat::Config::init().unwrap();
    pub static ref NEATNS: neatns::Config = neatns::Config::init().unwrap();
}
