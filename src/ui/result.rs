use ratatui::Frame;

use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use ratatui::text::{Text, Line};

use crate::state::AppState;
use crate::ui::layout::{base_layout, render_tabs};

pub fn render_results(frame: &mut Frame, state: &AppState) {
    let (tabs_area, content_area, status_area) = base_layout(frame);
    render_tabs(frame, tabs_area, &state.step);

   let result = &state.result;
    if let Some(result) = result {
        let text = Text::from(vec![
            Line::from(format!("Branch   : {}", result.branch)),
            Line::from(format!("Commit   : {}", result.branch)),
            Line::from(format!("PR title : {}", result.branch)),
        ]);

        let block = Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Results"));
        frame.render_widget(block, content_area);
    }

    let status = Paragraph::new("↑↓ navigate | Enter next | q quitter");
    frame.render_widget(status, status_area);
}