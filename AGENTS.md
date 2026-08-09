# Hexagon agent instructions

## Project

This is a Rust 2024 terminal UI for hexagonal chess. It uses `ratatui`/`crossterm` for rendering/input and keeps the game model in `src/board.rs`, `src/state.rs`, `src/pieces/`, `src/unit/`, and `src/utils/`.

## Workflow

- Keep changes small and focused on the requested behavior.
- Prefer explicit, type-checked game rules over ad-hoc flags or coordinate math scattered across the UI.
- Preserve the existing module layout unless a broader refactor is explicitly requested.
- Do not mix formatting-only or unrelated cleanup with feature/bug changes.

## Rust conventions

- Run `cargo fmt` after Rust edits.
- Run `cargo check` for normal validation; run `cargo test` when changing move logic, board geometry, or utility types with tests.
- Use `anyhow::Result` for top-level fallible app flow, matching `src/main.rs`.
- Keep most internal APIs `pub(crate)` unless there is a clear external API need.
- Unsafe construction such as `Cell::from_raw_parts` is for trusted constants only; prefer checked constructors in new runtime code.

## Game/model notes

- Board coordinates are represented by `Rank`, `File`, `Cell`, directions, deltas, and ranges. Use those types instead of raw integers where possible.
- Piece movement belongs with the piece implementations in `src/pieces/`; board state updates and move legality should stay centralized enough that capture, promotion, en passant, highlighting, and history remain consistent.
- Rendering details should not become the source of truth for game rules.

## Mark-based status rendering

- All text-based status in the TUI must use the existing custom `Mark` rendering, not normal text widgets.
- Marks are based on sixteen-segment displays and remain readable when the terminal font is reduced for clean hexagon edges.
- Before rendering an alphabetic character or symbol, check whether its mark exists. If it does not, create the mark first in `src/utils/mark.rs` and wire it into the relevant drawing dispatch.
- Preserve the mark-based style for status, result, and similar board-side UI text; do not replace it with ordinary Ratatui text merely for convenience.

## Repository navigation

- On every session start for this project, sync TokenSave MCP before doing repository navigation so its index reflects recent changes.
- Use TokenSave MCP only for repository navigation, semantic search, context gathering, callers/callees, and impact analysis.
- Use normal Pi tools only for exact file inspection after TokenSave identifies relevant files, for editing, and for command execution/validation.
- If TokenSave MCP is unavailable or cannot be synced, stop and ask the user before falling back to other navigation methods.

## Useful commands

```sh
cargo fmt
cargo check
cargo test
cargo run -- --help
cargo run -- --hide-pieces --depth 4
```
