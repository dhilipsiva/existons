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
    Button, Ellipse, Glyphs, Key, Line, MouseButton, MouseCursorEvent, PistonWindow, PressEvent,
    ReleaseEvent, RenderEvent, TextureSettings, Transformed, UpdateEvent, WindowSettings, clear,
    rectangle, text,
};
use rand::{Rng, rng};

//================================================================================
// New UI Components
//================================================================================

/// Defines the interactive tools the user can switch between.
#[derive(Debug, Clone, Copy, PartialEq)]
enum ToolMode {
    Observe,  // 🔎
    Entangle, // 🔗
    Operator, // 🏗️
    Disrupt,  // 🌊
}

//================================================================================
// Application Configuration
//================================================================================
struct Config {
    grid_dims: Vec<usize>,
    ga_dims: usize,
    cell_size: f64,
    observation_radius: f64,
    window_size: [f64; 2],
    background_color: [f32; 4],
    toolbar_color: [f32; 4],
    text_color: [f32; 4],
    font_size: u32,
}

impl Config {
    fn new() -> Self {
        let grid_dims = vec![120, 80];
        let ga_dims = 3;
        const CELL_SIZE: f64 = 8.0;

        let window_width = grid_dims.first().copied().unwrap_or(100) as f64 * CELL_SIZE;
        let window_height = grid_dims.get(1).copied().unwrap_or(100) as f64 * CELL_SIZE;

        Self {
            grid_dims,
            ga_dims,
            cell_size: CELL_SIZE,
            observation_radius: 50.0,
            window_size: [window_width, window_height],
            background_color: [0.0, 0.0, 0.0, 1.0],
            toolbar_color: [0.1, 0.1, 0.12, 1.0],
            text_color: [1.0, 1.0, 1.0, 0.9],
            font_size: 14,
        }
    }
}

fn main() {
    let config = Config::new();
    let mut universe = Universe::new(config.grid_dims.clone(), config.ga_dims);

    // --- Window and Asset Setup ---
    let mut window: PistonWindow = WindowSettings::new(
        "Existon Automaton: An Interactive Model of Source Science",
        config.window_size,
    )
    .exit_on_esc(true)
    .build()
    .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    let assets = Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    let font_path = assets.join("NotoSans-Regular.ttf");
    let mut glyphs = Glyphs::new(
        &font_path,
        window.create_texture_context(),
        TextureSettings::new(),
    )
    .expect("Could not load font");

    // --- Main Application State ---
    let mut mouse_pos = [0.0, 0.0];
    let mut current_tool = ToolMode::Observe;
    let mut entangle_first_partner: Option<u64> = None;
    let mut entanglement_flashes: Vec<(Vec<usize>, Vec<usize>, u8)> = Vec::new();

    // New: Track if mouse buttons are held down for painting
    let mut is_left_mouse_down = false;
    let mut is_right_mouse_down = false;

    while let Some(e) = window.next() {
        e.mouse_cursor(|pos| mouse_pos = pos);

        // Modified: Handle press and release events separately
        if let Some(button) = e.press_args() {
            handle_press(
                button,
                &mut universe,
                &config,
                &mut current_tool,
                &mut entangle_first_partner,
                &mut entanglement_flashes,
                &mut is_left_mouse_down,
                &mut is_right_mouse_down,
                mouse_pos,
            );
        }
        if let Some(button) = e.release_args() {
            handle_release(button, &mut is_left_mouse_down, &mut is_right_mouse_down);
        }

        apply_tool_effects(
            &mut universe,
            &config,
            &current_tool,
            mouse_pos,
            is_left_mouse_down,
            is_right_mouse_down,
        );

        if e.update_args().is_some() {
            if entangle_first_partner.is_none() {
                let triggered_pairs = universe.tick();
                for (id1, id2) in triggered_pairs {
                    let coord1 = universe.get_coord_from_index(id1 as usize);
                    let coord2 = universe.get_coord_from_index(id2 as usize);
                    entanglement_flashes.push((coord1, coord2, 15));
                }
            }

            entanglement_flashes.retain_mut(|(_, _, ttl)| {
                *ttl = ttl.saturating_sub(1);
                *ttl > 0
            });
        }

        if e.render_args().is_some() {
            window.draw_2d(&e, |c, g, device| {
                draw_app(
                    c,
                    g,
                    device,
                    &mut glyphs,
                    &universe,
                    &config,
                    &current_tool,
                    mouse_pos,
                    entangle_first_partner,
                    &entanglement_flashes,
                );
            });
        }
    }
}

