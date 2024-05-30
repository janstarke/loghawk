use std::{
    fs::File, io::{stdin, Read, Stdin}
};

use clio::ClioPath;

pub enum InputReader {
    Stdin(Stdin),
    File(File),
}

impl Read for InputReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            InputReader::Stdin(r) => r.read(buf),
            InputReader::File(r) => r.read(buf),
        }
    }
}

impl TryFrom<&ClioPath> for InputReader {
    type Error = anyhow::Error;

    fn try_from(input: &ClioPath) -> Result<Self, Self::Error> {
        if input.is_std() {
            Ok(Self::Stdin(stdin()))
        } else {
            Ok(Self::File(File::open(input.path())?))
        }
    }
}
