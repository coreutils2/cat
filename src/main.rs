use std::path::PathBuf;
use clap::{Parser};


#[derive(Debug, Parser)]
struct CliArgs {
    files: Vec<PathBuf>,
    #[clap(short = 'E', long)]
    show_ends: bool,
    #[clap(short = 'V', long)]
    version: bool,
}

const VERSION: &'static str = env!("BALLS");

fn main() {
    let args = CliArgs::parse();
    if args.version {
        println!("{}", VERSION);
        return;
    }
    // read stdin if no files were passed
    if args.files.len() == 0 {
        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("How does writing to stdin fail?");
        }
    }
    dbg!(args);
}
