pub fn prev_prime(nbr: u64) -> u64  {
    if is_prime(nbr) {
        return nbr;
    }

    let mut prev = nbr - 1; 
    loop {
        if is_prime(prev) {
            return prev;
        }
        prev -= 1
    }
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }

    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
