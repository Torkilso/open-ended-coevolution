use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "generations", default = "1000")]
    pub generations: usize,

    #[envconfig(from = "maze_population_capacity", default = "250")]
    pub maze_population_capacity: u32,

    #[envconfig(from = "maze_seed_amount", default = "10")]
    pub maze_seed_amount: u32,

    #[envconfig(from = "agent_population_capacity", default = "250")]
    pub agent_population_capacity: u32,

    #[envconfig(from = "agent_seed_amount", default = "20")]
    pub agent_seed_amount: u32,

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

    #[envconfig(from = "generations_between_save", default = "10")]
    pub generations_between_save: usize,

    #[envconfig(from = "varied_size_maze_default_borrow_amount", default = "2")]
    pub varied_size_maze_default_borrow_amount: u32,

    #[envconfig(from = "varied_size_agent_default_borrow_amount", default = "1")]
    pub varied_size_agent_default_borrow_amount: u32,

    #[envconfig(from = "varied_size_generations_between_search", default = "100")]
    pub varied_size_generations_between_search: usize,

    #[envconfig(from = "replacement_generations_between_search", default = "100")]
    pub replacement_generations_between_search: usize,
}
