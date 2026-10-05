# AGENTS.md

## Overview

Mining game in Rust on **macroquad 0.4.16** (edition 2024). Work in progress.
There is no test suite, and no clippy/rustfmt/toolchain config in the repo.

## LLM Specific Rules

- Shorten output messages to only contain information relevant to the current
  query

## Commands

- Run desktop: `cargo run`
- Compile check: `cargo check`
- Web build (same as CI):
  `cargo build --release --target wasm32-unknown-unknown`
- `cargo test` runs **zero tests** — it is not meaningful verification. Verify
  with `cargo check` and `cargo run`.
- Web target requires: `rustup target add wasm32-unknown-unknown`

## Naming gotcha

Package/crate name is `mining_game`, but the repo folder is `rust_game`. Web
assets hardcode `mining_game.wasm` (`index.html` and CI). Renaming the package
breaks the web build unless those references are updated too.

## Web deployment

- CI (`.github/workflows/deploy.yml`): on push to `main`, builds wasm and
  publishes `mining_game.wasm` + `index.html` to gh-pages. Uses older actions
  (`actions-rs/toolchain@v1`, `checkout@v2`, `gh-pages@v3`).
- The web build uses macroquad's **no-wasm-bindgen** miniquad path: `index.html`
  loads the raw wasm via `mq_js_bundle.js`. Adding crates that require
  `wasm-bindgen`/`web-sys` (or a `wasm-bindgen` build step) will not load as-is.
  Extra JS must be added to **both** `index.html` and the CI copy step.
- A stale `mining_game.wasm` is committed at the repo root for local
  `index.html` testing; CI always rebuilds it from source.

## Architecture

- Entrypoint `src/main.rs`: an explicit loop dispatches a `GameState` (defined
  in `src/common.rs`) to a per-screen `*_update() -> GameState`. Screens live in
  `main_menu.rs`, `hub.rs`, `level.rs`. No engine or UI framework beyond
  macroquad.
- `Level` is built once in `main` and `init()`ed once; switching screens does
  not currently rebuild it.
- UI uses macroquad's immediate `root_ui()`.
- Coordinates are `MapCoords { x, z }` (x/z, not x/y). `GameMap` is a flat
  `Vec<Option<Block>>` indexed as `x * height + z`.
- Mining flow: `level.rs::move_player` -> `GameMap::mine_block` -> `Block::mine`
  -> `MiningOutcome`. Only `level.rs` translates outcomes into player effects;
  keep that separation.
- Blocks are data-driven (`src/level/block.rs`): add a `BlockType` variant, a
  `static BlockDef`, and a `def()` match arm. `Block` stores per-tile `health`;
  shared stats live in `&'static BlockDef`. Do not hardcode stats in mining
  logic.
- Debug controls (`src/level/debug.rs`): Enter toggles the overlay, P toggles
  the top-down camera.
