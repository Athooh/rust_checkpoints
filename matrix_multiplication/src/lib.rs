#[derive(Debug)]
pub struct Matrix(
    pub (i32, i32),
    pub (i32, i32)
);

pub fn multiply(m: Matrix, multiplier: i32) -> Matrix {
    Matrix(
        (m.0.0 * multiplier, m.0.1 * multiplier),
        (m.1.0 * multiplier, m.1.1 * multiplier),
    )
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply() {
        let matrix = Matrix {
            row1: (1, 2),
            row2: (3, 4),
        };
        
        let result = multiply(matrix, 2);
        
        assert_eq!(result.row1.0, 2);
        assert_eq!(result.row1.1, 4);
        assert_eq!(result.row2.0, 6);
        assert_eq!(result.row2.1, 8);
    }
}