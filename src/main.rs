mod existon;
mod ga_core;
mod universe;

use crate::{existon::ConsciousnessState, universe::Universe};
use piston_window::{
    Button, Key, PistonWindow, PressEvent, RenderEvent, TextureSettings, Transformed, UpdateEvent,
    WindowSettings, clear, glyph_cache::rusttype::GlyphCache, rectangle, text,
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

    while let Some(e) = window.next() {
        if e.update_args().is_some() {
            universe.tick();
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::Up => {
                    universe.observation_rate *= 2.0;
                    println!("Observation Rate Increased: {}", universe.observation_rate);
                }
                Key::Down => {
                    universe.observation_rate /= 2.0;
                    println!("Observation Rate Decreased: {}", universe.observation_rate);
                }
                Key::Right => {
                    universe.decay_rate *= 2.0;
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
                _ => {}
            }
        }

        if e.render_args().is_some() {
            window.draw_2d(&e, |c, g, device| {
                clear([0.0, 0.0, 0.0, 1.0], g); // The void

                for y in 0..universe.height {
                    for x in 0..universe.width {
                        let idx = y * universe.width + x;
                        let existon = universe.grid[idx];
                        let x_pos = x as f64 * CELL_SIZE;
                        let y_pos = y as f64 * CELL_SIZE;

                        let color = match existon.consciousness {
                            // Potential Existons are the dim, shifting "quantum foam"
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

                let transform = c.transform.trans(10.0, 20.0); // Position for the first line
                let obs_text = format!("Observation Rate: {:.6}", universe.observation_rate);
                text::Text::new_color([0.8, 0.8, 0.8, 1.0], 14)
                    .draw(&obs_text, &mut glyphs, &c.draw_state, transform, g)
                    .unwrap();

                let transform2 = c.transform.trans(10.0, 40.0); // Position for the second line
                let decay_text = format!("Decay Rate: {:.6}", universe.decay_rate);
                text::Text::new_color([0.8, 0.8, 0.8, 1.0], 14)
                    .draw(&decay_text, &mut glyphs, &c.draw_state, transform2, g)
                    .unwrap();

                let transform3 = c.transform.trans(10.0, 60.0);
                let ent_text = format!(
                    "Entanglement: {:.0}%",
                    universe.entanglement_percentage * 100.0
                );
                text::Text::new_color([0.8, 0.8, 0.8, 1.0], 14)
                    .draw(&ent_text, &mut glyphs, &c.draw_state, transform3, g)
                    .unwrap();
                // You must call this once per frame for the glyph cache.
                glyphs.factory.encoder.flush(device);
            });
        }
    }
}
