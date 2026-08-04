use std::{io::BufRead, str::FromStr};

use anyhow::{Context, Result, anyhow};
use derive_more::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Default, Clone, Copy, Add, AddAssign, Sub, SubAssign)]
pub struct Coord {
    q: i64,
    r: i64,
}

impl Coord {
    pub const fn new(q: i64, r: i64) -> Self {
        Self { q, r }
    }

    pub const fn q(&self) -> i64 {
        self.q
    }

    pub const fn r(&self) -> i64 {
        self.r
    }

    pub const fn s(&self) -> i64 {
        -self.q - self.r
    }

    pub const fn metric(&self) -> i64 {
        (self.q().abs() + self.r().abs() + self.s().abs()) / 2
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    Northeast,
    Southeast,
    South,
    Southwest,
    Northwest,
}

impl Direction {
    fn to_coord(self) -> Coord {
        match self {
            Direction::North => Coord::new(0, -1),
            Direction::Northeast => Coord::new(1, -1),
            Direction::Southeast => Coord::new(1, 0),
            Direction::South => Coord::new(0, 1),
            Direction::Southwest => Coord::new(-1, 1),
            Direction::Northwest => Coord::new(-1, 0),
        }
    }
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "n" => Ok(Direction::North),
            "ne" => Ok(Direction::Northeast),
            "se" => Ok(Direction::Southeast),
            "s" => Ok(Direction::South),
            "sw" => Ok(Direction::Southwest),
            "nw" => Ok(Direction::Northwest),
            _ => Err(anyhow!("Not a direction")),
        }
    }
}

pub fn read_directions(input: impl BufRead) -> Result<Vec<Direction>> {
    input
        .split(b',')
        .map(|elem| {
            let s = String::from_utf8(elem.context("Failed to read input")?)
                .context("Input is not UTF-8")?;
            s.trim()
                .parse()
                .with_context(|| format!("Failed to parse `{s}`"))
        })
        .collect()
}

pub fn find_max_and_final(directions: &[Direction]) -> (Coord, Coord) {
    directions
        .iter()
        .fold((Coord::default(), Coord::default()), |(max, last), dir| {
            let next = last + dir.to_coord();
            (
                if next.metric() > max.metric() {
                    next
                } else {
                    max
                },
                next,
            )
        })
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    pub fn sample_input() {
        assert_eq!(
            find_max_and_final(
                &read_directions(Cursor::new("ne,ne,ne")).expect("Sample input parses")
            )
            .1
            .metric(),
            3
        );
        assert_eq!(
            find_max_and_final(
                &read_directions(Cursor::new("ne,ne,sw,sw")).expect("Sample input parses")
            )
            .1
            .metric(),
            0
        );
        assert_eq!(
            find_max_and_final(
                &read_directions(Cursor::new("ne,ne,s,s")).expect("Sample input parses")
            )
            .1
            .metric(),
            2
        );
        assert_eq!(
            find_max_and_final(
                &read_directions(Cursor::new("se,sw,se,sw,sw")).expect("Sample input parses")
            )
            .1
            .metric(),
            3
        );
    }
}
