use std::collections::BTreeMap;

pub fn factorize(x: i64) -> BTreeMap<i64,usize> {
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

#[cfg(test)]
mod tests {
    use super::factorize;

    #[test]
    fn test_factorize() {
        unimplemented!();
    }
}