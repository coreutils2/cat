mod constants;

use std::fs::File;
use std::io::{Read, Write};
use clap::{Parser};
use constants::*;

#[derive(Debug, Parser)]
struct CliArgs {
    /// Concatenate FILE(s) to standard output.
    /// With no FILE, or when FILE is -, read standard input.
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
    /// suppress repeated empty output lines
    #[clap(short = 's', long)]
    squeeze_blank: bool,
}

// TODO: ERROR MESSAGES
fn main() -> anyhow::Result<()> {
    let mut stdout = std::io::stdout();
    let mut stderr = std::io::stderr();
    // TODO: parse the --squeeze-blank flag according to the OS
    let (empty_lines, replacement) = match OS {
        "windows" => { ("\r\n\r\n", "\r\n") }
        _ => { ("\n\n", "\n") }
    };
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
                    match std::io::stdin().read_line(&mut input) {
                        Ok(l) => { if l == 0 { break; } }
                        Err(err) => {
                            writeln!(stdout, "{err}")?;
                            if err.to_string() == WINDOWS_ERR { break; }
                        }
                    };
                }
            }
            _ => {
                let mut file = File::open(&file_path)?;
                let mut buf = [0; BUFFER_SIZE];
                match args.number {
                    true => {
                        // TODO: fix this
                        writeln!(stderr, "Error: This flag has not been implemented yet!")?;
                        return anyhow::Ok(());
                    }
                    false => {
                        loop {
                            let bytes_read = file.read(&mut buf)?;
                            if bytes_read == 0 { break; }
                            let mut content = String::with_capacity(BUFFER_SIZE);
                            for byte in buf {
                                content.push(byte as char);
                            }
                            match args.squeeze_blank {
                                true => {
                                    // TODO: figure out how to do this with the buffer vector instead of the string
                                    let trimmed_content = content.replace(empty_lines, replacement);
                                    writeln!(stdout, "{trimmed_content}")?;
                                }
                                false => {
                                    writeln!(stdout, "{}", content)?;
                                }
                            };
                            stdout.flush()?;
                        }
                    }
                }
            }
        }
    }
    anyhow::Ok(())
}