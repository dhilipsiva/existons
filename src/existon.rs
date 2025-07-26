//! Defines the core `Existon` and its states of consciousness.
//!
//! An Existon is the fundamental unit of the simulation, representing a
//! "topological bit" whose state is described by a Geometric Algebra multivector.

use crate::ga_core::Multivector;

/// Represents the discrete states of consciousness for an Existon.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum ConsciousnessState {
    /// The Existon is in a superposition of states, unobserved.
    Potential,
    /// The Existon has been measured, collapsing its state to a definite value.
    Observed,
    /// The Existon is a fixed, stable entity that influences its neighbors.
    Operator,
}

/// The Existon: a primitive unit of reality and consciousness.
///
/// Each Existon has a unique ID, a state of consciousness, and a `Multivector`
/// which holds its underlying geometric state.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Existon {
    /// A unique identifier for the Existon.
    pub id: u64,
    /// The current consciousness state of the Existon.
    pub consciousness: ConsciousnessState,
    /// The Geometric Algebra multivector representing the Existon's state.
    pub state: Multivector,
}

impl Existon {
    /// Creates a new Existon with a unique ID, initialized in a random `Potential` state.
    pub fn new(id: u64) -> Self {
        Existon {
            id,
            consciousness: ConsciousnessState::Potential,
            // Initialize with a random state using the new constructor.
            state: Multivector::random(),
        }
    }

    /// The "It from Bit" event: collapses a `Potential` state to an `Observed` state.
    ///
    /// Upon observation, the state simplifies. As an example, we zero out the
    /// bivector (spinor) component, making its state more "classical".
    pub fn observe(&mut self) {
        if self.consciousness == ConsciousnessState::Potential {
            self.consciousness = ConsciousnessState::Observed;
            self.state.e01 = crate::ga_core::Mod3::new(0);
        }
    }

    /// Returns an `Observed` Existon to a new, random `Potential` state.
    ///
    /// This represents decoherence or the loss of a persistent observation, allowing
    /// "reality" to dissolve back into the quantum foam.
    pub fn decay(&mut self) {
        if self.consciousness == ConsciousnessState::Observed {
            self.consciousness = ConsciousnessState::Potential;
            // Return to a random superposition.
            self.state = Multivector::random();
        }
    }
}
