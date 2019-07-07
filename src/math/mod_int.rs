// Original: koba-e964
mod mod_int {
    use std::ops::*;
    pub trait Mod: Copy {
        fn m() -> i64;
    }
    #[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct ModInt<M> {
        pub x: i64,
        phantom: ::std::marker::PhantomData<M>,
    }
    impl<M: Mod> ModInt<M> {
        // x >= 0
        pub fn new(x: i64) -> Self {
            ModInt::new_internal(x % M::m())
        }
        fn new_internal(x: i64) -> Self {
            ModInt {
                x: x,
                phantom: ::std::marker::PhantomData,
            }
        }
        pub fn pow(self, mut e: i64) -> Self {
            debug_assert!(e >= 0);
            let mut sum = ModInt::new_internal(1);
            let mut cur = self;
            while e > 0 {
                if e % 2 != 0 {
                    sum *= cur;
                }
                cur *= cur;
                e /= 2;
            }
            sum
        }
        #[allow(dead_code)]
        pub fn inv(self) -> Self {
            self.pow(M::m() - 2)
        }
    }
    impl<M: Mod, T: Into<ModInt<M>>> Add<T> for ModInt<M> {
        type Output = Self;
        fn add(self, other: T) -> Self {
            let other = other.into();
            let mut sum = self.x + other.x;
            if sum >= M::m() {
                sum -= M::m();
            }
            ModInt::new_internal(sum)
        }
    }
    impl<M: Mod, T: Into<ModInt<M>>> Sub<T> for ModInt<M> {
        type Output = Self;
        fn sub(self, other: T) -> Self {
            let other = other.into();
            let mut sum = self.x - other.x;
            if sum < 0 {
                sum += M::m();
            }
            ModInt::new_internal(sum)
        }
    }
    impl<M: Mod, T: Into<ModInt<M>>> Mul<T> for ModInt<M> {
        type Output = Self;
        fn mul(self, other: T) -> Self {
            ModInt::new(self.x * other.into().x % M::m())
        }
    }
    impl<M: Mod, T: Into<ModInt<M>>> AddAssign<T> for ModInt<M> {
        fn add_assign(&mut self, other: T) {
            *self = *self + other;
        }
    }
    impl<M: Mod, T: Into<ModInt<M>>> SubAssign<T> for ModInt<M> {
        fn sub_assign(&mut self, other: T) {
            *self = *self - other;
        }
    }
    impl<M: Mod, T: Into<ModInt<M>>> MulAssign<T> for ModInt<M> {
        fn mul_assign(&mut self, other: T) {
            *self = *self * other;
        }
    }
    impl<M: Mod> Neg for ModInt<M> {
        type Output = Self;
        fn neg(self) -> Self {
            ModInt::new(0) - self
        }
    }
    impl<M> ::std::fmt::Display for ModInt<M> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            self.x.fmt(f)
        }
    }
    impl<M: Mod> ::std::fmt::Debug for ModInt<M> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            let (mut a, mut b, _) = red(self.x, M::m());
            if b < 0 {
                a = -a;
                b = -b;
            }
            write!(f, "{}/{}", a, b)
        }
    }
    impl<M: Mod> From<i64> for ModInt<M> {
        fn from(x: i64) -> Self {
            Self::new(x)
        }
    }
    // Finds the simplest fraction x/y congruent to r mod p.
    // The return value (x, y, z) satisfies x = y * r + z * p.
    fn red(r: i64, p: i64) -> (i64, i64, i64) {
        if r.abs() <= 10000 {
            return (r, 1, 0);
        }
        let mut nxt_r = p % r;
        let mut q = p / r;
        if 2 * nxt_r >= r {
            nxt_r -= r;
            q += 1;
        }
        if 2 * nxt_r <= -r {
            nxt_r += r;
            q -= 1;
        }
        let (x, z, y) = red(nxt_r, r);
        (x, y - q * z, z)
    }
} // mod mod_int

macro_rules! define_mod {
    ($struct_name: ident, $modulo: expr) => {
        #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct $struct_name {}
        impl mod_int::Mod for $struct_name {
            fn m() -> i64 {
                $modulo
            }
        }
    };
}
const MOD: i64 = 1_000_000_007;
define_mod!(P, MOD);
type ModInt = mod_int::ModInt<P>;

