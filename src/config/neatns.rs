use crate::network::activation;
use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "ARCHIVE_SEED_AMOUNT", default = "1")]
    pub archive_seed_amount: usize,

    #[envconfig(from = "INITIAL_NOVELTY_THRESHOLD", default = "6.0")]
    pub initial_novelty_threshold: f64,

    #[envconfig(from = "NOVELTY_FLOOR", default = "6.0")]
    pub novelty_floor: f64,

    #[envconfig(from = "AMOUNT_OF_NEIGHBORS", default = "15")]
    pub amount_of_neighbors: usize,
}
