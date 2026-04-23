// very beautiful Solution by Lasse Grosbol-Rais

pub enum Direction {
    Right = 0,
    Down,
    Left,
    Up,
}

impl Direction {
    pub fn shift(self) -> Direction {
        match (self as i32 + 1) % 4 {
            0 => Direction::Right,
            1 => Direction::Down,
            2 => Direction::Left,
            _ => Direction::Up,
        }
    }
}
impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut matrix = vec![vec![0; n]; n];

        let (mut x, mut y) = (0, 0);
        let tiles = n.pow(2) as i32;
        let mut steps: i32 = 1;

        let mut dir = Direction::Right;

        while steps <= tiles {
            matrix[y][x] = steps;

            use Direction::*;
            match dir {
                Right => {
                    if x == n - 1 || matrix[y][x + 1] != 0 {
                        dir = dir.shift();
                        y += 1;
                    } else {
                        x += 1;
                    }
                }
                Down => {
                    if y == n - 1 || matrix[y + 1][x] != 0 {
                        dir = dir.shift();
                        x -= 1;
                    } else {
                        y += 1;
                    }
                }
                Left => {
                    if x == 0 || matrix[y][x - 1] != 0 {
                        dir = dir.shift();
                        y -= 1;
                    } else {
                        x -= 1;
                    }
                }
                Up => {
                    if y == 0 || matrix[y - 1][x] != 0 {
                        dir = dir.shift();
                        x += 1;
                    } else {
                        y -= 1;
                    }
                }
            }

            steps += 1;
        }

        matrix
    }
}
