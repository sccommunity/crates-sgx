extern crate num_integer;
extern crate num_traits;

use std::prelude::v1::*;
use num_integer::Roots;
use num_traits::checked_pow;
use num_traits::{AsPrimitive, PrimInt, Signed};
use std::f64::MANTISSA_DIGITS;
use std::fmt::Debug;
use std::mem;

trait TestInteger: Roots + PrimInt + Debug + AsPrimitive<f64> + 'static {}

impl<T> TestInteger for T where T: Roots + PrimInt + Debug + AsPrimitive<f64> + 'static {}

/// Check that each root is correct
///
/// If `x` is positive, check `rⁿ ≤ x < (r+1)ⁿ`.
/// If `x` is negative, check `(r-1)ⁿ < x ≤ rⁿ`.
fn check<T>(v: &[T], n: u32)
where
    T: TestInteger,
{
    for i in v {
        let rt = i.nth_root(n);
        // println!("nth_root({:?}, {}) = {:?}", i, n, rt);
        if n == 2 {
            assert_eq!(rt, i.sqrt());
        } else if n == 3 {
            assert_eq!(rt, i.cbrt());
        }
        if *i >= T::zero() {
            let rt1 = rt + T::one();
            assert!(rt.pow(n) <= *i);
            if let Some(x) = checked_pow(rt1, n as usize) {
                assert!(*i < x);
            }
        } else {
            let rt1 = rt - T::one();
            assert!(rt < T::zero());
            assert!(*i <= rt.pow(n));
            if let Some(x) = checked_pow(rt1, n as usize) {
                assert!(x < *i);
            }
        };
    }
}

/// Get the maximum value that will round down as `f64` (if any),
/// and its successor that will round up.
///
/// Important because the `std` implementations cast to `f64` to
/// get a close approximation of the roots.
fn mantissa_max<T>() -> Option<(T, T)>
where
    T: TestInteger,
{
    let bits = if T::min_value().is_zero() {
        8 * mem::size_of::<T>()
    } else {
        8 * mem::size_of::<T>() - 1
    };
    if bits > MANTISSA_DIGITS as usize {
        let rounding_bit = T::one() << (bits - MANTISSA_DIGITS as usize - 1);
        let x = T::max_value() - rounding_bit;

        let x1 = x + T::one();
        let x2 = x1 + T::one();
        assert!(x.as_() < x1.as_());
        assert_eq!(x1.as_(), x2.as_());

        Some((x, x1))
    } else {
        None
    }
}

fn extend<T>(v: &mut Vec<T>, start: T, end: T)
where
    T: TestInteger,
{
    let mut i = start;
    while i < end {
        v.push(i);
        i = i + T::one();
    }
    v.push(i);
}

fn extend_shl<T>(v: &mut Vec<T>, start: T, end: T, mask: T)
where
    T: TestInteger,
{
    let mut i = start;
    while i != end {
        v.push(i);
        i = (i << 1) & mask;
    }
}

fn extend_shr<T>(v: &mut Vec<T>, start: T, end: T)
where
    T: TestInteger,
{
    let mut i = start;
    while i != end {
        v.push(i);
        i = i >> 1;
    }
}

fn pos<T>() -> Vec<T>
where
    T: TestInteger,
    i8: AsPrimitive<T>,
{
    let mut v: Vec<T> = vec![];
    if mem::size_of::<T>() == 1 {
        extend(&mut v, T::zero(), T::max_value());
    } else {
        extend(&mut v, T::zero(), i8::max_value().as_());
        extend(
            &mut v,
            T::max_value() - i8::max_value().as_(),
            T::max_value(),
        );
        if let Some((i, j)) = mantissa_max::<T>() {
            v.push(i);
            v.push(j);
        }
        extend_shl(&mut v, T::max_value(), T::zero(), !T::min_value());
        extend_shr(&mut v, T::max_value(), T::zero());
    }
    v
}

fn neg<T>() -> Vec<T>
where
    T: TestInteger + Signed,
    i8: AsPrimitive<T>,
{
    let mut v: Vec<T> = vec![];
    if mem::size_of::<T>() <= 1 {
        extend(&mut v, T::min_value(), T::zero());
    } else {
        extend(&mut v, i8::min_value().as_(), T::zero());
        extend(
            &mut v,
            T::min_value(),
            T::min_value() - i8::min_value().as_(),
        );
        if let Some((i, j)) = mantissa_max::<T>() {
            v.push(-i);
            v.push(-j);
        }
        extend_shl(&mut v, -T::one(), T::min_value(), !T::zero());
        extend_shr(&mut v, T::min_value(), -T::one());
    }
    v
}

