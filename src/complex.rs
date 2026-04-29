#![allow(dead_code)]

//! Define complex number
//!
//! The structure **Complex** provides feature to work with complex.
//!
//! # Usage of complex number
//! ```
//! use blars::complex::Complex;
//! use blars::compare::f32_is_approx_equal;
//!
//! let z: Complex<f32> = Complex::new(2.0, 1.0);
//! assert!(f32_is_approx_equal(z.re, 2.0, 0.01));
//! assert!(f32_is_approx_equal(z.im, 1.0, 0.01));
//!
//! let zc: Complex<f32> = z.conjugate();
//! assert!(f32_is_approx_equal(zc.re, 2.0, 0.01));
//! assert!(f32_is_approx_equal(zc.im, -1.0, 0.01));
//!
//! let z_abs: f32 = z.abs();
//! let zc_abs: f32 = zc.abs();
//! assert!(f32_is_approx_equal(z_abs, zc_abs, 0.01));
//! ```
//!
//! This module provides the operators '+', '-' (unary and binary version), '\*' and '/' implemented for **Complex**, as well as
//! the operators  '+=', '-=', '*=' and '/='.
//!
//! # Computation with complex number
//! ```
//! use blars::complex::Complex;
//! use blars::compare::f32_is_approx_equal;
//!
//! let mut z: Complex<f32> = Complex::new(2.0, 1.0);
//! let mut w: Complex<f32> = Complex::new(1.0, 2.0);
//!
//! let zneg: Complex<f32> = -z;
//! assert!(f32_is_approx_equal(zneg.re, -2.0, 0.01));
//! assert!(f32_is_approx_equal(zneg.im, -1.0, 0.01));
//!
//! let sum: Complex<f32> = z + w;
//! assert!(f32_is_approx_equal(sum.re, 3.0, 0.01));
//! assert!(f32_is_approx_equal(sum.im, 3.0, 0.01));
//!
//! let sub: Complex<f32> = z - w;
//! assert!(f32_is_approx_equal(sub.re, 1.0, 0.01));
//! assert!(f32_is_approx_equal(sub.im, -1.0, 0.01));
//!
//! let r: Complex<f32> = Complex::new(1.0, 0.0);
//!
//! z *= r;
//! assert!(f32_is_approx_equal(z.re, 2.0, 0.01));
//! assert!(f32_is_approx_equal(z.im, 1.0, 0.01));
//!
//! w /= r;
//! assert!(f32_is_approx_equal(w.re, 1.0, 0.01));
//! assert!(f32_is_approx_equal(w.im, 2.0, 0.01));
//! ```

use std::convert::From;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Structure defining a complex number
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex<T> {
    /// Real part
    pub re: T,
    /// Imaginary part
    pub im: T,
}

impl<T> Complex<T>
where
    T: Neg<Output = T>,
    T: Copy,
{
    /// Create complex number from real and imaginary part
    pub fn new(re: T, im: T) -> Complex<T> {
        return Self { re, im };
    }

    /// Compute the conjugate
    /// Let a complex number x + iy, then its conjugate corresponds to complex number x - iy
    pub fn conjugate(&self) -> Complex<T> {
        return Complex::new(self.re, -self.im);
    }
}

impl Complex<f32> {
    /// Compute the absolute value.
    /// Let complex number x + iy, M(x, y) the point in complex plan and O(0, 0) its origin.
    /// Then the absolute value correspond to norm of vector OM.
    pub fn abs(&self) -> f32 {
        return (self.re * self.re + self.im * self.im).sqrt();
    }
}

impl From<f32> for Complex<f32> {
    /// Convert a 32-bit floating point into 32-bit complex number with a imaginary part equal to zero.
    fn from(value: f32) -> Self {
        return Self { re: value, im: 0.0 };
    }
}

impl Complex<f64> {
    /// Compute the absolute value.
    /// Let complex number x + iy, M(x, y) the point in complex plan and O(0, 0) its origin.
    /// Then the absolute value correspond to norm of vector OM.
    pub fn abs(&self) -> f64 {
        return (self.re * self.re + self.im * self.im).sqrt();
    }
}

impl From<f64> for Complex<f64> {
    /// Convert a 64-bit floating point into 64-bit complex number with a imaginary part equal to zero.
    fn from(value: f64) -> Self {
        return Self { re: value, im: 0.0 };
    }
}

impl<T> Neg for Complex<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        return Self {
            re: -self.re,
            im: -self.im,
        };
    }
}

