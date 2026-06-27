//----------------------------------------------------------------------------------------------------------------------
// Project: UCI Engine Template Rust
// Version: 1.0.0
// Date:    2026-06-17
// Author:  Rohin Gosling
//
// Description:
//
//   Project-specific position adapter around cozy-chess.
//
//----------------------------------------------------------------------------------------------------------------------

use std::error::Error;
use std::fmt;

use cozy_chess::util::{display_uci_move as display_cozy_uci_move, parse_uci_move};
use cozy_chess::{Board, Move};

//----------------------------------------------------------------------------------------------------------------------
// Struct: Position
//
// Description:
//
//   Wraps the cozy-chess board so the rest of the project depends on a local position type.
//
//----------------------------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Position {
    // cozy-chess owns the rules of chess: legal move generation, FEN parsing, side-to-move tracking,
    // castling rights, and en-passant state.
    board: Board,
}

//----------------------------------------------------------------------------------------------------------------------
// Enum: PositionError
//
// Description:
//
//   Represents position setup and move-application failures in project-specific terms.
//
//----------------------------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PositionError {
    // FEN describes a full chess position in one text string.
    InvalidFen { text: String, message: String },

    // Move text did not have valid UCI coordinate notation for the current board.
    InvalidMoveText { text: String, message: String },

    // Move text was syntactically valid but not legal in the current chess position.
    IllegalMove { text: String, message: String },
}

//----------------------------------------------------------------------------------------------------------------------
// Implementation: Position
//
// Description:
//
//   Provides constructors, move application, move listing, and display helpers for a chess position.
//
//----------------------------------------------------------------------------------------------------------------------

impl Position {
    //------------------------------------------------------------------------------------------------------------------
    // Function: startpos
    //
    // Description:
    //
    //   Create a position initialized to the standard chess starting position.
    //
    //------------------------------------------------------------------------------------------------------------------

    pub fn startpos() -> Self {
        Self {
            board: Board::startpos(),
        }
    }

    //------------------------------------------------------------------------------------------------------------------
    // Function: from_fen
    //
    // Description:
    //
    //   Parse a FEN string into a position and translate parse failures to PositionError.
    //
    //------------------------------------------------------------------------------------------------------------------

    pub fn from_fen(fen: &str) -> Result<Self, PositionError> {
        // Rust's parse method delegates to cozy-chess's FromStr implementation for Board.

        let board = fen
            .parse::<Board>()
            .map_err(|error| PositionError::InvalidFen {
                text: fen.to_string(),
                message: error.to_string(),
            })?;

        Ok(Self { board })
    }

    //------------------------------------------------------------------------------------------------------------------
    // Method: apply_uci_move
    //
    // Description:
    //
    //   Parse and apply one UCI move to the current position.
    //
    //------------------------------------------------------------------------------------------------------------------

    pub fn apply_uci_move(&mut self, text: &str) -> Result<(), PositionError> {
        // UCI moves are written as source square plus destination square, with an optional promotion
        // piece: "e2e4", "g1f3", or "a7a8q".

        let chess_move =
            parse_uci_move(&self.board, text).map_err(|error| PositionError::InvalidMoveText {
                text: text.to_string(),
                message: error.to_string(),
            })?;

        // try_play checks legality before mutating the board, so illegal moves become errors instead
        // of corrupting the position.

        self.board
            .try_play(chess_move)
            .map_err(|error| PositionError::IllegalMove {
                text: text.to_string(),
                message: error.to_string(),
            })
    }

    //------------------------------------------------------------------------------------------------------------------
    // Method: legal_moves
    //
    // Description:
    //
    //   Collect all legal moves available in the current position.
    //
    //------------------------------------------------------------------------------------------------------------------

    pub fn legal_moves(&self) -> Vec<Move> {
        let mut legal_moves = Vec::new();

        // cozy-chess groups generated moves by piece. The callback appends each group into one flat
        // vector because this simple engine only needs a list it can choose from.

        self.board.generate_moves(|piece_moves| {
            legal_moves.extend(piece_moves);

            // Returning false tells cozy-chess to keep generating moves. A true return value would be
            // useful for early-exit searches that only need to know whether at least one move exists.

            false
        });

        legal_moves
    }

