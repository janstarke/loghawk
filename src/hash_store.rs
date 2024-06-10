use tlsh::{
    hash::{FuzzyHash, HexStringPrefix},
    FuzzyHashType,
};

pub struct HashStore<const SIZE_BUCKETS: usize> {
    bucket_probabilities: [[usize; 4]; SIZE_BUCKETS],
    q1_ratios: [usize; 16],
    q2_ratios: [usize; 16],
    lengths: [usize; 256],
}

impl<const SIZE_BUCKETS: usize> Default for HashStore<SIZE_BUCKETS> {
    fn default() -> Self {
        Self {
            bucket_probabilities: [[0; 4]; SIZE_BUCKETS],
            q1_ratios: Default::default(),
            q2_ratios: Default::default(),
            lengths: [0; 256],
        }
    }
}

fn most_probable_byte(a: &[usize]) -> usize {
    a.iter().enumerate().max_by_key(|i| i.1).unwrap().0
}

impl HashStore<128> {
    pub fn add_hash(&mut self, hash: &FuzzyHash<1, 128>) {
        self.q1_ratios[hash.qratios().q1ratio() as usize] += 1;
        self.q2_ratios[hash.qratios().q2ratio() as usize] += 1;
        self.lengths[hash.length().value() as usize] += 1;

        for (idx, byte) in hash.body().data().iter().enumerate() {
            let offset = idx * 4;
            let (b0, b1, b2, b3) = (
                ((byte & 0b11000000) >> 6) as usize,
                ((byte & 0b00110000) >> 4) as usize,
                ((byte & 0b00001100) >> 2) as usize,
                (byte & 0b00000011) as usize,
            );
            self.bucket_probabilities[offset][b0] += 1;
            self.bucket_probabilities[offset + 1][b1] += 1;
            self.bucket_probabilities[offset + 2][b2] += 1;
            self.bucket_probabilities[offset + 3][b3] += 1;
        }
    }

    pub fn most_probable_hash(&self) -> FuzzyHash<1, 128> {
        let q1 = most_probable_byte(&self.q1_ratios) as u8;
        let q2 = most_probable_byte(&self.q2_ratios) as u8;
        let length = most_probable_byte(&self.lengths) as u8;

        debug_assert_eq!(q1 & 0b11110000, 0);
        debug_assert_eq!(q2 & 0b11110000, 0);
        let q_byte = (q1 << 4) | q2;

        let bytes: Vec<u8> = (0..128 / 4)
            .map(|idx| {
                self.most_probable_bucket(idx * 4) << 6
                    | self.most_probable_bucket(idx * 4 + 1) << 4
                    | self.most_probable_bucket(idx * 4 + 2) << 2
                    | self.most_probable_bucket(idx * 4 + 3)
            })
            .collect();

        let checksum = 0;

        let hash_bytes = [&[checksum], &[length], &[q_byte], bytes.as_slice()].concat();
        assert_eq!(hash_bytes.len(), FuzzyHash::<1, 128>::SIZE_IN_BYTES);

        FuzzyHash::<1, 128>::from_str_bytes(&hash_bytes, Some(HexStringPrefix::Empty)).unwrap()
    }

    fn most_probable_bucket(&self, bucket_idx: usize) -> u8 {
        let byte = self.bucket_probabilities[bucket_idx]
            .iter()
            .enumerate()
            .max_by(|lhs, rhs| lhs.1.cmp(rhs.1))
            .unwrap()
            .0;
        debug_assert!(byte < 4);
        byte as u8
    }
}

#[cfg(test)]
mod tests {
    use super::most_probable_byte;

    #[test]
    fn test_most_probable_byte() {
        assert_eq!(most_probable_byte(&[1, 938, 0, 2893, 287, 23, 827]), 3);
    }
}
