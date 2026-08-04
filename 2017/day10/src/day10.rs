use std::{array, num::ParseIntError};

use itertools::Itertools;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Circle {
    data: Vec<usize>,
    skip: usize,
    position: usize,
}

impl Circle {
    pub fn new(count: usize) -> Self {
        Circle {
            data: (0..count).collect(),
            skip: 0,
            position: 0,
        }
    }

    pub fn step(&mut self, length: usize) {
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

    pub fn get(&self) -> &[usize] {
        &self.data
    }

    pub fn run_round(&self, lengths: &[usize]) -> Self {
        lengths.iter().fold(self.clone(), |mut acc, &v| {
            acc.step(v);
            acc
        })
    }

    pub fn get_hash(&self, lengths: &[usize]) -> Option<SparseHash> {
        if self.data.len() != 256 {
            return None;
        }

        let mut state = self.clone();
        for _ in 0..64 {
            state = lengths.iter().fold(state, |mut acc, &v| {
                acc.step(v);
                acc
            });
        }

        Some(SparseHash {
            data: state
                .data
                .iter()
                .map(|&v| v as u8)
                .collect_array()
                .expect("Length verified above"),
        })
    }
}

pub struct SparseHash {
    data: [u8; 256],
}

impl SparseHash {
    pub fn get_hash(&self) -> DenseHash {
        DenseHash {
            data: array::from_fn(|i| {
                self.data[i * 16..(i + 1) * 16]
                    .iter()
                    .fold(0, |acc, v| acc ^ v)
            }),
        }
    }
}

pub struct DenseHash {
    data: [u8; 16],
}

impl DenseHash {
    pub fn to_hex(&self) -> String {
        let mut hex_hash = String::with_capacity(self.data.len() * 2);
        for v in self.data {
            hex_hash.push_str(&format!("{v:02x}"));
        }
        hex_hash
    }
}

#[derive(Debug, Error)]
pub enum ReadLengthsError {
    #[error("Failed to parse `{v}`: {e}")]
    Parse { v: String, e: ParseIntError },
}

pub fn read_lengths_parsing(input: &str) -> Result<Vec<usize>, ReadLengthsError> {
    input
        .split(',')
        .map(|split| split.trim())
        .map(|split| {
            split.parse().map_err(|e| ReadLengthsError::Parse {
                v: split.to_owned(),
                e,
            })
        })
        .collect::<Result<_, _>>()
}

pub fn read_lengths_nonparsing(input: &str) -> Vec<usize> {
    let mut lengths: Vec<_> = input.trim().bytes().map(|v| v as usize).collect();
    lengths.extend([17, 31, 73, 47, 23]);

    lengths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_round() {
        let lengths = read_lengths_parsing("3, 4, 1, 5").expect("Sample input parses");

        let circle = Circle::new(5).run_round(&lengths);
        assert_eq!(circle.get(), &[3, 4, 2, 1, 0]);
    }

    #[test]
    fn full_hash() {
        assert_eq!(
            Circle::new(256)
                .get_hash(&read_lengths_nonparsing(""))
                .expect("Length is 256")
                .get_hash()
                .to_hex(),
            "a2582a3a0e66e6e86e3812dcb672a272"
        );

        assert_eq!(
            Circle::new(256)
                .get_hash(&read_lengths_nonparsing("AoC 2017"))
                .expect("Length is 256")
                .get_hash()
                .to_hex(),
            "33efeb34ea91902bb2f59c9920caa6cd"
        );

        assert_eq!(
            Circle::new(256)
                .get_hash(&read_lengths_nonparsing("1,2,3"))
                .expect("Length is 256")
                .get_hash()
                .to_hex(),
            "3efbe78a8d82f29979031a4aa0b16a9d"
        );

        assert_eq!(
            Circle::new(256)
                .get_hash(&read_lengths_nonparsing("1,2,4"))
                .expect("Length is 256")
                .get_hash()
                .to_hex(),
            "63960835bcdc130f0b66d7ff4f6a5a8e"
        );
    }
}
