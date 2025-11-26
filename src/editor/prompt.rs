use reedline::{Prompt, PromptEditMode, PromptHistorySearch};

pub struct CustomPrompt {
    pub text: String,
}

impl Prompt for CustomPrompt {
    fn render_prompt_left(&self) -> std::borrow::Cow<str> {
        std::borrow::Cow::Borrowed(&self.text)
    }

    fn render_prompt_right(&self) -> std::borrow::Cow<str> {
        std::borrow::Cow::Borrowed("")
    }

    fn render_prompt_indicator(&self, _edit_mode: PromptEditMode) -> std::borrow::Cow<str> {
        std::borrow::Cow::Borrowed("> ")
    }

    fn render_prompt_multiline_indicator(&self) -> std::borrow::Cow<str> {
        std::borrow::Cow::Borrowed("‣ ") // or any symbol for multi-line
    }

    fn render_prompt_history_search_indicator(
        &self,
        _search: PromptHistorySearch,
    ) -> std::borrow::Cow<str> {
        std::borrow::Cow::Borrowed("(reverse-i-search) ")
    }
}
