/// 半開区間[0,R) の間の数でdivで割ったときの剰余がmである個数
fn count_multiple(r: i64, div: i64, m: i64) -> i64 {
    if r == 0 {
        return 0;
    }

    let mut res = r / div;
    if m < r % div && 0 <= m {
        res += 1;
    }
    res
}

/// 閉区間[L,R] の間の数でdivで割ったときの剰余がmである個数
fn count_multiple_lr(l: i64, r: i64, div: i64, m: i64) -> i64 {
    // [0,r+1) - [0,l) = [l,r]
    count_multiple(r + 1, div, m) - count_multiple(l, div, m)
}

#[cfg(test)]
mod tests {
    use super::*;

    // verify: https://yukicoder.me/problems/no/836
    #[test]
    fn count_multiple_test1() {
        let l = 1;
        let r = 7;
        let n = 2;
        assert_eq!(count_multiple_lr(l, r, n, 0), 3);
        assert_eq!(count_multiple_lr(l, r, n, 1), 4);
    }

    #[test]
    fn count_multiple_test2() {
        let l = 1;
        let r = 1000000000000000000;
        let n = 11;
        assert_eq!(count_multiple_lr(l, r, n, 0), 90909090909090909);
        assert_eq!(count_multiple_lr(l, r, n, 1), 90909090909090910);
        assert_eq!(count_multiple_lr(l, r, n, 2), 90909090909090909);
        assert_eq!(count_multiple_lr(l, r, n, 3), 90909090909090909);
        assert_eq!(count_multiple_lr(l, r, n, 4), 90909090909090909);
        assert_eq!(count_multiple_lr(l, r, n, 5), 90909090909090909);
        assert_eq!(count_multiple_lr(l, r, n, 6), 90909090909090909);
        assert_eq!(count_multiple_lr(l, r, n, 7), 90909090909090909);
        assert_eq!(count_multiple_lr(l, r, n, 8), 90909090909090909);
        assert_eq!(count_multiple_lr(l, r, n, 9), 90909090909090909);
        assert_eq!(count_multiple_lr(l, r, n, 10), 90909090909090909);
    }
}
