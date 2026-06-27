//----------------------------------------------------------------------------------------------------------------------
// Project: UCI Engine Template Rust
// Version: 1.0.0
// Date:    2026-06-17
// Author:  Rohin Gosling
//
// Description:
//
//   Permissive parser for the subset of UCI used by this template.
//
//----------------------------------------------------------------------------------------------------------------------

use crate::search::limits::SearchLimits;
use crate::uci::command::{PositionCommand, UciCommand};

//----------------------------------------------------------------------------------------------------------------------
// Function: parse_uci_command
//
// Description:
//
//   Parse one line of UCI text into a command value.
//
//----------------------------------------------------------------------------------------------------------------------

pub fn parse_uci_command(text: &str) -> UciCommand {
    let trimmed_text = text.trim();

    if trimmed_text.is_empty() {
        return UciCommand::Unknown(String::new());
    }

    // UCI is line-oriented and space-delimited. The parser stays permissive: malformed commands are
    // converted to Unknown instead of causing the engine process to fail.

    let tokens = trimmed_text.split_whitespace().collect::<Vec<_>>();

    // The first token identifies the command. Command-specific helper functions parse the remaining
    // body so each UCI grammar rule stays small and readable.

    match tokens[0] {
        "uci" => UciCommand::Uci,
        "isready" => UciCommand::IsReady,
        "ucinewgame" => UciCommand::UciNewGame,
        "position" => parse_position_command(&tokens[1..])
            .map(UciCommand::Position)
            .unwrap_or_else(|| UciCommand::Unknown(trimmed_text.to_string())),
        "go" => UciCommand::Go(SearchLimits::parse_from_tokens(&tokens[1..])),
        "stop" => UciCommand::Stop,
        "debug" => parse_debug_command(&tokens)
            .map(UciCommand::Debug)
            .unwrap_or_else(|| UciCommand::Unknown(trimmed_text.to_string())),
        "setoption" => parse_setoption_command(&tokens[1..]),
        "quit" => UciCommand::Quit,
        _ => UciCommand::Unknown(trimmed_text.to_string()),
    }
}

//----------------------------------------------------------------------------------------------------------------------
// Function: parse_debug_command
//
// Description:
//
//   Parse the UCI debug command state.
//
//----------------------------------------------------------------------------------------------------------------------

fn parse_debug_command(tokens: &[&str]) -> Option<bool> {
    // The UCI spelling is "debug on" or "debug off". Returning None lets the caller mark anything
    // else as Unknown.

    match tokens.get(1).copied() {
        Some("on") => Some(true),
        Some("off") => Some(false),
        _ => None,
    }
}

//----------------------------------------------------------------------------------------------------------------------
// Function: parse_position_command
//
// Description:
//
//   Parse a UCI position command body into a position command value.
//
//----------------------------------------------------------------------------------------------------------------------

fn parse_position_command(tokens: &[&str]) -> Option<PositionCommand> {
    // A position command has two supported bases:
    // - "startpos" for the normal chess initial board.
    // - "fen ..." for an explicit Forsyth-Edwards Notation position.

    match tokens.first().copied() {
        Some("startpos") => parse_startpos_command(&tokens[1..]),
        Some("fen") => parse_fen_command(&tokens[1..]),
        _ => None,
    }
}

//----------------------------------------------------------------------------------------------------------------------
// Function: parse_startpos_command
//
// Description:
//
//   Parse the startpos form of the UCI position command.
//
//----------------------------------------------------------------------------------------------------------------------

fn parse_startpos_command(tokens: &[&str]) -> Option<PositionCommand> {
    if tokens.is_empty() {
        // "position startpos" is valid by itself and means no moves have been played yet.

        return Some(PositionCommand::Startpos { moves: Vec::new() });
    }

    if tokens[0] != "moves" {
        return None;
    }

    // After "moves", every remaining token is UCI long algebraic move text such as "e2e4".

    Some(PositionCommand::Startpos {
        moves: tokens[1..].iter().map(|token| token.to_string()).collect(),
    })
}

//----------------------------------------------------------------------------------------------------------------------
// Function: parse_fen_command
//
// Description:
//
//   Parse the FEN form of the UCI position command.
//
//----------------------------------------------------------------------------------------------------------------------

