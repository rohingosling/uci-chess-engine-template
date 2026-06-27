//----------------------------------------------------------------------------------------------------------------------
// Project: UCI Engine Template Rust
// Version: 1.0.0
// Date:    2026-06-17
// Author:  Rohin Gosling
//
// Description:
//
//   Integration tests for go-command behavior and bestmove output.
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
    // Spawn the real binary so the test covers command-line I/O and not only internal functions.

    let mut child = Command::cargo_bin("uci-engine-template")
        .unwrap()
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    // The engine reads line-oriented UCI text from stdin. Dropping the pipe after writing tells the
    // process there is no more input.

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
// Function: assert_valid_bestmove_line
//
// Description:
//
//   Assert that engine output contains a syntactically valid bestmove line.
//
//----------------------------------------------------------------------------------------------------------------------

fn assert_valid_bestmove_line(output: &str) {
    // Because the selector is random, the test validates UCI syntax instead of expecting one fixed
    // chess move.

    let bestmove_line = output
        .lines()
        .find(|line| line.starts_with("bestmove "))
        .expect("missing bestmove line");
    let move_text = bestmove_line.trim_start_matches("bestmove ");

    assert!(
        is_valid_uci_move_text(move_text),
        "invalid bestmove line: {bestmove_line}"
    );
}

//----------------------------------------------------------------------------------------------------------------------
// Function: is_valid_uci_move_text
//
// Description:
//
//   Return whether text has the shape of a UCI move or null move.
//
//----------------------------------------------------------------------------------------------------------------------

fn is_valid_uci_move_text(move_text: &str) -> bool {
    // "0000" is UCI's null move for positions where no legal move is available.

    if move_text == "0000" {
        return true;
    }

    // Normal UCI moves are four characters, such as "e2e4". Promotions add one piece character,
    // such as "a7a8q".

    let bytes = move_text.as_bytes();

    if bytes.len() != 4 && bytes.len() != 5 {
        return false;
    }

    let valid_from_square = (b'a'..=b'h').contains(&bytes[0]) && (b'1'..=b'8').contains(&bytes[1]);
    let valid_to_square = (b'a'..=b'h').contains(&bytes[2]) && (b'1'..=b'8').contains(&bytes[3]);
    let valid_promotion = bytes.len() == 4 || matches!(bytes[4], b'q' | b'r' | b'b' | b'n');

    valid_from_square && valid_to_square && valid_promotion
}

//----------------------------------------------------------------------------------------------------------------------
// Function: startpos_go_returns_bestmove
//
// Description:
//
//   Verify a go command from the starting position returns a bestmove.
//
//----------------------------------------------------------------------------------------------------------------------

#[test]
fn startpos_go_returns_bestmove() {
    let output = run_engine_script("position startpos\ngo movetime 1\nquit\n");

    assert_valid_bestmove_line(&output);
}

//----------------------------------------------------------------------------------------------------------------------
// Function: no_legal_move_returns_null_move
//
// Description:
//
//   Verify positions with no legal moves return the UCI null move.
//
//----------------------------------------------------------------------------------------------------------------------

#[test]
fn no_legal_move_returns_null_move() {
    let output =
        run_engine_script("position fen 7k/5K2/6Q1/8/8/8/8/8 b - - 0 1\ngo movetime 1\nquit\n");

    assert!(output.lines().any(|line| line == "bestmove 0000"));
}

//----------------------------------------------------------------------------------------------------------------------
// Function: unknown_command_does_not_crash
//
// Description:
//
//   Verify unknown commands do not prevent later commands from running.
//
//----------------------------------------------------------------------------------------------------------------------

#[test]
fn unknown_command_does_not_crash() {
    let output = run_engine_script("this-is-not-a-uci-command\nisready\nquit\n");

    assert!(output.lines().any(|line| line == "readyok"));
}
