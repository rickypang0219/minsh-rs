use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use dirs;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug, Default)]
pub struct ShellConfig {
    pub prompt: Option<String>,
    history_max_entries: Option<u64>,
}

pub struct ShellState {
    last_exit_code: i32,
    config: ShellConfig,
}

impl ShellConfig {
    pub fn load() -> Self {
        let config_path = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".config")
            .join(".minshrc.toml");

        if config_path.exists() {
            let content = fs::read_to_string(config_path).unwrap_or_default();
            match toml::from_str::<ShellConfig>(&content) {
                Ok(config) => {
                    println!("{:?}", &config);
                    config
                }
                Err(e) => {
                    eprintln!("ERROR {}", e);
                    println!("Use default config as fallback");
                    ShellConfig::default()
                }
            }
        } else {
            ShellConfig::default()
        }
    }
}

impl ShellState {
    pub fn new(config: ShellConfig) -> Self {
        ShellState {
            last_exit_code: 0,
            config,
        }
    }

    pub fn run_command(&mut self, input: &str) {
        let mut parts = input.trim().split_whitespace();
        let command = match parts.next() {
            Some(cmd) => cmd,
            None => return,
        };

        let args: Vec<&str> = parts.collect();

        match command {
            "exit" => std::process::exit(0),
            cmd => {
                let spawn_result = Command::new(cmd)
                    .args(args)
                    .stdin(Stdio::inherit())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn();
                match spawn_result {
                    Ok(mut child) => {
                        let _ = child
                            .wait()
                            .map(|status| self.last_exit_code = status.code().unwrap_or(1));
                    }
                    Err(e) => {
                        eprintln!("Command not found {}", cmd);
                        eprintln!("{} \n", e)
                    }
                }
            }
        }
    }
}
