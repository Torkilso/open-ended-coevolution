use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "generations", default = "1000")]
    pub generations: usize,

    #[envconfig(from = "maze_population_capacity", default = "250")]
    pub maze_population_capacity: usize,

    #[envconfig(from = "maze_seed_amount", default = "1")]
    pub maze_seed_amount: usize,

    #[envconfig(from = "agent_population_capacity", default = "250")]
    pub agent_population_capacity: usize,

    #[envconfig(from = "agent_seed_amount", default = "20")]
    pub agent_seed_amount: usize,

    #[envconfig(from = "find_seed_generation_limit", default = "100")]
    pub find_seed_generation_limit: usize,
}
