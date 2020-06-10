use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "run_regular_mcc", default = "false")]
    pub run_regular_mcc: bool,

    #[envconfig(from = "run_regular_speciated_mcc", default = "false")]
    pub run_regular_speciated_mcc: bool,

    #[envconfig(from = "run_varied_size_experiment", default = "false")]
    pub run_varied_size_experiment: bool,

    #[envconfig(from = "run_replacement_experiment", default = "true")]
    pub run_replacement_experiment: bool,

    #[envconfig(from = "batches", default = "5")]
    pub batches: u32,
}
