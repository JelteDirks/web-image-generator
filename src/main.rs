use clap::Parser;
use std::env;

fn main() {
    let args = CLIArgs::parse();
    println!("{:?}", args);
}

#[derive(Parser, Debug)]
struct CLIArgs {
    #[clap(short = 'o', long = "output", parse(from_os_str))]
    output: std::path::PathBuf,

    #[clap(short = 'i', long = "input", parse(from_os_str))]
    input: std::path::PathBuf,
}
