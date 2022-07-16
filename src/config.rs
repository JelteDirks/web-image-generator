use std::io::{BufReader};
use std::path::PathBuf;
use std::fs::File;
use serde::Deserialize;
use serde_json::from_reader;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub sizes: Vec<SizeDescription>,
}

#[derive(Deserialize, Debug)]
pub struct SizeDescription {
    #[serde(default)]
    pub tags: String,

    pub dimensions: (u32, u32),

    pub fill: Option<String>,

    pub filter: Option<String>,

    pub prepend: Option<String>,

    pub extension: Option<String>,
}


impl Config {
    pub fn from_path(location: PathBuf) -> Config {
        let file = match File::open(&location) {
            Ok(f) => {
                f
            }
            Err(e) => {
                eprintln!("error while opening config file: {:?}", &location);
                panic!("{}", e);
            },
        };

        let json = from_reader::<BufReader<File>, Config>(BufReader::new(file));
        return json.unwrap();
    }
}