fn parse_fen_command(tokens: &[&str]) -> Option<PositionCommand> {
    if tokens.is_empty() {
        return None;
    }

    // A FEN string itself contains spaces, so it cannot be read as just one token. The optional
    // "moves" marker tells us where the FEN ends and the trailing move list begins.

    let moves_marker_index = tokens.iter().position(|token| *token == "moves");
    let fen_token_count = moves_marker_index.unwrap_or(tokens.len());

    if fen_token_count == 0 {
        return None;
    }

    let fen = tokens[..fen_token_count].join(" ");

    // If "moves" was present, copy every token after it as a UCI move. If not, the move list is empty.

    let moves = moves_marker_index
        .map(|index| {
            tokens[index + 1..]
                .iter()
                .map(|token| token.to_string())
                .collect()
        })
        .unwrap_or_default();

    Some(PositionCommand::Fen { fen, moves })
}

//----------------------------------------------------------------------------------------------------------------------
// Function: parse_setoption_command
//
// Description:
//
//   Parse a UCI setoption command into name and optional value fields.
//
//----------------------------------------------------------------------------------------------------------------------

fn parse_setoption_command(tokens: &[&str]) -> UciCommand {
    // UCI options use free-form words after "name" and "value", so this parser finds the markers
    // first and then joins the words between them back into strings.

    let name_marker_index = tokens.iter().position(|token| *token == "name");
    let value_marker_index = tokens.iter().position(|token| *token == "value");

    // The option name spans every token after "name" until "value" begins. Missing names become an
    // empty string so the command can still be represented and ignored safely.

    let name = name_marker_index
        .map(|index| {
            let end_index = value_marker_index.unwrap_or(tokens.len());
            tokens[index + 1..end_index].join(" ")
        })
        .unwrap_or_default();

    // The value is optional in UCI because some option types are buttons or toggles rather than
    // string-valued settings.

    let value = value_marker_index.map(|index| tokens[index + 1..].join(" "));

    // Preserve the parsed shape even though this template does not implement configurable options yet.

    UciCommand::SetOption { name, value }
}

#[cfg(test)]
mod tests {
    use super::*;

    //------------------------------------------------------------------------------------------------------------------
    // Function: parse_uci_handshake
    //
    // Description:
    //
    //   Verify the uci command parses as a handshake command.
    //
    //------------------------------------------------------------------------------------------------------------------

    #[test]
    fn parse_uci_handshake() {
        assert_eq!(parse_uci_command("uci"), UciCommand::Uci);
    }

    //------------------------------------------------------------------------------------------------------------------
    // Function: parse_startpos_with_moves
    //
    // Description:
    //
    //   Verify startpos commands preserve their trailing move list.
    //
    //------------------------------------------------------------------------------------------------------------------

    #[test]
    fn parse_startpos_with_moves() {
        assert_eq!(
            parse_uci_command("position startpos moves e2e4 e7e5"),
            UciCommand::Position(PositionCommand::Startpos {
                moves: vec!["e2e4".to_string(), "e7e5".to_string()],
            })
        );
    }

    //------------------------------------------------------------------------------------------------------------------
    // Function: parse_fen_with_moves
    //
    // Description:
    //
    //   Verify FEN commands preserve the FEN and trailing move list.
    //
    //------------------------------------------------------------------------------------------------------------------

    #[test]
    fn parse_fen_with_moves() {
        // This command includes a six-field FEN followed by a move list. The parser must keep the FEN
        // fields together and not mistake the first move for part of the position.

        assert_eq!(
            parse_uci_command(
                "position fen rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 moves e2e4"
            ),
            UciCommand::Position(PositionCommand::Fen {
                fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
                moves: vec!["e2e4".to_string()],
            })
        );
    }

    //------------------------------------------------------------------------------------------------------------------
    // Function: parse_go_movetime
    //
    // Description:
    //
    //   Verify go movetime commands parse into search limits.
    //
    //------------------------------------------------------------------------------------------------------------------

    #[test]
    fn parse_go_movetime() {
        // A "go" command can carry many limit types; this test keeps the input tiny so it isolates
        // the fixed-move-time case.

        let command = parse_uci_command("go movetime 1000");

        // Pattern matching proves the parser returned the command variant that owns SearchLimits.

        match command {
            UciCommand::Go(limits) => assert_eq!(limits.move_time, Some(1000)),
            other => panic!("expected go command, got {other:?}"),
        }
    }
}