    //------------------------------------------------------------------------------------------------------------------
    // Method: display_uci_move
    //
    // Description:
    //
    //   Format a cozy-chess move as UCI text in the context of the current board.
    //
    //------------------------------------------------------------------------------------------------------------------

    pub fn display_uci_move(&self, chess_move: Move) -> String {
        format!("{}", display_cozy_uci_move(&self.board, chess_move))
    }

    //------------------------------------------------------------------------------------------------------------------
    // Value Accessor: board
    //
    // Description:
    //
    //   Return the underlying cozy-chess board.
    //
    //------------------------------------------------------------------------------------------------------------------

    pub fn board(&self) -> &Board {
        &self.board
    }
}

//----------------------------------------------------------------------------------------------------------------------
// Implementation: Default for Position
//
// Description:
//
//   Provides the standard chess starting position as the default position value.
//
//----------------------------------------------------------------------------------------------------------------------

impl Default for Position {
    //------------------------------------------------------------------------------------------------------------------
    // Function: default
    //
    // Description:
    //
    //   Return a position initialized to the standard chess starting position.
    //
    //------------------------------------------------------------------------------------------------------------------

    fn default() -> Self {
        Self::startpos()
    }
}

//----------------------------------------------------------------------------------------------------------------------
// Implementation: Display for PositionError
//
// Description:
//
//   Formats position errors as human-readable diagnostic strings.
//
//----------------------------------------------------------------------------------------------------------------------

impl fmt::Display for PositionError {
    //------------------------------------------------------------------------------------------------------------------
    // Method: fmt
    //
    // Description:
    //
    //   Write a readable description of a position error.
    //
    //------------------------------------------------------------------------------------------------------------------

    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Include both the failed FEN and cozy-chess's explanation so bad setup commands are easy
            // to diagnose from stderr.
            PositionError::InvalidFen { text, message } => {
                write!(formatter, "invalid FEN '{text}': {message}")
            }

            // InvalidMoveText means the move string could not be parsed in this board context.
            PositionError::InvalidMoveText { text, message } => {
                write!(formatter, "invalid UCI move '{text}': {message}")
            }

            // IllegalMove means the notation parsed, but chess rules rejected the move.
            PositionError::IllegalMove { text, message } => {
                write!(formatter, "illegal UCI move '{text}': {message}")
            }
        }
    }
}

//----------------------------------------------------------------------------------------------------------------------
// Implementation: Error for PositionError
//
// Description:
//
//   Marks PositionError as a standard Rust error type.
//
//----------------------------------------------------------------------------------------------------------------------

impl Error for PositionError {}

#[cfg(test)]
mod tests {
    use super::*;

    //------------------------------------------------------------------------------------------------------------------
    // Function: startpos_has_legal_moves
    //
    // Description:
    //
    //   Verify the standard chess starting position exposes twenty legal moves.
    //
    //------------------------------------------------------------------------------------------------------------------

    #[test]
    fn startpos_has_legal_moves() {
        let position = Position::startpos();

        assert_eq!(position.legal_moves().len(), 20);
    }

    //------------------------------------------------------------------------------------------------------------------
    // Function: legal_uci_move_can_be_applied
    //
    // Description:
    //
    //   Verify a legal UCI move mutates the position and changes the side to move.
    //
    //------------------------------------------------------------------------------------------------------------------

    #[test]
    fn legal_uci_move_can_be_applied() {
        let mut position = Position::startpos();

        position.apply_uci_move("e2e4").unwrap();

        assert_eq!(position.board().side_to_move(), cozy_chess::Color::Black);
    }

    //------------------------------------------------------------------------------------------------------------------
    // Function: illegal_uci_move_returns_error
    //
    // Description:
    //
    //   Verify an illegal UCI move is rejected with an error.
    //
    //------------------------------------------------------------------------------------------------------------------

    #[test]
    fn illegal_uci_move_returns_error() {
        let mut position = Position::startpos();

        assert!(position.apply_uci_move("e1e8").is_err());
    }
}
