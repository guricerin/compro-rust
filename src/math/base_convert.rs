// Verify:https://atcoder.jp/contests/tkppc4-1/tasks/tkppc4_1_c

/// nをbase進数に変換
pub fn base_convert(n: i64, base: i64) -> Vec<char> {
    let mut res = vec![];
    let mut n = n;

    loop {
        let rem = (n % base) as u8;
        let c: char = std::char::from_digit(rem as u32, 10).unwrap();
        res.push(c);
        n /= base;
        if n <= 0 {
            break;
        }
    }
    // rustのバージョンがあがれば、戻り値をStringにしてもよい
    // res.iter().rev().collect::<String>()
    res.iter().rev().map(|c| *c).collect::<Vec<char>>()
}

/// base進数のnを10進数に変換
pub fn convert_to_10base(n: &[char], base: i64) -> i64 {
    let n = n
        .iter()
        .map(|c| *c as i64 - '0' as i64)
        .collect::<Vec<i64>>();
    n.iter().fold(0, |sum, &n| sum * base + n)
}
