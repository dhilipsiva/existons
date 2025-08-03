#![allow(clippy::suspicious_arithmetic_impl)]
use rand::{Rng, rng};
use std::ops::{Add, Mul};

//================================================================================
// Mod3 - A Tristate Scalar Value {-1, 0, 1}
//================================================================================

/// Represents a tristate scalar value `{0, +1, -1}`.
///
/// This is the fundamental numeric type in this algebra, ensuring all calculations
/// remain within a minimal, closed system as described in Doug Matzke's work[cite: 145, 208, 1095].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
/// For example, `1 + 1 = -1` and `-1 + -1 = 1`[cite: 215, 1097].
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

/// A Geometric Algebra Multivector for a `Cl(p,0)` algebra over `Mod3` scalars.
///
/// This structure represents the complete state of a single Existon in a
/// `p`-dimensional space. It is a dynamic structure capable of handling the
/// hyperdimensional nature of Matzke's "Source Science"[cite: 99, 1212].
/// The `coefficients` vector holds the `Mod3` values for each basis blade.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Multivector {
    /// The number of basis vectors (dimensions) of the algebra.
    pub p: usize,
    /// The coefficients for the `2^p` basis blades. The index of the vector
    /// corresponds to the integer representation of the basis blade.
    /// E.g., for p=3: index 5 (0b101) is blade `e_0 * e_2`.
    pub coefficients: Vec<Mod3>,
}

impl Multivector {
    /// Creates a new zero `Multivector` in a space with `p` dimensions.
    pub fn zero(p: usize) -> Self {
        Multivector {
            p,
            coefficients: vec![Mod3::new(0); 1 << p],
        }
    }

    /// Creates a new `Multivector` with randomized `Mod3` coefficients.
    pub fn random(p: usize) -> Self {
        let mut rng = rng();
        let coefficients = (0..(1 << p))
            .map(|_| Mod3::new(rng.random_range(-1..=1)))
            .collect();
        Multivector { p, coefficients }
    }
}

/// Implements the core update rule: the Geometric Product `a * b`.
///
/// This defines how two Existons interact. It is a generalized implementation
/// for any `p`-dimensional `Cl(p,0)` algebra, where `e_i * e_i = 1`. The anticommutative
/// nature (`e_i * e_j = -e_j * e_i`) is handled by counting bit swaps[cite: 148, 1113].
impl Mul for &Multivector {
    type Output = Multivector;

    fn mul(self, rhs: &Multivector) -> Self::Output {
        // The two multivectors must be from the same algebra.
        assert_eq!(self.p, rhs.p);

        let mut result = Multivector::zero(self.p);
        let num_blades = 1 << self.p;

        // Iterate over all basis blades of the first multivector (a).
        for i in 0..num_blades {
            let a_coeff = self.coefficients[i];
            // Skip if the coefficient is zero, as it won't contribute to the sum.
            if a_coeff.0 == 0 {
                continue;
            }

            // Iterate over all basis blades of the second multivector (b).
            for j in 0..num_blades {
                let b_coeff = rhs.coefficients[j];
                // Skip if the coefficient is zero.
                if b_coeff.0 == 0 {
                    continue;
                }

                // The resulting basis blade is the XOR of the two input blades' bitmasks.
                // This correctly handles `e_i * e_i = 1` by removing common basis vectors.
                let result_blade = i ^ j;

                // --- CORRECTED SIGN CALCULATION ---
                // To find the sign, we count the number of times a basis vector from `rhs` (j)
                // must swap places with a basis vector of `self` (i) that has a higher index.
                let mut sign_flips = 0;
                for bit_j in 0..self.p {
                    // If the k-th basis vector exists in blade j...
                    if (j >> bit_j) & 1 != 0 {
                        // ...count how many basis vectors in blade i have a higher index.
                        // These are the vectors that e_k must be moved past, causing a sign flip.
                        let mask_i = i >> (bit_j + 1);
                        sign_flips += mask_i.count_ones();
                    }
                }

                let sign = if sign_flips % 2 == 0 { 1 } else { -1 };

                // Calculate the product of the coefficients and apply the sign.
                let product_coeff = a_coeff * b_coeff * Mod3::new(sign);

                // Add the result to the correct component of the final multivector.
                result.coefficients[result_blade] =
                    result.coefficients[result_blade] + product_coeff;
            }
        }
        result
    }
}

/// Implements component-wise addition for two `Multivector` instances.
///
/// This is used to sum the states of neighboring Existons to create an 'operator'[cite: 102, 1231].
impl Add for &Multivector {
    type Output = Multivector;
    fn add(self, rhs: &Multivector) -> Self::Output {
        assert_eq!(self.p, rhs.p);
        let mut result = Multivector::zero(self.p);
        for i in 0..(1 << self.p) {
            result.coefficients[i] = self.coefficients[i] + rhs.coefficients[i];
        }
        result
    }
}
