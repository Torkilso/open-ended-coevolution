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
    let now = Utc::now().format("%m%d%H%M%S%f").to_string();
    let results_base_path = format!("./results/{}", now);

    for i in 1..config::EXPERIMENTS.batches + 1 {
        println!("Running batch {}", i, );



        if config::EXPERIMENTS.run_regular_mcc {
            let results_path = format!("{}/regular_mcc", results_base_path);
            let mut analyzer = Analyzer::new(results_path);
            mcc::run_regular_mcc(&mut analyzer);
            analyzer.generate_results_files()
        }

        if config::EXPERIMENTS.run_regular_speciated_mcc {
            let results_path = format!("{}/regular_speciated_mcc", results_base_path);
            let mut analyzer = Analyzer::new(results_path);
            mcc::run_regular_speciated_mcc(&mut analyzer);
            analyzer.generate_results_files()
        }
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
