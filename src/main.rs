use clap::Parser;
use image::imageops::FilterType;
use std::ffi::OsString;
use std::path::PathBuf;

mod image_record;
use image_record::ImageRecord;

mod config;
use config::{Config, SizeDescription};

mod cli;
use cli::CLIArgs;

mod util;
use util::*;

fn main() {
    let args = CLIArgs::parse();
    let config = Config::from_path(args.configuration);
    let original = ImageRecord::new(args.input);

    // configuration and original file should be loaded properly here

    let mut output: PathBuf = args.output;

    check_output(&output);

    for size_description in config.sizes {
        let (width, height): (u32, u32) = size_description.dimensions;
        let mut filename = width.to_string();
        filename.push_str("x");
        filename.push_str(&height.to_string());
        filename.push_str(".png");
        // make resize an option in the config (fill / exact / fit)
        let new_image = original.as_ref().resize_to_fill(
            width,
            height,
            FilterType::Gaussian);

        output.push(filename);
        new_image.save(&output);
        output.pop();

        drop(new_image);
    }
}

