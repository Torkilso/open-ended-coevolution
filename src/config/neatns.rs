use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "generations_without_addition", default = "10")]
    pub generations_without_addition: u32,

    #[envconfig(from = "increase_novelty_threshold_limit", default = "4")]
    pub increase_novelty_threshold_limit: u32,

    #[envconfig(from = "INITIAL_NOVELTY_THRESHOLD", default = "6.0")]
    pub initial_novelty_threshold: f64,

    #[envconfig(from = "NOVELTY_FLOOR", default = "6.0")]
    pub novelty_floor: f64,

    #[envconfig(from = "AMOUNT_OF_NEIGHBORS", default = "15")]
    pub amount_of_neighbors: usize,
}
