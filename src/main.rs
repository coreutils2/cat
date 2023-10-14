use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use clap::{Parser};


#[derive(Debug, Parser)]
struct CliArgs {
    files: Vec<PathBuf>,
    /// number all output lines
    #[clap(short, long)]
    number_lines: bool,
    /// print the version and exit
    #[clap(short = 'V', long)]
    version: bool,
}

const VERSION_STRING: &'static str = env!("CARGO_PKG_VERSION");


fn main() {
    let args = CliArgs::parse();
    if args.version {
        println!("{}", VERSION_STRING);
        return;
    }

    // read stdin if no files were passed
    if args.files.len() == 0 {
        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)
                .map_err(|e| { eprintln!("{:?}", e) })
                .expect("How does writing to stdin fail?");
        }
    }

    // read all files
    for file_path in args.files {
        let file = File::open(&file_path);
        let reader = match file {
            Ok(ok) => { BufReader::new(ok) }
            Err(err) => {
                eprintln!("Error: {err:?}");
                return;
            }
        };
        for (index, line) in reader.lines().enumerate() {
            match line {
                Ok(ref l) => {
                    match args.number_lines {
                        true => {
                            println!("{index} {}", line.expect("Well shit."));
                        }
                        false => {
                            println!("{}", l);
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Error: {err:?}");
                    return;
                }
            }
        }
    }
}
