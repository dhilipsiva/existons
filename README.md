# Existons: An Interactive Model of Source Science

### A Note to Doug Matzke

Doug,

This project is an attempt to create a visual, interactive simulation based on the concepts I've gathered from your work on Source Science and the Existon model. Please note that I do not pretend to fully understand the depth of your theories, but I was deeply inspired by them. I have tried to implement the core mechanics as faithfully as my understanding allows, in the hopes of creating a small "laboratory" where these ideas can be seen in action. This is my humble tribute to your fascinating work.

## What is This?

This program is a real-time cellular automaton built in the Rust programming language. It is designed to be a visual model of your "Existon" concept, exploring how a complex reality might emerge from a simple, underlying computational substrate.

The simulation attempts to model several key principles derived from your work:

  * **A Computational Substrate:** The universe is a grid of primitive entities called "Existons."
  * **Geometric Algebra:** The state of each Existon and its interactions are governed by the rules of a **hyperdimensional Geometric Algebra (specifically, `Cl(p,0)`)**.
  * **"It from Bit":** "Reality" is not fundamental. An `Observed` state is a collapse from a `Potential` state, triggered by observation.
  * **Non-Locality:** A percentage of Existons are entangled, where an action on one instantly affects its partner, regardless of distance.
  * **Dynamic Equilibrium:** The simulation is not a one-way street. Observed reality can decay back into potentiality, and the quantum foam itself is in a constant state of flux, preventing a static "heat death."

## The Code: A Conceptual Breakdown

The project is broken down into four main modules, each handling a distinct part of the simulation.

### `ga_core.rs`: The Algebraic Foundation

This file is the mathematical heart of the simulation. It contains no logic about consciousness or universes, only the raw algebraic rules.

  * **`Mod3`:** This represents the fundamental tristate number system `{ -1, 0, 1 }` that underpins the algebra. [cite\_start]It includes custom, wrapping addition rules (e.g., `1 + 1 = -1`) as described in your work[cite: 1095, 1097].
  * **`Multivector`:** This is the data structure for a single Existon's state. It's a **dynamic Geometric Algebra multivector for a `p`-dimensional space**. [cite\_start]It holds a vector of `2^p` coefficients, one for each basis blade[cite: 1154].
  * **`impl Mul for &Multivector`:** This is the most critical piece of the file. [cite\_start]It is a **generalized implementation of the Geometric Product for the `Cl(p,0)` algebra**, defining the rules for how two Existon states interact and combine[cite: 1113].

### `existon.rs`: The Unit of Reality

This file defines the "Existon" itself as a software object.

  * **`Existon` Struct:** This struct combines a unique `id` with a `Multivector` state and, crucially, a `ConsciousnessState`.
  * **`ConsciousnessState` Enum:** This is a key conceptual model with three variants:
      * `Potential`: The default state. A superposition of possibilities, visualized as the colorful, shifting "quantum foam."
      * `Observed`: The result of a "measurement" or collapse. An actualized, definite state, visualized as a bright, stable pixel.
      * `Operator`: A special, user-defined state that is stable and influences its neighbors without changing.

### `universe.rs`: The Fabric of Spacetime

This file defines the grid where the Existons live and orchestrates the rules of their evolution from one moment (`tick`) to the next.

  * **`Universe` Struct:** Contains the **N-dimensional grid** of all Existons and the simulation's "physical constants" (like `observation_rate`, `decay_rate`, etc.).
  * **`tick()` method:** This is the engine of the simulation. In each tick, it applies the following rules:
    1.  **Local Interaction:** It calculates a local "Operator" for each Existon by summing the states of its **neighbors in N-dimensional space**. The Existon's next state is then calculated by multiplying its current state by this Operator using the Geometric Product.
    2.  **State Transitions:** It applies probabilistic rules for `Potential` cells to become `Observed` (observation), `Observed` cells to return to `Potential` (decay), and `Potential` cells to re-randomize their state (fluctuation).
    3.  **Non-Local Entanglement:** If a newly `Observed` Existon has an entangled partner, that partner is also instantly collapsed to an `Observed` state, demonstrating action at a distance.

### `main.rs`: The Laboratory Interface

