use clap::Parser;
use clio::Input;
use getset::Getters;

#[derive(Getters, Parser, Debug)]
#[getset(get="pub")]
#[clap(name="laxa", author, version, about, long_about=None)]
pub struct Cli {
    /// CSV file to display, use '-' for stdin
    #[clap(value_parser, default_value="-")]
    csv_file: Input
}