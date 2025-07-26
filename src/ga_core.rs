// src/ga_core.rs

use std::ops::{Add, Mul};

/// Represents the scalar values in Matzke's minimal algebra: {0, +1, -1}.
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Mod3(pub i8);

impl Mod3 {
    pub fn new(val: i8) -> Self {
        Mod3(val.signum())
    }
}

// Implement Modulo 3 addition as specified in the papers.
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

impl Mul for Mod3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Mod3(self.0 * rhs.0)
    }
}

/// A 2D Geometric Algebra Multivector. This is the state of an Existon.
/// It's a combination of a scalar, two vectors (e0, e1), and a bivector (e01).
/// Added Copy and Clone to fix move errors.
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Multivector {
    pub s: Mod3,   // Scalar component
    pub e0: Mod3,  // Vector e0 component
    pub e1: Mod3,  // Vector e1 component
    pub e01: Mod3, // Bivector e01 component (the "spinor")
}

impl Multivector {
    pub fn zero() -> Self {
        Multivector {
            s: Mod3::new(0),
            e0: Mod3::new(0),
            e1: Mod3::new(0),
            e01: Mod3::new(0),
        }
    }
}

// The core update rule: the Geometric Product.
// This defines how two Existons (multivectors) interact.
// Rules: e0*e0=1, e1*e1=1, e0*e1 = -e1*e0
impl Mul for Multivector {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Multivector::zero();
        // Scalar part
        result.s = result.s + (self.s * rhs.s);
        result.s = result.s + (self.e0 * rhs.e0);
        result.s = result.s + (self.e1 * rhs.e1);
        result.s = result.s + (self.e01 * rhs.e01 * Mod3::new(-1)); // e01*e01 = -1

        // e0 vector part
        result.e0 = result.e0 + (self.s * rhs.e0);
        result.e0 = result.e0 + (self.e0 * rhs.s);
        result.e0 = result.e0 + (self.e1 * rhs.e01 * Mod3::new(-1));
        result.e0 = result.e0 + (self.e01 * rhs.e1);

        // e1 vector part
        result.e1 = result.e1 + (self.s * rhs.e1);
        result.e1 = result.e1 + (self.e1 * rhs.s);
        result.e1 = result.e1 + (self.e0 * rhs.e01);
        result.e1 = result.e1 + (self.e01 * rhs.e0 * Mod3::new(-1));

        // e01 bivector part
        result.e01 = result.e01 + (self.s * rhs.e01);
        result.e01 = result.e01 + (self.e01 * rhs.s);
        result.e01 = result.e01 + (self.e0 * rhs.e1);
        result.e01 = result.e01 + (self.e1 * rhs.e0 * Mod3::new(-1));

        result
    }
}

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
