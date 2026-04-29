#![allow(dead_code)]

//! For floating point vector, it compute the sum of absolutes values.
//!
//! # Exemple with floating point values
//! ```
//! use blars::complex::Complex;
//! use blars::compare::f32_is_approx_equal;
//! use blars::vec_ops::abs_sum::abs_sum_f32;
//!
//! let x: Vec<f32> = vec![1.0, -2.0, -3.0, 4.0, -5.0, 6.0];
//!
//! let mut stride: usize = 1;
//! let mut result: f32 = abs_sum_f32(&x, stride);
//! assert!(f32_is_approx_equal(result, 21.0, 0.01));
//!
//! stride = 2;
//! result = abs_sum_f32(&x, stride);
//! assert!(f32_is_approx_equal(result, 9.0, 0.01));
//! ```
//! For complex vector, it compute the sum of (|Re(z_i)| + |Im(z_i)|) values.
//!
//! # Exemple with Complex floating point values
//! ```
//! use blars::complex::Complex;
//! use blars::compare::f32_is_approx_equal;
//! use blars::vec_ops::abs_sum::abs_sum_complexf32;
//!
//! let x: Vec<Complex<f32>> = vec![Complex::new(1.0, 1.0), Complex::new(-2.0, 1.0), Complex::new(-3.0, 1.0),
//!                                 Complex::new(4.0, 1.0), Complex::new(-5.0, 1.0), Complex::new(6.0, 1.0)];
//!
//! let mut stride: usize = 1;
//! let mut result: f32 = abs_sum_complexf32(&x, stride);
//! assert!(f32_is_approx_equal(result, 27.0, 0.01));
//!
//! stride = 2;
//! result = abs_sum_complexf32(&x, stride);
//! assert!(f32_is_approx_equal(result, 12.0, 0.01));
//! ```

use crate::complex::Complex;

/// Compute the sum of absolutes values for vector of f32
/// It takes in argument a slice of f32 value containing the data
/// and a stride between elements of vector
pub fn abs_sum_f32<T>(x: &T, stride: usize) -> f32
where
    T: std::convert::AsRef<[f32]>,
{
    if stride == 1 {
        return x.as_ref().iter().fold(0.0, |acc, elem| acc + elem.abs());
    } else {
        return x
            .as_ref()
            .iter()
            .step_by(stride)
            .fold(0.0, |acc, elem| acc + elem.abs());
    }
}

/// Compute the sum of absolutes values for vector of f64
/// It takes in argument a slice of f64 value containing the data
/// and a stride between elements of vector
pub fn abs_sum_f64<T>(x: &T, stride: usize) -> f64
where
    T: std::convert::AsRef<[f64]>,
{
    if stride == 1 {
        return x.as_ref().iter().fold(0.0, |acc, elem| acc + elem.abs());
    } else {
        return x
            .as_ref()
            .iter()
            .step_by(stride)
            .fold(0.0, |acc, elem| acc + elem.abs());
    }
}

/// Compute the sum of absolutes values for vector of Complex 32-bits float
/// It takes in argument a slice of Complex 32-bits float value containing the data
/// and a stride between elements of vector
pub fn abs_sum_complexf32<T>(x: &T, stride: usize) -> f32
where
    T: std::convert::AsRef<[Complex<f32>]>,
{
    if stride == 1 {
        return x
            .as_ref()
            .iter()
            .fold(0.0, |acc, elem| acc + elem.re.abs() + elem.im.abs());
    } else {
        return x
            .as_ref()
            .iter()
            .step_by(stride)
            .fold(0.0, |acc, elem| acc + elem.re.abs() + elem.im.abs());
    }
}

/// Compute the sum of absolutes values for vector of Complex 64-bits float
/// It takes in argument a slice of Complex 64-bits float value containing the data
/// and a stride between elements of vector
pub fn abs_sum_complexf64<T>(x: &T, stride: usize) -> f64
where
    T: std::convert::AsRef<[Complex<f64>]>,
{
    if stride == 1 {
        return x
            .as_ref()
            .iter()
            .fold(0.0, |acc, elem| acc + elem.re.abs() + elem.im.abs());
    } else {
        return x
            .as_ref()
            .iter()
            .step_by(stride)
            .fold(0.0, |acc, elem| acc + elem.re.abs() + elem.im.abs());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compare::{f32_is_approx_equal, f64_is_approx_equal};

    #[test]
    fn test_abs_sum_f32() {
        let x: Vec<f32> = vec![1.0, -2.0, -3.0, 4.0, -5.0, 6.0];
        let mut stride: usize = 1;

        let precision: f32 = 0.01;

        let mut reference: f32 = 21.0;
        let mut result: f32 = abs_sum_f32(&x, stride);
        assert!(f32_is_approx_equal(result, reference, precision));

        stride = 2;

        reference = 9.0;
        result = abs_sum_f32(&x, stride);
        assert!(f32_is_approx_equal(result, reference, precision));
    }

    #[test]
    fn test_abs_sum_f64() {
        let x: Vec<f64> = vec![1.0, -2.0, -3.0, 4.0, -5.0, 6.0];
        let mut stride: usize = 1;

        let precision: f64 = 0.01;

        let mut reference: f64 = 21.0;
        let mut result: f64 = abs_sum_f64(&x, stride);
        assert!(f64_is_approx_equal(result, reference, precision));

        stride = 2;

        reference = 9.0;
        result = abs_sum_f64(&x, stride);
        assert!(f64_is_approx_equal(result, reference, precision));
    }

    #[test]
    fn test_abs_sum_complexf32() {
        let x: Vec<Complex<f32>> = vec![
            Complex::new(1.0, 1.0),
            Complex::new(-2.0, 1.0),
            Complex::new(-3.0, 1.0),
            Complex::new(4.0, 1.0),
            Complex::new(-5.0, 1.0),
            Complex::new(6.0, 1.0),
        ];

        let mut stride: usize = 1;

        let precision: f32 = 0.01;

        let mut reference: f32 = 27.0;
        let mut result: f32 = abs_sum_complexf32(&x, stride);
        assert!(f32_is_approx_equal(result, reference, precision));

        stride = 2;

        reference = 12.0;
        result = abs_sum_complexf32(&x, stride);
        assert!(f32_is_approx_equal(result, reference, precision));
    }

    #[test]
    fn test_abs_sum_complexf64() {
        let x: Vec<Complex<f64>> = vec![
            Complex::new(1.0, 1.0),
            Complex::new(-2.0, 1.0),
            Complex::new(-3.0, 1.0),
            Complex::new(4.0, 1.0),
            Complex::new(-5.0, 1.0),
            Complex::new(6.0, 1.0),
        ];

        let mut stride: usize = 1;

        let precision: f64 = 0.01;

        let mut reference: f64 = 27.0;
        let mut result: f64 = abs_sum_complexf64(&x, stride);
        assert!(f64_is_approx_equal(result, reference, precision));

        stride = 2;

        reference = 12.0;
        result = abs_sum_complexf64(&x, stride);
        assert!(f64_is_approx_equal(result, reference, precision));
    }
}
