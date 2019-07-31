/// x ^ y の繰り返し二乗法
pub fn mod_pow(x: i64, y: i64, m: i64) -> i64 {
    let mut x = x;
    let mut y = y;
    let mut res = 1;
    while y > 0 {
        if y & 1 == 1 {
            res = res * x % m;
        }
        x = x * x % m;
        y >>= 1;
    }
    res
}
