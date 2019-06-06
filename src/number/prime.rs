use std::collections::BTreeMap;

/// 素因数分解
pub fn prime_factorize(x: i64) -> BTreeMap<i64, usize> {
    let mut map = BTreeMap::new();
    let mut x = x;
    for prime in 2.. {
        if prime * prime > x {
            break;
        }

        while x % prime == 0 {
            *map.entry(prime).or_insert(0) += 1;
            x /= prime;
        }
    }
    if x != 1 {
        *map.entry(x).or_insert(0) += 1;
    }
    map
}

pub fn is_prime(n: i64) -> bool {
    if n <= 1 {
        return false;
    }
    for prime in 2.. {
        if prime * prime > n {
            break;
        }
        if n % prime == 0 {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::is_prime;
    use super::prime_factorize;

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(11), true);
        assert_eq!(is_prime(121), false);
    }
    #[test]
    fn test_factorize() {
        unimplemented!();
    }
}
