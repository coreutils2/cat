mod constants;

use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use clap::{Parser};
use constants::*;

#[derive(Debug, Parser)]
struct CliArgs {
    /// con-(cat2)-enate FILE(s) to standard output.
    /// With no FILE, or when FILE is -, read standard input.
    ///
    /// To stop reading standard input use CTRL+D in *nix systems
    /// and CTRL+Z on Windows.
    file: Vec<String>,
    /// number all output lines
    #[clap(short = 'n', long)]
    number: bool,
    ///  number nonempty output lines, overrides -n
    #[clap(short = 'b', long)]
    number_non_blank: bool,
    /// print the version and exit
    #[clap(short = 'V', long)]
    version: bool,
    /// remove all empty lines
    #[clap(short = 's', long)]
    squeeze_blank: bool,
}

// TODO: ERROR MESSAGES
fn main() -> anyhow::Result<()> {
    let mut stdout = std::io::stdout();
    let stdin = std::io::stdin();
    // TODO: parse the --squeeze-blank flag according to the OS
    // let (empty_lines, replacement) = match OS {
    //     "windows" => { ("\r\n\r\n", "\r\n") }
    //     _ => { ("\n\n", "\n") }
    // };
    let args = CliArgs::parse();
    if args.version {
        writeln!(stdout, "{}", VERSION_STRING)?;
    }

    // read all files
    for file_path in args.file {
        match file_path.as_str() {
            // for stdin
            "-" => {
                loop {
                    let mut input = String::new();
                    match stdin.read_line(&mut input) {
                        Ok(n) => {
                            // if we get EOF from *nix systems, break;
                            if n == 0 { break; }
                        }
                        Err(err) => {
                            // break; when receiving CTRL-Z on windows
                            // source: https://stackoverflow.com/a/16136924/19984790
                            if err.kind() == ErrorKind::InvalidData { break; }
                        }
                    };
                }
            }
            _ => {
                let file = File::open(&file_path)?;
                loop {
                    let reader = BufReader::new(&file);
                    let mut i: usize = 1;
                    for line in reader.lines() {
                        let content = line?;
                        match args.squeeze_blank {
                            true => { if content.is_empty() { continue; } }
                            false => {}
                        }
                        match args.number_non_blank {
                            true => {
                                if !content.is_empty() {
                                    writeln!(stdout, "{} {content}", i)?;
                                    i += 1;
                                }
                            }
                            false => {
                                match args.number {
                                    true => {
                                        writeln!(stdout, "{} {content}", i)?;
                                        i += 1;
                                    }
                                    // if we have reached here no flags were passed
                                    // so print normally
                                    false => {
                                        writeln!(stdout, "{content}")?;
                                    }
                                }
                            }
                        }
                    }
                    break;
                }
            }
        }
    }
    anyhow::Ok(())
}