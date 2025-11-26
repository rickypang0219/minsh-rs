use std::env;
use std::path::Path;

use nu_ansi_term::{Color, Style};
use reedline::{Highlighter, StyledText};

pub struct DynamicHighlighter {
    builtins: Vec<String>,
}

impl DynamicHighlighter {
    pub fn new() -> Self {
        Self {
            builtins: vec!["cd".to_string(), "exit".to_string(), "history".to_string()],
        }
    }

    pub fn command_exists(&self, cmd: &str) -> bool {
        if self.builtins.contains(&cmd.to_string()) {
            return true;
        }
        if cmd.contains('/') || cmd.contains('\\') {
            return Path::new(cmd).exists();
        }
        if let Ok(paths) = env::var("PATH") {
            for dir in env::split_paths(&paths) {
                let full_path = dir.join(cmd);
                // On Unix, we strictly check is_file.
                // In a robust shell, we'd also check executable permissions (chmod +x)
                if full_path.is_file() {
                    return true;
                }

                // Optional: Handle Windows extensions (cmd.exe, cmd.bat)
                #[cfg(windows)]
                {
                    if dir.join(format!("{}.exe", cmd)).exists() {
                        return true;
                    }
                    if dir.join(format!("{}.cmd", cmd)).exists() {
                        return true;
                    }
                    if dir.join(format!("{}.bat", cmd)).exists() {
                        return true;
                    }
                }
            }
        }
        false
    }
}

impl Highlighter for DynamicHighlighter {
    fn highlight(&self, line: &str, _cursor: usize) -> StyledText {
        let mut styled = StyledText::new();
        let non_command_style = Style::new().fg(Color::Rgb(138, 148, 159));

        let mut parts = line.split_whitespace();
        let command_part = parts.next();

        if let Some(cmd) = command_part {
            let style = if self.command_exists(cmd) {
                Style::new().fg(Color::Green)
            } else {
                Style::new().fg(Color::Red) // Error color for unknown commands
            };

            let cmd_len = cmd.len();

            let start_index = line.find(cmd).unwrap_or(0);
            let end_index = start_index + cmd_len;

            if start_index > 0 {
                styled.push((non_command_style, line[..start_index].to_string()));
            }

            styled.push((style, line[start_index..end_index].to_string()));

            if end_index < line.len() {
                styled.push((non_command_style, line[end_index..].to_string()));
            }
        } else {
            styled.push((non_command_style, line.to_string()));
        }

        styled
    }
}
