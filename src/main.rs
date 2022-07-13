use clap::Parser;

mod image_record;
use image_record::ImageRecord;

mod config;
use config::Config;

mod cli;
use cli::CLIArgs;

mod util;
use util::{convert, check_output};

fn main() {
    let args = CLIArgs::parse();
    let config = Config::from_path(args.configuration);
    let original = ImageRecord::new(args.input);

    check_output(&args.output);

    for size_description in config.sizes {
        convert(size_description, &original, args.output.clone());
    }
}

