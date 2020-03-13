use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "mutate_structure", default = "0.5")]
    pub mutate_structure: f32,

    #[envconfig(from = "add_wall", default = "0.1")]
    pub add_wall: f32,

    #[envconfig(from = "delete_wall", default = "0.001")]
    pub delete_wall: f32,

    #[envconfig(from = "add_waypoint", default = "0.1")]
    pub add_waypoint: f32,

    #[envconfig(from = "delete_waypoint", default = "0.001")]
    pub delete_waypoint: f32,

    #[envconfig(from = "cell_dimension", default = "32")]
    pub cell_dimension: u32,
}