/// FFT (in-place, verified as NTT only)
/// R: Ring + Copy
/// Verified by: https://codeforces.com/contest/1096/submission/47672373
mod fft {
    use std::ops::*;
    /// n should be a power of 2. zeta is a primitive n-th root of unity.
    /// one is unity
    /// Note that the result should be multiplied by 1/sqrt(n).
    pub fn transform<R>(f: &mut [R], zeta: R, one: R)
    where
        R: Copy + Add<Output = R> + Sub<Output = R> + Mul<Output = R>,
    {
        let n = f.len();
        assert!(n.is_power_of_two());
        {
            let mut i = 0;
            for j in 1..n - 1 {
                let mut k = n >> 1;
                loop {
                    i ^= k;
                    if k <= i {
                        break;
                    }
                    k >>= 1;
                }
                if j < i {
                    f.swap(i, j);
                }
            }
        }
        let mut zetapow = Vec::new();
        {
            let mut m = 1;
            let mut cur = zeta;
            while m < n {
                zetapow.push(cur);
                cur = cur * cur;
                m *= 2;
            }
        }
        let mut m = 1;
        while m < n {
            let base = zetapow.pop().unwrap();
            let mut r = 0;
            while r < n {
                let mut w = one;
                for s in r..r + m {
                    let u = f[s];
                    let d = f[s + m] * w;
                    f[s] = u + d;
                    f[s + m] = u - d;
                    w = w * base;
                }
                r += 2 * m;
            }
            m *= 2;
        }
    }
}

mod arbitrary_mod {
    use super::fft;
    use super::mod_int;
    const MOD1: i64 = 1012924417;
    const MOD2: i64 = 1224736769;
    const MOD3: i64 = 1007681537;
    const G1: i64 = 5;
    const G2: i64 = 3;
    const G3: i64 = 3;
    define_mod!(P1, MOD1);
    define_mod!(P2, MOD2);
    define_mod!(P3, MOD3);

    fn zmod(mut a: i64, b: i64) -> i64 {
        a %= b;
        if a < 0 {
            a += b;
        }
        a
    }
    fn ext_gcd(mut a: i64, mut b: i64) -> (i64, i64, i64) {
        let mut x = 0;
        let mut y = 1;
        let mut u = 1;
        let mut v = 0;
        while a != 0 {
            let q = b / a;
            x -= q * u;
            std::mem::swap(&mut x, &mut u);
            y -= q * v;
            std::mem::swap(&mut y, &mut v);
            b -= q * a;
            std::mem::swap(&mut b, &mut a);
        }
        (b, x, y)
    }
    fn invmod(a: i64, b: i64) -> i64 {
        let x = ext_gcd(a, b).1;
        zmod(x, b)
    }

    // This function is ported from http://math314.hateblo.jp/entry/2015/05/07/014908
    fn garner(mut mr: Vec<(i64, i64)>, mo: i64) -> i64 {
        mr.push((mo, 0));

        let mut coffs = vec![1; mr.len()];
        let mut constants = vec![0; mr.len()];
        for i in 0..mr.len() - 1 {
            let v = zmod(mr[i].1 - constants[i], mr[i].0) * invmod(coffs[i], mr[i].0) % mr[i].0;
            assert!(v >= 0);
            for j in i + 1..mr.len() {
                constants[j] += coffs[j] * v % mr[j].0;
                constants[j] %= mr[j].0;
                coffs[j] = coffs[j] * mr[i].0 % mr[j].0;
            }
        }
        constants[mr.len() - 1]
    }

    // f *= g, g is destroyed
    fn convolution_friendly<P: mod_int::Mod>(a: &[i64], b: &[i64], gen: i64) -> Vec<i64> {
        use mod_int::ModInt;
        let d = a.len();
        let mut f = vec![ModInt::<P>::new(0); d];
        let mut g = vec![ModInt::<P>::new(0); d];
        for i in 0..d {
            f[i] = a[i].into();
            g[i] = b[i].into();
        }
        let zeta = ModInt::new(gen).pow((P::m() - 1) / d as i64);
        fft::transform(&mut f, zeta, ModInt::new(1));
        fft::transform(&mut g, zeta, ModInt::new(1));
        for i in 0..d {
            f[i] *= g[i];
        }
        fft::transform(&mut f, zeta.inv(), ModInt::new(1));
        let inv = ModInt::new(d as i64).inv();
        let mut ans = vec![0; d];
        for i in 0..d {
            ans[i] = (f[i] * inv).x;
        }
        ans
    }

