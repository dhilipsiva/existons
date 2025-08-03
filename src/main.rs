//! # Existon Automaton
//!
//! This is the main entry point and event loop for the N-dimensional simulation.
//! It is responsible for:
//! 1. Setting up the application window and configuration.
//! 2. Initializing the N-dimensional `Universe` with a `p`-dimensional GA space.
//! 3. Running the main event loop to handle user input, simulation ticks, and rendering a 2D slice of the universe.

mod existon;
mod ga_core;
mod universe;

use crate::{existon::ConsciousnessState, universe::Universe};
use find_folder::Search;
use piston_window::{
    Button, Glyphs, Key, MouseButton, MouseCursorEvent, PistonWindow, PressEvent, RenderEvent,
    TextureSettings, Transformed, UpdateEvent, WindowSettings, clear, rectangle, text,
};

// --- Application Configuration ---

/// A struct to hold all the static configuration values for the application.
struct Config {
    /// The dimensions of the simulation grid (e.g., `vec![120, 80]`).
    grid_dims: Vec<usize>,
    /// The dimensionality of the Geometric Algebra space for each Existon.
    ga_dims: usize,
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
        // --- KEY PARAMETERS TO EXPERIMENT WITH ---
        // Defines the grid shape. We will visualize the first two dimensions.
        let grid_dims = vec![120, 80];
        // Defines the mathematical space of the Existons. p=3 allows for qutrits.
        let ga_dims = 3;

        const CELL_SIZE: f64 = 8.0;

        // Window size is based on the first two grid dimensions for visualization.
        let window_width = grid_dims.first().copied().unwrap_or(100) as f64 * CELL_SIZE;
        let window_height = grid_dims.get(1).copied().unwrap_or(100) as f64 * CELL_SIZE;

        Self {
            grid_dims,
            ga_dims,
            cell_size: CELL_SIZE,
            window_size: [window_width, window_height],
            background_color: [0.0, 0.0, 0.0, 1.0], // Black
            text_color: [1.0, 1.0, 1.0, 0.9],       // White
            font_size: 14,
            line_height: 18.0,
        }
    }
}

// --- Main Application Logic ---

fn main() {
    let config = Config::new();
    let mut universe = Universe::new(config.grid_dims.clone(), config.ga_dims);

    // --- Window and Asset Setup ---
    let mut window: PistonWindow = WindowSettings::new(
        "Existon Automaton: A Model of Matzke's Source Science",
        config.window_size,
    )
    .exit_on_esc(true)
    .build()
    .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    let assets = Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
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
            Key::Up => universe.observation_rate = (universe.observation_rate * 2.0).min(1.0),
            Key::Down => universe.observation_rate /= 2.0,
            Key::Right => universe.decay_rate = (universe.decay_rate * 2.0).min(1.0),
            Key::Left => universe.decay_rate /= 2.0,
            Key::F => universe.fluctuation_rate = (universe.fluctuation_rate * 2.0).min(1.0),
            Key::E => {
                let current = universe.entanglement_percentage;
                universe.entanglement_percentage = if (current - 0.20).abs() < 0.01 {
                    0.01
                } else {
                    current + 0.05
                };
                universe.re_entangle();
            }
            Key::R => *universe = Universe::new(config.grid_dims.clone(), config.ga_dims),
            _ => {}
        },

        // Mouse Controls (operates on the visible 2D slice)
        Button::Mouse(button) => {
            let grid_coord = [
                (mouse_pos[0] / config.cell_size) as usize,
                (mouse_pos[1] / config.cell_size) as usize,
            ];
            match button {
                MouseButton::Left => universe.set_operator(&grid_coord),
                MouseButton::Right => universe.clear_operator(&grid_coord),
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
    glyphs: &mut Glyphs,
    universe: &Universe,
    config: &Config,
) {
    clear(config.background_color, g);

    // --- Draw the 2D slice of the Grid ---
    let (width, height) = (config.grid_dims[0], config.grid_dims[1]);
    for y in 0..height {
        for x in 0..width {
            // We are only visualizing a 2D slice, so higher dimensions are fixed at 0.
            let mut coord = vec![0; universe.grid_dims.len()];
            coord[0] = x;
            coord[1] = y;

            if let Some(idx) = universe.get_index_from_coord(&coord) {
                let existon = &universe.grid[idx];
                let x_pos = x as f64 * config.cell_size;
                let y_pos = y as f64 * config.cell_size;

                // --- Updated Color Logic for Dynamic Multivector ---
                let color = match existon.consciousness {
                    ConsciousnessState::Potential => {
                        let s = existon.state.coefficients.get(0).map_or(0, |c| c.0); // Scalar
                        let e0 = existon.state.coefficients.get(1).map_or(0, |c| c.0); // e0
                        let e1 = existon.state.coefficients.get(2).map_or(0, |c| c.0); // e1
                        let e01 = existon.state.coefficients.get(3).map_or(0, |c| c.0); // e01

                        let r = (s + 1) as f32 * 0.35;
                        let g = (e0 + 1) as f32 * 0.35;
                        let b = (e1 + 1) as f32 * 0.35;
                        let a = (e01 + 1) as f32 * 0.4 + 0.5;
                        [r, g, b, a]
                    }
                    ConsciousnessState::Observed => [1.0, 1.0, 0.8, 1.0],
                    ConsciousnessState::Operator => [0.0, 1.0, 1.0, 1.0],
                };
                rectangle(
                    color,
                    [x_pos, y_pos, config.cell_size, config.cell_size],
                    c.transform,
                    g,
                );
            }
        }
    }

    // --- Draw the UI Text Overlay ---
    let display_lines = vec![
        format!(
            "[Up/Down] Observation Rate: {:.4}",
            universe.observation_rate
        ),
        format!("[Left/Right] Decay Rate: {:.4}", universe.decay_rate),
        format!("[F] Fluctuation Rate:     {:.4}", universe.fluctuation_rate),
        format!(
            "[E] Entanglement:         {:.0}%",
            universe.entanglement_percentage * 100.0
        ),
        String::from(""),
        format!(
            "Grid Dims: {:?}, GA Dims: {}",
            config.grid_dims, config.ga_dims
        ),
    ];

    for (i, line) in display_lines.iter().enumerate() {
        let transform = c
            .transform
            .trans(10.0, 20.0 + (i as f64 * config.line_height));
        text::Text::new_color(config.text_color, config.font_size)
            .draw(line, glyphs, &c.draw_state, transform, g)
            .unwrap();
    }
    glyphs.factory.encoder.flush(device);
}
