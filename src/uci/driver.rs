//----------------------------------------------------------------------------------------------------------------------
// Project: UCI Engine Template Rust
// Version: 1.0.0
// Date:    2026-06-17
// Author:  Rohin Gosling
//
// Description:
//
//   UCI command loop and command dispatch.
//
//----------------------------------------------------------------------------------------------------------------------

use std::io;
use std::io::{BufRead, Write};

use crate::engine::selector::MoveSelector;
use crate::engine::session::EngineSession;
use crate::uci::command::UciCommand;
use crate::uci::parser::parse_uci_command;
use crate::uci::response;

//----------------------------------------------------------------------------------------------------------------------
// Function: run_uci_driver
//
// Description:
//
//   Read UCI command lines, dispatch each command, and stop when quit is received.
//
//----------------------------------------------------------------------------------------------------------------------

pub fn run_uci_driver<R, W, S>(
    input: R,
    mut output: W,
    session: &mut EngineSession<S>,
) -> io::Result<()>
where
    R: BufRead,
    W: Write,
    S: MoveSelector,
{
    for line_result in input.lines() {
        // BufRead::lines returns Result<String, io::Error>. The question mark keeps the loop small:
        // if reading stdin fails, the error is returned to main immediately.

        let line = line_result?;

        // Parsing converts loose protocol text into an enum so the rest of the engine can use normal
        // Rust pattern matching instead of string comparisons everywhere.

        let command = parse_uci_command(&line);

        // dispatch_uci_command returns false only for "quit"; every other command keeps the session
        // alive so the GUI can continue the conversation.

        if !dispatch_uci_command(command, &mut output, session)? {
            break;
        }
    }

    Ok(())
}

//----------------------------------------------------------------------------------------------------------------------
// Function: dispatch_uci_command
//
// Description:
//
//   Execute one parsed UCI command and return whether the driver should continue.
//
//----------------------------------------------------------------------------------------------------------------------

fn dispatch_uci_command<W, S>(
    command: UciCommand,
    output: &mut W,
    session: &mut EngineSession<S>,
) -> io::Result<bool>
where
    W: Write,
    S: MoveSelector,
{
    match command {
        UciCommand::Uci => {
            // The UCI startup handshake is ordered: identity lines first, then "uciok" to announce
            // that the engine has finished describing itself.

            writeln!(output, "{}", response::engine_name())?;
            writeln!(output, "{}", response::engine_author())?;
            writeln!(output, "{}", response::uciok())?;
            output.flush()?;
        }
        UciCommand::IsReady => {
            // Chess GUIs use "isready" as a synchronization point after setup commands.

            writeln!(output, "{}", response::readyok())?;
            output.flush()?;
        }
        UciCommand::UciNewGame => {
            // Reset only the per-game state. The process and configured selector remain alive.

            session.new_game();
        }
        UciCommand::Position(position_command) => {
            // Bad position text should not crash the engine process. UCI diagnostics belong on stderr
            // because stdout is machine-read protocol output.

            if let Err(error) = session.set_position(position_command) {
                eprintln!("position error: {error}");
            }
        }
        UciCommand::Go(limits) => {
            // This template returns a move immediately. A stronger engine would start a real search,
            // obey the limits, and eventually print the same bestmove response shape.

            let move_text = session.best_move_text(&limits);

            writeln!(output, "{}", response::bestmove(&move_text))?;
            output.flush()?;
        }
        UciCommand::Stop => {}
        UciCommand::Debug(debug_enabled) => {
            session.set_debug_enabled(debug_enabled);
        }
        UciCommand::SetOption { name, value } => {
            // Options are parsed now so adding real configurable settings later only touches the
            // session layer, not the protocol parser.

            if session.is_debug_enabled() {
                eprintln!("debug: ignored option '{name}' with value '{value:?}'");
            }
        }
        UciCommand::Quit => {
            return Ok(false);
        }
        UciCommand::Unknown(text) => {
            // The protocol allows engines to ignore commands they do not understand.

            if session.is_debug_enabled() {
                eprintln!("debug: ignored unknown command '{text}'");
            }
        }
    }

    Ok(true)
}
