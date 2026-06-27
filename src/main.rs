//----------------------------------------------------------------------------------------------------------------------
// Project: UCI Engine Template Rust
// Version: 1.0.0
// Date:    2026-06-17
// Author:  Rohin Gosling
//
// Description:
//
//   Command-line entry point for the UCI engine process.
//
//----------------------------------------------------------------------------------------------------------------------

use std::io;
use std::io::BufWriter;
use std::process::ExitCode;

use uci_engine_template::engine::random::RandomMoveSelector;
use uci_engine_template::engine::session::EngineSession;
use uci_engine_template::uci::driver::run_uci_driver;

//----------------------------------------------------------------------------------------------------------------------
// Function: main
//
// Description:
//
//   Initialize the default engine session and run the UCI command driver.
//
//----------------------------------------------------------------------------------------------------------------------

fn main() -> ExitCode {
    // Capture the process streams once at startup. Rust locks them below so the UCI driver can read
    // and write efficiently without repeatedly asking the operating system for the same handles.

    let standard_input = io::stdin();
    let standard_output = io::stdout();

    // UCI engines are normally launched as child processes by a chess GUI. The GUI writes command
    // lines to standard input and reads protocol responses from standard output, so the engine keeps
    // those streams open for the whole session.

    let input_handle = standard_input.lock();
    let output_handle = BufWriter::new(standard_output.lock());

    // EngineSession owns the current board position and delegates move choice to the selector. This
    // template starts with a deliberately simple selector so the UCI plumbing is easy to see first.

    let selector = RandomMoveSelector::new();
    let mut session = EngineSession::new(selector);

    // run_uci_driver owns the command loop. Any I/O error is reported on stderr because stdout is
    // reserved for strict UCI protocol text.

    match run_uci_driver(input_handle, output_handle, &mut session) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("uci driver error: {error}");
            ExitCode::FAILURE
        }
    }
}