This is the final layer that brings the abstract simulation to life. It handles setting up the window, translating the universe's state into pixels, and processing user input.

  * **Setup:** It initializes the window, loads the font for the UI, and creates the initial `Universe`, **defining its grid and GA dimensions.**
  * **Event Loop:** It runs a continuous loop that:
      * **Handles Input:** Listens for keyboard and mouse events to change simulation parameters or place Operators.
      * **Ticks the Universe:** Calls `universe.tick()` to advance the simulation one step.
      * **Renders the State:** Calls the `draw_app` function to **visualize a 2D slice** of the grid and the UI. It maps the `ConsciousnessState` and internal `Multivector` values of each Existon to a specific color.

-----

## How to Run (on Windows)

You do **not** need Nix or any complex setup. This project can be built and run with the standard Rust toolchain.

#### Step 1: Install the Rust Toolchain

If you don't have Rust installed, it's a very simple process:

1.  Go to the official Rust website: [https://rustup.rs/](https://rustup.rs/)
2.  Download the `rustup-init.exe` installer and run it.
3.  Choose the default installation options. This will install `rustc` (the compiler) and `cargo` (the build tool and package manager).

#### Step 2: Download the Project Code

1.  On the GitHub page for this project, click the green `<> Code` button.
2.  Select "Download ZIP".
3.  Extract the ZIP file to a folder on your computer (e.g., `C:\Users\Doug\Desktop\existons`).

#### Step 3: Build and Run the Simulation

1.  Open a Command Prompt (`cmd.exe`) or PowerShell.
2.  Navigate to the project folder you just extracted. For example:
    ```cmd
    cd C:\Users\Doug\Desktop\existons
    ```
3.  Run the following command:
    ```cmd
    cargo run --release
    ```
    The first time you run this, Cargo will download all the necessary dependencies and compile the project. This may take a few minutes. The `--release` flag enables optimizations, making the simulation run much faster.

After it finishes, the simulation window will appear. To run it again in the future, you can just run the same command, or double-click the executable file located at `target\release\existons.exe`.

-----

## Using the Simulation: An Observer's Guide

### Reading the Pixels

The window displays a 2D slice of the grid of Existons, with each pixel's color representing its current state:

  * **Colorful, Shifting "Foam":** These are `Potential` Existons. Their color and transparency are directly mapped from the first few components of their `Multivector` state. This shows the constant, underlying activity of the "quantum foam."
  * **Bright White/Yellow:** These are `Observed` Existons. They represent points of "actualized" reality that have collapsed from a potential state.
  * **Bright Cyan:** This is a user-placed `Operator`. It is a stable, fixed point in the grid that constantly influences its neighbors.
  * **Black:** The background color, representing the void.

### Controls and Parameters

The UI in the top-left corner displays the current simulation parameters, which you can change live using the following controls.

**Note:** The core dimensional parameters (`grid_dims` and `ga_dims`) are now set directly in the code inside `main.rs`. We encourage you to experiment with these values\!

| Control       | Parameter        | Conceptual Effect                                                                                                |
| :------------ | :--------------- | :--------------------------------------------------------------------------------------------------------------- |
| **`[Up/Down]`** | Observation Rate | The probability of a `Potential` state spontaneously collapsing. Higher values cause reality to "crystallize" faster. |
| **`[Left/Right]`**| Decay Rate       | The probability of an `Observed` state dissolving back into potentiality. Higher values make reality less "sticky." |
| **`[F]`** | Fluctuation Rate | The "quantum jitter." A chance for any `Potential` cell to re-randomize its state, preventing the simulation from stagnating. |
| **`[E]`** | Entanglement     | Cycles the percentage of non-locally connected pairs (1%, 5%, 10%, 20%), changing how interconnected the universe is. |
| **`[R]`** | Reset Universe   | Resets the entire simulation to a new, random initial state.                                                     |
| **`[L-Click]`** | Place Operator   | "Paints" a stable, cyan-colored Operator cell at the mouse cursor's position.                                    |
| **`[R-Click]`** | Erase Operator   | Resets the cell under the cursor to a `Potential` state.                                                         |
| **`[ESC]`** | Close Window     | Exits the application.                                                                                           |
