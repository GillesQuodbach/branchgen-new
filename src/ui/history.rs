use ratatui::Frame;
use ratatui::prelude::{Line, Text};
use ratatui::widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState};
use crate::state::AppState;
use crate::storage::load_history;
use crate::ui::layout::{base_layout, render_tabs};

pub fn render_history(frame: &mut Frame, state: &AppState) {
    let (tabs_area, content_area, status_area) = base_layout(frame);
    render_tabs(frame, tabs_area, &state.step);

    let history = load_history().unwrap_or_default();
    let mut lines: Vec<Line> = vec![];

    if history.is_empty() {
        lines.push(Line::from("No history"))
    } else {
        for (i, item) in history.iter().enumerate() {
                if i > 0 {

                lines.push(Line::from(("─────────────────────────────")));
                }
                lines.push(Line::from(format!("date    :{}", item.date)));
                lines.push(Line::from(format!("branch  :{}", item.branch)));
                lines.push(Line::from(format!("commit  :{}", item.commit)));
                lines.push(Line::from(format!("pr_title:{}", item.pr_title)));
        }
    }

    let total_lines = lines.len() as u16;
    let visible_height = content_area.height;

    let mut scrollbar_state = ScrollbarState::new(total_lines as usize)
        .position(state.history_scroll);

    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);

    let block = Paragraph::new(Text::from(lines))
        .block(Block::default().borders(Borders::ALL).title("History"))
        .scroll((state.history_scroll as u16, 0));
    frame.render_widget(block, content_area);

    frame.render_stateful_widget(scrollbar,content_area, &mut scrollbar_state);

    let status = Paragraph::new("↑↓ navigate | Enter next | q quitter");
    frame.render_widget(status, status_area);
}