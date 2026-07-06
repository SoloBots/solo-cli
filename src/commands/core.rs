use clap::Subcommand;
use crate::SessionContext;

#[derive(Subcommand, Debug, Clone)]
pub enum CoreCommands {
    Status,         // prints out information about where the core repo is, the git state etc
    CompileLocal,   // runs the compiler
    CompileRobot,   // cross compiles 
    Back,           // Go back to the main solo menu
}

pub fn handle_core_command(subcommand: Option<CoreCommands>, context: &mut SessionContext) {
    match subcommand {
        None => {
            println!("Entering Core subsystem. Type 'back' to go to main menu.");
            *context = SessionContext::Core;
        }
        Some(CoreCommands::Status) => {
            println!("⚙️ Core initialized successfully.");
            //TODO 
        }
        Some(CoreCommands::CompileLocal) => {
            println!("📊 Core metrics: CPU 2%, Mem 42MB.");
        }
        Some(CoreCommands::CompileRobot) => {
            println!("📊 Core metrics: CPU 2%, Mem 42MB.");
        }
        Some(CoreCommands::Back) => {
            println!("Returning to main menu.");
            *context = SessionContext::Main;
        }
    }
}