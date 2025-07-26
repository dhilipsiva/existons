// src/existon.rs

use crate::ga_core::{Mod3, Multivector};
use rand::Rng;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum ConsciousnessState {
    Potential, // The Existon is in superposition, unobserved.
    Observed,  // The Existon has been measured, collapsing its state.
}

/// The Existon: a "topological bit" and the primitive unit of consciousness.
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Existon {
    pub id: u64,
    pub consciousness: ConsciousnessState,
    pub state: Multivector, // The GA multivector representing its quantum state.
}

impl Existon {
    pub fn new(id: u64) -> Self {
        let mut rng = rand::rng();
        // Initialize in a random potential state.
        Existon {
            id,
            consciousness: ConsciousnessState::Potential,
            state: Multivector {
                s: Mod3::new(rng.random_range(-1..=1)),
                e0: Mod3::new(rng.random_range(-1..=1)),
                e1: Mod3::new(rng.random_range(-1..=1)),
                e01: Mod3::new(rng.random_range(-1..=1)),
            },
        }
    }

    /// The "It from Bit" event: observation collapses potentiality.
    pub fn observe(&mut self) {
        if self.consciousness == ConsciousnessState::Potential {
            self.consciousness = ConsciousnessState::Observed;
            // Upon observation, the state simplifies. For example, we can zero out
            // the bivector (spinor) component, making it more "classical".
            self.state.e01 = Mod3::new(0);
        }
    }

    pub fn decay(&mut self) {
        if self.consciousness == ConsciousnessState::Observed {
            self.consciousness = ConsciousnessState::Potential;
            // Return to a random superposition
            let mut rng = rand::rng();
            self.state = Multivector {
                s: Mod3::new(rng.random_range(-1..=1)),
                e0: Mod3::new(rng.random_range(-1..=1)),
                e1: Mod3::new(rng.random_range(-1..=1)),
                e01: Mod3::new(rng.random_range(-1..=1)),
            };
        }
    }
}
