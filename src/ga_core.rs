#![allow(clippy::suspicious_arithmetic_impl)]
use std::ops::{Add, Mul};

use rand::Rng;

//================================================================================
// Mod3 - A Tristate Scalar Value {-1, 0, 1}
//================================================================================

/// Represents a tristate scalar value `{0, +1, -1}`.
///
/// This is the fundamental numeric type in this algebra, ensuring all calculations
/// remain within a minimal, closed system as described in Doug Matzke's work.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Mod3(pub i8);

impl Mod3 {
    /// Creates a new `Mod3` value, normalizing any `i8` to its sign (`-1`, `0`, or `1`).
    pub fn new(val: i8) -> Self {
        Mod3(val.signum())
    }
}

/// Implements a custom wrapping addition `Z(3)`.
///
/// If the sum exceeds the bounds of `{-1, 1}`, it wraps around.
/// For example, `1 + 1 = -1` and `-1 + -1 = 1`.
impl Add for Mod3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let sum = self.0 + rhs.0;
        if sum > 1 {
            Mod3(-1)
        } else if sum < -1 {
            Mod3(1)
        } else {
            Mod3(sum)
        }
    }
}

/// Implements standard multiplication for `Mod3` values.
impl Mul for Mod3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Mod3(self.0 * rhs.0)
    }
}

//================================================================================
// Multivector - The State of an Existon
//================================================================================

/// A 2D Geometric Algebra Multivector for the Cl(2,0) algebra.
///
/// This structure represents the complete state of a single Existon. It's a composite
/// value containing a scalar (grade-0), two vectors (grade-1), and a bivector (grade-2).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Multivector {
    /// Grade-0 component (scalar).
    pub s: Mod3,
    /// Grade-1 component (e0 vector basis).
    pub e0: Mod3,
    /// Grade-1 component (e1 vector basis).
    pub e1: Mod3,
    /// Grade-2 component (e01 bivector basis, also the "pseudoscalar" or "spinor").
    pub e01: Mod3,
}

impl Multivector {
    /// Creates a new `Multivector` with all components set to zero.
    pub fn zero() -> Self {
        Multivector {
            s: Mod3::new(0),
            e0: Mod3::new(0),
            e1: Mod3::new(0),
            e01: Mod3::new(0),
        }
    }

    /// Creates a new `Multivector` with randomized tristate components.
    pub fn random() -> Self {
        let mut rng = rand::rng();
        Multivector {
            s: Mod3::new(rng.random_range(-1..=1)),
            e0: Mod3::new(rng.random_range(-1..=1)),
            e1: Mod3::new(rng.random_range(-1..=1)),
            e01: Mod3::new(rng.random_range(-1..=1)),
        }
    }
}

/// Implements the core update rule: the Geometric Product `a * b`.
///
/// This defines how two Existons interact. The product is derived from the
/// basis vector multiplication rules for Cl(2,0) algebra:
/// - `e0 * e0 = 1`
/// - `e1 * e1 = 1`
/// - `e0 * e1 = e01`
/// - `e1 * e0 = -e01`
/// - `e01 * e01 = -1`
impl Mul for Multivector {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Multivector::zero();

        // --- Grade-0 (Scalar) component calculation ---
        result.s = result.s + (self.s * rhs.s); // s*s
        result.s = result.s + (self.e0 * rhs.e0); // e0*e0 -> 1
        result.s = result.s + (self.e1 * rhs.e1); // e1*e1 -> 1
        result.s = result.s + (self.e01 * rhs.e01 * Mod3::new(-1)); // e01*e01 -> -1

        // --- Grade-1 (e0 Vector) component calculation ---
        result.e0 = result.e0 + (self.s * rhs.e0); // s*e0
        result.e0 = result.e0 + (self.e0 * rhs.s); // e0*s
        result.e0 = result.e0 + (self.e1 * rhs.e01 * Mod3::new(-1)); // e1*e01 -> -e0
        result.e0 = result.e0 + (self.e01 * rhs.e1); // e01*e1 -> e0

        // --- Grade-1 (e1 Vector) component calculation ---
        result.e1 = result.e1 + (self.s * rhs.e1); // s*e1
        result.e1 = result.e1 + (self.e1 * rhs.s); // e1*s
        result.e1 = result.e1 + (self.e0 * rhs.e01); // e0*e01 -> e1
        result.e1 = result.e1 + (self.e01 * rhs.e0 * Mod3::new(-1)); // e01*e0 -> -e1

        // --- Grade-2 (e01 Bivector) component calculation ---
        result.e01 = result.e01 + (self.s * rhs.e01); // s*e01
        result.e01 = result.e01 + (self.e01 * rhs.s); // e01*s
        result.e01 = result.e01 + (self.e0 * rhs.e1); // e0*e1 -> e01
        result.e01 = result.e01 + (self.e1 * rhs.e0 * Mod3::new(-1)); // e1*e0 -> -e01

        result
    }
}

/// Implements component-wise addition for two `Multivector` instances.
///
/// This is used to sum the states of neighboring Existons to create an 'operator'.
impl Add for Multivector {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Multivector {
            s: self.s + rhs.s,
            e0: self.e0 + rhs.e0,
            e1: self.e1 + rhs.e1,
            e01: self.e01 + rhs.e01,
        }
    }
}
