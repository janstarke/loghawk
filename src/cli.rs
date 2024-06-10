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

    /// Mask unicode characters.
    /// 
    /// Normally, all characters are displayed as they are. But there are some
    /// special characters which would not be visible this way, such as the
    /// Left-to-Right-Mark. To display such characters, you need to enable this
    /// switch. The result will be that any characters which are not any of
    /// alphanumeric, whitespace or ascii will be displayed as Unicode code
    /// point (e.g. U-200E) and highlighted using a color (preferably red)
    #[clap(long("mask-unicode"), short('U'))]
    mask_unicode: bool
}
