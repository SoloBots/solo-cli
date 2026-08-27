use std::io;
use std::path::Path;
use std::process::Command;

/// Clones a repository and runs a list of setup commands inside its directory.
pub fn clone_and_run(repo_url: &str, folder_name: &Path) -> io::Result<()> {
    // 1. Extract the repo name from the URL (e.g., "https://github.com/user/repo.git" -> "repo")
    let repo_name = repo_url
        .split('/')
        .last()
        .unwrap_or("")
        .trim_end_matches(".git");

    if repo_name.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid repository URL",
        ));
    }

    // 2. Construct the expected path: parent_folder/repo_name
    let target_path = folder_name.join(repo_name);

    // 3. Check if that specific folder already exists
    if target_path.exists() && target_path.is_dir() {
        println!("❗ Repository '{}' is already cloned. Skipping.", repo_name);
        return Ok(());
    }

    println!("📥 Cloning repository: {}...", repo_url);

    // 1. Run the `git clone` command
    let clone_status = Command::new("git")
        .args(["clone", repo_url])
        .current_dir(Path::new(folder_name))
        .status()?; // .status() runs the command and waits for it to finish

    if !clone_status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to clone the repository.",
        ));
    }

    /*
    // 2. Define the path to the newly cloned directory
    let repo_path = Path::new(folder_name);

    println!("📂 Entering directory: {} and running setup...", folder_name);

    // 3. Run commands *inside* that directory using `.current_dir()`
    // Example: Running `cargo build`
    let build_status = Command::new("cargo")
        .arg("build")
        .current_dir(repo_path) // <-- This is your "cd folder_name"
        .status()?;

    if !build_status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to run cargo build inside the repository.",
        ));
    }
    */
    println!("✅ Setup completed successfully!");
    Ok(())
}
