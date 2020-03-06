use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "population_size", default = "100")]
    pub population_size: usize,
}