impl<T> Add for Complex<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    /// Perform the sum of two complex numbers.
    /// Let z = a + ib and w = c + id two complex numbers.
    /// Then z + w = (a + c) + i(b + d)
    fn add(self, rhs: Self) -> Self::Output {
        return Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        };
    }
}

impl<T> AddAssign for Complex<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl<T> Sub for Complex<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    /// Perform the substration of two complex numbers.
    /// Let z = a + ib and w = c + id two complex numbers.
    /// Then z - w = (a - c) + i(b - d)
    fn sub(self, rhs: Self) -> Self::Output {
        return Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        };
    }
}

impl<T> SubAssign for Complex<T>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.re -= rhs.re;
        self.im -= rhs.im;
    }
}

impl<T> Mul for Complex<T>
where
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Copy,
{
    type Output = Self;

    /// Perform the substration of two complex numbers.
    /// Let z = a + ib and w = c + id two complex numbers.
    /// Then z * w = (ac - bd) + i(ad + bc)
    fn mul(self, rhs: Self) -> Self::Output {
        return Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        };
    }
}

impl<T> MulAssign for Complex<T>
where
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Copy,
{
    fn mul_assign(&mut self, rhs: Self) {
        let re: T = self.re;

        self.re = re * rhs.re - self.im * rhs.im;
        self.im = re * rhs.im + self.im * rhs.re;
    }
}

impl<T> Div for Complex<T>
where
    T: Div<Output = T>
        + Mul<Output = T>
        + Add<Output = T>
        + Sub<Output = T>
        + Neg<Output = T>
        + Copy,
{
    type Output = Self;

    /// Perform the substration of two complex numbers.
    /// Let z = a + ib and w = c + id two complex numbers.
    /// Then z / w = ((ac + bd) / (a * a + b * b)) + i((bc - ad) / (a * a + b * b))
    fn div(self, rhs: Self) -> Self::Output {
        let abs_square: T = rhs.re * rhs.re + rhs.im * rhs.im;
        let result: Complex<T> = self * rhs.conjugate();

        return Self {
            re: result.re / abs_square,
            im: result.im / abs_square,
        };
    }
}

