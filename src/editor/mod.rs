use std::path::PathBuf;

use nu_ansi_term::{Color, Style};
use reedline::{
    default_emacs_keybindings, ColumnarMenu, DefaultHinter, DefaultValidator, Emacs,
    FileBackedHistory, KeyCode, KeyModifiers, MenuBuilder, Reedline, ReedlineEvent, ReedlineMenu,
};

pub mod completor;
pub mod completor_test;
pub mod highlighter;
pub mod prompt;

// use completor::MinshCompleter;
use completor_test::MinshCompleter;
use highlighter::DynamicHighlighter;

pub fn create_editor() -> Reedline {
    let history_path = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".minshrc_history");

    let completer = Box::new(MinshCompleter::new());

    let highlighter = Box::new(DynamicHighlighter::new());

    let completion_menu = Box::new(
        ColumnarMenu::default()
            .with_columns(8)
            .with_traversal_direction(reedline::TraversalDirection::Vertical)
            .with_name("completion_menu"),
    );

    let history = Box::new(
        FileBackedHistory::with_file(1000, history_path).expect("Error configuring history"),
    );

    let hint_style = Style::new().fg(Color::DarkGray);

    let mut keybindings = default_emacs_keybindings();
    keybindings.add_binding(
        KeyModifiers::NONE,
        KeyCode::Tab,
        ReedlineEvent::UntilFound(vec![
            ReedlineEvent::Menu("completion_menu".to_string()),
            ReedlineEvent::MenuNext,
        ]),
    );

    let edit_mode = Box::new(Emacs::new(keybindings));

    Reedline::create()
        .with_history(history)
        .with_menu(ReedlineMenu::EngineCompleter(completion_menu))
        .with_completer(completer)
        .with_highlighter(highlighter)
        .with_hinter(Box::new(DefaultHinter::default().with_style(hint_style)))
        .with_validator(Box::new(DefaultValidator))
        .with_edit_mode(edit_mode)
}
