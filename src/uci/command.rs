//----------------------------------------------------------------------------------------------------------------------
// Project: UCI Engine Template Rust
// Version: 1.0.0
// Date:    2026-06-17
// Author:  Rohin Gosling
//
// Description:
//
//   Parsed UCI command types.
//
//----------------------------------------------------------------------------------------------------------------------

use crate::search::limits::SearchLimits;

//----------------------------------------------------------------------------------------------------------------------
// Enum: UciCommand
//
// Description:
//
//   Represents parsed UCI protocol commands understood by this engine.
//
//----------------------------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UciCommand {
    // Begin the UCI handshake. The engine replies with identity lines followed by "uciok".
    Uci,

    // Ask whether the engine is ready to accept more commands. A ready engine replies "readyok".
    IsReady,

    // Tell the engine that the next commands belong to a fresh game.
    UciNewGame,

    // Replace the current board with a starting position or FEN, then apply optional moves.
    Position(PositionCommand),

    // Start choosing a move using any search limits supplied by the GUI.
    Go(SearchLimits),

    // Stop a search in progress. The current template searches synchronously, so this is a no-op.
    Stop,

    // Enable or disable diagnostic output. Diagnostics go to stderr, never stdout.
    Debug(bool),

    // Configure an engine option. The template accepts the command shape but has no options yet.
    SetOption { name: String, value: Option<String> },

    // End the process-level UCI session.
    Quit,

    // Preserve unrecognized text so the driver can ignore it or log it in debug mode.
    Unknown(String),
}

//----------------------------------------------------------------------------------------------------------------------
// Enum: PositionCommand
//
// Description:
//
//   Represents the supported forms of the UCI position command.
//
//----------------------------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PositionCommand {
    // UCI "startpos" means the standard chess starting position. The optional move list then plays
    // forward from that initial board.
    Startpos { moves: Vec<String> },

    // UCI "fen" carries a full FEN string. The optional move list then plays forward from that FEN.
    Fen { fen: String, moves: Vec<String> },
}

//----------------------------------------------------------------------------------------------------------------------
// Implementation: PositionCommand
//
// Description:
//
//   Provides accessors shared by the supported position-command variants.
//
//----------------------------------------------------------------------------------------------------------------------

impl PositionCommand {
    //------------------------------------------------------------------------------------------------------------------
    // Value Accessor: moves
    //
    // Description:
    //
    //   Return the UCI moves attached to the position command.
    //
    //------------------------------------------------------------------------------------------------------------------

    pub fn moves(&self) -> &[String] {
        // Both variants store trailing UCI move text in the same shape, so callers can ask for moves
        // without caring whether the base position came from "startpos" or "fen".

        match self {
            PositionCommand::Startpos { moves } | PositionCommand::Fen { moves, .. } => moves,
        }
    }
}
