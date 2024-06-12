use std::{env, process::Command};

fn main() {
    let project_dir = env::var("PROJECT_DIR").expect("PROJECT_DIR not set");
    println!("cargo::warning=Current directory: {:?}", project_dir);

    // Change the current working directory to project_dir
    env::set_current_dir(&project_dir).expect("Failed to change directory");

    let output = Command::new("doppler")
        .arg("secrets")
        .arg("download")
        .arg("--no-file")
        .output()
        .expect("Failed to execute command");
    println!("cargo::warning=hello from build.rs",);

    if output.status.success() {
        // Command executed successfully
        let stdout = String::from_utf8_lossy(&output.stdout);
        let secrets =
            serde_json::from_str::<serde_json::Value>(&stdout).expect("Failed to parse JSON");
        // println!("cargo::warning=Command output: {:?}", secrets);

        // Assuming `secrets` is a serde_json::Value containing a JSON object
        if let serde_json::Value::Object(secrets_map) = secrets {
            for (key, value) in secrets_map {
                if let serde_json::Value::String(val_str) = value {
                    println!("cargo:rustc-env={}={}", key, val_str);
                    println!(
                        "cargo::warning=Setting environment variable: {}={}",
                        key, val_str
                    )
                } else {
                    // Handle the case where the value is not a string
                    eprintln!("cargo::warning=Value for key '{}' is not a string.", key);
                }
            }
        } else {
            // Handle the case where `secrets` is not a JSON object
            eprintln!("cargo::warning=Expected a JSON object for secrets.");
        }
    } else {
        // Command failed
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("cargo::warning=Command failed: {:?}", stderr);
    }
}
