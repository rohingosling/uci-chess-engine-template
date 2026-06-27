//----------------------------------------------------------------------------------------------------------------------
// Project: UCI Engine Template Rust
// Version: 1.0.0
// Date:    2026-06-17
// Author:  Rohin Gosling
//
// Description:
//
//   Mutable state for a single UCI engine process.
//
//----------------------------------------------------------------------------------------------------------------------

use std::error::Error;
use std::fmt;

use cozy_chess::Move;

use crate::board::position::{Position, PositionError};
use crate::engine::selector::MoveSelector;
use crate::search::limits::SearchLimits;
use crate::uci::command::PositionCommand;

//----------------------------------------------------------------------------------------------------------------------
// Struct: EngineSession
//
// Description:
//
//   Stores engine process state, including the current position, selector, and debug flag.
//
//----------------------------------------------------------------------------------------------------------------------

pub struct EngineSession<S>
where
    S: MoveSelector,
{
    position: Position,
    selector: S,
    debug_enabled: bool,
}

//----------------------------------------------------------------------------------------------------------------------
// Enum: EngineError
//
// Description:
//
//   Represents errors surfaced by the engine session layer.
//
//----------------------------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub enum EngineError {
    Position(PositionError),
}

//----------------------------------------------------------------------------------------------------------------------
// Implementation: EngineSession
//
// Description:
//
//   Provides UCI session state management and move-selection entry points.
//
//----------------------------------------------------------------------------------------------------------------------

impl<S> EngineSession<S>
where
    S: MoveSelector,
{
    //------------------------------------------------------------------------------------------------------------------
    // Function: new
    //
    // Description:
    //
    //   Create an engine session from a move selector.
    //
    //------------------------------------------------------------------------------------------------------------------

    pub fn new(selector: S) -> Self {
        // The session starts from the normal chess opening position. UCI clients can replace it with
        // a later "position" command before asking for a move.

        Self {
            position: Position::startpos(),
            selector,
            debug_enabled: false,
        }
    }

    //------------------------------------------------------------------------------------------------------------------
    // Method: new_game
    //
    // Description:
    //
    //   Reset the current position for a new UCI game.
    //
    //------------------------------------------------------------------------------------------------------------------

    pub fn new_game(&mut self) {
        // UCI "ucinewgame" is a hint that previous game history should not influence the next game.

        self.position = Position::startpos();
    }

    //------------------------------------------------------------------------------------------------------------------
    // Mutator: set_debug_enabled
    //
    // Description:
    //
    //   Set whether the session emits diagnostic debug output.
    //
    //------------------------------------------------------------------------------------------------------------------

    pub fn set_debug_enabled(&mut self, debug_enabled: bool) {
        self.debug_enabled = debug_enabled;
    }

    //------------------------------------------------------------------------------------------------------------------
    // Predicate Accessor: is_debug_enabled
    //
    // Description:
    //
    //   Return whether diagnostic debug output is enabled.
    //
    //------------------------------------------------------------------------------------------------------------------

    pub fn is_debug_enabled(&self) -> bool {
        self.debug_enabled
    }

    //------------------------------------------------------------------------------------------------------------------
    // Method: set_position
    //
    // Description:
    //
    //   Apply a parsed UCI position command atomically to the session.
    //
    //------------------------------------------------------------------------------------------------------------------

    pub fn set_position(&mut self, command: PositionCommand) -> Result<(), EngineError> {
        // Build the requested position in a temporary value first. If the FEN or a move is bad, the
        // function returns an error and self.position is left unchanged.

        let mut next_position = match command {
            PositionCommand::Startpos { .. } => Position::startpos(),
            PositionCommand::Fen { ref fen, .. } => Position::from_fen(fen)?,
        };

        // UCI represents game history as a base position plus a list of moves. Replaying the moves is
        // what makes castling rights, en-passant state, and side-to-move line up with the GUI.

        for move_text in command.moves() {
            next_position.apply_uci_move(move_text)?;
        }

        // Commit the fully validated position atomically.

        self.position = next_position;

        Ok(())
    }

    //------------------------------------------------------------------------------------------------------------------
    // Method: best_move
    //
    // Description:
    //
    //   Ask the configured selector for the best move in the current position.
    //
    //------------------------------------------------------------------------------------------------------------------

    pub fn best_move(&mut self, limits: &SearchLimits) -> Option<Move> {
        // The selector is a trait so the engine can swap RandomMoveSelector for a real searcher later
        // without changing the UCI driver.

        self.selector.select_move(&self.position, limits)
    }

    //------------------------------------------------------------------------------------------------------------------
    // Method: best_move_text
    //
    // Description:
    //
    //   Return the selected move as UCI text, or the UCI null move when no move exists.
    //
    //------------------------------------------------------------------------------------------------------------------

    pub fn best_move_text(&mut self, limits: &SearchLimits) -> String {
        // UCI requires "bestmove 0000" when the side to move has no legal move, such as checkmate or
        // stalemate. Otherwise the move is formatted in UCI coordinate notation.

        match self.best_move(limits) {
            Some(chess_move) => self.position.display_uci_move(chess_move),
            None => "0000".to_string(),
        }
    }
}

//----------------------------------------------------------------------------------------------------------------------
// Implementation: From PositionError for EngineError
//
// Description:
//
//   Converts position-layer errors into engine-session errors.
//
//----------------------------------------------------------------------------------------------------------------------

impl From<PositionError> for EngineError {
    //------------------------------------------------------------------------------------------------------------------
    // Function: from
    //
    // Description:
    //
    //   Wrap a PositionError in an EngineError.
    //
    //------------------------------------------------------------------------------------------------------------------

    fn from(error: PositionError) -> Self {
        Self::Position(error)
    }
}

//----------------------------------------------------------------------------------------------------------------------
// Implementation: Display for EngineError
//
// Description:
//
//   Formats engine errors as human-readable diagnostic strings.
//
//----------------------------------------------------------------------------------------------------------------------

impl fmt::Display for EngineError {
    //------------------------------------------------------------------------------------------------------------------
    // Method: fmt
    //
    // Description:
    //
    //   Write a readable description of an engine error.
    //
    //------------------------------------------------------------------------------------------------------------------

    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EngineError::Position(error) => write!(formatter, "{error}"),
        }
    }
}

//----------------------------------------------------------------------------------------------------------------------
// Implementation: Error for EngineError
//
// Description:
//
//   Marks EngineError as a standard Rust error type.
//
//----------------------------------------------------------------------------------------------------------------------

impl Error for EngineError {}