/// Handles all discrete press input events (key/mouse down).
fn handle_press(
    button: Button,
    universe: &mut Universe,
    config: &Config,
    current_tool: &mut ToolMode,
    entangle_first_partner: &mut Option<u64>,
    entanglement_flashes: &mut Vec<(Vec<usize>, Vec<usize>, u8)>,
    is_left_mouse_down: &mut bool,
    is_right_mouse_down: &mut bool,
    mouse_pos: [f64; 2],
) {
    match button {
        Button::Keyboard(key) => {
            *entangle_first_partner = None;
            match key {
                Key::D1 => *current_tool = ToolMode::Observe,
                Key::D2 => *current_tool = ToolMode::Entangle,
                Key::D3 => *current_tool = ToolMode::Operator,
                Key::D4 => *current_tool = ToolMode::Disrupt,
                Key::R => *universe = Universe::new(config.grid_dims.clone(), config.ga_dims),
                _ => {}
            }
        }
        Button::Mouse(button) => match button {
            MouseButton::Left => {
                *is_left_mouse_down = true;
                handle_mouse_click(
                    universe,
                    config,
                    current_tool,
                    entangle_first_partner,
                    entanglement_flashes,
                    mouse_pos,
                );
            }
            MouseButton::Right => {
                *is_right_mouse_down = true;
                // For now, let right-click only work in Operator mode
                if *current_tool == ToolMode::Operator {
                    let clicked_coord = get_coord_from_pos(mouse_pos, config);
                    universe.clear_operator(&clicked_coord);
                }
            }
            _ => {}
        },
        _ => {}
    }
}

/// New: Handles mouse release events to stop painting.
fn handle_release(button: Button, is_left_mouse_down: &mut bool, is_right_mouse_down: &mut bool) {
    if let Button::Mouse(button) = button {
        match button {
            MouseButton::Left => *is_left_mouse_down = false,
            MouseButton::Right => *is_right_mouse_down = false,
            _ => {}
        }
    }
}

/// Handles the specific action of a single left mouse click for the active tool.
fn handle_mouse_click(
    universe: &mut Universe,
    config: &Config,
    current_tool: &ToolMode,
    entangle_first_partner: &mut Option<u64>,
    entanglement_flashes: &mut Vec<(Vec<usize>, Vec<usize>, u8)>,
    mouse_pos: [f64; 2],
) {
    let clicked_coord = get_coord_from_pos(mouse_pos, config);
    let clicked_idx = universe.get_index_from_coord(&clicked_coord);

    match *current_tool {
        ToolMode::Observe => {
            // Strong observation is now a continuous effect while mouse is held down
        }
        ToolMode::Entangle => {
            if let Some(idx) = clicked_idx {
                if universe.grid[idx].consciousness == ConsciousnessState::Potential {
                    if let Some(id1) = *entangle_first_partner {
                        let id2 = universe.grid[idx].id;
                        if id1 != id2 {
                            universe.entangle_pair(id1, id2);
                            let coord1 = universe.get_coord_from_index(id1 as usize);
                            let coord2 = universe.get_coord_from_index(id2 as usize);
                            entanglement_flashes.push((coord1, coord2, 15));
                            *entangle_first_partner = None;
                        }
                    } else {
                        *entangle_first_partner = Some(universe.grid[idx].id);
                    }
                }
            }
        }
        ToolMode::Operator => {
            // Handled by continuous effect
        }
        ToolMode::Disrupt => {
            for_cells_in_radius(config, mouse_pos, |coord| {
                if let Some(idx) = universe.get_index_from_coord(&coord) {
                    universe.disrupt_cell(idx);
                }
            });
        }
    }
}

