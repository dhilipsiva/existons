//! Defines the `Universe`, which contains the N-dimensional grid of `Existon`
//! instances and orchestrates the primary simulation rules.

use crate::existon::{ConsciousnessState, Existon};
use crate::ga_core::Multivector;
use rand::{prelude::*, rng};
use std::collections::HashMap;

//================================================================================
// Universe
//================================================================================

/// Represents the simulation space, containing all Existons and simulation parameters.
/// The grid is a generic N-dimensional lattice.
#[derive(Debug, Clone)]
pub struct Universe {
    /// The number of dimensions of the Geometric Algebra space for each Existon.
    pub ga_dims: usize,
    /// The dimensions of the simulation grid (e.g., `vec![120, 80]` for a 2D grid).
    pub grid_dims: Vec<usize>,
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
    /// Creates a new `Universe` with given grid dimensions and GA dimensions.
    pub fn new(grid_dims: Vec<usize>, ga_dims: usize) -> Self {
        let size: usize = grid_dims.iter().product();
        let mut grid = Vec::with_capacity(size);
        for i in 0..size {
            // Each Existon is created within the specified p-dimensional GA space.
            grid.push(Existon::new(i as u64, ga_dims));
        }

        let initial_entanglement = 0.05;
        let entangled_pairs = Self::_generate_entangled_pairs(size, initial_entanglement);

        Universe {
            grid_dims,
            ga_dims,
            grid,
            entangled_pairs,
            observation_rate: 0.0005,
            decay_rate: 0.01,
            entanglement_percentage: initial_entanglement,
            fluctuation_rate: 0.001,
        }
    }

    /// The main simulation step, where all rules are applied to each Existon.
    pub fn tick(&mut self) {
        let mut next_grid = self.grid.clone();
        let mut observed_in_tick = Vec::new();
        let mut rng = rng();

        // 1. Local Interaction & State Transition Step (N-Dimensional)
        for idx in 0..self.grid.len() {
            if self.grid[idx].consciousness == ConsciousnessState::Operator {
                continue;
            }

            // A. Get the N-dimensional coordinate and neighbors for the current cell.
            let coord = self.get_coord_from_index(idx);
            let neighbor_indices = self.get_neighbors(&coord);

            // B. Compute the local operator from the sum of neighbors' states.
            let mut operator = Multivector::zero(self.ga_dims);
            for neighbor_idx in neighbor_indices {
                operator = &operator + &self.grid[neighbor_idx].state;
            }

            // C. Apply the operator via the Geometric Product for the next state.
            next_grid[idx].state = &operator * &self.grid[idx].state;

            // D. Apply state transition rules based on probabilities.
            if self.grid[idx].consciousness == ConsciousnessState::Potential {
                if rng.random_bool(self.observation_rate) {
                    next_grid[idx].observe();
                    observed_in_tick.push(next_grid[idx].id);
                } else if rng.random_bool(self.fluctuation_rate) {
                    // Re-randomizes the state by decaying and recreating.
                    next_grid[idx] = Existon::new(next_grid[idx].id, self.ga_dims);
                }
            } else if self.grid[idx].consciousness == ConsciousnessState::Observed {
                if rng.random_bool(self.decay_rate) {
                    next_grid[idx].decay();
                }
            }
        }

        // 2. Nonlocal (Entanglement) Step
        let entanglement_inversion = self.entanglement_inversion_operator();
        for id in observed_in_tick {
            if let Some(&partner_id) = self.entangled_pairs.get(&id) {
                let partner_idx = partner_id as usize;
                if next_grid[partner_idx].consciousness == ConsciousnessState::Potential {
                    next_grid[partner_idx].observe();
                    // Correlate the partner's state by inverting it upon collapse.
                    next_grid[partner_idx].state =
                        &next_grid[partner_idx].state * &entanglement_inversion;
                }
            }
        }

        self.grid = next_grid;
    }

    /// Clears and regenerates the map of entangled pairs.
    pub fn re_entangle(&mut self) {
        let size = self.grid.len();
        self.entangled_pairs = Self::_generate_entangled_pairs(size, self.entanglement_percentage);
    }

