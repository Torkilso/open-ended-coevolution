use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "generations", default = "100")]
    pub generations: usize,

    #[envconfig(from = "maze_population_capacity", default = "250")]
    pub maze_population_capacity: usize,

    #[envconfig(from = "maze_seed_amount", default = "10")]
    pub maze_seed_amount: usize,

    #[envconfig(from = "agent_population_capacity", default = "250")]
    pub agent_population_capacity: usize,

    #[envconfig(from = "agent_seed_amount", default = "20")]
    pub agent_seed_amount: usize,

    #[envconfig(from = "find_seed_generation_limit", default = "50")]
    pub find_seed_generation_limit: usize,

    #[envconfig(from = "speciation_threshold", default = "0.85")]
    pub speciation_threshold: f64,

    #[envconfig(from = "agent_selection_limit", default = "10")]
    pub agent_selection_limit: usize,

    #[envconfig(from = "agent_selection_limit", default = "10")]
    pub maze_selection_limit: usize,
}
