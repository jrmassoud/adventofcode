use std::io::BufRead;

use anyhow::{Context, Result, anyhow, bail};

pub struct Pipes {
    data: Vec<Vec<usize>>,
}

impl Pipes {
    pub fn get_reachable_from(&self, idx: usize) -> Vec<bool> {
        if idx >= self.data.len() {
            return vec![false; self.data.len()];
        }

        let mut stack = vec![idx];
        let mut visited = vec![false; self.data.len()];
        visited[idx] = true;

        while let Some(i) = stack.pop() {
            for &next in &self.data[i] {
                if visited[next] {
                    continue;
                }
                visited[next] = true;

                stack.push(next);
            }
        }

        visited
    }

    pub fn count_groups(&self) -> usize {
        let mut visited = vec![false; self.data.len()];
        let mut n_groups = 0;

        for i in 0..self.data.len() {
            if visited[i] {
                continue;
            }

            for (old, new) in visited.iter_mut().zip(self.get_reachable_from(i)) {
                *old |= new;
            }

            n_groups += 1;
        }

        n_groups
    }
}

pub fn read_pipes(input: impl BufRead) -> Result<Pipes> {
    let data: Vec<Vec<usize>> = input
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            let no = idx + 1;
            let line = line?;
            let (name, joins) = line
                .split_once("<->")
                .ok_or_else(|| anyhow!("Line {no} has no `<->`"))?;
            let candidate_idx: usize = name
                .trim()
                .parse()
                .with_context(|| format!("Failed to parse `{name}` on line {no}"))?;
            if candidate_idx != idx {
                bail!("Line {no} has index {candidate_idx} (should be {idx})");
            }

            joins
                .split(',')
                .map(|v| {
                    v.trim()
                        .parse()
                        .with_context(|| format!("Failed to parse `{v}` on line {no}"))
                })
                .collect()
        })
        .collect::<Result<_>>()?;

    if let Some(&v) = data.iter().flatten().find(|&&v| v >= data.len()) {
        bail!("Index {v} does not exist");
    }

    Ok(Pipes { data })
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use anyhow::Context;

    use super::*;

    #[test]
    fn sample_input() -> Result<()> {
        let pipes = read_pipes(Cursor::new(
            "\
0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5",
        ))
        .context("Failed to parse sample input")?;
        assert_eq!(
            pipes.get_reachable_from(0),
            [true, false, true, true, true, true, true]
        );
        assert_eq!(pipes.count_groups(), 2);
        Ok(())
    }
}
