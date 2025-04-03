struct Matrix {
    data: Vec<i32>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    /// Constructs a new matrix filled with sequential numbers
    fn new(rows: usize, cols: usize) -> Self {
        let data = (0..(rows * cols) as i32).collect();
        Matrix { data, rows, cols }
    }

    /// Convert (row, col) to 1D index. Assumes valid bounds.
    fn index(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    /// Unsafe method: no bounds checking!
    unsafe fn get_unchecked(&self, row: usize, col: usize) -> &i32 {
        let idx = self.index(row, col);
        self.data.get_unchecked(idx)
    }

    /// Safe wrapper: performs bounds checking first
    fn get(&self, row: usize, col: usize) -> Option<&i32> {
        if row < self.rows && col < self.cols {
            // SAFETY: Bounds are already checked
            Some(unsafe { self.get_unchecked(row, col) })
        } else {
            None
        }
    }
}

fn main() {
    let matrix = Matrix::new(3, 4);

    // Valid access
    if let Some(value) = matrix.get(1, 2) {
        println!("Value at (1, 2): {}", value);
    }

    // Invalid access (row out of bounds)
    match matrix.get(5, 0) {
        Some(value) => println!("Unexpected value: {}", value),
        None => println!("(5, 0) is out of bounds."),
    }

    // Invalid access (col out of bounds)
    match matrix.get(2, 10) {
        Some(value) => println!("Unexpected value: {}", value),
        None => println!("(2, 10) is out of bounds."),
    }
}
