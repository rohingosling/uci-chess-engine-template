//----------------------------------------------------------------------------------------------------------------------
// Project: UCI Engine Template Rust
// Version: 1.0.0
// Date:    2026-06-17
// Author:  Rohin Gosling
//
// Description:
//
//   Integration tests for position-command setup before go commands.
//
//----------------------------------------------------------------------------------------------------------------------

use std::io::Write;
use std::process::{Command, Stdio};

use assert_cmd::prelude::*;

//----------------------------------------------------------------------------------------------------------------------
// Function: run_engine_script
//
// Description:
//
//   Run the engine binary with a scripted UCI command sequence.
//
//----------------------------------------------------------------------------------------------------------------------

fn run_engine_script(script: &str) -> String {
    // Each script is a tiny UCI conversation. The test writes every command up front and then reads
    // the complete stdout after the engine exits.

    let mut child = Command::cargo_bin("uci-engine-template")
        .unwrap()
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    // Closing stdin after the script avoids a test hang if the engine keeps waiting for more input.

    let mut standard_input = child.stdin.take().unwrap();
    standard_input.write_all(script.as_bytes()).unwrap();
    drop(standard_input);

    let output = child.wait_with_output().unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8(output.stdout).unwrap()
}

//----------------------------------------------------------------------------------------------------------------------
// Function: has_bestmove_line
//
// Description:
//
//   Return whether engine output contains a bestmove line.
//
//----------------------------------------------------------------------------------------------------------------------

fn has_bestmove_line(output: &str) -> bool {
    // The random selector makes the exact move unstable, so these tests assert the protocol shape.

    output.lines().any(|line| line.starts_with("bestmove "))
}

//----------------------------------------------------------------------------------------------------------------------
// Function: startpos_moves_then_go_returns_bestmove
//
// Description:
//
//   Verify a startpos move list can be applied before searching.
//
//----------------------------------------------------------------------------------------------------------------------

#[test]
fn startpos_moves_then_go_returns_bestmove() {
    let output = run_engine_script("position startpos moves e2e4 e7e5\ngo movetime 1\nquit\n");

    assert!(has_bestmove_line(&output));
}

//----------------------------------------------------------------------------------------------------------------------
// Function: fen_then_go_returns_bestmove
//
// Description:
//
//   Verify a FEN position can be applied before searching.
//
//----------------------------------------------------------------------------------------------------------------------

#[test]
fn fen_then_go_returns_bestmove() {
    let output = run_engine_script(
        "position fen rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1\ngo depth 1\nquit\n",
    );

    assert!(has_bestmove_line(&output));
}
