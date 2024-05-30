use clap::{Parser, ValueEnum};
use clio::Input;
use getset::Getters;

#[derive(Clone, Debug, ValueEnum)]
pub enum FileFormat {
    Csv,
    Txt,
}

#[derive(Getters, Parser, Debug)]
#[getset(get = "pub")]
#[clap(name="laxa", author, version, about, long_about=None)]
pub struct Cli {
    /// file to display, use '-' for stdin
    #[clap(value_parser, default_value = "-")]
    file: Input,

    /// format of the input file
    #[clap(short('F'), long("format"), value_enum, default_value_t=FileFormat::Csv)]
    file_format: FileFormat,

    /// delimiter for CSV and TXT formats
    #[clap(default_value_t = ',')]
    delimiter: char,
}
