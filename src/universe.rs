// src/universe.rs

use crate::existon::{ConsciousnessState, Existon};
use crate::ga_core::{Mod3, Multivector};
use rand::Rng;
use std::collections::HashMap;

pub struct Universe {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Existon>,
    /// Models nonlocality: Maps an Existon's ID to its entangled partner's ID.
    pub entangled_pairs: HashMap<u64, u64>,
}

impl Universe {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        let mut grid = Vec::with_capacity(size);
        for i in 0..size {
            grid.push(Existon::new(i as u64));
        }

        // Create some random entangled pairs to model nonlocality.
        let mut entangled_pairs = HashMap::new();
        let mut rng = rand::rng();
        let num_pairs = size / 20; // Entangle 5% of Existons
        for _ in 0..num_pairs {
            let id1 = rng.random_range(0..size) as u64;
            let id2 = rng.random_range(0..size) as u64;
            if id1 != id2
                && !entangled_pairs.contains_key(&id1)
                && !entangled_pairs.contains_key(&id2)
            {
                entangled_pairs.insert(id1, id2);
                entangled_pairs.insert(id2, id1);
            }
        }

        Universe {
            width,
            height,
            grid,
            entangled_pairs,
        }
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    /// The main simulation step, where all of Matzke's rules come into play.
    pub fn tick(&mut self) {
        let mut next_grid = self.grid.clone();
        let mut observed_in_tick = vec![];

        // 1. Local Interaction & Computation Step
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = self.get_index(x, y);

                // The Update Rule: S(t+1) = Operator * S(t)
                // The 'Operator' is formed from the sum of neighbors' states.
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

                // Apply the operator via the Geometric Product
                next_grid[idx].state = operator * self.grid[idx].state;

                // Rule: An Existon has a small chance to be spontaneously "observed"
                if self.grid[idx].consciousness == ConsciousnessState::Potential
                    && rand::rng().random_bool(0.0005)
                {
                    next_grid[idx].observe();
                    observed_in_tick.push(next_grid[idx].id);
                }

                // Rule: An Observed Existon has a chance to decay back to Potential
                if self.grid[idx].consciousness == ConsciousnessState::Observed
                    && rand::rng().random_bool(0.01)
                {
                    next_grid[idx].decay();
                }
            }
        }

        // 2. Nonlocal (Entanglement) Step
        for id in observed_in_tick {
            if let Some(partner_id) = self.entangled_pairs.get(&id) {
                let partner_idx = *partner_id as usize;
                if next_grid[partner_idx].consciousness == ConsciousnessState::Potential {
                    next_grid[partner_idx].observe();
                    // Entangled particles have correlated states. We model this by
                    // inverting the partner's state upon collapse.
                    next_grid[partner_idx].state = next_grid[partner_idx].state
                        * Multivector {
                            s: Mod3::new(-1),
                            e0: Mod3::new(0),
                            e1: Mod3::new(0),
                            e01: Mod3::new(0),
                        };
                }
            }
        }

        self.grid = next_grid;
    }
}
