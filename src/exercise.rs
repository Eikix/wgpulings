use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Clone)]
pub struct Exercise {
    pub name: String,
    pub path: String,
    pub mode: Mode,
    pub hint: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Compile,
    Run,
}

#[derive(Deserialize)]
struct Info {
    exercises: Vec<Exercise>,
}

impl Exercise {
    pub fn load_all() -> Result<Vec<Exercise>, Box<dyn std::error::Error>> {
        let info_toml = fs::read_to_string("info.toml")?;
        let info: Info = toml::from_str(&info_toml)?;
        Ok(info.exercises)
    }

    pub fn is_done(&self) -> bool {
        let content = match fs::read_to_string(&self.path) {
            Ok(c) => c,
            Err(_) => return false,
        };

        // Check if file doesn't contain "I AM NOT DONE"
        !content.contains("I AM NOT DONE")
    }

    pub fn compile(&self) -> Result<(), String> {
        let path = PathBuf::from(&self.path);
        let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");

        match extension {
            "rs" => self.compile_rust(),
            "wgsl" => self.compile_wgsl(),
            _ => Err(format!("Unknown file extension: {}", extension)),
        }
    }

    fn compile_rust(&self) -> Result<(), String> {
        // Create a temporary project to compile the exercise
        let temp_dir = std::env::temp_dir().join(format!("wgpulings_{}", self.name));
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).map_err(|e| e.to_string())?;

        // Copy the exercise file
        let src_dir = temp_dir.join("src");
        fs::create_dir_all(&src_dir).map_err(|e| e.to_string())?;
        fs::copy(&self.path, src_dir.join("main.rs")).map_err(|e| e.to_string())?;

        // Copy any associated .wgsl file (same name, different extension)
        let wgsl_path = PathBuf::from(&self.path).with_extension("wgsl");
        if wgsl_path.exists() {
            let wgsl_name = wgsl_path.file_name().unwrap();
            fs::copy(&wgsl_path, src_dir.join(wgsl_name)).map_err(|e| e.to_string())?;
        }

        // Create a minimal Cargo.toml
        let cargo_toml = format!(
            r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
wgpu = "0.19"
pollster = "0.3"
winit = "0.29"
bytemuck = {{ version = "1.14", features = ["derive"] }}
image = "0.24"
cgmath = "0.18"
"#,
            self.name
        );
        fs::write(temp_dir.join("Cargo.toml"), cargo_toml).map_err(|e| e.to_string())?;

        // Run cargo check
        let output = std::process::Command::new("cargo")
            .arg("check")
            .arg("--quiet")
            .current_dir(&temp_dir)
            .output()
            .map_err(|e| e.to_string())?;

        if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(stderr.to_string())
        }
    }

    fn compile_wgsl(&self) -> Result<(), String> {
        // For WGSL files, we just check basic syntax
        let content = fs::read_to_string(&self.path).map_err(|e| e.to_string())?;

        // Basic validation: check for common syntax errors
        if content.contains("fn ") && content.contains("{") && content.contains("}") {
            Ok(())
        } else {
            Err("Invalid WGSL syntax".to_string())
        }
    }

    pub fn run(&self) -> Result<String, String> {
        if self.mode == Mode::Compile {
            return Err("This exercise is compile-only. Use verify or watch mode.".to_string());
        }

        // Create a temporary project to run the exercise
        let temp_dir = std::env::temp_dir().join(format!("wgpulings_{}", self.name));
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).map_err(|e| e.to_string())?;

        // Copy the exercise file
        let src_dir = temp_dir.join("src");
        fs::create_dir_all(&src_dir).map_err(|e| e.to_string())?;
        fs::copy(&self.path, src_dir.join("main.rs")).map_err(|e| e.to_string())?;

        // Copy any associated .wgsl file (same name, different extension)
        let wgsl_path = PathBuf::from(&self.path).with_extension("wgsl");
        if wgsl_path.exists() {
            let wgsl_name = wgsl_path.file_name().unwrap();
            fs::copy(&wgsl_path, src_dir.join(wgsl_name)).map_err(|e| e.to_string())?;
        }

        // Create a minimal Cargo.toml
        let cargo_toml = format!(
            r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
wgpu = "0.19"
pollster = "0.3"
winit = "0.29"
bytemuck = {{ version = "1.14", features = ["derive"] }}
image = "0.24"
cgmath = "0.18"
"#,
            self.name
        );
        fs::write(temp_dir.join("Cargo.toml"), cargo_toml).map_err(|e| e.to_string())?;

        // Run cargo run
        let output = std::process::Command::new("cargo")
            .arg("run")
            .arg("--quiet")
            .current_dir(&temp_dir)
            .output()
            .map_err(|e| e.to_string())?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            Ok(stdout.to_string())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(stderr.to_string())
        }
    }
}
