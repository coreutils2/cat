mod constants;

use std::fs::File;
use std::io::{Read, Write, BufRead};
use std::path::PathBuf;
use clap::{Parser};
use constants::*;

#[derive(Debug, Parser)]
struct CliArgs {
    files: Vec<PathBuf>,
    /// number all output lines
    #[clap(short, long)]
    number_lines: bool,
    /// print the version and exit
    #[clap(short = 'V', long)]
    version: bool,
    /// skip empty lines
    #[clap(short = 'S', long)]
    skip_empty_lines: bool,
}


// TODO: ERROR MESSAGES
fn main() -> anyhow::Result<()> {
    let mut stdout = std::io::stdout();
    let mut stderr = std::io::stderr();
    let args = CliArgs::parse();
    if args.version {
        writeln!(stdout, "{}", VERSION_STRING)?;
    }

    // read stdin if no files were passed
    if args.files.len() == 0 {
        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
        }
    }

    // read all files
    for file_path in args.files {
        let mut file = File::open(&file_path)?;
        let mut buf = vec![0; BUFFER_SIZE];
        match args.number_lines {
            true => {
                // TODO: fix this
                writeln!(stderr, "Error: This flag has not been implemented yet!")?;
                return anyhow::Ok(());
            }
            false => {
                loop {
                    let bytes_read = file.read(&mut buf)?;
                    if bytes_read == 0 { break; }
                    match args.skip_empty_lines {
                        true => {
                            // TODO: fix
                            writeln!(stderr, "Error: This flag has not been implemented yet!")?;
                            return anyhow::Ok(());
                        }
                        false => {}
                    };
                    let content = String::from_utf8_lossy(&buf);
                    writeln!(stdout, "{content}")?;
                    stdout.flush()?;
                }
            }
        }
    }
    anyhow::Ok(())
}
