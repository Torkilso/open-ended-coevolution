use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "AMOUNT_OF_NEIGHBORS", default = "15")]
    pub amount_of_neighbors: usize,
}
