use std::collections::BTreeMap;

/// 素因数分解
/// O(√n)
pub fn prime_factorize(x: i64) -> BTreeMap<i64, i64> {
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

/// vec[idx]がtrue -> idxが素数である配列を返す
/// O(n log log n)
pub fn eratosthenes_sieve(n: i64) -> Vec<bool> {
    let n = n as usize + 1;
    let mut res = vec![true; n];
    res[0] = false;
    res[1] = false;
    for i in 2..n {
        if !res[i] {
            continue;
        }
        for j in 2..n {
            if i * j >= n {
                break;
            }
            // 素数の倍数は素数ではない
            res[i * j] = false;
        }
    }
    res
}

/// n の約数の個数
pub fn divisor_num(n: i64) -> i64 {
    let primes = prime_factorize(n);
    let mut res = 1i64;
    for (_p, q) in primes.iter() {
        res *= q + 1;
    }
    res
}

/// n の約数の列挙
/// O(√n)
pub fn divisors(n: i64) -> Vec<i64> {
    let mut res = vec![];
    let n = n as usize;
    for i in 1.. {
        if i * i > n {
            break;
        }
        if n % i == 0 {
            res.push(i as i64);
            if i * i != n {
                res.push(n as i64 / i as i64);
            }
        }
    }
    res.sort();
    res
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
