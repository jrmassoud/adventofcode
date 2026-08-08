use crate::knot_hash::KnotHasher;

pub const GRID_SIZE: usize = 128;

#[derive(Debug)]
pub struct Grid {
    data: [[bool; GRID_SIZE]; GRID_SIZE],
}

impl Grid {
    pub fn new(key: &str) -> Self {
        let mut data = [[false; GRID_SIZE]; GRID_SIZE];

        for (i, row) in data.iter_mut().enumerate() {
            let hash = KnotHasher::new(&format!("{key}-{i}")).hash();
            let hash_data = hash.get();
            for (j, v) in row.iter_mut().enumerate() {
                *v = hash_data[j / 8] & (1 << (7 - j % 8)) != 0;
            }
        }

        Grid { data }
    }

    pub fn count_squares(&self) -> usize {
        self.data.iter().flatten().filter(|&&v| v).count()
    }

    pub fn count_regions(&self) -> usize {
        let mut last_region = 0;

        let mut visited = [[false; GRID_SIZE]; GRID_SIZE];
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                if !self.data[i][j] || visited[i][j] {
                    continue;
                }

                last_region += 1;

                let mut stack = vec![(i, j)];
                while let Some((k, l)) = stack.pop() {
                    if !self.data[k][l] || visited[k][l] {
                        continue;
                    }
                    visited[k][l] = true;

                    if k > 0 {
                        stack.push((k - 1, l));
                    }
                    if k < GRID_SIZE - 1 {
                        stack.push((k + 1, l));
                    }
                    if l > 0 {
                        stack.push((k, l - 1));
                    }
                    if l < GRID_SIZE - 1 {
                        stack.push((k, l + 1));
                    }
                }
            }
        }

        last_region
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            data: [[false; GRID_SIZE]; GRID_SIZE],
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn sample_key() {
        let grid = Grid::new("flqrgnkx");
        assert_eq!(grid.count_squares(), 8108);
        assert_eq!(grid.count_regions(), 1242);
    }
}
