use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "mutate_wall", default = "0.05")]
    pub mutate_wall: f64,

    #[envconfig(from = "mutate_passage", default = "0.05")]
    pub mutate_passage: f64,

    #[envconfig(from = "mutate_waypoint", default = "0.05")]
    pub mutate_waypoint: f64,

    #[envconfig(from = "add_wall", default = "0.1")]
    pub add_wall: f64,

    #[envconfig(from = "delete_wall", default = "0.005")]
    pub delete_wall: f64,

    #[envconfig(from = "add_waypoint", default = "0.1")]
    pub add_waypoint: f64,

    #[envconfig(from = "increase_size", default = "0.1")]
    pub increase_size: f64,

    #[envconfig(from = "cell_dimension", default = "32.0")]
    pub cell_dimension: f64,
}
