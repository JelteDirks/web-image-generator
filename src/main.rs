use clap::Parser;

mod image_record;
use image_record::ImageRecord;

mod config;
use config::{Config, SizeDescription};

mod cli;
use cli::CLIArgs;

fn main() {
    let args = CLIArgs::parse();
    let original = ImageRecord::new(args.input);
    let config = Config::from_path(args.configuration);

    // configuration and original file should be loaded properly here

    println!("config = {:?}", config);
}

