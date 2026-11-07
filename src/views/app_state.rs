use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AppState {
    pub folder_paths: Vec<String>,
}

// Provide sensible defaults if no config file exists yet
impl Default for AppState {
    fn default() -> Self {
        Self {
            folder_paths: Vec::new(),
        }
    }
}