use std::{collections::HashMap, io::BufRead, ops::ControlFlow, str::FromStr};

use anyhow::{Context, Result, anyhow, bail};

const MIN_PROGRAM: u8 = b'a';

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Spin { count: usize },
    Exchange { a: usize, b: usize },
    Partner { a: u8, b: u8 },
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(count_str) = s.strip_prefix('s') {
            let count = count_str
                .parse()
                .with_context(|| format!("Failed to parse count `{count_str}`"))?;
            return Ok(Instruction::Spin { count });
        } else if let Some(suffix) = s.strip_prefix('x') {
            let (a_str, b_str) = suffix
                .split_once('/')
                .ok_or_else(|| anyhow!("Argument `{suffix}` does not contain a /"))?;
            let a = a_str
                .parse()
                .with_context(|| format!("Failed to parse A argument `{a_str}`"))?;
            let b = b_str
                .parse()
                .with_context(|| format!("Failed to parse B argument `{b_str}`"))?;
            return Ok(Instruction::Exchange { a, b });
        } else if let Some(suffix) = s.strip_prefix('p') {
            let (a_str, b_str) = suffix
                .split_once('/')
                .ok_or_else(|| anyhow!("Argument `{suffix}` does not contain a /"))?;
            if a_str.as_bytes().len() != 1 {
                bail!("Argument `{a_str}` is too long");
            }
            let a = a_str.as_bytes()[0];
            if b_str.as_bytes().len() != 1 {
                bail!("Argument `{b_str}` is too long");
            }
            let b = b_str.as_bytes()[0];
            if a < MIN_PROGRAM || b < MIN_PROGRAM {
                bail!("Invalid register");
            }
            return Ok(Instruction::Partner { a, b });
        }

        Err(anyhow!("`{s}` is not an instruction"))
    }
}

#[derive(Debug, Clone)]
pub struct State {
    instructions: Vec<Instruction>,
    programs: Vec<u8>,
}

impl State {
    pub fn new(instructions: Vec<Instruction>, count: usize) -> Self {
        State {
            instructions,
            programs: (0..count).map(|i| MIN_PROGRAM + i as u8).collect(),
        }
    }

    fn step(&mut self, instruction: Instruction) -> Result<()> {
        match instruction {
            Instruction::Spin { count } => {
                let count = count % self.programs.len();
                self.programs.rotate_right(count);
            }
            Instruction::Exchange { a, b } => {
                if a >= self.programs.len() || b >= self.programs.len() {
                    bail!("Index out of range");
                }
                self.programs.swap(a, b);
            }
            Instruction::Partner { a, b } => {
                let (i, j) = match self.programs.iter().enumerate().try_fold(
                    (None, None),
                    |(f, g), (i, &x)| {
                        let r = (f.or((x == a).then_some(i)), g.or((x == b).then_some(i)));
                        if let (Some(alpha), Some(beta)) = r {
                            ControlFlow::Break((alpha, beta))
                        } else {
                            ControlFlow::Continue(r)
                        }
                    },
                ) {
                    ControlFlow::Continue(_) => {
                        bail!("Nonexistent programs");
                    }
                    ControlFlow::Break(r) => r,
                };
                self.programs.swap(i, j);
            }
        }

        Ok(())
    }

    pub fn run(mut self, n: usize) -> Result<Vec<u8>> {
        let mut visited = HashMap::from([(self.programs.clone(), 0)]);
        let mut j = 0;
        while j < n {
            for i in 0..self.instructions.len() {
                self.step(self.instructions[i])?;
            }

            j += 1;
            if let Some(idx) = visited.get(&self.programs) {
                let diff = j - idx;
                j += ((n - j) / diff) * diff;
            } else {
                visited.insert(self.programs.clone(), j);
            }
        }

        Ok(self.programs)
    }
}

pub fn read_instructions(input: impl BufRead) -> Result<Vec<Instruction>> {
    input
        .split(b',')
        .map(|arg| {
            let arg = arg?;
            let arg = str::from_utf8(&arg)?;
            arg.trim()
                .parse()
                .with_context(|| format!("Failed to parse `{}`", String::from(arg)))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn read_instructions_works() -> Result<()> {
        let instructions = read_instructions(Cursor::new("s1,x3/4,pe/b"))?;
        assert_eq!(
            instructions,
            [
                Instruction::Spin { count: 1 },
                Instruction::Exchange { a: 3, b: 4 },
                Instruction::Partner { a: b'e', b: b'b' }
            ]
        );

        Ok(())
    }

    #[test]
    fn correct_sample() -> Result<()> {
        let state = State::new(
            vec![
                Instruction::Spin { count: 1 },
                Instruction::Exchange { a: 3, b: 4 },
                Instruction::Partner { a: b'e', b: b'b' },
            ],
            5,
        );
        assert_eq!(state.clone().run(1)?, "baedc".as_bytes());
        assert_eq!(state.run(2)?, "ceadb".as_bytes());
        Ok(())
    }
}
