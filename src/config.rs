use std::path::PathBuf;
use std::fs::File;

pub struct Config {
    location: PathBuf,
}

impl Config {
    pub fn new(location: PathBuf) -> Config {
        // setup the configuration for this conversion operation

        let file = match File::open(&location) {
            Ok(f) => {
                println!("using configuration file: {:?}", &location);
                f
            }
            Err(e) => {
                eprintln!("error while opening config file: {:?}", &location);
                panic!("{}", e);
            },
        };

        return Config {
            location
        }
    }
}
