use anyhow::bail;
use csv::StringRecord;
use tlsh::Tlsh;

use crate::line_hash::LineHash;

#[derive(Debug)]
pub struct LogLine {
    key: String,
    contents: Vec<String>,
    hash: LineHash,
}

impl LogLine {
    pub fn new(key: String, contents: String) -> Result<Self, anyhow::Error> {
        let hash = match LineHash::try_from(&contents[..]) {
            Ok(hash) => hash,
            Err(why) => bail!("unable to generate hash: {why}"),
        };
        Ok(Self {
            key,
            contents: vec![contents],
            hash,
        })
    }

    pub fn key_value(&self) -> &str {
        &self.key
    }

    pub fn contents(&self, index: usize) -> Option<&str> {
        self.contents.get(index).map(|x| x.as_str())
    }

    pub fn iter_contents(&self) -> impl Iterator<Item = &str> {
        self.contents.iter().map(|s| &s[..])
    }

    pub fn fuzzy_hash(&self) -> &Tlsh {
        self.hash.inner()
    }

    pub fn compare(&self, hash: &Tlsh) -> u32 {
        self.hash.compare(hash)
    }
}

impl TryFrom<StringRecord> for LogLine {
    type Error = anyhow::Error;

    fn try_from(record: StringRecord) -> Result<Self, Self::Error> {
        let mut contents: Vec<String> = record.into_iter().map(String::from).collect();

        if contents.is_empty() {
            bail!("missing key column");
        }

        let key = contents.remove(0);
        let hash = LineHash::try_from_iterator(contents.iter().map(|s| &s[..]))?;
        Ok(Self {
            key,
            contents,
            hash,
        })
    }
}
