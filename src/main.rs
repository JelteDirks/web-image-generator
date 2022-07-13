use clap::Parser;
use std::time::Instant;
use std::io::{self, Write};

mod image_record;
use image_record::ImageRecord;

mod config;
use config::Config;

mod cli;
use cli::CLIArgs;

mod util;
use util::{convert, check_output};

fn main() {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let now = Instant::now();
    handle.write(format!("start={:?}\n", now.elapsed()).as_bytes()).expect("error while writing to stdout");

    let args = CLIArgs::parse();
    let config = Config::from_path(args.configuration);
    let original = ImageRecord::new(args.input);

    check_output(&args.output);

    for size_description in config.sizes {
        convert(size_description, &original, args.output.clone());
    }
    handle.write(format!("finish={:?}\n", now.elapsed()).as_bytes()).expect("error while writing to stdout");
}

