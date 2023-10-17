mod constants;
mod utils;

use std::any::Any;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, ErrorKind, Read, Write};
use clap::{Parser};
use constants::*;
use utils::*;


// TODO: ERROR MESSAGES
fn main() -> anyhow::Result<()> {
    let mut stdout = std::io::stdout();
    let mut stdin = std::io::stdin();
    // TODO: parse the --squeeze-blank flag according to the OS
    // let (empty_lines, replacement) = match OS {
    //     "windows" => { ("\r\n\r\n", "\r\n") }
    //     _ => { ("\n\n", "\n") }
    // };
    let args = CliArgs::parse();
    if args.version {
        writeln!(stdout, "{}", VERSION_STRING)?;
    }
    let no_args = no_args_passed(&args);
    if no_args {
        for file_path in args.file {
            match file_path.as_str() {
                // standard input
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
                        }
                    }
                }
                _ => {
                    let file = OpenOptions::new()
                        .read(true)
                        .open(file_path)?;
                    let mut file_reader = BufReader::with_capacity(BUFFER_SIZE, file);
                    let mut buffer = vec![0; BUFFER_SIZE];
                    loop {
                        let bytes = file_reader.read(&mut buffer)?;
                        // nothing more left to read
                        if bytes == 0 { break; }
                        let string = String::from_utf8_lossy(&buffer);
                        writeln!(stdout, "{}", string)?;
                    }
                }
            }
        }
    };

    return anyhow::Ok(());
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