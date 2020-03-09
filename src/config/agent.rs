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

    #[envconfig(from = "delete_neuron", default = "270.0")] // pointing south east
    pub start_offset: f64,
}
