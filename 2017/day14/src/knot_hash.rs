use std::array;

const DATA_LEN: usize = 256;
pub const KNOT_HASH_LEN: usize = 16;

#[derive(Debug, Clone)]
pub struct KnotHasher {
    data: [u8; DATA_LEN],
    skip: usize,
    position: usize,
    lengths: Vec<usize>,
}

impl KnotHasher {
    pub fn new(input: &str) -> Self {
        let lengths = input
            .bytes()
            .map(|i| i as usize)
            .chain([17, 31, 73, 47, 23])
            .collect();
        KnotHasher {
            data: array::from_fn(|i| i as u8),
            skip: 0,
            position: 0,
            lengths,
        }
    }

    fn step(&mut self, length: usize) {
        if length > self.data.len() || self.data.is_empty() {
            return;
        }

        let mut i = self.position;
        let mut j = (length + self.position + self.data.len() - 1) % self.data.len();
        let n_steps = length / 2;
        for _ in 0..n_steps {
            self.data.swap(i, j);
            i = (i + 1) % self.data.len();
            j = (j + self.data.len() - 1) % self.data.len();
        }

        self.position = (self.position + length + self.skip) % self.data.len();
        self.skip += 1;
    }

    fn run(&mut self) {
        for i in 0..self.lengths.len() {
            self.step(self.lengths[i]);
        }
    }

    pub fn hash(mut self) -> KnotHash {
        for _ in 0..64 {
            self.run()
        }

        KnotHash {
            data: array::from_fn(|i| {
                self.data[i * 16..(i + 1) * 16]
                    .iter()
                    .fold(0, |acc, v| acc ^ v)
            }),
        }
    }
}

#[derive(Debug)]
pub struct KnotHash {
    data: [u8; KNOT_HASH_LEN],
}

impl KnotHash {
    pub fn get(&self) -> &[u8; KNOT_HASH_LEN] {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hasher() {
        assert_eq!(
            KnotHasher::new("").hash().get(),
            &[
                0xa2, 0x58, 0x2a, 0x3a, 0x0e, 0x66, 0xe6, 0xe8, 0x6e, 0x38, 0x12, 0xdc, 0xb6, 0x72,
                0xa2, 0x72
            ]
        );

        assert_eq!(
            KnotHasher::new("AoC 2017").hash().get(),
            &[
                0x33, 0xef, 0xeb, 0x34, 0xea, 0x91, 0x90, 0x2b, 0xb2, 0xf5, 0x9c, 0x99, 0x20, 0xca,
                0xa6, 0xcd
            ]
        );

        assert_eq!(
            KnotHasher::new("1,2,3").hash().get(),
            &[
                0x3e, 0xfb, 0xe7, 0x8a, 0x8d, 0x82, 0xf2, 0x99, 0x79, 0x03, 0x1a, 0x4a, 0xa0, 0xb1,
                0x6a, 0x9d
            ]
        );

        assert_eq!(
            KnotHasher::new("1,2,4").hash().get(),
            &[
                0x63, 0x96, 0x08, 0x35, 0xbc, 0xdc, 0x13, 0x0f, 0x0b, 0x66, 0xd7, 0xff, 0x4f, 0x6a,
                0x5a, 0x8e
            ]
        );
    }
}
