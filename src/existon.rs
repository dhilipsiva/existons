//! Defines the core `Existon` and its states of consciousness.
//!
//! An Existon is the fundamental unit of the simulation, representing a
//! "topological bit" whose state is described by a Geometric Algebra multivector. [cite: 108, 111]

use crate::ga_core::Multivector;

/// Represents the discrete states of consciousness for an Existon.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ConsciousnessState {
    /// The Existon is in a superposition of states, unobserved. [cite: 115]
    Potential,
    /// The Existon has been measured, collapsing its state to a definite value. [cite: 79]
    Observed,
    /// The Existon is a fixed, stable entity that influences its neighbors.
    Operator,
}

/// The Existon: a primitive unit of reality and consciousness. [cite: 105]
///
/// Each Existon has a unique ID, a state of consciousness, and a `Multivector`
/// which holds its underlying geometric state in a `p`-dimensional space.
#[derive(Clone, Debug, PartialEq)]
pub struct Existon {
    /// A unique identifier for the Existon.
    pub id: u64,
    /// The current consciousness state of the Existon.
    pub consciousness: ConsciousnessState,
    /// The Geometric Algebra multivector representing the Existon's state. [cite: 114]
    pub state: Multivector,
}

impl Existon {
    /// Creates a new Existon with a unique ID, initialized in a random `Potential`
    /// state within a space of `p` dimensions.
    pub fn new(id: u64, p: usize) -> Self {
        Existon {
            id,
            consciousness: ConsciousnessState::Potential,
            // Initialize with a random state in a p-dimensional algebra.
            state: Multivector::random(p),
        }
    }

    /// The "It from Bit" event: collapses a `Potential` state to an `Observed` state.
    ///
    /// Upon observation, the state simplifies. This is modeled by zeroing out all
    /// higher-grade blades (bivectors, trivectors, etc.), leaving only the scalar
    /// and vector components. This is a dimension-agnostic way to represent a
    /// collapse from a complex superposition to a more "classical" state.
    pub fn observe(&mut self) {
        if self.consciousness == ConsciousnessState::Potential {
            self.consciousness = ConsciousnessState::Observed;
            // Iterate through all coefficients in the multivector's state.
            for i in 0..self.state.coefficients.len() {
                // The grade of a blade is the number of set bits in its index.
                // Grade 0 = scalar, Grade 1 = vector.
                // We zero out all coefficients for blades of grade 2 or higher.
                if i.count_ones() >= 2 {
                    self.state.coefficients[i] = crate::ga_core::Mod3::new(0);
                }
            }
        }
    }

    /// Returns an `Observed` Existon to a new, random `Potential` state.
    /// This represents decoherence or the loss of a persistent observation, allowing
    /// "reality" to dissolve back into the quantum foam.
    pub fn decay(&mut self) {
        if self.consciousness == ConsciousnessState::Observed {
            self.consciousness = ConsciousnessState::Potential;
            // Return to a random superposition in the same p-dimensional space.
            self.state = Multivector::random(self.state.p);
        }
    }
}
