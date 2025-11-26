use reedline::{Completer, Span, Suggestion};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub struct MinshCompleter {
    builtins: Vec<String>,
    path_commands: Vec<String>,
}

impl MinshCompleter {
    pub fn new() -> Self {
        let builtins = vec!["cd", "exit", "vim", "cargo", "ls", "mkdir", "rm", "git"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();

        let path_commands = Self::load_path_commands();

        Self {
            builtins,
            path_commands,
        }
    }

    /// Load all executable commands inside PATH
    fn load_path_commands() -> Vec<String> {
        let mut cmds = vec![];

        if let Ok(path_var) = env::var("PATH") {
            for dir in path_var.split(':') {
                let p = Path::new(dir);
                if let Ok(entries) = fs::read_dir(p) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.is_file() {
                            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                                cmds.push(name.to_string());
                            }
                        }
                    }
                }
            }
        }

        cmds.sort();
        cmds.dedup();
        cmds
    }

    /// Helper: list only directories for `cd`
    fn directory_suggestions(prefix: &str, span: Span) -> Vec<Suggestion> {
        let path = PathBuf::from(if prefix.is_empty() { "." } else { prefix });

        let parent = if path.is_dir() {
            path.clone()
        } else {
            path.parent().unwrap_or(Path::new(".")).to_path_buf()
        };

        let prefix_str = path.file_name().and_then(|s| s.to_str()).unwrap_or("");

        let mut suggestions = vec![];

        if let Ok(entries) = fs::read_dir(parent) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_dir() {
                    if let Some(dir_name) = p.file_name().and_then(|s| s.to_str()) {
                        if dir_name.starts_with(prefix_str) {
                            suggestions.push(Suggestion {
                                value: dir_name.to_string(),
                                description: Some("Directory".into()),
                                span,
                                append_whitespace: true,
                                style: None,
                                extra: None,
                            });
                        }
                    }
                }
            }
        }

        suggestions
    }

    fn command_suggestions(&self, prefix: &str, span: Span) -> Vec<Suggestion> {
        let mut all_cmds = vec![];
        all_cmds.extend(self.builtins.clone());
        all_cmds.extend(self.path_commands.clone());

        all_cmds
            .into_iter()
            .filter(|cmd| cmd.starts_with(prefix))
            .map(|cmd| Suggestion {
                value: cmd,
                description: Some("Command".into()),
                span,
                append_whitespace: true,
                style: None,
                extra: None,
            })
            .collect()
    }
}

impl Completer for MinshCompleter {
    fn complete(&mut self, line: &str, pos: usize) -> Vec<Suggestion> {
        // -------- Parse current word + its Span ------
        let mut start = pos;
        let bytes = line.as_bytes();
        while start > 0 && !bytes[start - 1].is_ascii_whitespace() {
            start -= 1;
        }
        let end = pos;

        let current_word = &line[start..end];
        let span = Span::new(start, end);

        // -------- Split full input into tokens -------
        let tokens: Vec<&str> = line.split_whitespace().collect();

        // Case 1: No tokens yet — suggest commands
        if tokens.is_empty() {
            return self.command_suggestions(current_word, span);
        }

        // Case 2: First token is "cd" => directory completion
        if tokens[0] == "cd" {
            return Self::directory_suggestions(current_word, span);
        }

        // Case 3: First token is something else => command completion
        if tokens.len() == 1 {
            return self.command_suggestions(current_word, span);
        }

        // Case 4: For now, fallback to empty (you can add argument completers later)
        vec![]
    }
}
