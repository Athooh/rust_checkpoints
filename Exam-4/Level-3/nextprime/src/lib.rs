pub fn is_prime(n: u64) -> bool {
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