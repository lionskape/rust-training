#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////

use rand::{thread_rng, Rng};
use std::{collections::VecDeque, vec};

/// Represents a grid of boolean values.
pub struct BoolGrid {
    pub width: usize,
    pub height: usize,
    pub field: Vec<Vec<bool>>,
}

impl BoolGrid {
    /// Creates a new grid with all values initialized as `false`.
    ///
    /// # Arguments
    ///
    /// * `width` - grid width.
    /// * `height` - grid height.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            field: vec![vec![false; width]; height],
        }
    }

    /// Creates a new grid with every value initialized randomly.
    ///
    /// # Arguments
    ///
    /// * `width` - grid width.
    /// * `height` - grid height.
    /// * `vacancy` - probability of any given value being equal
    /// to `false`.
    pub fn random(width: usize, height: usize, vacancy: f64) -> Self {
        let mut field = vec![vec![false; width]; height];
        // thread_rng().gen_bool(1. - vacancy)
        for x in 0..width {
            for y in 0..height {
                field[y][x] = thread_rng().gen_bool(1. - vacancy);
            }
        }

        Self {
            width,
            height,
            field,
        }
    }

    /// Returns grid width.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns grid height.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns the current value of a given cell.
    /// The caller must ensure that `x` and `y` are valid.
    ///
    /// # Arguments
    ///
    /// * `x` - must be >= 0 and < grid width.
    /// * `y` - must be >= 0 and < grid height.
    ///
    /// # Panics
    ///
    /// If `x` or `y` is out of bounds, this method may panic
    /// (or return incorrect result).
    pub fn get(&self, x: usize, y: usize) -> bool {
        if (x >= self.width) || (y >= self.height) {
            panic!("Check coordinates");
        } else {
            self.field[y][x]
        }
    }

    /// Sets a new value to a given cell.
    /// The caller must ensure that `x` and `y` are valid.
    ///
    /// # Arguments
    ///
    /// * `x` - must be >= 0 and < grid width.
    /// * `y` - must be >= 0 and < grid height.
    ///
    /// # Panics
    ///
    /// If `x` or `y` is out of bounds, this method may panic
    /// (or set value to some other unspecified cell).
    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        if x >= self.width || y >= self.height {
            panic!("Check coordinates");
        } else {
            self.field[y][x] = value;
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

/// Returns `true` if the given grid percolates. That is, if there is a path
/// from any cell with `y` == 0 to any cell with `y` == `height` - 1.
/// If the grid is empty (`width` == 0 or `height` == 0), it percolates.
pub fn percolates(grid: &BoolGrid) -> bool {
    let field = &grid.field;

    if field.is_empty() || field[0].is_empty() {
        return true;
    }

    let height = grid.height;
    let width = grid.width;

    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut visited = vec![vec![false; width]; height];
    let mut queue = VecDeque::new();

    for x in 0..width {
        if !field[0][x] {
            queue.push_back((0, x));
            visited[0][x] = true;
        }
    }

    while let Some((row, col)) = queue.pop_front() {
        if row == height - 1 {
            return true;
        }

        for (dr, dc) in directions.iter() {
            let r = row as isize + dr;
            let c = col as isize + dc;

            if r >= 0 && r < height as isize && c >= 0 && c < width as isize {
                let r = r as usize;
                let c = c as usize;

                if !visited[r][c] && !field[r][c] {
                    queue.push_back((r, c));
                    visited[r][c] = true;
                }
            }
        }
    }
    false
}

////////////////////////////////////////////////////////////////////////////////

const N_TRIALS: u64 = 10000;

/// Returns an estimate of the probability that a random grid with given
/// `width, `height` and `vacancy` probability percolates.
/// To compute an estimate, it runs `N_TRIALS` of random experiments,
/// in each creating a random grid and checking if it percolates.
pub fn evaluate_probability(width: usize, height: usize, vacancy: f64) -> f64 {
    let mut perc_count = 0;
    for _ in 0..N_TRIALS {
        let grid = BoolGrid::random(width, height, vacancy);
        if percolates(&grid) {
            perc_count += 1;
        }
    }
    return perc_count as f64 / N_TRIALS as f64;
}
