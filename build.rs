use std::{env, process::Command};

fn main() {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    println!("cargo::warning=Current directory: {:?}", current_dir);

    let output = Command::new("doppler")
        .arg("secrets")
        .arg("download")
        .arg("--no-file")
        .output()
        .expect("Failed to execute command");
    println!(
        "cargo::warning=hello from build.rs {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    if output.status.success() {
        // Command executed successfully
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("cargo::warning=Command output");
    } else {
        // Command failed
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("cargo::warning=Command failed: {}", stderr);
    }
}
