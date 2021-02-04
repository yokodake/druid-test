#![allow(unused_macros)]

use std::cmp::Ordering;
use std::ops::{Mul, Div, Add, Sub, Neg};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Ratio<A> { num: A 
                    , den: A
                    }

pub type Rational = Ratio<isize>;

impl<A> Ratio<A> {
pub const fn new(num: A, den: A) -> Self {
    Ratio{num, den}
}

pub const fn as_ref(&self) -> Ratio<&A> {
    Ratio::new(&self.den, &self.num)
}
pub fn numerator(self) -> A {
    self.num
}
pub fn denominator(self) -> A {
    self.den
}
}

impl Rational {
    pub const INFINITY : Rational = Self::new(1, 0);
    pub const NAN : Rational = Self::new(0, 0);
    pub const ZERO : Rational = Self::new(0, 1);
    pub const ONE : Rational = Self::new(1, 1);

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
        Rational::new(n / x, d / x)
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
        Self::reduce(self.num * other.den + other.num * self.num, self.den*other.den)
    }
}
impl Sub<Rational> for Rational {
    type Output = Rational;
    #[inline]
    fn sub(self, other: Rational) -> Rational {
        Self::reduce(self.num * other.den - other.num * self.num, self.den*other.den)
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
        Self::reduce(self.num * other.den, other.num * self.den)
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
    fn sum<I: Iterator<Item = Rational>>(iter: I) -> Self {
        iter.fold(Rational::ZERO, Add::add)
    }
}
impl<'a> std::iter::Sum<&'a Rational> for Rational {
    fn sum<I: Iterator<Item = &'a Rational>>(iter: I) -> Self {
        iter.fold(Rational::ZERO, Add::add)
    }
}

mod test_rational {
    #[test]
    fn test_gcd() {
        use super::Rational;
        assert_eq!(Rational::gcd(10, 2) , 2);
        assert_eq!(Rational::gcd(10, 3) , 1);
        assert_eq!(Rational::gcd(0 , 3) , 3);
        assert_eq!(Rational::gcd(3 , 3) , 3);
        assert_eq!(Rational::gcd(56, 42), 14);
        assert_eq!(Rational::gcd(3 , -3), 3);
        assert_eq!(Rational::gcd(-6, 3) , 3);
        assert_eq!(Rational::gcd(-4, -2), 2);
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
// forward_ref_binop! { impl Div, sub for Rational, Rational }
forward_ref_unop!  { impl Neg, neg for Rational }
from_integers_rational! { u32 isize i32 }