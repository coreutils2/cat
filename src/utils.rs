use clap::Parser;

#[derive(Debug, Parser)]
pub struct CliArgs {
    /// Concatenate FILE(s) to standard output.
    /// With no FILE, or when FILE is -, read standard input.
    ///
    /// To stop reading standard input use CTRL+D in *nix systems
    /// and CTRL+Z on Windows.
    pub file: Vec<String>,
    /// number all output lines
    #[clap(short = 'n', long)]
    pub number: bool,
    ///  number nonempty output lines, overrides -n
    #[clap(short = 'b', long)]
    pub number_non_blank: bool,
    /// print the version and exit
    #[clap(short = 'V', long)]
    pub version: bool,
    /// remove all empty lines
    #[clap(short = 's', long)]
    pub squeeze_blank: bool,
}

pub enum NumberingMode {
    None,
    NonBlank,
    All,
}

pub fn no_args_passed(args: &CliArgs) -> bool {
    let res;
    res = (args.number & args.number_non_blank & args.squeeze_blank) == false;
    return res;
}