    pub fn arbmod_convolution(a: &mut [i64], b: &mut [i64], mo: i64) -> Vec<i64> {
        use super::mod_int::Mod;
        let d = a.len();
        assert!(d.is_power_of_two());
        assert_eq!(d, b.len());
        for x in a.iter_mut() {
            *x = zmod(*x, mo);
        }
        for x in b.iter_mut() {
            *x = zmod(*x, mo);
        }
        let x = convolution_friendly::<P1>(&a, &b, G1);
        let y = convolution_friendly::<P2>(&a, &b, G2);
        let z = convolution_friendly::<P3>(&a, &b, G3);

        let mut ret = vec![0; d];
        let mut mr = [(0, 0); 3];
        for i in 0..d {
            mr[0] = (P1::m(), x[i]);
            mr[1] = (P2::m(), y[i]);
            mr[2] = (P3::m(), z[i]);
            ret[i] = garner(mr.to_vec(), mo);
        }
        ret
    }
}

// f *= g, g is not destroyed
fn convolution(f: &mut [i64], g: &mut [i64]) {
    let ans = arbitrary_mod::arbmod_convolution(f, g, MOD);
    for i in 0..f.len() {
        f[i] = ans[i];
    }
}

fn grow(d: i64, v: i64, mut h: Vec<i64>, invfac: &[ModInt]) -> Vec<i64> {
    assert_eq!(h.len() as i64, d + 1);
    let dd = d as usize;
    let dm = ModInt::new(d);
    let vm = ModInt::new(v);

    let mut aux = vec![1; dd];

    let mut f = vec![0; 4 * dd];
    let mut g = vec![0; 4 * dd];
    for i in 0..dd + 1 {
        f[i] = (invfac[i] * invfac[dd - i] * h[i]).x;
        if (dd + i) % 2 != 0 {
            f[i] = if f[i] == 0 { 0 } else { MOD - f[i] };
        }
    }
    let oldf = f.clone();
    for (idx, &a) in [dm + 1, dm * vm.inv(), dm * vm.inv() + dm + 1]
        .iter()
        .enumerate()
    {
        for i in 0..4 * dd {
            f[i] = oldf[i];
        }
        for i in 0..4 * dd {
            g[i] = 0;
        }
        for i in 1..2 * dd + 2 {
            g[i] = (a - d + i as i64 - 1).inv().x;
        }
        convolution(&mut f, &mut g);
        let mut prod = 1;
        for i in 0..dd + 1 {
            prod = prod * (a - i as i64).x % MOD;
            assert_ne!(prod, 0);
        }
        for i in 0..dd + 1 {
            f[dd + i + 1] = f[dd + i + 1] * prod % MOD;
            prod = prod * (a + i as i64 + 1).x % MOD;
            prod = prod * (a - d + i as i64).inv().x % MOD;
        }
        match idx {
            1 => {
                for i in 0..dd + 1 {
                    h[i] = h[i] * f[dd + 1 + i] % MOD;
                }
            }
            0 => {
                for i in 0..dd {
                    aux[i] = f[dd + 1 + i];
                }
            }
            2 => {
                for i in 0..dd {
                    aux[i] = aux[i] * f[dd + 1 + i] % MOD;
                }
            }
            _ => unreachable!(),
        }
    }
    h.extend_from_slice(&aux);
    h
}

fn gen_seq(d: i64, v: i64) -> Vec<i64> {
    assert!(d > 0 && (d as u64).is_power_of_two());
    let dd = d as usize;

    // precompute factorial and its inv
    let mut fac = vec![ModInt::new(0); 2 * dd + 1];
    let mut invfac = vec![ModInt::new(0); 2 * dd + 1];
    fac[0] = ModInt::new(1);
    for i in 1..2 * dd + 1 {
        fac[i] = fac[i - 1] * (i as i64);
    }
    invfac[2 * dd] = fac[2 * dd].inv();
    for i in (0..2 * dd).rev() {
        invfac[i] = invfac[i + 1] * (i as i64 + 1);
    }
    let mut size = 1;
    // Initialized with [g_1(0), g_1(v)].
    let mut seq = vec![1.into(), (v + 1).into()];
    while size < d {
        seq = grow(size, v, seq, &invfac);
        size *= 2;
    }
    assert_eq!(size, d);
    seq
}

fn fact(n: i64) -> ModInt {
    let d = 1 << 15;
    let aux = gen_seq(d, d);
    // eprintln!("{:?}", aux);
    let mut ans = ModInt::new(1);
    let lim = std::cmp::min(d, (n + 1) / d);
    for i in 0..lim {
        ans *= aux[i as usize];
    }
    for i in lim * d..n {
        ans *= i + 1;
    }
    ans
}

fn fact_naive(n: i64) -> ModInt {
    let mut ans = ModInt::new(1);
    for i in 1..n + 1 {
        ans *= i;
    }
    ans
}
