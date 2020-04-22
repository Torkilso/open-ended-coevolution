extern crate chrono;
extern crate elapsed;
extern crate envconfig;
#[macro_use]
extern crate envconfig_derive;
extern crate lazy_static;

use chrono::Utc;

use crate::analytics::Analyzer;

mod analytics;
mod config;
mod maze;
mod mcc;
mod neatns;
mod simulator;
mod utils;
mod visualization;

fn main() {
    // TODO DONE add visualisation of seeds
    // TODO DONE add visualisation of 10 random mazes at end
    // TODO add visualisation of largest maze at end
    // TODO add visualisation of most complex maze at end
    // TODO add visualisation of longest route at end

    // TODO add automatic generation of video of longest route at end

    // TODO add automatic generation of graphs over key numbers after run
    // TODO OR automatic generation of file with quantitative results aka numbers for each generation

    let now = Utc::now().format("%m%d%H%M%S%f").to_string();
    let results_base_path = format!("./results/{}", now);

    let analyzer = Analyzer::new(results_base_path);

    if config::EXPERIMENTS.run_regular_mcc {
        mcc::run_regular_mcc(&analyzer);
    }

    if config::EXPERIMENTS.run_regular_speciated_mcc {
        mcc::run_regular_speciated_mcc(&analyzer)
    }

    // TODO add option to run with varied size and prioritzing in species
    // TODO add option to run with gradual species replacement
    // TODO add option to run with sudden species replacement
    if config::EXPERIMENTS.run_varied_size_experiment {
        //mcc::run_varied_size_experiment()
    }

    if config::EXPERIMENTS.run_gradual_replacement_experiment {
        //mcc::run_gradual_replacement_experiment()
    }

    if config::EXPERIMENTS.run_sudden_replacement_experiment {
        //mcc::run_sudden_replacement_experiment()
    }
}
