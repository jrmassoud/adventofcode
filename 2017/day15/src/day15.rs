use std::{
    io::{self, BufRead},
    num::ParseIntError,
};

use derive_more::{Display, Error};

const GENERATOR_MODULUS: u64 = 2147483647;

const GENERATOR_A_FACTOR: u64 = 16807;
const GENERATOR_B_FACTOR: u64 = 48271;

const MATCH_COUNT: usize = 40_000_000;
const MATCH_COUNT_FILTERED: usize = 5_000_000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Generator {
    previous: u64,
    factor: u64,
}

impl Generator {
    pub fn new(starting: u64, factor: u64) -> Self {
        Generator {
            previous: starting,
            factor,
        }
    }
}

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.previous = (self.previous * self.factor) % GENERATOR_MODULUS;
        Some(self.previous)
    }
}

#[derive(Debug, Display, Error)]
pub enum ReadInputError {
    #[display("Failed to read line {no}: {source}")]
    Io { no: usize, source: io::Error },
    #[display("Unexpected format on line {no}")]
    BadFormat { no: usize },
    #[display("Failed to parse `{val}` on line {no}: {source}")]
    Parse {
        val: String,
        no: usize,
        source: ParseIntError,
    },
    #[display("Unexpected EOF")]
    UnexpectedEOF,
}

pub fn read_input(input: impl BufRead) -> Result<(Generator, Generator), ReadInputError> {
    let mut lines = input.lines();

    let gen_a_line = lines
        .next()
        .ok_or(ReadInputError::UnexpectedEOF)?
        .map_err(|source| ReadInputError::Io { no: 1, source })?;

    let gen_a_val = gen_a_line
        .trim()
        .strip_prefix("Generator A starts with ")
        .ok_or(ReadInputError::BadFormat { no: 1 })?;
    let gen_a_val = gen_a_val.parse().map_err(|source| ReadInputError::Parse {
        val: gen_a_val.to_owned(),
        no: 1,
        source,
    })?;

    let gen_a = Generator::new(gen_a_val, GENERATOR_A_FACTOR);

    let gen_b_line = lines
        .next()
        .ok_or(ReadInputError::UnexpectedEOF)?
        .map_err(|source| ReadInputError::Io { no: 2, source })?;

    let gen_b_val = gen_b_line
        .trim()
        .strip_prefix("Generator B starts with ")
        .ok_or(ReadInputError::BadFormat { no: 2 })?;
    let gen_b_val = gen_b_val.parse().map_err(|source| ReadInputError::Parse {
        val: gen_b_val.to_owned(),
        no: 2,
        source,
    })?;

    let gen_b = Generator::new(gen_b_val, GENERATOR_B_FACTOR);

    Ok((gen_a, gen_b))
}

pub fn count_matches(gen_a: Generator, gen_b: Generator) -> usize {
    gen_a
        .take(MATCH_COUNT)
        .zip(gen_b)
        .filter(|(a, b)| (a & 0xFFFF) == (b & 0xFFFF))
        .count()
}

pub fn count_matches_filtered(gen_a: Generator, gen_b: Generator) -> usize {
    gen_a
        .filter(|v| v % 4 == 0)
        .take(MATCH_COUNT_FILTERED)
        .zip(gen_b.filter(|v| v % 8 == 0))
        .filter(|(a, b)| (a & 0xFFFF) == (b & 0xFFFF))
        .count()
}

#[cfg(test)]
mod tests {
    use std::{error::Error, io::Cursor};

    use super::*;

    #[test]
    fn read_input_works() -> Result<(), Box<dyn Error>> {
        let (gen_a, gen_b) = read_input(Cursor::new(
            "Generator A starts with 65\n\
             Generator B starts with 8921",
        ))?;

        assert_eq!(gen_a, Generator::new(65, GENERATOR_A_FACTOR));
        assert_eq!(gen_b, Generator::new(8921, GENERATOR_B_FACTOR));

        Ok(())
    }

    #[test]
    fn count_matches_works() {
        let gen_a = Generator::new(65, GENERATOR_A_FACTOR);
        let gen_b = Generator::new(8921, GENERATOR_B_FACTOR);

        assert_eq!(count_matches(gen_a, gen_b), 588);
    }

    #[test]
    fn count_matches_filtered_works() {
        let gen_a = Generator::new(65, GENERATOR_A_FACTOR);
        let gen_b = Generator::new(8921, GENERATOR_B_FACTOR);

        assert_eq!(count_matches_filtered(gen_a, gen_b), 309);
    }
}