/// Applies continuous effects for the active tool.
fn apply_tool_effects(
    universe: &mut Universe,
    config: &Config,
    current_tool: &ToolMode,
    mouse_pos: [f64; 2],
    is_left_mouse_down: bool,
    is_right_mouse_down: bool,
) {
    let mut rng = rng();
    match *current_tool {
        ToolMode::Observe => {
            let passive_observation_prob = 0.1;
            for_cells_in_radius(config, mouse_pos, |coord| {
                if let Some(idx) = universe.get_index_from_coord(&coord) {
                    // Strong observation if mouse is down, otherwise passive
                    let should_observe = is_left_mouse_down
                        || (universe.grid[idx].consciousness == ConsciousnessState::Potential
                            && rng.random_bool(passive_observation_prob));
                    if should_observe {
                        universe.observe_cell(idx);
                    }
                }
            });
        }
        ToolMode::Operator => {
            if is_left_mouse_down {
                let coord = get_coord_from_pos(mouse_pos, config);
                universe.set_operator(&coord);
            } else if is_right_mouse_down {
                let coord = get_coord_from_pos(mouse_pos, config);
                universe.clear_operator(&coord);
            }
        }
        ToolMode::Disrupt => {
            if is_left_mouse_down {
                for_cells_in_radius(config, mouse_pos, |coord| {
                    if let Some(idx) = universe.get_index_from_coord(&coord) {
                        universe.disrupt_cell(idx);
                    }
                });
            }
        }

        // Will add Disrupt logic here later
        _ => {}
    }
}

/// New utility to get a grid coordinate from a pixel position.
fn get_coord_from_pos(mouse_pos: [f64; 2], config: &Config) -> Vec<usize> {
    let mut coord = vec![0; config.grid_dims.len()];
    coord[0] = (mouse_pos[0] / config.cell_size).max(0.0) as usize;
    if config.grid_dims.len() > 1 {
        coord[1] = (mouse_pos[1] / config.cell_size).max(0.0) as usize;
    }
    coord
}

