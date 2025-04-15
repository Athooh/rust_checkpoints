pub fn next_prime(nbr: u64) -> u64 {
    if is_prime(nbr) {
        return nbr;
    }
    
    let mut next = nbr + 1;
    loop {
        if is_prime(next) {
            return next;
        }
        next += 1;
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
    fn test_next_prime() {
        assert_eq!(next_prime(0), 2);
        assert_eq!(next_prime(1), 2);
        assert_eq!(next_prime(2), 2);
        assert_eq!(next_prime(3), 3);
        assert_eq!(next_prime(4), 5);
        assert_eq!(next_prime(11), 11);
        assert_eq!(next_prime(12), 13);
        assert_eq!(next_prime(200), 211);
    }
}