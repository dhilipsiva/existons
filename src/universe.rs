//! Defines the `Universe`, which contains the grid of `Existon` instances
//! and orchestrates the primary simulation rules.

use crate::existon::{ConsciousnessState, Existon};
use crate::ga_core::{Mod3, Multivector};
use rand::Rng;
use std::collections::HashMap;

// --- Named Constants for Clarity ---

/// A fixed multivector state used when placing a stable `Operator` on the grid.
const FIXED_OPERATOR_STATE: Multivector = Multivector {
    s: Mod3(0),
    e0: Mod3(1),
    e1: Mod3(0),
    e01: Mod3(0),
};

/// The operator applied to an Existon's state when its entangled partner is observed.
/// Multiplying by this scalar-only multivector effectively inverts the state.
const ENTANGLEMENT_INVERSION: Multivector = Multivector {
    s: Mod3(-1),
    e0: Mod3(0),
    e1: Mod3(0),
    e01: Mod3(0),
};

//================================================================================
// Universe
//================================================================================

/// Represents the simulation space, containing all Existons and simulation parameters.
#[derive(Debug)]
pub struct Universe {
    /// The width of the simulation grid.
    pub width: usize,
    /// The height of the simulation grid.
    pub height: usize,
    /// A flat vector containing all `Existon` instances in the grid.
    pub grid: Vec<Existon>,
    /// Models non-locality by mapping an Existon's ID to its entangled partner's ID.
    pub entangled_pairs: HashMap<u64, u64>,
    /// The probability of a `Potential` Existon being spontaneously observed each tick.
    pub observation_rate: f64,
    /// The probability of an `Observed` Existon decaying back into a `Potential` state.
    pub decay_rate: f64,
    /// The percentage of Existons that are entangled with a partner.
    pub entanglement_percentage: f64,
    /// The probability of a `Potential` Existon spontaneously re-randomizing its state.
    pub fluctuation_rate: f64,
}

impl Universe {
    /// Creates a new `Universe` with a given width and height, populating it
    /// with Existons in a random `Potential` state.
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        let mut grid = Vec::with_capacity(size);
        for i in 0..size {
            grid.push(Existon::new(i as u64));
        }

        let initial_entanglement = 0.05; // Start with 5% entanglement
        let entangled_pairs = Self::_generate_entangled_pairs(size, initial_entanglement);

        Universe {
            width,
            height,
            grid,
            entangled_pairs,
            observation_rate: 0.0005,
            decay_rate: 0.01,
            entanglement_percentage: initial_entanglement,
            fluctuation_rate: 0.001,
        }
    }

    /// The main simulation step, where all rules are applied to each Existon.
    ///
    /// This method uses a "read from old, write to new" pattern by cloning the grid.
    /// This ensures that all calculations within a single tick are based on the state
    /// of the universe at the beginning of that tick.
    pub fn tick(&mut self) {
        let mut next_grid = self.grid.clone();
        let mut observed_in_tick = Vec::new();

        // 1. Local Interaction & State Transition Step
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = self.get_index(x, y);

                // Skip stable `Operator` cells entirely.
                if self.grid[idx].consciousness == ConsciousnessState::Operator {
                    continue;
                }

                // A. Compute the local operator from the sum of neighbors' states.
                let mut operator = Multivector::zero();
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let nx = (x as i32 + dx).rem_euclid(self.width as i32) as usize;
                        let ny = (y as i32 + dy).rem_euclid(self.height as i32) as usize;
                        let neighbor_idx = self.get_index(nx, ny);
                        operator = operator + self.grid[neighbor_idx].state;
                    }
                }

                // B. Apply the operator via the Geometric Product for the next state.
                next_grid[idx].state = operator * self.grid[idx].state;

                // C. Apply state transition rules based on probabilities.
                if self.grid[idx].consciousness == ConsciousnessState::Potential {
                    if rand::rng().random_bool(self.observation_rate) {
                        next_grid[idx].observe();
                        observed_in_tick.push(next_grid[idx].id);
                    } else if rand::rng().random_bool(self.fluctuation_rate) {
                        next_grid[idx].decay(); // Re-randomizes the state
                    }
                } else if self.grid[idx].consciousness == ConsciousnessState::Observed {
                    if rand::rng().random_bool(self.decay_rate) {
                        next_grid[idx].decay();
                    }
                }
            }
        }

        // 2. Nonlocal (Entanglement) Step
        for id in observed_in_tick {
            if let Some(&partner_id) = self.entangled_pairs.get(&id) {
                let partner_idx = partner_id as usize;
                if next_grid[partner_idx].consciousness == ConsciousnessState::Potential {
                    next_grid[partner_idx].observe();
                    // Correlate the partner's state by inverting it upon collapse.
                    next_grid[partner_idx].state =
                        next_grid[partner_idx].state * ENTANGLEMENT_INVERSION;
                }
            }
        }

        self.grid = next_grid;
    }

    /// Clears and regenerates the map of entangled pairs based on the current
    /// `entanglement_percentage`.
    pub fn re_entangle(&mut self) {
        let size = self.width * self.height;
        self.entangled_pairs = Self::_generate_entangled_pairs(size, self.entanglement_percentage);
    }

    /// Places a stable `Operator` cell on the grid.
    pub fn set_operator(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            let idx = self.get_index(x, y);
            self.grid[idx].consciousness = ConsciousnessState::Operator;
            self.grid[idx].state = FIXED_OPERATOR_STATE;
        }
    }

    /// Clears an `Operator` cell, returning it to a `Potential` state.
    pub fn clear_operator(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            let idx = self.get_index(x, y);
            // `decay()` conveniently resets the cell to a random potential state.
            self.grid[idx].decay();
        }
    }

    /// Calculates the 1D index for a 2D grid coordinate.
    fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    /// Private helper to generate a new map of entangled pairs.
    fn _generate_entangled_pairs(size: usize, percentage: f64) -> HashMap<u64, u64> {
        let mut entangled_pairs = HashMap::new();
        let mut rng = rand::rng();
        let num_pairs = (size as f64 * percentage) as usize;

        for _ in 0..num_pairs {
            let id1 = rng.random_range(0..size) as u64;
            let id2 = rng.random_range(0..size) as u64;

            // Ensure we don't entangle a particle with itself or re-entangle existing pairs.
            if id1 != id2
                && !entangled_pairs.contains_key(&id1)
                && !entangled_pairs.contains_key(&id2)
            {
                entangled_pairs.insert(id1, id2);
                entangled_pairs.insert(id2, id1);
            }
        }
        entangled_pairs
    }
}
