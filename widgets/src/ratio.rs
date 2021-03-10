#![allow(unused_macros)]

use std::cmp::Ordering;
use std::ops::{Mul, Div, Add, Sub, Neg};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Ratio<A> { num: A 
                    , den: A
                    }

pub type Rational = Ratio<isize>;

impl<A> Ratio<A> {
pub const fn as_ref(&self) -> Ratio<&A> {
    Ratio {num: &self.num, den: &self.den}
}
pub fn numerator(self) -> A {
    self.num
}
pub fn denominator(self) -> A {
    self.den
}
}

impl Rational {
    pub const INFINITY : Rational = Self {num: 1, den: 0};
    pub const NAN  : Rational = Self {num: 0, den: 0};
    pub const ZERO : Rational = Self {num: 0, den: 1};
    pub const ONE  : Rational = Self {num: 1, den: 1};

    #[inline]
    pub fn new(num: isize, den: isize) -> Self {
        if den.is_negative() {
            Self::reduce(num.neg(), den.abs())
        } else {
            Self::reduce(num, den)
        }
    }

    /// if negative => only numerator is neg
    pub fn sign(&self) -> Self {
        if self.den.is_negative() {
            Rational::new(self.num.neg(), self.den.neg())
        } else {
            *self
        }
    }

    pub fn gcd(rhs: isize, lhs: isize) -> isize {
        let mut r = rhs;
        let mut l = lhs;
        while r != 0 {
            let tmp = r;
            r = l % tmp;
            l = tmp
        }
        l.abs()
    } 

    #[inline]
    pub fn reduce(n: isize, d: isize) -> Self {
        let x = Self::gcd(n , d);
        Rational {num: n / x, den: d / x}
    }
}

impl Ord for Rational {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        (self.num * other.den).cmp(&(other.num * self.den))
    }
}
impl PartialOrd for Rational {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Add<Rational> for Rational {
    type Output = Rational;
    #[inline]
    fn add(self, other: Rational) -> Rational {
        Self::reduce(self.num * other.den + other.num * self.den, self.den*other.den)
    }
}
impl Sub<Rational> for Rational {
    type Output = Rational;
    #[inline]
    fn sub(self, other: Rational) -> Rational {
        Self::reduce(self.num * other.den - other.num * self.den, self.den*other.den)
    }
}
impl Mul<Rational> for Rational {
    type Output = Rational;
    #[inline]
    fn mul(self, other: Rational) -> Rational {
        Self::reduce(self.num * other.num, self.den * other.den)
    }
}
impl Div<Rational> for Rational {
    type Output = Rational;
    #[inline]
    fn div(self, other: Rational) -> Rational {
        Rational::reduce(self.num * other.den, self.den * other.num)
    }
}
impl Neg for Rational {
    type Output = Rational;
    #[inline]
    fn neg(self) -> Rational {
        Self::new(self.num.neg(), self.den)
    }
}

use std::convert::TryFrom;
impl TryFrom<usize> for Rational {
    type Error = <isize as TryFrom<usize>>::Error;
    fn try_from(u: usize) -> Result<Self, Self::Error> {
        Ok(Rational::new(isize::try_from(u)?, 1))
    }
}
impl TryFrom<u64> for Rational {
    type Error = <isize as TryFrom<u64>>::Error;
    fn try_from(u: u64) -> Result<Self, Self::Error> {
        Ok(Rational::new(isize::try_from(u)?, 1))
    }
}

impl std::iter::Sum<Rational> for Rational {
    #[inline]
    fn sum<I: Iterator<Item = Rational>>(iter: I) -> Self {
        iter.fold(Rational::ZERO, Add::add)
    }
}
impl<'a> std::iter::Sum<&'a Rational> for Rational {
    #[inline]
    fn sum<I: Iterator<Item = &'a Rational>>(iter: I) -> Self {
        iter.fold(Rational::ZERO, Add::add)
    }
}

