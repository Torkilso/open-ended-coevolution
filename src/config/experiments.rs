use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "run_regular_mcc", default = "true")]
    pub run_regular_mcc: bool,

    #[envconfig(from = "run_regular_speciated_mcc", default = "false")]
    pub run_regular_speciated_mcc: bool,

    #[envconfig(from = "run_varied_size_experiment", default = "false")]
    pub run_varied_size_experiment: bool,

    #[envconfig(from = "run_gradual_replacement_experiment", default = "false")]
    pub run_gradual_replacement_experiment: bool,

    #[envconfig(from = "run_sudden_replacement_experiment", default = "false")]
    pub run_sudden_replacement_experiment: bool,
}
