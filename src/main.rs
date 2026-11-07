mod commands;
mod views;

use clap::Parser;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

use commands::{Cli, execute_command};
use views::app_state::AppState;

pub enum SessionContext {
    Main,
    Core,
    Robot,
    ML,
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    // 1. Load existing state (or create default if it doesn't exist)
    let mut state: AppState = confy::load(views::constants::APP_NAME, "config")?;
    
    println!("Current saved folders: {:?}", state.folder_paths);

    // 2. Modify the state (e.g., adding a new path during this run)
    let new_path = String::from("/path/to/some/folder");
    if !state.folder_paths.contains(&new_path) {
        state.folder_paths.push(new_path);
    }

    // 3. Save the updated state back to disk
    confy::store(views::constants::APP_NAME, "config", &state)?;
    println!("State successfully saved!");

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
    Ok(())
}

fn run_interactive_session() -> Result<(), ReadlineError> {
    let mut rl = DefaultEditor::new()?;
    let mut context = SessionContext::Main;

    loop {
        // Dynamic prompt update based on state
        let prompt = match context {
            SessionContext::Main => "solo> ",
            SessionContext::Core => "solo (core)> ",
            SessionContext::Robot => "solo (robot)> ",
            SessionContext::ML => "solo (ml)> ",
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
                    SessionContext::Robot => {
                        args.push("robot");
                        args.extend(input.split_whitespace());
                    }
                    SessionContext::ML => {
                        args.push("ml");
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