#[cfg(test)]
mod test_rational {
    use super::Rational;
    #[test]
    fn test_gcd() {
        assert_eq!(Rational::gcd(10, 2) , 2);
        assert_eq!(Rational::gcd(10, 3) , 1);
        assert_eq!(Rational::gcd(0 , 3) , 3);
        assert_eq!(Rational::gcd(3 , 3) , 3);
        assert_eq!(Rational::gcd(56, 42), 14);
        assert_eq!(Rational::gcd(3 , -3), 3);
        assert_eq!(Rational::gcd(-6, 3) , 3);
        assert_eq!(Rational::gcd(-4, -2), 2);
    } 
    #[test]
    fn test_add() {
        assert_eq!(Rational::ZERO + Rational::ZERO, 0.into());
        assert_eq!(Rational::ONE + Rational::ZERO, 1.into());
        assert_eq!(Rational::ZERO + Rational::ONE, 1.into());
        assert_eq!(Rational::ONE + Rational::ONE, 2.into());
        assert_eq!(Rational::new(1, 2) + Rational::new(7, 14), Rational::ONE);
        assert_eq!(Rational::new(1, 3) + Rational::new(2, 6), Rational::new(2, 3));
        assert_eq!(Rational::new(3, 14) + Rational::new(23, 25), Rational::new(397, 350));
        assert_eq!(Rational::new(859, 82) + Rational::new(10, 12), Rational::new(1391, 123));
        assert_eq!(Rational::new(-1,2) + Rational::new(7, 14), Rational::ZERO);
        assert_eq!((Rational::new(1, 3) + Rational::new(4, -6)).sign(), Rational::new(-1, 3));
    }
    #[test]
    fn test_mul() {
        assert_eq!(Rational::ZERO * Rational::ZERO, 0.into());
        assert_eq!(Rational::ONE * Rational::ZERO, 0.into());
        assert_eq!(Rational::ZERO * Rational::ONE, 0.into());
        assert_eq!(Rational::ONE * Rational::ONE, 1.into());
        assert_eq!(Rational::new(1, 2) * Rational::new(7, 14), Rational::new(1, 4));
        assert_eq!(Rational::new(1, 3) * Rational::new(2, 6), Rational::new(1, 9));
        assert_eq!(Rational::new(3, 14) * Rational::new(23, 25), Rational::new(69, 350));
        assert_eq!(Rational::new(859, 82) * Rational::new(10, 12), Rational::new(4295, 492));
        assert_eq!((Rational::new(-1,2) * Rational::new(7, 14)).sign(), Rational::new(-1, 4));
        assert_eq!((Rational::new(1, 3) * Rational::new(4, -6)).sign(), Rational::new(-2, 9));
    }
    #[test]
    fn test_sub() {
        assert_eq!(Rational::ZERO - Rational::ZERO, 0.into());
        assert_eq!(Rational::ONE - Rational::ZERO, 1.into());
        assert_eq!(Rational::ZERO - Rational::ONE, (-1).into());
        assert_eq!(Rational::ONE - Rational::ONE, 0.into());
        assert_eq!(Rational::ONE - Rational::new(1, 2), Rational::new(1, 2));
        assert_eq!(Rational::new(1, 3) - Rational::new(2, 6), Rational::ZERO);
        assert_eq!(Rational::new(3, 14) - Rational::new(23, 25), Rational::new(-247, 350));
        assert_eq!(Rational::new(859, 82) - Rational::new(10, 12), Rational::new(1186, 123));
        assert_eq!((Rational::new(-1,2) - Rational::new(7, 14)).sign(), (-1).into());
        assert_eq!((Rational::new(1, 3) - Rational::new(2, -6)).sign(), Rational::new(2, 3));
    }
    #[test]
    fn test_div() {
        // assert_eq!(Rational::ZERO / Rational::ZERO, Rational::NAN);
        assert_eq!(Rational::ZERO / Rational::ONE, 0.into());
        assert_eq!(Rational::ONE / Rational::ONE, 1.into());
        assert_eq!(Rational::ONE / Rational::new(1, 2), 2.into());
        assert_eq!(Rational::ONE / Rational::new(2, 1), Rational::new(1, 2));
        assert_eq!(Rational::new(1, 2) / Rational::new(14, 7), Rational::new(1, 4));
        assert_eq!(Rational::new(1, 3) / Rational::new(2, 6), Rational::new(1, 1));
        assert_eq!(Rational::new(3, 14) / Rational::new(23, 25), Rational::new(75, 322));
        assert_eq!(Rational::new(859, 82) * Rational::new(10, 12), Rational::new(4295, 492));
        assert_eq!((Rational::new(-1,2) * Rational::new(7, 14)).sign(), Rational::new(-1, 4));
        assert_eq!((Rational::new(1, 3) * Rational::new(4, -6)).sign(), Rational::new(-2, 9));
    }
}

// see rust's std src/core/internal_macros.rs 

// implements the unary operator "op &T"
// based on "op T" where T is expected to be `Copy`able
macro_rules! from_integers_rational {
    ($( $src: ty )+ ) => {$(
        impl From<$src> for Rational {
            #[inline]
            fn from(u: $src) -> Rational {
                Rational::new(u as isize, 1)
            }
        }
    )+}
}
macro_rules! forward_ref_unop {
    (impl $imp:ident, $method:ident for $t:ty) => {
        impl $imp for &$t {
            type Output = <$t as $imp>::Output;

            #[inline]
            fn $method(self) -> <$t as $imp>::Output {
                $imp::$method(*self)
            }
        }
    }
}

// implements binary operators "&T op U", "T op &U", "&T op &U"
// based on "T op U" where T and U are expected to be `Copy`able
macro_rules! forward_ref_binop {
    (impl $imp:ident, $method:ident for $t:ty, $u:ty) => {
        impl<'a> $imp<$u> for &'a $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, other)
            }
        }

        impl $imp<&$u> for $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: &$u) -> <$t as $imp<$u>>::Output {
                $imp::$method(self, *other)
            }
        }

        impl $imp<&$u> for &$t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: &$u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, *other)
            }
        }
    }
}

// implements "T op= &U", based on "T op= U"
// where U is expected to be `Copy`able
macro_rules! forward_ref_op_assign {
    (impl $imp:ident, $method:ident for $t:ty, $u:ty) => {
        impl $imp<&$u> for $t {
            #[inline]
            fn $method(&mut self, other: &$u) {
                $imp::$method(self, *other);
            }
        }
    }
}

forward_ref_binop! { impl Add, add for Rational, Rational }
forward_ref_binop! { impl Sub, sub for Rational, Rational }
forward_ref_binop! { impl Mul, mul for Rational, Rational }
forward_ref_binop! { impl Div, div for Rational, Rational }
forward_ref_unop!  { impl Neg, neg for Rational }
from_integers_rational! { u32 isize i32 }