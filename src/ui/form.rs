use crate::config::FieldType;
use ratatui::Frame;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use crate::state::AppState;
use crate::ui::layout::{base_layout, render_layout, render_tabs};

pub fn render_select_type(frame: &mut Frame, state: &AppState) {
    let (tabs_area, content_area, status_area) = base_layout(frame);

    // tabs
    render_layout(frame, tabs_area);
}

pub fn render_fields(frame: &mut Frame, state: &AppState) {
    let (tabs_area, content_area, status_area) = base_layout(frame);
    render_tabs(frame, tabs_area, &state.step);

    let items: Vec<ListItem> = state.config.fields
        .iter()
        .enumerate()
        .map(|(i, field)| {
            let is_active = i == state.form.selected_field;
            let style = if is_active {
                Style::default().fg(Color::White)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            let label = match field.field_type {
                FieldType::Select => {
                    // on affiche la valeur selectionnée ou le label
                    let value = state.form.user_inputs
                        .get(&field.key)
                        .cloned()
                        .unwrap_or_else(|| field.label.clone());
                    format!("{}: ← {} →", field.label, value)
                },
                FieldType::Text | FieldType::Number => {
                    let value = state.form.user_inputs
                        .get(&field.key)
                        .cloned()
                        .unwrap_or_default();
                    format!("{}: {}", field.label, value)
                },
            };
            ListItem::new(label).style(style)
        }).collect();

    let list = List::new(items).block(Block::default().borders(Borders::ALL).title("Fields"));

    frame.render_widget(list, content_area);

    let status = Paragraph::new("↑↓ navigate | Enter next | q quitter");
    frame.render_widget(status, status_area);
}