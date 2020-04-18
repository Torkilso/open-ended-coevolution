extern crate elapsed;
extern crate envconfig;
#[macro_use]
extern crate envconfig_derive;
extern crate lazy_static;
extern crate chrono;

use std::fs;
use std::time::SystemTime;
use crate::analytics::Analyzer;
use chrono::{Datelike, Timelike, Utc, TimeZone};
use std::path::Path;

mod analytics;
mod config;
mod maze;
mod mcc;
mod neatns;
mod simulator;
mod visualization;
mod utils;

fn main() {
    // TODO add logging utility
    // TODO add automatic visualisation after run
    // TODO add automatic generation of graphs over key numbers after run OR add

    let now = Utc::now().format("%m%d%H%M%S%f").to_string();
    let base_path = format!("./results/{}", now);

    let analyzer = Analyzer::new(base_path);




    /*

    let analyzer = Analyzer::new("");

    if config::EXPERIMENTS.visualise_results {
        let directory_name = "";

        let now = SystemTime::now();

        fs::create_dir_all(directory_name)?;

    }

    fs::create_dir_all("/some/dir")?;


    if config::MCC.run_regular_mcc {
        mcc::run_regular_mcc();
    }

    if config::MCC.run_regular_speciated_mcc {
        mcc::run_regular_speciated_mcc()
    }

    // TODO add option to run with varied size and prioritzing in species
    // TODO add option to run with gradual species replacement
    // TODO add option to run with sudden species replacement
    if config::MCC.run_varied_size_experiment {
        //mcc::run_varied_size_experiment()
    }

    if config::MCC.run_gradual_replacement_experiment {
        //mcc::run_gradual_replacement_experiment()
    }

    if config::MCC.run_sudden_replacement_experiment {
        //mcc::run_sudden_replacement_experiment()
    }*/
}
