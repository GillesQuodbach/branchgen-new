use std::collections::HashMap;
use crate::config::AppConfig;

pub struct AppState {
    pub step: Step,
    pub form: FormState,
    pub result: Option<GeneratedResult>,
    pub config: AppConfig,
    pub should_quit: bool,
    pub history_scroll: usize,
    pub history_scroll_limitation: usize,
    pub git_message: Option<String>,
    pub result_selected_line: usize,
}

#[derive(PartialEq)]
pub enum Step {
    FillFields,
    ShowResults,
    History
}

pub struct FormState {
    pub user_inputs: HashMap<String, String>,
    pub selected_field: usize,
    pub select_input_position: usize,
    pub cursor_position: usize,
}

pub struct GeneratedResult {
    pub branch: String,
    pub commit: String,
    pub pr_title: String,
}