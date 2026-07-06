mod commands;
mod views;

use clap::Parser;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

use commands::{Cli, execute_command};

pub enum SessionContext {
    Main,
    Core,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(command) => {
            let mut temp_context = SessionContext::Main;
            execute_command(command, &mut temp_context);
        }
        None => {
            println!("Welcome to the Solo Session! Type 'exit' or press Ctrl+D to leave.");
            if let Err(e) = run_interactive_session() {
                eprintln!("Session error: {}", e);
            }
        }
    }
}

fn run_interactive_session() -> Result<(), ReadlineError> {
    let mut rl = DefaultEditor::new()?;
    let mut context = SessionContext::Main;

    loop {
        // Dynamic prompt update based on state
        let prompt = match context {
            SessionContext::Main => "solo> ",
            SessionContext::Core => "solo (core)> ",
        };

        let readline = rl.readline(prompt);

        match readline {
            Ok(line) => {
                let input = line.trim();
                if input.is_empty() {
                    continue;
                }

                let _ = rl.add_history_entry(input);
                let mut args = vec!["solo"];

                match context {
                    SessionContext::Main => {
                        args.extend(input.split_whitespace());
                    }
                    SessionContext::Core => {
                        args.push("core");
                        args.extend(input.split_whitespace());
                    }
                }

                match Cli::try_parse_from(args) {
                    Ok(parsed_cli) => {
                        if let Some(cmd) = parsed_cli.command {
                            let should_continue = execute_command(cmd, &mut context);
                            if !should_continue {
                                break;
                            }
                        }
                    }
                    Err(e) => e.print().unwrap(),
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                println!("Exiting...");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}
