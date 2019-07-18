pub mod num {
    use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
    pub trait NumOps<Rhs = Self, Output = Self>:
        Add<Rhs, Output = Output>
        + Sub<Rhs, Output = Output>
        + Mul<Rhs, Output = Output>
        + Div<Rhs, Output = Output>
        + Rem<Rhs, Output = Output>
        + Neg<Output = Output>
    {
    }

    impl<T, Rhs, Output> NumOps<Rhs, Output> for T where
        T: Add<Rhs, Output = Output>
            + Sub<Rhs, Output = Output>
            + Mul<Rhs, Output = Output>
            + Div<Rhs, Output = Output>
            + Rem<Rhs, Output = Output>
            + Neg<Output = Output>
    {
    }

    pub trait Num: PartialEq + NumOps + From<i32> {}

    impl Num for i32 {}
    impl Num for i64 {}

    /// Greatest common divisor
    pub fn gcd<T: Num + Copy>(a: T, b: T) -> T {
        if b == 0.into() {
            a
        } else {
            gcd(b, a % b)
        }
    }

    /// Least common multiple
    pub fn lcm<T: Num + Copy>(a: T, b: T) -> T {
        let g = gcd(a, b);
        a / g * b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(num::gcd(57, 3), 3);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(num::lcm(12, 8), 24);
    }
}
