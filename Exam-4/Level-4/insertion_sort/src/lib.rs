pub fn insertion_sort(slice :&mut [i32], steps:usize){
    let len = slice.len();
    let pas = len.min(steps);
    for x in 1..=pas{
        let mut  n = x;
        while n>0 &&slice[n]<slice[n-1]  {
            slice.swap(n, n-1);
            n -= 1;
        }
    }
    }