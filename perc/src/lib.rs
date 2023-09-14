#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////

/// Represents a grid of boolean values.
pub struct BoolGrid {
    width: usize,
    height: usize,
    grid: Vec<Vec<bool>>,
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
            grid: vec![vec![false; width]; height],
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
        let mut g = BoolGrid::new(width, height);
        for i in 0..height {
            for j in 0..width {
                if rand::random::<f64>() <= vacancy {
                    g.grid[i][j] = false
                } else {
                    g.grid[i][j] = true
                }
            }
        }
        return g;
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
        if y >= self.height() {
            panic!("is out of range: y={} grid.height={}", y, self.height())
        }
        if x >= self.width() {
            panic!("is out of range: x={} grid.width={}", x, self.width())
        }
        return self.grid[y][x];
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
        if y >= self.height() {
            panic!("is out of range: y={} grid.height={}", y, self.height())
        }
        if x >= self.width() {
            panic!("is out of range: x={} grid.width={}", x, self.width())
        }
        self.grid[y][x] = value
    }

    // TODO: your code here.
}

////////////////////////////////////////////////////////////////////////////////

/// Returns `true` if the given grid percolates. That is, if there is a path
/// from any cell with `y` == 0 to any cell with `y` == `height` - 1.
/// If the grid is empty (`width` == 0 or `height` == 0), it percolates.
pub fn percolates(grid: &BoolGrid) -> bool {
    if grid.width() == 0 || grid.height() == 0 {
        return true;
    }
    let neighbours: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut visited = vec![vec![false; grid.width()]; grid.height()];
    let mut stack: Vec<(usize, usize)> = grid.grid[0]
        .iter()
        .enumerate()
        .filter(|&(_, cell)| !*cell)
        .map(|(i, _)| (0, i))
        .rev()
        .collect();
    while let Some((cur_i, cur_j)) = stack.pop() {
        visited[cur_i][cur_j] = true;
        if cur_i == grid.height() - 1 {
            return true;
        }
        for (n_i, n_j) in neighbours {
            let i = cur_i as isize + n_i;
            let j = cur_j as isize + n_j;
            if i < 0 || i >= grid.height() as isize || j < 0 || j >= grid.width() as isize {
                continue;
            }
            if !visited[i as usize][j as usize] && !grid.get(j as usize, i as usize) {
                stack.push((i as usize, j as usize));
            }
        }
    }
    return false;
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
