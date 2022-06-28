use std::io::{BufReader};
use std::path::PathBuf;
use std::fs::File;
use serde::{Serialize, Deserialize};
use serde_json::from_reader;

type BRF = BufReader<File>;

pub struct Config {
    location: PathBuf,
}

#[derive(Deserialize, Serialize, Debug)]
struct CfgFile {
    sizes: Vec<SizeDescription>,
}

#[derive(Deserialize, Serialize, Debug)]
struct SizeDescription {
    tags: String,
    dimensions: (u32, u32),
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

        let json = from_reader::<BRF, CfgFile>(BufReader::new(file));

        println!("content: {:?}", json);

        return Config {
            location
        }
    }
}