macro_rules! test_roots {
    ($I:ident, $U:ident) => {
        mod $I {
            use super::check;
            use super::neg;
            use num_integer::Roots;
            use super::pos;
            use std::mem;

            pub fn zeroth_root() {
                (123 as $I).nth_root(0);
            }

            pub fn sqrt() {
                check(&pos::<$I>(), 2);
            }

            pub fn sqrt_neg() {
                (-123 as $I).sqrt();
            }

            pub fn cbrt() {
                check(&pos::<$I>(), 3);
            }

            pub fn cbrt_neg() {
                check(&neg::<$I>(), 3);
            }

            pub fn nth_root() {
                let bits = 8 * mem::size_of::<$I>() as u32 - 1;
                let pos = pos::<$I>();
                for n in 4..bits {
                    check(&pos, n);
                }
            }

            pub fn nth_root_neg() {
                let bits = 8 * mem::size_of::<$I>() as u32 - 1;
                let neg = neg::<$I>();
                for n in 2..bits / 2 {
                    check(&neg, 2 * n + 1);
                }
            }

            pub fn bit_size() {
                let bits = 8 * mem::size_of::<$I>() as u32 - 1;
                assert_eq!($I::max_value().nth_root(bits - 1), 2);
                assert_eq!($I::max_value().nth_root(bits), 1);
                assert_eq!($I::min_value().nth_root(bits), -2);
                assert_eq!(($I::min_value() + 1).nth_root(bits), -1);
            }
        }

        mod $U {
            use super::check;
            use num_integer::Roots;
            use super::pos;
            use std::mem;

            pub fn zeroth_root() {
                (123 as $U).nth_root(0);
            }

            pub fn sqrt() {
                check(&pos::<$U>(), 2);
            }

            pub fn cbrt() {
                check(&pos::<$U>(), 3);
            }

            pub fn nth_root() {
                let bits = 8 * mem::size_of::<$I>() as u32 - 1;
                let pos = pos::<$I>();
                for n in 4..bits {
                    check(&pos, n);
                }
            }

            pub fn bit_size() {
                let bits = 8 * mem::size_of::<$U>() as u32;
                assert_eq!($U::max_value().nth_root(bits - 1), 2);
                assert_eq!($U::max_value().nth_root(bits), 1);
            }
        }
    };
}

test_roots!(i8, u8);
test_roots!(i16, u16);
test_roots!(i32, u32);
test_roots!(i64, u64);
test_roots!(i128, u128);
test_roots!(isize, usize);

pub fn run_tests() {
    use sgx_tunittest::*;
    rsgx_unit_tests!(
        || should_panic!(i8::zeroth_root()),
        i8::sqrt,
        || should_panic!(i8::sqrt_neg()),
        i8::cbrt,
        i8::cbrt_neg,
        i8::nth_root,
        i8::nth_root_neg,
        i8::bit_size,
        || should_panic!(u8::zeroth_root()),
        u8::sqrt,
        u8::cbrt,
        u8::nth_root,
        u8::bit_size,
        || should_panic!(i16::zeroth_root()),
        i16::sqrt,
        || should_panic!(i16::sqrt_neg()),
        i16::cbrt,
        i16::cbrt_neg,
        i16::nth_root,
        i16::nth_root_neg,
        i16::bit_size,
        || should_panic!(u16::zeroth_root()),
        u16::sqrt,
        u16::cbrt,
        u16::nth_root,
        u16::bit_size,
        || should_panic!(i32::zeroth_root()),
        i32::sqrt,
        || should_panic!(i32::sqrt_neg()),
        i32::cbrt,
        i32::cbrt_neg,
        i32::nth_root,
        i32::nth_root_neg,
        i32::bit_size,
        || should_panic!(u32::zeroth_root()),
        u32::sqrt,
        u32::cbrt,
        u32::nth_root,
        u32::bit_size,
        || should_panic!(i64::zeroth_root()),
        i64::sqrt,
        || should_panic!(i64::sqrt_neg()),
        i64::cbrt,
        i64::cbrt_neg,
        i64::nth_root,
        i64::nth_root_neg,
        i64::bit_size,
        || should_panic!(u64::zeroth_root()),
        u64::sqrt,
        u64::cbrt,
        u64::nth_root,
        u64::bit_size,
        || should_panic!(i128::zeroth_root()),
        i128::sqrt,
        || should_panic!(i128::sqrt_neg()),
        i128::cbrt,
        i128::cbrt_neg,
        i128::nth_root,
        i128::nth_root_neg,
        i128::bit_size,
        || should_panic!(u128::zeroth_root()),
        u128::sqrt,
        u128::cbrt,
        u128::nth_root,
        u128::bit_size,
        || should_panic!(isize::zeroth_root()),
        isize::sqrt,
        || should_panic!(isize::sqrt_neg()),
        isize::cbrt,
        isize::cbrt_neg,
        isize::nth_root,
        isize::nth_root_neg,
        isize::bit_size,
        || should_panic!(usize::zeroth_root()),
        usize::sqrt,
        usize::cbrt,
        usize::nth_root,
        usize::bit_size,
    );
}
