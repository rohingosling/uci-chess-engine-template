//----------------------------------------------------------------------------------------------------------------------
// Project: UCI Engine Template Rust
// Version: 1.0.0
// Date:    2026-06-17
// Author:  Rohin Gosling
//
// Description:
//
//   Library entry point for the UCI engine template.
//
//----------------------------------------------------------------------------------------------------------------------

pub mod board;
pub mod engine;
pub mod search;
pub mod uci;

// These constants are used in the UCI "id" response during the startup handshake.

pub const ENGINE_NAME: &str = "UCI Engine Template Rust";
pub const ENGINE_AUTHOR: &str = "Rohin Gosling";
