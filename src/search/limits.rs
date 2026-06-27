//----------------------------------------------------------------------------------------------------------------------
// Project: UCI Engine Template Rust
// Version: 1.0.0
// Date:    2026-06-17
// Author:  Rohin Gosling
//
// Description:
//
//   Representation of UCI go command search limits.
//
//----------------------------------------------------------------------------------------------------------------------

//----------------------------------------------------------------------------------------------------------------------
// Struct: SearchLimits
//
// Description:
//
//   Stores parsed UCI go-command search and time-control limits.
//
//----------------------------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SearchLimits {
    // Restrict search to these UCI move strings when the GUI supplies "searchmoves".
    pub search_moves: Vec<String>,

    // Clock values from the GUI are in milliseconds.
    pub white_time: Option<u64>,
    pub black_time: Option<u64>,
    pub white_increment: Option<u64>,
    pub black_increment: Option<u64>,
    pub moves_to_go: Option<u64>,
    pub depth: Option<u64>,
    pub nodes: Option<u64>,
    pub mate: Option<u64>,
    pub move_time: Option<u64>,

    // "go infinite" means search until a later "stop" command.
    pub infinite: bool,
}

//----------------------------------------------------------------------------------------------------------------------
// Implementation: SearchLimits
//
// Description:
//
//   Provides parsing behavior for UCI go-command limit tokens.
//
//----------------------------------------------------------------------------------------------------------------------

impl SearchLimits {
    //------------------------------------------------------------------------------------------------------------------
    // Function: parse_from_tokens
    //
    // Description:
    //
    //   Parse UCI go-command tokens into a SearchLimits value.
    //
    //------------------------------------------------------------------------------------------------------------------

    pub fn parse_from_tokens(tokens: &[&str]) -> Self {
        let mut limits = Self::default();
        let mut token_index = 0;

        // UCI "go" accepts a sequence of optional keyword/value pairs. This loop walks left to right,
        // consuming one keyword and its value at a time.

        while token_index < tokens.len() {
            match tokens[token_index] {
                // Limit the engine to a GUI-supplied list of candidate moves.
                "searchmoves" => {
                    token_index += 1;

                    // searchmoves is the one variable-length section. It continues until the next
                    // recognized go-limit keyword starts a new section.

                    while token_index < tokens.len() && !is_go_limit_keyword(tokens[token_index]) {
                        limits.search_moves.push(tokens[token_index].to_string());
                        token_index += 1;
                    }
                }

                // Remaining white clock time, in milliseconds.
                "wtime" => {
                    limits.white_time = parse_next_number(tokens, token_index);
                    token_index += 2;
                }

                // Remaining black clock time, in milliseconds.
                "btime" => {
                    limits.black_time = parse_next_number(tokens, token_index);
                    token_index += 2;
                }

                // White increment added after each move, in milliseconds.
                "winc" => {
                    limits.white_increment = parse_next_number(tokens, token_index);
                    token_index += 2;
                }

                // Black increment added after each move, in milliseconds.
                "binc" => {
                    limits.black_increment = parse_next_number(tokens, token_index);
                    token_index += 2;
                }

                // Number of moves expected before the next time-control boundary.
                "movestogo" => {
                    limits.moves_to_go = parse_next_number(tokens, token_index);
                    token_index += 2;
                }

                // Fixed search depth measured in plies, where one ply is one move by one side.
                "depth" => {
                    limits.depth = parse_next_number(tokens, token_index);
                    token_index += 2;
                }

                // Maximum number of search tree nodes the engine should examine.
                "nodes" => {
                    limits.nodes = parse_next_number(tokens, token_index);
                    token_index += 2;
                }

                // Search for a forced mate within the requested number of moves.
                "mate" => {
                    limits.mate = parse_next_number(tokens, token_index);
                    token_index += 2;
                }

                // Search for exactly this many milliseconds and then return a move.
                "movetime" => {
                    limits.move_time = parse_next_number(tokens, token_index);
                    token_index += 2;
                }

                // Keep searching until the GUI sends a later "stop" command.
                "infinite" => {
                    limits.infinite = true;
                    token_index += 1;
                }
                _ => {
                    // Unknown tokens are skipped so newer GUI options do not break this template.

                    token_index += 1;
                }
            }
        }

        limits
    }
}

//----------------------------------------------------------------------------------------------------------------------
// Function: parse_next_number
//
// Description:
//
//   Parse the numeric token immediately after a recognized go-command keyword.
//
//----------------------------------------------------------------------------------------------------------------------

fn parse_next_number(tokens: &[&str], token_index: usize) -> Option<u64> {
    // Bad or missing numbers become None. The caller can still keep any other limits that parsed.

    tokens
        .get(token_index + 1)
        .and_then(|token| token.parse::<u64>().ok())
}

//----------------------------------------------------------------------------------------------------------------------
// Function: is_go_limit_keyword
//
// Description:
//
//   Return whether a token starts a recognized UCI go-command limit.
//
//----------------------------------------------------------------------------------------------------------------------

fn is_go_limit_keyword(token: &str) -> bool {
    // This list marks tokens that end the variable-length "searchmoves" section.

    matches!(
        token,
        "wtime"
            | "btime"
            | "winc"
            | "binc"
            | "movestogo"
            | "depth"
            | "nodes"
            | "mate"
            | "movetime"
            | "infinite"
            | "ponder"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    //------------------------------------------------------------------------------------------------------------------
    // Function: parse_common_go_limits
    //
    // Description:
    //
    //   Verify common UCI go-command limits are parsed into the expected fields.
    //
    //------------------------------------------------------------------------------------------------------------------

    #[test]
    fn parse_common_go_limits() {
        // This compact command mixes clock, fixed-time, fixed-depth, and infinite-search options.

        let limits = SearchLimits::parse_from_tokens(&[
            "wtime", "60000", "btime", "30000", "winc", "1000", "binc", "500", "movetime", "250",
            "depth", "2", "infinite",
        ]);

        // Each recognized keyword should land in the matching optional field.

        assert_eq!(limits.white_time, Some(60000));
        assert_eq!(limits.black_time, Some(30000));
        assert_eq!(limits.white_increment, Some(1000));
        assert_eq!(limits.black_increment, Some(500));
        assert_eq!(limits.move_time, Some(250));
        assert_eq!(limits.depth, Some(2));
        assert!(limits.infinite);
    }

    //------------------------------------------------------------------------------------------------------------------
    // Function: parse_search_moves_until_next_limit
    //
    // Description:
    //
    //   Verify searchmoves parsing stops at the next go-command limit keyword.
    //
    //------------------------------------------------------------------------------------------------------------------

    #[test]
    fn parse_search_moves_until_next_limit() {
        // The "depth" token proves that searchmoves stops at the next recognized go-limit keyword.

        let limits =
            SearchLimits::parse_from_tokens(&["searchmoves", "e2e4", "d2d4", "depth", "1"]);

        // The candidate move list and the following depth limit should both survive parsing.

        assert_eq!(limits.search_moves, vec!["e2e4", "d2d4"]);
        assert_eq!(limits.depth, Some(1));
    }
}