/// Handles all drawing logic for the application.
fn draw_app(
    c: piston_window::Context,
    g: &mut piston_window::G2d,
    device: &mut piston_window::GfxDevice,
    glyphs: &mut Glyphs,
    universe: &Universe,
    config: &Config,
    current_tool: &ToolMode,
    mouse_pos: [f64; 2],
    entangle_first_partner: Option<u64>,
    entanglement_flashes: &[(Vec<usize>, Vec<usize>, u8)],
) {
    clear(config.background_color, g);

    // --- Draw the 2D slice of the Grid ---
    let (width, height) = (config.grid_dims[0], config.grid_dims[1]);
    for y in 0..height {
        for x in 0..width {
            let mut coord = vec![0; universe.grid_dims.len()];
            coord[0] = x;
            if coord.len() > 1 {
                coord[1] = y;
            }

            if let Some(idx) = universe.get_index_from_coord(&coord) {
                // *** THIS ENTIRE BLOCK WAS MISSING ***
                let existon = &universe.grid[idx];
                let x_pos = x as f64 * config.cell_size;
                let y_pos = y as f64 * config.cell_size;

                let color = match existon.consciousness {
                    ConsciousnessState::Potential => {
                        let s = existon.state.coefficients.get(0).map_or(0, |c| c.0);
                        let e0 = existon.state.coefficients.get(1).map_or(0, |c| c.0);
                        let e1 = existon.state.coefficients.get(2).map_or(0, |c| c.0);
                        let e01 = existon.state.coefficients.get(3).map_or(0, |c| c.0);

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
                // *** END OF MISSING BLOCK ***
            }
        }
    }

    // --- Draw Entanglement Selection Highlight ---
    if let Some(id) = entangle_first_partner {
        let coord = universe.get_coord_from_index(id as usize);
        if !coord.is_empty() {
            let x_pos = coord[0] as f64 * config.cell_size;
            let y_pos = if coord.len() > 1 {
                coord[1] as f64 * config.cell_size
            } else {
                0.0
            };
            rectangle(
                [1.0, 0.8, 0.0, 0.5], // Transparent yellow border
                [x_pos, y_pos, config.cell_size, config.cell_size],
                c.transform,
                g,
            );
        }
    }

    // --- Draw Entanglement Flashes ---
    for (coord1, coord2, ttl) in entanglement_flashes.iter() {
        if !coord1.is_empty() && !coord2.is_empty() {
            let c1_x = (coord1[0] as f64 + 0.5) * config.cell_size;
            let c1_y = if coord1.len() > 1 {
                (coord1[1] as f64 + 0.5) * config.cell_size
            } else {
                config.cell_size / 2.0
            };
            let c2_x = (coord2[0] as f64 + 0.5) * config.cell_size;
            let c2_y = if coord2.len() > 1 {
                (coord2[1] as f64 + 0.5) * config.cell_size
            } else {
                config.cell_size / 2.0
            };

            let alpha = (*ttl as f32) / 15.0;
            let line = Line::new([1.0, 1.0, 1.0, alpha], 1.5);
            line.draw([c1_x, c1_y, c2_x, c2_y], &c.draw_state, c.transform, g);
        }
    }

    // Draw the visual effect for the active tool
    match *current_tool {
        ToolMode::Observe => {
            let radius = config.observation_radius;
            let circle = Ellipse::new([1.0, 1.0, 0.8, 0.1]); // Faint yellow
            circle.draw(
                [
                    mouse_pos[0] - radius,
                    mouse_pos[1] - radius,
                    radius * 2.0,
                    radius * 2.0,
                ],
                &c.draw_state,
                c.transform,
                g,
            );
        }
        ToolMode::Disrupt => {
            let radius = config.observation_radius;
            let circle = Ellipse::new([0.5, 0.0, 1.0, 0.15]); // Faint purple
            circle.draw(
                [
                    mouse_pos[0] - radius,
                    mouse_pos[1] - radius,
                    radius * 2.0,
                    radius * 2.0,
                ],
                &c.draw_state,
                c.transform,
                g,
            );
        }
        _ => {}
    };
    // Draw the Toolbar
    draw_toolbar(c, g, glyphs, config, current_tool);
    glyphs.factory.encoder.flush(device);
}

/// Draws the interactive toolbar at the bottom of the screen.
fn draw_toolbar(
    c: piston_window::Context,
    g: &mut piston_window::G2d,
    glyphs: &mut Glyphs,
    config: &Config,
    current_tool: &ToolMode,
) {
    let toolbar_height = 40.0;
    let window_height = config.window_size[1];
    let toolbar_y = window_height - toolbar_height;

    rectangle(
        config.toolbar_color,
        [0.0, toolbar_y, config.window_size[0], toolbar_height],
        c.transform,
        g,
    );

    let tools = [
        (ToolMode::Observe, "[1] Observe 🔎"),
        (ToolMode::Entangle, "[2] Entangle 🔗"),
        (ToolMode::Operator, "[3] Operator 🏗️"),
        (ToolMode::Disrupt, "[4] Disrupt 🌊"),
    ];

    let mut start_x = 20.0;
    let text_y = toolbar_y + toolbar_height / 2.0 + (config.font_size as f64 / 2.0) - 2.0;

    for (tool_mode, tool_text) in tools.iter() {
        let is_active = tool_mode == current_tool;
        let color = if is_active {
            [1.0, 0.8, 0.0, 1.0]
        } else {
            config.text_color
        };

        text::Text::new_color(color, config.font_size)
            .draw(
                tool_text,
                glyphs,
                &c.draw_state,
                c.transform.trans(start_x, text_y),
                g,
            )
            .unwrap();
        start_x += 200.0;
    }
}

/// Utility function to iterate over all grid cells within a given pixel radius of a point.
fn for_cells_in_radius<F>(config: &Config, center_pos: [f64; 2], mut callback: F)
where
    F: FnMut(Vec<usize>),
{
    let radius_sq = config.observation_radius * config.observation_radius;
    let cell_radius_x = (config.observation_radius / config.cell_size).ceil() as i32;
    let cell_radius_y = (config.observation_radius / config.cell_size).ceil() as i32;

    let center_grid_x = (center_pos[0] / config.cell_size) as i32;
    let center_grid_y = (center_pos[1] / config.cell_size) as i32;

    for dy in -cell_radius_y..=cell_radius_y {
        for dx in -cell_radius_x..=cell_radius_x {
            let cell_x = center_grid_x + dx;
            let cell_y = center_grid_y + dy;

            let cell_center_x = (cell_x as f64 + 0.5) * config.cell_size;
            let cell_center_y = (cell_y as f64 + 0.5) * config.cell_size;
            let dist_sq =
                (cell_center_x - center_pos[0]).powi(2) + (cell_center_y - center_pos[1]).powi(2);

            if dist_sq <= radius_sq {
                let mut coord = vec![0; config.grid_dims.len()];
                coord[0] = cell_x.rem_euclid(config.grid_dims[0] as i32) as usize;
                if config.grid_dims.len() > 1 {
                    coord[1] = cell_y.rem_euclid(config.grid_dims[1] as i32) as usize;
                }
                callback(coord);
            }
        }
    }
}
