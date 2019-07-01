pub mod mod_int {
    use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

    type Num = usize;
    const MOD: Num = 1e9 as Num + 7;

    #[derive(Copy, Clone)]
    pub struct ModInt<T:Copy + Clone>(pub T);
} // mod mod_int