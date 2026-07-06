pub mod core;
pub mod git;

use crate::SessionContext;
use crate::views;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "solo", version = "1.0", about = "An interactive CLI tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Init,   // init the whole system
    Status, // Check the status of your solo session
    Do {
        item: String,
    }, // Do an awesome action
    Dashboard,
    Core {
        #[command(subcommand)]
        subcommand: Option<core::CoreCommands>,
    },
    Scaffold {
        /// Optional space-separated list of folders to build (e.g. src tests docs)
        #[arg(num_args = 1..)]
        folders: Vec<String>,
    },
    Exit, // Exit the interactive session
}

/// Where your routing logic lives
pub fn execute_command(cmd: Commands, context: &mut SessionContext) -> bool {
    match cmd {
        Commands::Init => {
            println!("✨ Solo System Initialization");
            if let Err(e) = git::clone_and_run("https://github.com/BerlinUnited/xabsleditor.git", "/home/stella/solo_test") {
                eprintln!("❌ Error during setup: {}", e);
            }
            if let Err(e) = git::clone_and_run("git@scm.cms.hu-berlin.de:berlinunited/naoth-2020.git", "/home/stella/solo_test") {
                eprintln!("❌ Error during setup: {}", e);
            }
            if let Err(e) = git::clone_and_run("git@scm.cms.hu-berlin.de:berlinunited/tools/naoth-deeplearning.git", "/home/stella/solo_test") {
                eprintln!("❌ Error during setup: {}", e);
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
        Commands::Scaffold { folders } => {
            let folders_to_create = if folders.is_empty() {
                // Default structure blueprint if they pass no arguments
                vec!["src".to_string(), "tests".to_string(), "docs".to_string()]
            } else {
                folders
            };

            println!("Launching Interactive Folder Picker...");
            let mut picker = views::folder_picker::FolderPicker::new();

            match picker.run() {
                Ok(Some(chosen_path)) => {
                    println!("📍 Targets will be built inside: {}", chosen_path.display());
                    if let Err(e) =
                        views::folder_picker::create_subfolders(&chosen_path, &folders_to_create)
                    {
                        eprintln!("❌ Failed creating directories: {}", e);
                    } else {
                        println!("✅ Project workspace scaffolded beautifully!");
                    }
                }
                Ok(None) => println!("Scaffold cancelled."),
                Err(e) => eprintln!("System Picker Error: {}", e),
            }
        }
        Commands::Exit => {
            println!("Goodbye!");
            return false;
        }
    }
    true
}
