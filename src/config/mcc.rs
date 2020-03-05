use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct MCCConfig {
    #[envconfig(from = "generations", default = "10")]
    pub generations: i32,

    #[envconfig(from = "maze_population_capacity", default = "250")]
    pub maze_population_capacity: i32,

    #[envconfig(from = "maze_seed_amount", default = "20")]
    pub maze_seed_amount: i32,

    #[envconfig(from = "navigator_population_capacity", default = "250")]
    pub navigator_population_capacity: usize,

    #[envconfig(from = "navigator_seed_amount", default = "20")]
    pub navigator_seed_amount: i32,
}
