use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "mutate_weight", default = "0.001")]
    pub mutate_weight: f32,

    #[envconfig(from = "add_connection", default = "0.001")]
    pub add_connection: f32,

    #[envconfig(from = "add_neuron", default = "0.001")]
    pub add_neuron: f32,

    #[envconfig(from = "delete_neuron", default = "0.001")]
    pub delete_neuron: f32,

    #[envconfig(from = "start_offset", default = "315.0")] // pointing south east
    pub start_offset: f64,

    #[envconfig(from = "max_speed", default = "3.0")]
    pub max_speed: f64,

    #[envconfig(from = "agent_radius", default = "8.0")]
    pub agent_radius: f64,
}
