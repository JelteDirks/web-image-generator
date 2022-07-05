use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
pub struct CLIArgs {
    // needs some more information for doing the task
    // some of the things that could be useful is
    // 1. output directory location
    // 2. the dimension package that is required?
    // 3. which filter to choose from when resizing (advanced)

    #[clap(short='i', long="input", parse(from_os_str))]
    pub input: PathBuf,

    #[clap(short='o', long="output", default_value="./converted", parse(from_os_str))]
    pub output: PathBuf,

    #[clap(short='c', long="config", parse(from_os_str))]
    pub configuration: PathBuf,
}
