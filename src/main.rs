mod existon;
mod ga_core;
mod universe;

use crate::{existon::ConsciousnessState, universe::Universe};
use piston_window::{
    Button, Key, MouseButton, MouseCursorEvent, PistonWindow, PressEvent, RenderEvent,
    TextureSettings, Transformed, UpdateEvent, WindowSettings, clear,
    glyph_cache::rusttype::GlyphCache, rectangle, text,
};

fn main() {
    const WIDTH: usize = 120;
    const HEIGHT: usize = 80;
    const CELL_SIZE: f64 = 8.0;
    let window_size = [WIDTH as f64 * CELL_SIZE, HEIGHT as f64 * CELL_SIZE];

    let mut universe = Universe::new(WIDTH, HEIGHT);
    let mut window: PistonWindow = WindowSettings::new(
        "Existon Automaton: A Model of Matzke's Source Science",
        window_size,
    )
    .exit_on_esc(true)
    .build()
    .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let font_path = assets.join("FiraSans-Regular.ttf");
    let mut glyphs = GlyphCache::new(
        &font_path,
        window.create_texture_context(),
        TextureSettings::new(),
    )
    .expect("Could not load font");

    let mut mouse_pos = [0.0, 0.0];

    while let Some(e) = window.next() {
        e.mouse_cursor(|pos| {
            mouse_pos = pos;
        });

        if e.update_args().is_some() {
            universe.tick();
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::Up => {
                    universe.observation_rate = (universe.observation_rate * 2.0).min(1.0);
                    println!("Observation Rate Increased: {}", universe.observation_rate);
                }
                Key::Down => {
                    universe.observation_rate /= 2.0;
                    println!("Observation Rate Decreased: {}", universe.observation_rate);
                }
                // Clamp the upper bound to 1.0
                Key::Right => {
                    universe.decay_rate = (universe.decay_rate * 2.0).min(1.0);
                    println!("Decay Rate Increased: {}", universe.decay_rate);
                }
                Key::Left => {
                    universe.decay_rate /= 2.0;
                    println!("Decay Rate Decreased: {}", universe.decay_rate);
                }
                Key::E => {
                    let current_percent = universe.entanglement_percentage;
                    if (current_percent - 0.01).abs() < f64::EPSILON {
                        universe.entanglement_percentage = 0.05;
                    } else if (current_percent - 0.05).abs() < f64::EPSILON {
                        universe.entanglement_percentage = 0.10;
                    } else if (current_percent - 0.10).abs() < f64::EPSILON {
                        universe.entanglement_percentage = 0.20;
                    } else {
                        universe.entanglement_percentage = 0.01;
                    }
                    universe.re_entangle();
                    println!(
                        "Entanglement Percentage set to: {}%",
                        universe.entanglement_percentage * 100.0
                    );
                }
                // Clamp the upper bound to 1.0
                Key::F => {
                    universe.fluctuation_rate = (universe.fluctuation_rate * 2.0).min(1.0);
                    println!("Fluctuation Rate set to: {}", universe.fluctuation_rate);
                }
                // ADD THIS CASE FOR THE RESET KEY
                Key::R => {
                    universe = Universe::new(WIDTH, HEIGHT);
                    println!("--- UNIVERSE RESET ---");
                }
                _ => {}
            }
        }

        if let Some(Button::Mouse(button)) = e.press_args() {
            let (grid_x, grid_y) = (
                (mouse_pos[0] / CELL_SIZE) as usize,
                (mouse_pos[1] / CELL_SIZE) as usize,
            );

            match button {
                // Left-click to place an operator
                MouseButton::Left => universe.set_operator(grid_x, grid_y),
                // Right-click to erase an operator
                MouseButton::Right => universe.clear_operator(grid_x, grid_y),
                _ => {}
            }
        }

        // In src/main.rs

        if e.render_args().is_some() {
            window.draw_2d(&e, |c, g, device| {
                clear([0.0, 0.0, 0.0, 1.0], g); // The void

                // This block for drawing the existons remains the same
                for y in 0..universe.height {
                    for x in 0..universe.width {
                        let idx = y * universe.width + x;
                        let existon = universe.grid[idx];
                        let x_pos = x as f64 * CELL_SIZE;
                        let y_pos = y as f64 * CELL_SIZE;

                        let color = match existon.consciousness {
                            ConsciousnessState::Potential => {
                                let r = (existon.state.s.0 + 1) as f32 * 0.35;
                                let g = (existon.state.e0.0 + 1) as f32 * 0.35;
                                let b = (existon.state.e1.0 + 1) as f32 * 0.35;
                                let a = (existon.state.e01.0 + 1) as f32 * 0.4 + 0.5;
                                [r, g, b, a]
                            }
                            ConsciousnessState::Observed => [1.0, 1.0, 0.8, 1.0],
                            ConsciousnessState::Operator => [0.0, 1.0, 1.0, 1.0],
                        };
                        let rect = [x_pos, y_pos, CELL_SIZE, CELL_SIZE];
                        rectangle(color, rect, c.transform, g);
                    }
                }

                // --- NEW UNIFIED TEXT RENDERING BLOCK ---
                // We build a vector of all the lines we want to draw.
                let display_lines = vec![
                    format!(
                        "[Up/Down] Observation Rate: {:.6}",
                        universe.observation_rate
                    ),
                    format!("[Left/Right] Decay Rate: {:.6}", universe.decay_rate),
                    format!("[F] Fluctuation Rate:     {:.6}", universe.fluctuation_rate),
                    format!(
                        "[E] Entanglement:         {:.0}%",
                        universe.entanglement_percentage * 100.0
                    ),
                    format!("[R] Reset Universe"),
                    format!("[ESC] Close Window"),
                    format!(""), // Blank line for separation
                    format!("[L-Click] Place Operator"),
                    format!("[R-Click] Erase Operator"),
                ];

                for (i, line) in display_lines.iter().enumerate() {
                    let transform = c.transform.trans(10.0, 20.0 + (i as f64 * 18.0));
                    // Change the color here for black text
                    text::Text::new_color([0.0, 0.0, 0.0, 1.0], 14)
                        .draw(line, &mut glyphs, &c.draw_state, transform, g)
                        .unwrap();
                }

                glyphs.factory.encoder.flush(device);
            });
        }
    }
}
