#![allow(dead_code)]

//! Implementation of some comparaison function for floating point

/// Compare two f32 values with precision given in argument.
/// If the reference is equal to zero, we return true if |value| < precision, false otherwise.
/// Else, we return true if |value - reference| / |reference| < precision, false otherwise.
pub fn f32_is_approx_equal(value: f32, reference: f32, precision: f32) -> bool {
    let abs_reference: f32 = reference.abs();

    if abs_reference > 0.0 {
        return (value - reference).abs() / abs_reference < precision;
    } else {
        return value.abs() < precision;
    }
}

/// Compare two f64 values with precision given in argument.
/// If the reference is equal to zero, we return true if |value| < precision, false otherwise.
/// Else, we return true if |value - reference| / |reference| < precision, false otherwise.
pub fn f64_is_approx_equal(value: f64, reference: f64, precision: f64) -> bool {
    let abs_reference: f64 = reference.abs();

    if abs_reference > 0.0 {
        return (value - reference).abs() / abs_reference < precision;
    } else {
        return value.abs() < precision;
    }
}