impl<T> DivAssign for Complex<T>
where
    T: DivAssign + Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Neg<Output = T> + Copy,
{
    fn div_assign(&mut self, rhs: Self) {
        let abs_square: T = rhs.re * rhs.re + rhs.im * rhs.im;

        *self *= rhs.conjugate();

        self.re /= abs_square;
        self.im /= abs_square;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compare::f32_is_approx_equal;

    #[test]
    fn test_complex_new() {
        let re: f32 = 1.0;
        let im: f32 = 1.0;
        let z: Complex<f32> = Complex::new(re, im);

        let precision: f32 = 0.01;
        assert!(f32_is_approx_equal(z.re, re, precision));
        assert!(f32_is_approx_equal(z.im, im, precision));
    }

    #[test]
    fn test_complex_conjugate() {
        let re: f32 = 1.0;
        let im: f32 = 1.0;
        let z: Complex<f32> = Complex::new(re, im);
        let zc: Complex<f32> = z.conjugate();

        let precision: f32 = 0.01;
        assert!(f32_is_approx_equal(zc.re, re, precision));
        assert!(f32_is_approx_equal(zc.im, -im, precision));
    }

    #[test]
    fn test_complex_abs() {
        let re: f32 = 1.0;
        let im: f32 = 1.0;
        let z: Complex<f32> = Complex::new(re, im);

        let abs_reference: f32 = (re * re + im * im).sqrt();
        let precision: f32 = 0.01;
        assert!(f32_is_approx_equal(z.abs(), abs_reference, precision));
    }

    #[test]
    fn test_complex_neg() {
        let re: f32 = 1.0;
        let im: f32 = 1.0;
        let z: Complex<f32> = Complex::new(re, im);
        let zneg: Complex<f32> = -z;

        let precision: f32 = 0.01;
        assert!(f32_is_approx_equal(zneg.re, -re, precision));
        assert!(f32_is_approx_equal(zneg.im, -im, precision));
    }

    #[test]
    fn test_complex_add() {
        let lhs: Complex<f32> = Complex::new(1.0, 2.0);
        let rhs: Complex<f32> = Complex::new(2.0, 1.0);

        let z: Complex<f32> = lhs + rhs;

        let precision: f32 = 0.01;
        assert!(f32_is_approx_equal(z.re, lhs.re + rhs.re, precision));
        assert!(f32_is_approx_equal(z.im, lhs.im + rhs.im, precision));
    }

    #[test]
    fn test_complex_from_float() {
        let lhs: Complex<f32> = Complex::new(1.0, 2.0);
        let rhs: Complex<f32> = Complex::from(2.0);

        let z: Complex<f32> = lhs + rhs;

        let precision: f32 = 0.01;
        assert!(f32_is_approx_equal(z.re, lhs.re + rhs.re, precision));
        assert!(f32_is_approx_equal(z.im, lhs.im + rhs.im, precision));

        let value: f32 = 3.0;
        let w: Complex<f32> = Complex::from(value) * lhs;

        assert!(f32_is_approx_equal(w.re, value * lhs.re, precision));
        assert!(f32_is_approx_equal(w.im, value * lhs.im, precision));
    }

    #[test]
    fn test_complex_add_assign() {
        let lhs: Complex<f32> = Complex::new(1.0, 2.0);
        let rhs: Complex<f32> = Complex::new(2.0, 1.0);

        let mut z: Complex<f32> = lhs;
        z += rhs;

        let precision: f32 = 0.01;
        assert!(f32_is_approx_equal(z.re, lhs.re + rhs.re, precision));
        assert!(f32_is_approx_equal(z.im, lhs.im + rhs.im, precision));
    }

    #[test]
    fn test_complex_sub() {
        let lhs: Complex<f32> = Complex::new(1.0, 2.0);
        let rhs: Complex<f32> = Complex::new(2.0, 1.0);

        let z: Complex<f32> = lhs - rhs;

        let precision: f32 = 0.01;
        assert!(f32_is_approx_equal(z.re, lhs.re - rhs.re, precision));
        assert!(f32_is_approx_equal(z.im, lhs.im - rhs.im, precision));
    }

    #[test]
    fn test_complex_sub_assign() {
        let lhs: Complex<f32> = Complex::new(1.0, 2.0);
        let rhs: Complex<f32> = Complex::new(2.0, 1.0);

        let mut z: Complex<f32> = lhs;
        z -= rhs;

        let precision: f32 = 0.01;
        assert!(f32_is_approx_equal(z.re, lhs.re - rhs.re, precision));
        assert!(f32_is_approx_equal(z.im, lhs.im - rhs.im, precision));
    }

    #[test]
    fn test_complex_mul() {
        let lhs: Complex<f32> = Complex::new(1.0, 2.0);
        let rhs: Complex<f32> = Complex::new(2.0, 1.0);

        let z: Complex<f32> = lhs * rhs;

        let precision: f32 = 0.01;

        assert!(f32_is_approx_equal(
            z.re,
            lhs.re * rhs.re - lhs.im * rhs.im,
            precision
        ));

        assert!(f32_is_approx_equal(
            z.im,
            lhs.re * rhs.im + lhs.im * rhs.re,
            precision
        ));
    }

    #[test]
    fn test_complex_mul_assign() {
        let lhs: Complex<f32> = Complex::new(1.0, 2.0);
        let rhs: Complex<f32> = Complex::new(2.0, 1.0);

        let mut z: Complex<f32> = lhs;
        z *= rhs;

        let precision: f32 = 0.01;

        assert!(f32_is_approx_equal(
            z.re,
            lhs.re * rhs.re - lhs.im * rhs.im,
            precision
        ));

        assert!(f32_is_approx_equal(
            z.im,
            lhs.re * rhs.im + lhs.im * rhs.re,
            precision
        ));
    }
    #[test]
    fn test_complex_div() {
        let lhs: Complex<f32> = Complex::new(1.0, 2.0);
        let rhs: Complex<f32> = Complex::new(1.0, 0.0);

        let z: Complex<f32> = lhs / rhs;

        let precision: f32 = 0.01;
        assert!(f32_is_approx_equal(z.re, lhs.re, precision));
        assert!(f32_is_approx_equal(z.im, lhs.im, precision));
    }

    #[test]
    fn test_complex_div_assign() {
        let lhs: Complex<f32> = Complex::new(1.0, 2.0);
        let rhs: Complex<f32> = Complex::new(2.0, 0.0);

        let mut z: Complex<f32> = lhs;
        z /= rhs;

        let precision: f32 = 0.01;
        assert!(f32_is_approx_equal(z.re, lhs.re * 0.5, precision));
        assert!(f32_is_approx_equal(z.im, lhs.im * 0.5, precision));
    }
}
