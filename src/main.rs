//! # Existon Automaton
//!
//! This is the main entry point and event loop for the simulation.
//! It is responsible for:
//! 1. Setting up the application window and configuration.
//! 2. Initializing the `Universe` and font assets.
//! 3. Running the main event loop to handle user input, simulation ticks, and rendering.

mod existon;
mod ga_core;
mod universe;

use crate::{existon::ConsciousnessState, universe::Universe};
// TO (this is the updated, correct version):
use piston_window::{
    Button, Glyphs, Key, MouseButton, MouseCursorEvent, PistonWindow, PressEvent, RenderEvent,
    TextureSettings, Transformed, UpdateEvent, WindowSettings, clear, rectangle, text,
};
// --- Application Configuration ---

/// A struct to hold all the static configuration values for the application.
struct Config {
    width: usize,
    height: usize,
    cell_size: f64,
    window_size: [f64; 2],
    background_color: [f32; 4],
    text_color: [f32; 4],
    font_size: u32,
    line_height: f64,
}

impl Config {
    /// Creates a new configuration with default values.
    fn new() -> Self {
        const WIDTH: usize = 120;
        const HEIGHT: usize = 80;
        const CELL_SIZE: f64 = 8.0;
        Self {
            width: WIDTH,
            height: HEIGHT,
            cell_size: CELL_SIZE,
            window_size: [WIDTH as f64 * CELL_SIZE, HEIGHT as f64 * CELL_SIZE],
            background_color: [0.0, 0.0, 0.0, 1.0], // Black
            text_color: [0.0, 0.0, 0.0, 1.0],       // Black
            font_size: 14,
            line_height: 18.0,
        }
    }
}

// --- Main Application Logic ---

fn main() {
    let config = Config::new();
    let mut universe = Universe::new(config.width, config.height);

    // --- Window and Asset Setup ---
    let mut window: PistonWindow = WindowSettings::new(
        "Existon Automaton: A Model of Matzke's Source Science",
        config.window_size,
    )
    .exit_on_esc(true)
    .build()
    .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let font_path = assets.join("FiraSans-Regular.ttf");
    let mut glyphs = Glyphs::new(
        &font_path,
        window.create_texture_context(),
        TextureSettings::new(),
    )
    .expect("Could not load font");

    // --- Main Event Loop ---
    let mut mouse_pos = [0.0, 0.0];
    while let Some(e) = window.next() {
        // --- Event Handling ---
        e.mouse_cursor(|pos| mouse_pos = pos);

        // Handle user input for keyboard and mouse
        if let Some(button) = e.press_args() {
            handle_input(button, &mut universe, &config, mouse_pos);
        }

        // Update the simulation state
        if e.update_args().is_some() {
            universe.tick();
        }

        // Render the new state
        if e.render_args().is_some() {
            window.draw_2d(&e, |c, g, device| {
                draw_app(c, g, device, &mut glyphs, &universe, &config);
            });
        }
    }
}

/// Handles all user input events (keyboard and mouse).
fn handle_input(button: Button, universe: &mut Universe, config: &Config, mouse_pos: [f64; 2]) {
    match button {
        // Keyboard Controls
        Button::Keyboard(key) => match key {
            Key::Up => {
                universe.observation_rate = (universe.observation_rate * 2.0).min(1.0);
                println!("Observation Rate Increased: {}", universe.observation_rate);
            }
            Key::Down => {
                universe.observation_rate /= 2.0;
                println!("Observation Rate Decreased: {}", universe.observation_rate);
            }
            Key::Right => {
                universe.decay_rate = (universe.decay_rate * 2.0).min(1.0);
                println!("Decay Rate Increased: {}", universe.decay_rate);
            }
            Key::Left => {
                universe.decay_rate /= 2.0;
                println!("Decay Rate Decreased: {}", universe.decay_rate);
            }
            Key::F => {
                universe.fluctuation_rate = (universe.fluctuation_rate * 2.0).min(1.0);
                println!("Fluctuation Rate set to: {}", universe.fluctuation_rate);
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
            Key::R => {
                *universe = Universe::new(config.width, config.height);
                println!("--- UNIVERSE RESET ---");
            }
            _ => {}
        },

        // Mouse Controls
        Button::Mouse(button) => {
            let (grid_x, grid_y) = (
                (mouse_pos[0] / config.cell_size) as usize,
                (mouse_pos[1] / config.cell_size) as usize,
            );
            match button {
                MouseButton::Left => universe.set_operator(grid_x, grid_y),
                MouseButton::Right => universe.clear_operator(grid_x, grid_y),
                _ => {}
            }
        }
        _ => {}
    }
}

/// Handles all drawing logic for the application.
fn draw_app(
    c: piston_window::Context,
    g: &mut piston_window::G2d,
    device: &mut piston_window::GfxDevice,
    // UPDATE THIS LINE:
    glyphs: &mut Glyphs,
    universe: &Universe,
    config: &Config,
) {
    // 1. Clear the screen with the background color.
    clear(config.background_color, g);

    // 2. Draw the grid of Existons.
    for y in 0..universe.height {
        for x in 0..universe.width {
            let idx = y * universe.width + x;
            let existon = universe.grid[idx];
            let x_pos = x as f64 * config.cell_size;
            let y_pos = y as f64 * config.cell_size;

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
            let rect = [x_pos, y_pos, config.cell_size, config.cell_size];
            rectangle(color, rect, c.transform, g);
        }
    }

    // 3. Draw the UI text overlay.
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
        let transform = c
            .transform
            .trans(10.0, 20.0 + (i as f64 * config.line_height));
        text::Text::new_color(config.text_color, config.font_size)
            .draw(line, glyphs, &c.draw_state, transform, g)
            .unwrap();
    }

    // 4. Flush the glyph cache to the screen.
    glyphs.factory.encoder.flush(device);
}
