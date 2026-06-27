//----------------------------------------------------------------------------------------------------------------------
// Project: UCI Engine Template Rust
// Version: 1.0.0
// Date:    2026-06-17
// Author:  Rohin Gosling
//
// Description:
//
//   Random legal move selector.
//
//----------------------------------------------------------------------------------------------------------------------

use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

use crate::board::position::Position;
use crate::engine::selector::MoveSelector;
use crate::search::limits::SearchLimits;

//----------------------------------------------------------------------------------------------------------------------
// Struct: RandomMoveSelector
//
// Description:
//
//   Stores the random number generator used by the random legal move selector.
//
//----------------------------------------------------------------------------------------------------------------------

pub struct RandomMoveSelector {
    random_number_generator: ThreadRng,
}

//----------------------------------------------------------------------------------------------------------------------
// Implementation: RandomMoveSelector
//
// Description:
//
//   Provides construction behavior for the random legal move selector.
//
//----------------------------------------------------------------------------------------------------------------------

impl RandomMoveSelector {
    //------------------------------------------------------------------------------------------------------------------
    // Function: new
    //
    // Description:
    //
    //   Create a random move selector with a thread-local random number generator.
    //
    //------------------------------------------------------------------------------------------------------------------

    pub fn new() -> Self {
        Self {
            random_number_generator: rand::thread_rng(),
        }
    }
}

//----------------------------------------------------------------------------------------------------------------------
// Implementation: Default for RandomMoveSelector
//
// Description:
//
//   Uses the normal constructor as the selector default.
//
//----------------------------------------------------------------------------------------------------------------------

impl Default for RandomMoveSelector {
    //------------------------------------------------------------------------------------------------------------------
    // Function: default
    //
    // Description:
    //
    //   Create the default random move selector.
    //
    //------------------------------------------------------------------------------------------------------------------

    fn default() -> Self {
        Self::new()
    }
}

//----------------------------------------------------------------------------------------------------------------------
// Implementation: MoveSelector for RandomMoveSelector
//
// Description:
//
//   Selects one legal move at random from the current position.
//
//----------------------------------------------------------------------------------------------------------------------

impl MoveSelector for RandomMoveSelector {
    //------------------------------------------------------------------------------------------------------------------
    // Method: select_move
    //
    // Description:
    //
    //   Return a randomly selected legal move, or None when no legal moves exist.
    //
    //------------------------------------------------------------------------------------------------------------------

    fn select_move(
        &mut self,
        position: &Position,
        _limits: &SearchLimits,
    ) -> Option<cozy_chess::Move> {
        // SearchLimits is ignored by this toy selector. The parameter remains in the trait so a later
        // engine can obey time, depth, node, or move-filter limits without changing the caller.

        let legal_moves = position.legal_moves();

        // choose returns a reference into the vector; copied turns that borrowed move into an owned
        // Move value that can safely leave this function.

        legal_moves
            .choose(&mut self.random_number_generator)
            .copied()
    }
}
