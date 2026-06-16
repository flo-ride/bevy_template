# Bevy Game Template

A robust starting template for 2D/3D games built with the [Bevy Engine](https://bevyengine.org/) in Rust.

## Overview

This project serves as a foundation for developing games with Bevy (v0.18.1), coming pre-configured with a selection of excellent ecosystem crates and optimized build profiles to accelerate your workflow.

### Features & Included Crates
- **Engine**: [Bevy](https://github.com/bevyengine/bevy) (v0.18.1)
- **Tilemaps**: [bevy_ecs_tilemap](https://github.com/StarArawn/bevy_ecs_tilemap) for performant 2D tile-based rendering.
- **Input**: [leafwing-input-manager](https://github.com/Leafwing-Studios/leafwing-input-manager) for powerful, declarative action-based input handling.
- **Debugging**: [bevy-inspector-egui](https://github.com/jakobhellermann/bevy-inspector-egui) (optional, via the `dev` feature) for real-time ECS inspection.
- **Utilities**: `rand` for random number generation and `husky-rs` for Git hooks.

### Project Structure
The `src/` directory is organized into a modular structure:

```tree
src/
├── main.rs
├── core/
│ ├── mod.rs
│ ├── states.rs
│ ├── events.rs
│ └── resources.rs
├── features/
│ ├── player/
│ │ ├── mod.rs
│ │ ├── components.rs
│ │ ├── systems.rs
│ │ └── plugin.rs
│ ├── enemy/
│ ├── combat/
│ └── inventory/
├── ui/
│ ├── mod.rs
│ ├── hud.rs
│ └── menus.rs
└── plugins.rs
```

## Getting Started

### Prerequisites
- [Rust toolchain](https://rustup.rs/) (edition 2024)

### Running the Project

**Standard Build**
```sh
cargo run
```

**Development Build (Recommended)**
Runs the game with dynamic linking (for significantly faster iterative compile times) and enables the debug inspector:
```sh
cargo run --features dev
```

### Build Profiles

This template includes tailored build profiles in `Cargo.toml`:
- `dev`: High optimization for dependencies (`opt-level = 3`), low for your code (`opt-level = 1`) to ensure fast compile times while keeping the game running smoothly.
- `release`: Maximized for runtime performance (`lto = "thin"`, `codegen-units = 1`).
- `wasm-release`: Specifically optimized for size to reduce web payloads (`opt-level = "s"`, stripped debug info).

## Nix & Flake

This project includes a `flake.nix` configured with `bevy-flake` and `rust-overlay` for reproducible development environments.

### Usage

**Enter the Development Shell**
This will drop you into a shell with the required Rust toolchain, system dependencies, and CLI tools (like `bevy-cli`) pre-installed.
```sh
nix develop
```

**Build the Project**
To build the project using Nix:
```sh
nix build
```

**Run the Project**
To run the game directly:
```sh
nix run
```

## Building for Web (WASM)

To build for web, make sure you have the WASM target installed:
```sh
rustup target add wasm32-unknown-unknown
cargo build --profile wasm-release --target wasm32-unknown-unknown
```
