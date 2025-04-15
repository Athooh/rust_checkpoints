pub struct Matrix(pub Vec<Vec<i32>>);

impl Matrix {
    pub fn new(slice: &[&[i32]]) -> Self {
        let matrix = slice.iter().map(|&row| row.to_vec()).collect();
        Matrix(matrix)
    }
}

use std::fmt;

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.0 {
            write!(f, "(")?;
            for (i, val) in row.iter().enumerate() {
                if i != 0 {
                    write!(f, " ")?;
                }
                write!(f, "{}", val)?;
            }
            write!(f, ")\n")?;
        }
        Ok(())
    }
}
