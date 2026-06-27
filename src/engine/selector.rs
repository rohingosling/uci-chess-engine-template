//----------------------------------------------------------------------------------------------------------------------
// Project: UCI Engine Template Rust
// Version: 1.0.0
// Date:    2026-06-17
// Author:  Rohin Gosling
//
// Description:
//
//   Move-selection trait used by engine sessions.
//
//----------------------------------------------------------------------------------------------------------------------

use cozy_chess::Move;

use crate::board::position::Position;
use crate::search::limits::SearchLimits;

//----------------------------------------------------------------------------------------------------------------------
// Trait: MoveSelector
//
// Description:
//
//   Defines the move-selection interface used by the UCI engine session.
//
//----------------------------------------------------------------------------------------------------------------------

pub trait MoveSelector {
    //------------------------------------------------------------------------------------------------------------------
    // Method: select_move
    //
    // Description:
    //
    //   Choose a move for the current position under the requested search limits.
    //
    //------------------------------------------------------------------------------------------------------------------

    fn select_move(&mut self, position: &Position, limits: &SearchLimits) -> Option<Move>;
}
