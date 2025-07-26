// src/main.rs

mod existon;
mod ga_core;
mod universe;

use crate::existon::ConsciousnessState;
use crate::universe::Universe;
use piston_window::*;

fn main() {
    const WIDTH: usize = 120;
    const HEIGHT: usize = 80;
    const CELL_SIZE: f64 = 8.0;
    let window_size = [WIDTH as f64 * CELL_SIZE, HEIGHT as f64 * CELL_SIZE];

    // Fixed: Added the window size argument to `WindowSettings::new`
    let mut universe = Universe::new(WIDTH, HEIGHT);
    let mut window: PistonWindow = WindowSettings::new(
        "Existon Automaton: A Model of Matzke's Source Science",
        window_size,
    )
    .exit_on_esc(true)
    // .no_default_flags(true) // <-- REMOVE THIS LINE
    .build()
    .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e)); // <-- SUGGESTED CHANGE

    while let Some(e) = window.next() {
        if e.update_args().is_some() {
            universe.tick();
        }

        if e.render_args().is_some() {
            window.draw_2d(&e, |c, g, _| {
                clear([0.0, 0.0, 0.0, 1.0], g); // The void

                for y in 0..universe.height {
                    for x in 0..universe.width {
                        let idx = y * universe.width + x;
                        let existon = universe.grid[idx];
                        let x_pos = x as f64 * CELL_SIZE;
                        let y_pos = y as f64 * CELL_SIZE;

                        let color = match existon.consciousness {
                            // Potential Existons are the dim, shifting "quantum foam"
                            // Fixed: Accessing the public field `0` of the Mod3 struct
                            ConsciousnessState::Potential => {
                                let r = (existon.state.s.0 + 1) as f32 * 0.1;
                                let g = (existon.state.e0.0 + 1) as f32 * 0.1;
                                let b = (existon.state.e1.0 + 1) as f32 * 0.1;
                                let a = (existon.state.e01.0 + 1) as f32 * 0.2 + 0.3;
                                [r, g, b, a]
                            }
                            // Observed Existons are bright, definite points of reality
                            ConsciousnessState::Observed => [1.0, 1.0, 0.8, 1.0],
                        };

                        // Fixed: Removed extra comma and added the rectangle dimensions
                        let rect = [x_pos, y_pos, CELL_SIZE, CELL_SIZE];
                        rectangle(color, rect, c.transform, g);
                    }
                }
            });
        }
    }
}
