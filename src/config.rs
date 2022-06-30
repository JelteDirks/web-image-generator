use std::io::{BufReader};
use std::path::PathBuf;
use std::fs::File;
use serde::{Serialize, Deserialize};
use serde_json::from_reader;

type BRF = BufReader<File>;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub sizes: Vec<SizeDescription>,
}

#[derive(Deserialize, Debug)]
pub struct SizeDescription {
    #[serde(default)]
    pub tags: String,

    pub dimensions: (u32, u32),

    #[serde(default)]
    pub extension: Extension,
}

#[derive(Deserialize, Debug)]
pub struct Extension(String);

impl Default for Extension {
    fn default() -> Extension {
        return Extension("jpeg".to_string());
    }
}

impl Config {
    pub fn from_path(location: PathBuf) -> Config {
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

        let json = from_reader::<BRF, Config>(BufReader::new(file));
        return json.unwrap();
    }
}
