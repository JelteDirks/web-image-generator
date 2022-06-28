use clap::Parser;
use image::io::Reader as ImageReader;
use std::env;

fn main() {
    let args = CLIArgs::parse();
    let img = match ImageReader::open(args.input) {
        Ok(i) => i, // image could be opened
        Err(e) => panic!("{}", e),
    };

    println!("{:?}", img.into_dimensions());
}

#[derive(Parser, Debug)]
struct CLIArgs {
    #[clap(short = 'i', long = "input", parse(from_os_str))]
    input: std::path::PathBuf,
}

struct Original {
    dimensions: (u32, u32),
    path: std::path::PathBuf,
}

