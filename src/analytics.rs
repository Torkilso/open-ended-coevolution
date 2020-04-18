use crate::neatns::Seeds;
use std::path::Path;
use std::fs;

/*pub struct Logger {

}

impl Logger {
    pub fn new() -> Logger {
        Logger {}
    }


}*/

pub struct Analyzer {
    write_base_path: String
}

impl Analyzer {
    pub fn new(write_base_path: String) -> Analyzer {
        Analyzer::create_main_directory(write_base_path.clone());
        Analyzer {
            write_base_path
        }
    }

    fn create_main_directory(path: String) -> std::io::Result<()> {
        fs::create_dir_all(path)?;
        Ok(())
    }

    pub fn visualize_seeds(&self, seeds: Seeds) {}
}