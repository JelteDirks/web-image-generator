use clap::Parser;
use std::thread;
use std::sync::Arc;

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
    let original_image = ImageRecord::new(args.input);

    check_output(&args.output);

    let arc_image = Arc::new(original_image);
    let mut handles = Vec::new();
    let mut counter = 0;

    for size_description in config.sizes {
        counter += 1;
        let output_clone = args.output.clone();
        let img = Arc::clone(&arc_image);
        let handle = thread::spawn(move || {
            convert(size_description, img, output_clone);
            println!("finished converting in thread {:?}", counter);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

