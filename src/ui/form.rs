use crate::config::FieldType;
use ratatui::Frame;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem, Paragraph};
use crate::state::AppState;
use crate::ui::layout::{base_layout, render_tabs, Theme};

pub fn render_select_type(frame: &mut Frame, state: &AppState) {
    let (tabs_area, _content_area, _status_area) = base_layout(frame);
    render_tabs(frame, tabs_area, &state.step);
}

pub fn render_fields(frame: &mut Frame, state: &AppState) {
    let (tabs_area, content_area, status_area) = base_layout(frame);
    render_tabs(frame, tabs_area, &state.step);

    let items: Vec<ListItem> = state.config.fields
        .iter()
        .enumerate()
        .map(|(i, field)| {
            let is_active = i == state.form.selected_field;

            let line = match field.field_type {
                FieldType::Select => {
                    let value = state.form.user_inputs
                        .get(&field.key)
                        .cloned()
                        .unwrap_or_else(|| field.label.clone());
                    Line::from(vec![
                        Span::styled(
                            format!("  {}: ", field.label),
                            Style::default().fg(Theme::TEXT_MUTED),
                        ),
                        Span::styled("← ", Style::default().fg(Theme::ACCENT)),
                        Span::styled(value, Style::default().fg(Theme::GREEN)),
                        Span::styled(" →", Style::default().fg(Theme::ACCENT)),
                    ])
                }
                FieldType::Text | FieldType::Number => {
                    let value = state.form.user_inputs
                        .get(&field.key)
                        .cloned()
                        .unwrap_or_default();
                    Line::from(vec![
                        Span::styled(
                            format!("  {}: ", field.label),
                            Style::default().fg(Theme::TEXT_MUTED),
                        ),
                        Span::styled(value, Style::default().fg(Theme::GREEN)),
                    ])
                }
            };

            let style = if is_active {
                Style::default()
                    .bg(Theme::BG_SURFACE)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            ListItem::new(line).style(style)
        })
        .collect();

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .border_style(Style::default().fg(Theme::BORDER))
        .title(Span::styled(" Fields ", Style::default().fg(Theme::ACCENT)))
        .style(Style::default().bg(Theme::BG));

    let list = List::new(items).block(block);
    frame.render_widget(list, content_area);

    let status = Paragraph::new(Line::from(vec![
        Span::styled(" ↑↓ ", Style::default().fg(Theme::ACCENT)),
        Span::styled("Navigate  ", Style::default().fg(Theme::TEXT_MUTED)),
        Span::styled("←→ ", Style::default().fg(Theme::ACCENT)),
        Span::styled("Select  ", Style::default().fg(Theme::TEXT_MUTED)),
        Span::styled("Tab ", Style::default().fg(Theme::ACCENT)),
        Span::styled("Next  ", Style::default().fg(Theme::TEXT_MUTED)),
        Span::styled("BackTab ", Style::default().fg(Theme::ACCENT)),
        Span::styled("Prev  ", Style::default().fg(Theme::TEXT_MUTED)),
        Span::styled("Enter ", Style::default().fg(Theme::ACCENT)),
        Span::styled("Generate  ", Style::default().fg(Theme::TEXT_MUTED)),
        Span::styled("q ", Style::default().fg(Theme::ACCENT)),
        Span::styled("Quit", Style::default().fg(Theme::TEXT_MUTED)),
    ])).style(Style::default().bg(Theme::BG_SURFACE));

    frame.render_widget(status, status_area);
}