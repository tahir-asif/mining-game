# AGENTS.md

## Overview

- Cargo workspace: root package `mining_game` (game), `shared` (level schema,
  blocks, items), `level_editor` (native eframe editor). The editor uses egui
  0.36 while the game gets egui 0.31 through `egui-macroquad`; their UI types
  are not interchangeable.
- Repo folder is `rust_game` but the package/artifact is `mining_game`; web
  assets hardcode `mining_game.wasm`.
- No tests (`cargo test` runs zero), no rustfmt/clippy/toolchain config.

## LLM Rules

- Your role in this project is to help strategize the best way to implement
  given features, not to implement them yourself. This is a primarily human made
  project.
- When you do produce code, give high level overviews of changes/additions that
  explain what to do, not show every single line.
- Don't write comments in code.
- Keep your responses shorter, focused on technical details and explain why you
  made choices that aren't obvious.

## Commands

- Full compile check: `cargo check --workspace`; plain `cargo check` skips
  `level_editor`.
- Level editor: `cargo editor` (alias = `run --release -p level_editor`). Run
  from the repo root; the optional path arg defaults to `levels.ron`.
- `cargo test` runs zero tests — verify with `cargo check`.
- Web build: `cargo build --release --target wasm32-unknown-unknown`. CI on
  `main` copies the wasm, `index.html`, and everything in `static/` to Pages.
- No formatter/linter config; don't mass-format unrelated code.

## Web

- `index.html` loads raw wasm via `load("mining_game.wasm")` and needs manual
  `<script>` tags for `static/*.js`. CI copies all of `static/`, but adding a JS
  plugin still requires adding its tag to `index.html`.
- `quad-storage.js` is required for web saves but is currently missing from
  `index.html`; add it before the `load(...)` call when fixing web saves.
