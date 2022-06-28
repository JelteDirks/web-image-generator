use clap::Parser;
use image::io::Reader as ImageReader;
use std::env;

fn main() {
    let args = CLIArgs::parse();
    let img = match ImageReader::open(args.input) {
        Ok(i) => i, // image could be opened
        Err(e) => panic!("{}", e), // error while opening original file
    };

    println!("{:?}", img.into_dimensions());
}

#[derive(Parser, Debug)]
struct CLIArgs {
    // needs some more information for doing the task
    // some of the things that could be useful is
    // 1. output directory location
    // 2. the dimension package that is required?
    // 3. which filter to choose from when resizing (advanced)

    #[clap(short = 'i', long = "input", parse(from_os_str))]
    input: std::path::PathBuf,
}

struct Original {
    dimensions: (u32, u32),
    path: std::path::PathBuf,
}

