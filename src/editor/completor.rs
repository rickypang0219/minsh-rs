use reedline::{Completer, DefaultCompleter, Span, Suggestion};

pub struct MinshCompleter {
    file_completer: DefaultCompleter,
    command_registry: Vec<String>,
}

impl MinshCompleter {
    pub fn new() -> Self {
        let commands = vec!["cd", "exit", "vim", "cargo", "ls", "mkdir", "rm", "git"];
        let commands_string: Vec<String> = commands.iter().map(|cmd| cmd.to_string()).collect();
        Self {
            file_completer: DefaultCompleter::new_with_wordlen(commands_string.clone(), 2),
            command_registry: commands_string,
        }
    }

    pub fn get_command_suggestions(
        &self,
        partial: &str,
        start_pos: usize,
        end_pos: usize,
    ) -> Vec<Suggestion> {
        self.command_registry
            .iter()
            .filter(|cmd| cmd.starts_with(partial))
            .map(|cmd| Suggestion {
                value: cmd.to_string(),
                description: Some("Command".to_string()),
                style: None,
                span: Span::new(start_pos, end_pos),
                append_whitespace: true,
                extra: None,
            })
            .collect()
    }
}

impl Completer for MinshCompleter {
    fn complete(&mut self, line: &str, pos: usize) -> Vec<Suggestion> {
        let start = line[..pos]
            .rfind(|c: char| c.is_whitespace())
            .map(|i| i + 1)
            .unwrap_or(0);

        let partial = &line[start..pos];
        let mut suggestions = Vec::new();

        if start == 0 {
            suggestions.extend(self.get_command_suggestions(partial, start, pos));
        }

        let mut file_suggestions = self.file_completer.complete(line, pos);
        suggestions.append(&mut file_suggestions);

        suggestions
    }
}
