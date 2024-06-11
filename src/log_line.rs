use anyhow::bail;
use csv::StringRecord;

#[derive(Debug)]
pub struct LogLine {
    key: String,
    contents: Vec<String>,
}

impl LogLine {
    pub fn new(key: String, contents: String) -> Result<Self, anyhow::Error> {
        Ok(Self {
            key,
            contents: vec![contents],
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
}

impl TryFrom<StringRecord> for LogLine {
    type Error = anyhow::Error;

    fn try_from(record: StringRecord) -> Result<Self, Self::Error> {
        let mut contents: Vec<String> = record.into_iter().map(String::from).collect();

        if contents.is_empty() {
            bail!("missing key column");
        }

        let key = contents.remove(0);
        Ok(Self {
            key,
            contents,
        })
    }
}
