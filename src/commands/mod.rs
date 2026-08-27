pub mod core;
pub mod git;

use crate::SessionContext;
use crate::views;
use clap::{ Parser, Subcommand };
use dialoguer::Confirm;

#[derive(Parser)]
#[command(name = "solo", version = "1.0", about = "An interactive CLI tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Init, // init the whole system
    Status, // Check the status of your solo session
    Do {
        item: String,
    }, // Do an awesome action
    Dashboard,
    Core {
        #[command(subcommand)]
        subcommand: Option<core::CoreCommands>,
    },
    Exit, // Exit the interactive session
}

pub fn ask_confirm(prompt: &str, default_value: bool) -> bool {
    Confirm::new().with_prompt(prompt).default(default_value).interact().unwrap_or(false) // Fallback if user cancels (e.g., Ctrl+C)
}

/// Where your routing logic lives
pub fn execute_command(cmd: Commands, context: &mut SessionContext) -> bool {
    match cmd {
        Commands::Init => {
            println!("✨ Solo System Initialization");

            if ask_confirm("Check for system dependencies?", true) {
                println!("🔍 Checking dependencies...");
                if let Some(path) = views::path_functions::check_binary_exists("uv") {
                    println!("✅ 'uv' is available at: {}", path.display());
                } else {
                    println!("❌ 'uv' could not be found.");
                }

                if let Some(path) = views::path_functions::check_binary_exists("git") {
                    println!("✅ 'git' is available at: {}", path.display());
                } else {
                    println!("❌ 'git' could not be found.");
                }
                if let Some(path) = views::path_functions::check_binary_exists("npm") {
                    println!("✅ 'npm' is available at: {}", path.display());
                } else {
                    println!("❌ 'npm' could not be found.");
                }
            } else {
                println!("⏭️ Skipping dependency check.");
            }
            if ask_confirm("Clone Repos?", true) {
                let mut picker = views::folder_picker::FolderPicker::new();

                match picker.run() {
                    Ok(Some(chosen_path)) => {
                        println!("🔄 Repos will be cloned inside: {}", chosen_path.display());
                        if
                            let Err(e) = git::clone_and_run(
                                "git@github.com:SoloBots/solo-cli.git",
                                &chosen_path
                            )
                        {
                            eprintln!("❌ Error during setup: {}", e);
                        }
                        if
                            let Err(e) = git::clone_and_run(
                                "git@github.com:SoloBots/solo-docs.git",
                                &chosen_path
                            )
                        {
                            eprintln!("❌ Error during setup: {}", e);
                        }
                        if
                            let Err(e) = git::clone_and_run(
                                "git@github.com:SoloBots/solo-core-ros.git",
                                &chosen_path
                            )
                        {
                            eprintln!("❌ Error during setup: {}", e);
                        }
                        if
                            let Err(e) = git::clone_and_run(
                                "git@github.com:SoloBots/solo-core.git",
                                &chosen_path
                            )
                        {
                            eprintln!("❌ Error during setup: {}", e);
                        }
                        if
                            let Err(e) = git::clone_and_run(
                                "git@github.com:SoloBots/Network-Manager.git",
                                &chosen_path
                            )
                        {
                            eprintln!("❌ Error during setup: {}", e);
                        }
                        if
                            let Err(e) = git::clone_and_run(
                                "https://github.com/RoboCup-HumanoidSoccerLeague/GameController.git",
                                &chosen_path
                            )
                        {
                            eprintln!("❌ Error during setup: {}", e);
                        }
                    }
                    Ok(None) => println!("Repo cloning cancelled."),
                    Err(e) => eprintln!("System Picker Error: {}", e),
                }
            } else {
                println!("⏭️ Skipping repo cloning");
            }
        }
        Commands::Status => {
            println!("✨ System status: Fully operational.");
        }
        Commands::Do { item } => {
            println!("🚀 Doing action on: {}", item);
        }
        Commands::Dashboard => {
            println!("Opening dashboard...");
            if let Err(e) = views::dashboard::run() {
                eprintln!("TUI Error: {}", e);
            }
        }
        Commands::Core { subcommand } => {
            core::handle_core_command(subcommand, context);
        }
        Commands::Exit => {
            println!("Goodbye!");
            return false;
        }
    }
    true
}
