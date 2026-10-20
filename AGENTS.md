# AGENTS.md

## Overview

- WIP mining puzzle game: Rust edition 2024, `macroquad 0.4.16`,
  `egui-macroquad 0.17.3`.
- Cargo workspace: root package `mining_game` (game), `shared` (level schema,
  blocks, items), `level_editor` (native eframe editor). The editor uses egui
  0.36 while the game gets egui 0.31 through `egui-macroquad`; their UI types
  are not interchangeable.
- Repo folder is `rust_game` but the package/artifact is `mining_game`; web
  assets hardcode `mining_game.wasm`.
- No tests (`cargo test` runs zero), no rustfmt/clippy/toolchain config.

## LLM Rules

- Your role in this project is to help strategize the best way to implement
  given features, not to implement them yourself. This is a primarily human
  made project.
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

## Levels pipeline (`build.rs` codegen)

- `levels.ron` is the source of truth. `build.rs` parses `shared::RawLevel`
  and writes `levels_generated.rs` to `OUT_DIR`; `src/level/levels.rs`
  `include!`s it. Edit the RON, never the generated code.
- `RawLevel` has no id: `build.rs` stamps `id` from the array index, so array
  order defines level ids. Inserting or reordering levels shifts ids and
  therefore save progression/hub unlocks (`level.id <=
  profile.latest_completed_level`).
- `build.rs` fails the build on wrong row/column counts, unknown block ids,
  out-of-bounds or non-empty spawn, a chest payload not on an
  `ItemChest`/`GoalChest` or with an invalid item id, a `Collect` goal not
  backed by exactly one `GoalChest` at the goal cell with a payload, a
  `MineAllGold` level containing a `GoalChest`, and `MineAllGold(n)` where `n`
  does not equal the actual gold block count. Fix `levels.ron` rather than
  weakening validation.
- Grids are `grid[z][x]`; `build.rs` flattens row-major (`z*width+x`), while
  `GameMap` stores column-major (`x*height+z`) via `index_map`. Keep both
  directions consistent when touching either side.
- Adding a `RawLevel` field touches `shared/src/level.rs`, `build.rs`
  (validation + codegen), and `LevelData` in `src/level/levels.rs`. A new
  `RawGoal` variant also needs `build.rs::goal_to_rust`, plus runtime `Goal`,
  `is_goal_achieved`, and `to_str`.
- Chests are `chests: [((x, z), item_id)]`; `item_id` is a stable `ItemId`
  looked up in `shared::items::ITEMS`. Add a new item as a static plus
  `ItemEntry` with a new unique id; never renumber existing ids because saves
  store them. The build script rejects unknown chest item ids.
- The editor auto-syncs `MineAllGold` counts when saving.

## Blocks

- `shared/src/block.rs::BlockType` is the single block id table (`repr(u8)`,
  explicit values, `0` = empty). Runtime `blocktype_from_id` and the editor
  palette both derive from it.
- Adding a block: add the variant and `name` arm in `shared/src/block.rs`, the
  `stats` arm plus a `BlockDef` static in `src/level/block.rs`, and a color arm
  in `level_editor/src/palette.rs::block_color`. The editor palette list itself
  updates automatically via `strum::EnumIter`.
- Block colors are duplicated in the runtime `BlockDef` and the editor palette;
  keep them in sync.
- Mining is category-driven (`Wall`/`Rock`/`Ore`/`Crystal`/`Chest`) in
  `Block::mine`. Per-tile health lives on `Block`; shared stats and drops live
  on `BlockDef`. Don't hardcode stats in `level.rs`.

## Architecture

- `src/main.rs` loops on `GameState`; each screen's `*_update()` returns
  `TransitionState`, and `Game::set_state` runs side effects (load profile,
  start level, bank winnings, persist). Keep side effects there.
- `TransitionState::GoTo` goes through `Game::verify_goto`, which only allows
  `Settings`/`LoadSave`/`MainMenu`; new screen states need an explicit arm or
  the transition silently does nothing.
- `Level::new(id, ...)` panics for an unknown id; a fresh level is built per
  play from profile-derived stats. `get_level` returns `Option`.
- Coordinates are `MapCoords { x, z }` (x/z, not x/y). `GameMap` is a flat
  `Vec<Option<Block>>` with storage index `x*height+z`.
- Mining flow: `level.rs::move_player` -> `GameMap::mine_block` ->
  `Block::mine` -> `MiningOutcome`; only `level.rs` converts outcomes into
  player effects. Keep it that way.
- All UI (menus, HUD, pause/win/loss) uses egui via `egui_macroquad::ui`.
  There is no `root_ui` or debug overlay.
- Level editor UI and editing operations are in `level_editor/src/lib.rs`
  (one large file); `main.rs` only parses the path and launches eframe.
- Profiles save through `SavedProfile` in `src/save.rs`, not serde on
  `Profile`; adding profile fields touches `profile.rs` and `save.rs`. Use
  `#[serde(default)]` on new saved fields so old saves still load.

## Input / saves / web

- Cmd+W quits (macOS; `main.rs` checks `LeftSuper`). In-level: arrows/WASD
  move, hold Shift to mine, Esc toggles pause (not exit).
- Saves use quad-storage slots 1..=3 under `save_slot_N`. Native storage is
  `local.data` in the process CWD (gitignored; delete it to reset saves); web
  uses localStorage.
- `index.html` loads raw wasm via `load("mining_game.wasm")` and needs manual
  `<script>` tags for `static/*.js`. CI copies all of `static/`, but adding a
  JS plugin still requires adding its tag to `index.html`.
- `quad-storage.js` is required for web saves but is currently missing from
  `index.html`; add it before the `load(...)` call when fixing web saves.
- The wasm target needs `.cargo/config.toml`'s
  `link-arg=--allow-undefined`; don't remove it. Avoid adding
  wasm-bindgen/web-sys-dependent crates to the game.
- Root `mining_game.wasm` is a stale committed artifact for local testing;
  rebuild rather than trust it.
