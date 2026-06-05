use ratatui::Frame;
use crate::state::{AppState, Step};

pub mod form;
pub mod input;
pub mod layout;
pub mod result;
pub mod history;

pub fn render(frame: &mut Frame, state: &AppState) {
    match state.step {
        Step::FillFields => form::render_fields(frame, state),
        Step::ShowResults => result::render_results(frame, state),
        Step::History => history::render_history(frame, state),
    }
}