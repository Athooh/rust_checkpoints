use std::fmt::Debug;
use std::ops::Add;

#[derive(Debug)]
pub struct Garage<T>
where
    T: Add<Output = T> + Copy + Debug,
{
    pub left: Option<T>,
    pub right: Option<T>,
}

impl<T> Garage<T>
where
    T: Add<Output = T> + Copy + Debug,
{
    pub fn move_to_right(&mut self) {
        if let Some(left_val) = self.left {
            if let Some(right_val) = self.right {
                self.right = Some(left_val + right_val);
            } else {
                self.right = Some(left_val);
            }
            self.left = None;
        }
    }

    pub fn move_to_left(&mut self) {
        if let Some(right_val) = self.right {
            if let Some(left_val) = self.left {
                self.left = Some(left_val + right_val);
            } else {
                self.left = Some(right_val);
            }
            self.right = None;
        }
    }
}
