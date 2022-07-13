use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
pub struct CLIArgs {
    #[clap(short='i', long="input", parse(from_os_str))]
    pub input: PathBuf,

    #[clap(short='o', long="output", default_value="./converted", parse(from_os_str))]
    pub output: PathBuf,

    #[clap(short='c', long="config", parse(from_os_str))]
    pub configuration: PathBuf,
}
