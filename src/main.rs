use clap::Parser;
use std::path::PathBuf;

mod image_record;
use image_record::ImageRecord;
mod config;
use config::Config;

fn main() {
    let args = CLIArgs::parse();
    let original = ImageRecord::new(args.input);
    let config = Config::new(args.configuration);

    println!("using image file: {:?}", original.get_path());
}

#[derive(Parser)]
struct CLIArgs {
    // needs some more information for doing the task
    // some of the things that could be useful is
    // 1. output directory location
    // 2. the dimension package that is required?
    // 3. which filter to choose from when resizing (advanced)

    #[clap(short = 'i', long = "input", parse(from_os_str))]
    input: PathBuf,

    #[clap(short = 'c', long = "config", parse(from_os_str))]
    configuration: PathBuf,
}
