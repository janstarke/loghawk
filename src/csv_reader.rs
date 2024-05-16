use std::{
    fs::File, io::{stdin, Read, Stdin}
};

use clio::ClioPath;

pub enum CsvReader {
    Stdin(Stdin),
    File(File),
}

impl Read for CsvReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            CsvReader::Stdin(r) => r.read(buf),
            CsvReader::File(r) => r.read(buf),
        }
    }
}

impl TryFrom<&ClioPath> for CsvReader {
    type Error = anyhow::Error;

    fn try_from(input: &ClioPath) -> Result<Self, Self::Error> {
        if input.is_std() {
            Ok(Self::Stdin(stdin()))
        } else {
            Ok(Self::File(File::open(input.path())?))
        }
    }
}
