# Specification: Rust Random UCI Chess Engine Template

## Table of Contents

- [1. Project summary](#1-project-summary)
- [2. Project location](#2-project-location)
- [3. Primary goals](#3-primary-goals)
- [4. Non-goals](#4-non-goals)
- [5. Technology requirements](#5-technology-requirements)
- [6. Executable behavior](#6-executable-behavior)
- [7. UCI command support](#7-uci-command-support)
- [8. Search limits model](#8-search-limits-model)
- [9. Board and move requirements](#9-board-and-move-requirements)
- [10. Move selector interface](#10-move-selector-interface)
- [11. Output requirements](#11-output-requirements)
- [12. Error handling requirements](#12-error-handling-requirements)
- [13. Testing requirements](#13-testing-requirements)
- [14. Manual smoke test](#14-manual-smoke-test)
- [15. Arena workflow](#15-arena-workflow)
- [16. GitHub release workflow](#16-github-release-workflow)
- [17. Acceptance criteria](#17-acceptance-criteria)
- [18. Future extension ideas](#18-future-extension-ideas)

## 1. Project summary

This project is a minimal Rust UCI chess engine template.

The first implementation plays random legal moves. It is intended for learning, experimentation, and project bootstrapping, not competitive chess strength.

The engine must be usable in Arena and other UCI-compatible chess GUIs.

## 2. Project location

The repository root is the Rust project root.

```text
project-root/
├─ Cargo.toml
├─ README.md
├─ docs/
├─ src/
└─ tests/
```

Project source files, documentation, tests, and build configuration belong at that project root.

## 3. Primary goals

The engine must:

- demonstrate the minimum useful structure of a UCI engine;
- compile as a Rust command-line executable;
- communicate through standard input and standard output;
- support the core UCI commands needed by Arena;
- generate only legal chess moves;
- select a random legal move when asked to move;
- be modular enough that the random move selector can be replaced later;
- include automated tests for the basic protocol workflow.

## 4. Non-goals

The first version does not need:

- strong chess play;
- search;
- evaluation;
- pondering;
- multithreaded thinking;
- opening books;
- endgame tablebases;
- UCI option tuning;
- PGN support;
- Chess960 support beyond what the board library naturally provides;
- a GUI;
- networking;
- telemetry.

## 5. Technology requirements

### 5.1 Language and build system

- Rust stable.
- Cargo.
- Binary crate with a companion library crate.

### 5.2 Runtime dependencies

Use a small dependency set:

- `cozy-chess` for chess board representation, legal move generation, FEN handling, and move conversion support.
- `rand` for random move selection.

### 5.3 Development dependencies

Use integration-test helpers such as:

- `assert_cmd`;
- `predicates`;
- equivalent crates if the implementation chooses another clean test approach.

### 5.4 Dependency constraints

- Avoid GPL dependencies in the default template.
- Avoid unnecessary runtime frameworks.
- Avoid async dependencies in the first version.
- Avoid network dependencies.

## 6. Executable behavior

The engine is a command-line process.

At startup, it should not print banners, logs, or prompts. It should wait for UCI commands on stdin.

Standard output is reserved for UCI protocol responses. Logs and diagnostics must go to standard error.

The process must exit cleanly after receiving:

```text
quit
```

## 7. UCI command support

### 7.1 `uci`

Input:

```text
uci
```

Required output:

```text
id name Random Rust UCI Engine
id author Rohin Gosling
uciok
```

The exact engine name may change if the project is renamed, but the response must end with `uciok`.

### 7.2 `isready`

Input:

```text
isready
```

Required output:

```text
readyok
```

The engine must respond to `isready` even if future versions are doing initialization or search work.

### 7.3 `debug`

Inputs:

```text
debug on
debug off
```

Required behavior:

- `debug on` enables diagnostic logging to standard error.
- `debug off` disables diagnostic logging.
- No debug output should be written to standard output.

### 7.4 `setoption`

Input examples:

```text
setoption name Hash value 128
setoption name Threads value 1
```

Required behavior for the first version:

- recognize the command;
- do not crash;
- ignore unknown options;
- optionally log ignored options to standard error in debug mode.

The first version does not need to expose engine options.

### 7.5 `ucinewgame`

Input:

```text
ucinewgame
```

Required behavior:

- reset game-specific state;
- reset the current position to the standard start position or mark the session as a new game awaiting an explicit `position` command;
- do not print a response unless later required by a separate command such as `isready`.

Recommended GUI sequence:

```text
ucinewgame
isready
```

Response:

```text
readyok
```

### 7.6 `position startpos`

Input:

```text
position startpos
```

Required behavior:

- set the current board to the standard chess starting position.

### 7.7 `position startpos moves ...`

Input example:

```text
position startpos moves e2e4 e7e5 g1f3
```

Required behavior:

- set the board to the starting position;
- apply each listed UCI move in order;
- validate moves through the board adapter;
- avoid panics on illegal or malformed moves.

If a move is invalid, the engine should log a diagnostic to standard error and either stop applying further moves or reject the whole position update. The chosen behavior must be documented in code comments or README.

### 7.8 `position fen ...`

Input example:

```text
position fen rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
```

Required behavior:

- parse the supplied FEN;
- set the current board to that position;
- avoid panics on invalid FEN.

A full FEN has six fields:

```text
piece-placement side-to-move castling-rights en-passant halfmove-clock fullmove-number
```

The parser should preserve all six fields when passing the FEN to the board layer.

### 7.9 `position fen ... moves ...`

Input example:

```text
position fen rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 moves e2e4 e7e5
```

Required behavior:

- parse and set the FEN position;
- apply all listed UCI moves in order.

### 7.10 `go`

Input examples:

```text
go
go movetime 1000
go wtime 60000 btime 60000 winc 1000 binc 1000
go depth 1
go infinite
```

Required behavior:

- parse common search-limit fields where practical;
- request a move from the active `MoveSelector`;
- print exactly one `bestmove` line for each `go` command unless the process is exiting.

Output example:

```text
bestmove e2e4
```

The chosen move must be legal in the current position.

When no legal moves exist, output:

```text
bestmove 0000
```

### 7.11 `stop`

Input:

```text
stop
```

Required behavior for the first version:

- recognize the command;
- do not crash;
- no-op is acceptable because random move selection is immediate.

Future versions with long-running search must stop searching and return the best move found so far.

### 7.12 `quit`

Input:

```text
quit
```

Required behavior:

- exit the engine process cleanly;
- do not hang;
- do not require additional input.

### 7.13 Unknown commands

Required behavior:

- do not crash;
- ignore the command;
- optionally log to standard error in debug mode.

## 8. Search limits model

Create a `SearchLimits` type to represent the UCI `go` command.

Recommended fields:

```rust
pub struct SearchLimits {
    pub searchmoves: Vec<String>,
    pub wtime: Option<u64>,
    pub btime: Option<u64>,
    pub winc: Option<u64>,
    pub binc: Option<u64>,
    pub movestogo: Option<u64>,
    pub depth: Option<u64>,
    pub nodes: Option<u64>,
    pub mate: Option<u64>,
    pub movetime: Option<u64>,
    pub infinite: bool,
}
```

The random selector may ignore these values. The fields exist to prevent the UCI layer from being redesigned later.

## 9. Board and move requirements

The board layer must:

- represent the current chess position;
- support start position creation;
- support FEN parsing;
- apply UCI moves;
- generate legal moves;
- display moves in public UCI format.

The engine must not return pseudo-legal moves. It must return legal moves.

Special cases that should be handled by the board library and covered by smoke tests over time:

- castling;
- promotion;
- en passant;
- check;
- checkmate;
- stalemate.

## 10. Move selector interface

The random engine must be implemented behind a trait.

Recommended shape:

```rust
pub trait MoveSelector {
    fn select_move(
        &mut self,
        position: &Position,
        limits: &SearchLimits,
    ) -> Option<cozy_chess::Move>;
}
```

The first implementation is:

```text
RandomMoveSelector
```

Required behavior:

- collect legal moves from the current position;
- return `None` if no legal moves exist;
- otherwise return one randomly selected legal move.

The UCI driver must not depend directly on `RandomMoveSelector`. It should depend on the session or trait abstraction.

## 11. Output requirements

Every UCI response should be line-oriented and flushed.

Required stdout examples:

```text
id name Random Rust UCI Engine
id author Rohin Gosling
uciok
readyok
bestmove e2e4
bestmove 0000
```

Forbidden on stdout:

```text
Starting engine...
Debug: parsed command
Error: invalid move
panic trace
```

Diagnostics belong on stderr.

## 12. Error handling requirements

The engine should avoid panics for normal GUI input.

Required behavior:

- invalid command: ignore or log to stderr;
- invalid FEN: log to stderr and keep a safe previous/default position;
- invalid move: log to stderr and keep a safe position;
- no legal moves: `bestmove 0000`;
- I/O failure: exit cleanly if possible.

The first version should be forgiving rather than strict.

## 13. Testing requirements

### 13.1 Unit tests

Unit tests should cover:

- UCI command parsing;
- `go` search-limit parsing;
- position command parsing;
- response formatting;
- board adapter methods where practical.

### 13.2 Integration tests

Integration tests should spawn the engine executable and communicate over stdin/stdout.

Required tests:

1. `uci` returns identity lines and `uciok`.
2. `isready` returns `readyok`.
3. `position startpos` followed by `go movetime 1` returns `bestmove ...`.
4. `position startpos moves e2e4 e7e5` followed by `go movetime 1` returns `bestmove ...`.
5. A no-legal-move position does not hang and returns `bestmove 0000` or the documented equivalent.
6. Unknown commands do not crash the process.

Because the engine is random, tests must not expect a specific move unless the position has only one legal move.

## 14. Manual smoke test

From the project root:

```powershell
cargo build --release
```

Run the executable manually and type:

```text
uci
isready
position startpos
go movetime 1000
quit
```

Expected response shape:

```text
id name Random Rust UCI Engine
id author Rohin Gosling
uciok
readyok
bestmove <legal-uci-move>
```

## 15. Arena workflow

Local developer workflow:

1. Build the release binary from the project root.
2. Confirm the executable exists under `target/release/`.
3. Smoke-test the engine manually in a terminal.
4. Open Arena.
5. Install a new engine.
6. Select the compiled executable.
7. Choose UCI if Arena asks for the protocol.
8. Start a game or analysis session.
9. Confirm that the engine replies with legal random moves.

## 16. GitHub release workflow

For end users, the preferred release artifact is a prebuilt Windows executable zip.

Recommended release package:

```text
random-uci-engine-vX.Y.Z-windows-x86_64.zip
```

The zip should contain:

- the engine executable;
- README or installation notes;
- license file.

Developer build-from-source workflow:

```powershell
git clone <repo-url>
cd <repo>
cargo build --release
```

Then install the resulting binary into Arena or another UCI-compatible GUI.

## 17. Acceptance criteria

The scaffold is acceptable when:

- all project files are inside the Cargo project root;
- `cargo build` succeeds;
- `cargo test` succeeds;
- `cargo fmt` has been run;
- `cargo clippy -- -D warnings` passes or unavailable Clippy is clearly reported;
- the engine responds correctly to `uci`;
- the engine responds correctly to `isready`;
- the engine returns a legal `bestmove` after `position startpos` and `go`;
- the engine does not write non-protocol diagnostics to stdout;
- the random move selector is implemented behind a trait;
- the project README explains how to build and test with Arena.

## 18. Future extension ideas

Future versions may add:

- a material-counting selector;
- minimax;
- alpha-beta search;
- quiescence search;
- transposition tables;
- iterative deepening;
- time management;
- UCI options;
- neural-network inference;
- Monte Carlo tree search;
- entropy-based experimental move selection;
- opening book support;
- endgame tablebase support;
- cross-platform GitHub Actions releases.

These should extend the existing architecture rather than collapse the UCI, board, and engine layers together.
