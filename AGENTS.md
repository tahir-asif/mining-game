# AGENTS.md

## Overview

- WIP mining puzzle game: Rust edition 2024, `macroquad 0.4.16`,
  `egui-macroquad`.
- Cargo workspace: root package `mining_game` (the game), `level_schema` (shared
  RON level types), `level_editor` (native egui editor).
- Repo folder is `rust_game` but the package is `mining_game`; web assets
  hardcode `mining_game.wasm` (`index.html`, `.github/workflows/deploy.yml`).
- No test suite (zero `#[test]`), no rustfmt/clippy/toolchain config.

## LLM Rules

- Your role in this project is to help strategize the best way to implement
  given features, not to implement them yourself. This is a primarily human made
  project
- When you do produce code, give high level overviews of changes/additions that
  explain what to do, not show every single line
- Don't write comments in code
- Keep your responses shorter, focused on technical details and explain why you
  made choices that aren't obvious

## Commands

- Compile check: `cargo check`, add `--workspace` flag to check all workspaces
- Level editor: `cargo editor` (alias = `run --release -p level_editor`);
  optional path arg, defaults to `levels.ron`
- `cargo test` runs zero tests — verify with `cargo check`

## Levels pipeline (build.rs codegen — easy to get wrong)

- `levels.ron` is the source of truth; `build.rs` generates
  `levels_generated.rs` (in `OUT_DIR`), included by `src/level/levels.rs`. Edit
  the RON, never the generated code.
- `LEVELS` is indexed positionally (`level_data(id) == LEVELS[id]`), so each
  level's `id` must equal its 0-based index in `levels.ron`. Keep ids contiguous
  and ordered.
- Grid is `grid[z][x]`; `build.rs` flattens it row-major (z*width+x) and
  `GameMap::generate_level` decodes it the same way.
- Adding a level field touches three places: `level_schema::RawLevel`,
  `build.rs` codegen, and `LevelData` in `src/level/levels.rs`.
- Chest item names in `levels.ron` are raw Rust identifiers: `"DIAMOND_PICK"`
  becomes `&DIAMOND_PICK`, resolving to a `static` in `src/save.rs` (levels.rs
  does `use save::*`). A chest name with no matching static fails to compile;
  add the static first. Chests currently only yield `Pickaxe`s.

## Block IDs are duplicated and currently disagree

- Editor palette: `level_editor/src/palette.rs` (`BLOCKS`).
- Runtime mapping: `src/level/map.rs::block_from_id` (+ `BlockType`/`BlockDef`
  in `src/level/block.rs`).
- These tables do not match today (e.g. id 2 is `GoldOre` in the editor but
  `Rock1` at runtime). Change blocks in both, plus `BlockType`/`BlockDef` for a
  new runtime block.
- Editor-only ids `Spawn(255)` and `Chest(254)` live in the level's
  `spawn`/`chests`, not in the grid.

## Architecture

- `src/main.rs` loops on `GameState` (in `src/game.rs`) dispatching to
  per-screen `*_update() -> TransitionState` (`main_menu.rs`, `hub.rs`,
  `src/level.rs`). Transitions both switch screens and run side effects via
  `Game::set_state` (load profile, build level, bank winnings).
- Menus/HUD use egui via `egui_macroquad::ui`; only the in-level debug overlay
  uses macroquad `root_ui`.
- `Level::new(id, ...)` panics for an unknown id; a fresh `Level` is built per
  play from the profile's derived stats.
- Coordinates are `MapCoords { x, z }` (x/z, not x/y). `GameMap` is a flat
  `Vec<Option<Block>>` with storage index `x*height+z`.
- Mining flow: `level.rs::move_player` -> `GameMap::mine_block` -> `Block::mine`
  -> `MiningOutcome`; only `level.rs` converts outcomes to player effects. Keep
  it that way.
- Blocks are data-driven (`src/level/block.rs`): add a `BlockType` variant, a
  `static BlockDef`, and a `def()` arm. Per-tile `health` is on `Block`, shared
  stats on `&'static BlockDef`; don't hardcode stats in mining logic.

## Input / gotchas

- Quit: Cmd+W. Level: arrows/WASD move, hold Shift to mine, Esc exits the level.
- Debug overlay: Enter toggles, P toggles top-down camera, J/K/L move camera
  position, U/I/O move camera target (hold Shift to reverse).
- Web build uses macroquad's no-wasm-bindgen path: `index.html` loads raw wasm
  via `static/*.js` (CI copies them next to `index.html`). Adding crates needing
  `wasm-bindgen`/`web-sys` breaks the web build; new JS must be added to
  `index.html` and the CI copy step. A stale `mining_game.wasm` is committed at
  the root for local testing.
