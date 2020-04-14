extern crate elapsed;
extern crate envconfig;
#[macro_use]
extern crate envconfig_derive;
extern crate lazy_static;

mod config;
mod maze;
mod mcc;
mod neatns;
mod simulator;
mod visualization;

fn main() {
    mcc::run_with_speciation();
    //mcc::run_without_speciation();
}
