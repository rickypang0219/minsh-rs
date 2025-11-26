use std::io;

use reedline::{DefaultPrompt, Signal};

pub mod shell;
use shell::{ShellConfig, ShellState};

pub mod editor;
use editor::create_editor;

fn main() -> Result<(), io::Error> {
    let config = ShellConfig::load();
    let mut state = ShellState::new(config.clone());

    let prompt = DefaultPrompt::default();

    let mut line_editor = create_editor();

    loop {
        let sig = line_editor.read_line(&prompt);

        match sig {
            Ok(Signal::Success(buffer)) => {
                state.run_command(&buffer);
            }
            Ok(Signal::CtrlD) | Ok(Signal::CtrlC) => {
                println!("\nGoodbye!");
                break;
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
                break;
            }
        }
    }
    Ok(())
}
