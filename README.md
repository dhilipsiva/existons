## TODO

### ✅ 1. Add "Control Knobs" for Exploration

Right now, the simulation runs on fixed constants. The next level is to let users change these parameters in real-time to see how the "universe" responds. This turns demo into an experiment.

**What to Implement:**
Add keyboard controls to modify these key values during runtime.

  * **Observation Rate:** Let the UP/DOWN arrow keys increase or decrease the spontaneous observation chance (currently `0.0005` in `universe.rs`). This answers the question: *“How much 'measurement' is needed to bring forth reality?”*
  * **Decay Rate:** Let the LEFT/RIGHT arrow keys control the decay chance (currently `0.01` in `universe.rs`). This explores the question: *“How stable is our observed reality?”*
  * **Entanglement Percentage:** Let a key like 'E' re-initialize the universe with more or fewer entangled pairs. This asks: *“How connected does the universe need to be?”*

**How to Do It:**
In `main.rs` event loop, add a section to handle key presses:

```rust
// In  main.rs `while let Some(e) = window.next()` loop:
if let Some(Button::Keyboard(key)) = e.press_args() {
    // Example for changing decay rate (you'd need to pass it to universe)
    match key {
        Key::Up => println!("Increase Observation Rate!"), // Implement logic
        Key::Down => println!("Decrease Observation Rate!"), // Implement logic
        _ => {}
    }
}
```

-----

### 🔬 2. Enhance the Visualization

Current visualization shows the binary state: `Potential` or `Observed`. You can make it far more insightful by visualizing the *internal state* of the `Potential` cells.

**What to Implement:**
Change the color mapping for `Potential` cells to reflect the values of their multivector components (`s`, `e0`, `e1`, `e01`).

**How to Do It:**
Update the color calculation in `main.rs`:

```rust
// In main.rs `window.draw_2d` block:
let color = match existon.consciousness {
    ConsciousnessState::Potential => {
        // Visualize the internal state of the quantum foam
        let r = (existon.state.s.0 + 1) as f32 * 0.25;  // Scalar -> Red
        let g = (existon.state.e0.0 + 1) as f32 * 0.25; // Vector e0 -> Green
        let b = (existon.state.e1.0 + 1) as f32 * 0.25; // Vector e1 -> Blue
        // Bivector e01 could be alpha or brightness
        [r, g, b, 0.7] 
    }
    ConsciousnessState::Observed => [1.0, 1.0, 0.8, 1.0],
};
```

This will transform the "dark" areas from a uniform color into a rich, subtly shifting tapestry that reveals the underlying mathematical patterns.

-----

### 🎨 3. Introduce a Stable "Operator"

The universe currently evolves based on the chaotic sum of its neighbors. A powerful next step is to introduce a stable, fixed structure and see how the quantum foam interacts with it.

**What to Implement:**
Allow the user to "paint" a specific, non-changing `Multivector` onto the grid with a mouse click. This acts as a boundary condition or a fixed object.

**How to Do It:**

1.  In `main.rs`, handle mouse-click events to get the coordinates.
2.  At those coordinates, set the `Existon`'s state to a predefined `Multivector` (e.g., a pure vector state like `e0=1, others=0`).
3.  In `universe.rs`, modify the `tick` logic to exclude these special cells from being updated, so they remain fixed.

This demonstrates how a persistent structure can organize the chaotic foam around it, leading to profound visual demonstrations of form emerging from the void.
