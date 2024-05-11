use std::{
    fmt::Debug, fs::File, io::{stdin, Read, Stdin}
};

use clio::ClioPath;
use csv::StringRecord;

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

pub struct CsvData {
    records: Vec<StringRecord>,
}

impl CsvData {
    pub fn records(&self) -> impl Iterator<Item = &StringRecord> {
        self.records.iter()
    }

    pub fn window(&self, first: usize, count: usize) -> impl Iterator<Item = &StringRecord> {
        self.records[first..first + count].iter()
    }
    pub fn len(&self) -> usize {
        self.records.len()
    }
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}

impl TryFrom<&ClioPath> for CsvData {
    type Error = anyhow::Error;

    fn try_from(path: &ClioPath) -> Result<Self, Self::Error> {
        let mut reader = csv::Reader::from_reader(CsvReader::try_from(path)?);
        let mut records = Vec::new();
        for record in reader.records() {
            records.push(record?);
        }
        Ok(Self { records })
    }
}

impl Debug for CsvData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CsvData").field("records", &self.records.len()).finish_non_exhaustive()
    }
}