    /// Places a stable `Operator` cell on the grid at an N-dimensional coordinate.
    pub fn set_operator(&mut self, coord: &[usize]) {
        if let Some(idx) = self.get_index_from_coord(coord) {
            self.grid[idx].consciousness = ConsciousnessState::Operator;
            self.grid[idx].state = self.fixed_operator_state();
        }
    }

    /// Clears an `Operator` cell, returning it to a `Potential` state.
    pub fn clear_operator(&mut self, coord: &[usize]) {
        if let Some(idx) = self.get_index_from_coord(coord) {
            self.grid[idx].decay();
            // Decay only works on Observed, so we ensure it's reset correctly.
            if self.grid[idx].consciousness == ConsciousnessState::Operator {
                self.grid[idx] = Existon::new(self.grid[idx].id, self.ga_dims);
            }
        }
    }

    // --- N-Dimensional Helper Functions ---

    /// Calculates the 1D index for an N-dimensional grid coordinate.
    pub fn get_index_from_coord(&self, coord: &[usize]) -> Option<usize> {
        if coord.len() != self.grid_dims.len() {
            return None;
        }
        let mut index = 0;
        let mut stride = 1;
        for (i, &c) in coord.iter().enumerate() {
            if c >= self.grid_dims[i] {
                return None;
            }
            index += stride * c;
            stride *= self.grid_dims[i];
        }
        Some(index)
    }

    /// Calculates the N-dimensional coordinate from a 1D grid index.
    pub fn get_coord_from_index(&self, mut index: usize) -> Vec<usize> {
        let mut coord = vec![0; self.grid_dims.len()];
        let mut stride = self.grid.len();
        for (i, &dim) in self.grid_dims.iter().enumerate().rev() {
            stride /= dim;
            coord[i] = index / stride;
            index %= stride;
        }
        coord
    }

    /// Gets the indices of all neighbors for a given N-dimensional coordinate (Moore neighborhood).
    fn get_neighbors(&self, coord: &[usize]) -> Vec<usize> {
        let mut neighbors = Vec::new();
        let n_dims = self.grid_dims.len();

        // This iterator generates all {-1, 0, 1} combinations for N dimensions.
        for i in 0..(3_i32.pow(n_dims as u32)) {
            let mut offset = Vec::new();
            let mut temp = i;
            // The all-zero offset is the cell itself, so we skip it.
            if temp == 0 {
                continue;
            }

            for _ in 0..n_dims {
                offset.push(temp % 3 - 1);
                temp /= 3;
            }

            let neighbor_coord: Vec<usize> = coord
                .iter()
                .zip(offset.iter())
                .enumerate()
                .map(|(d, (&c, &o))| (c as i32 + o).rem_euclid(self.grid_dims[d] as i32) as usize)
                .collect();

            if let Some(idx) = self.get_index_from_coord(&neighbor_coord) {
                neighbors.push(idx);
            }
        }
        neighbors
    }

    // --- Dynamic Operator Generators ---

    /// Generates a fixed multivector state for a stable `Operator`.
    fn fixed_operator_state(&self) -> Multivector {
        let mut mv = Multivector::zero(self.ga_dims);
        // As an example, make it a pure vector state (e.g., e_0 = 1).
        if self.ga_dims > 0 {
            mv.coefficients[1] = crate::ga_core::Mod3::new(1); // Blade e_0
        }
        mv
    }

    /// Generates the operator for entanglement, which inverts a state.
    fn entanglement_inversion_operator(&self) -> Multivector {
        let mut mv = Multivector::zero(self.ga_dims);
        // Scalar value of -1.
        mv.coefficients[0] = crate::ga_core::Mod3::new(-1);
        mv
    }

    /// Private helper to generate a new map of entangled pairs.
    fn _generate_entangled_pairs(size: usize, percentage: f64) -> HashMap<u64, u64> {
        let mut entangled_pairs = HashMap::new();
        let mut rng = rng();
        let num_pairs = (size as f64 * percentage / 2.0) as usize;
        let mut available_ids: Vec<u64> = (0..size as u64).collect();
        available_ids.shuffle(&mut rng);

        for _ in 0..num_pairs {
            if available_ids.len() < 2 {
                break;
            }
            let id1 = available_ids.pop().unwrap();
            let id2 = available_ids.pop().unwrap();
            entangled_pairs.insert(id1, id2);
            entangled_pairs.insert(id2, id1);
        }
        entangled_pairs
    }
}
