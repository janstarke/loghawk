use std::hash::Hash;

use tlsh::{FuzzyHashType, GeneratorType, Tlsh, TlshGenerator};

#[derive(Debug)]
pub struct LineHash {
    inner: Tlsh,
    body: Vec<u8>,
}

impl LineHash {
    pub fn try_from_iterator<'a, I>(iter: I) -> anyhow::Result<Self>
    where
        I: Iterator<Item = &'a str>,
    {   
        let mut gen = TlshGenerator::new();
        for c in iter {
            gen.update(c.as_bytes());
        }
        let hash = gen.finalize()?;
        let body = hash.body().data();
        Ok(Self {
            inner: hash,
            body: body.to_vec()
        })
    }

    pub fn inner(&self) -> &Tlsh {
        &self.inner
    }

    pub fn compare(&self, hash: &Tlsh) -> u32 {
        self.inner.compare(hash)
    }
}

impl Hash for LineHash {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.body.hash(state)
    }
}

impl Eq for LineHash {}
impl PartialEq for LineHash {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner && self.body == other.body
    }
}

impl PartialOrd for LineHash {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.inner.length().value().partial_cmp(&other.inner.length().value()) {
            Some(core::cmp::Ordering::Equal) => {
                self.body.partial_cmp(&other.body)
            }
            ord => ord,
        }
    }
}

impl TryFrom<&str> for LineHash {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from_iterator(vec![value].into_iter())
    }
}