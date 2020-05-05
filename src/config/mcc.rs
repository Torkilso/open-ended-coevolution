use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "generations", default = "200")]
    pub generations: usize,

    #[envconfig(from = "maze_population_capacity", default = "250")]
    pub maze_population_capacity: u32,

    #[envconfig(from = "maze_seed_amount", default = "10")]
    pub maze_seed_amount: usize,

    #[envconfig(from = "agent_population_capacity", default = "250")]
    pub agent_population_capacity: usize,

    #[envconfig(from = "agent_seed_amount", default = "20")]
    pub agent_seed_amount: usize,

    #[envconfig(from = "find_seed_generation_limit", default = "200")]
    pub find_seed_generation_limit: usize,

    #[envconfig(from = "speciation_threshold", default = "0.85")]
    pub speciation_threshold: f64,

    #[envconfig(from = "agent_selection_limit", default = "40")]
    pub agent_selection_limit: usize,

    #[envconfig(from = "maze_selection_limit", default = "10")]
    pub maze_selection_limit: usize,

    #[envconfig(from = "default_maze_size", default = "10")]
    pub default_maze_size: usize,

    #[envconfig(from = "varied_size_generation_jumps", default = "10")]
    pub varied_size_generation_jumps: usize,

    #[envconfig(from = "varied_size_default_borrow_amount", default = "10")]
    pub varied_size_default_borrow_amount: u32,

    #[envconfig(from = "varied_size_minimum_generation", default = "50")]
    pub varied_size_minimum_generation: usize,
}
