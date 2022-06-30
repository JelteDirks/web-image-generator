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

    let mut output = PathBuf::new();
    output.push("./converted");

    let mut i: u32 = 0;
    for size_description in config.sizes {
        i = i + 1;
        println!("dim {:?}", size_description.dimensions);

        let (width, height): (u32, u32) = size_description.dimensions;
        let mut filename = width.to_string();
        filename.push_str("x");
        filename.push_str(&(height.to_string()));
        filename.push_str(".png");
        let new_image = original.as_ref().resize_to_fill(
            width,
            height,
            FilterType::Gaussian);

        output.push(filename);
        new_image.save(&output);
        println!("output {:?}", &output);
        output.pop();
    }
}

