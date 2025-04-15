#[derive(Debug, PartialEq, Eq)]
pub struct Matrix (pub(i32, i32), pub (i32, i32));

pub fn multiply(m: Matrix, multiplier: i32) -> Matrix{
    Matrix(
        (m.0 .0 * multiplier, m.0 .1 * multiplier),
        (m.1 .0 * multiplier, m.1 .1 * multiplier)
    )

}
pub fn multiply2(m: Matrix, multiplier: i32) -> Matrix{
    let Matrix ((a, b),(c,d))=m;
    Matrix(
        (a * multiplier,b * multiplier),
        (c * multiplier,d * multiplier)
    )
}
