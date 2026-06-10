use ratatui::Frame;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState};
use crate::state::AppState;
use crate::storage::load_history;
use crate::ui::layout::{base_layout, render_tabs, Theme};

pub fn render_history(frame: &mut Frame, state: &AppState) {
    let (tabs_area, content_area, status_area) = base_layout(frame);
    render_tabs(frame, tabs_area, &state.step);

    let history = load_history().unwrap_or_default();
    let mut lines: Vec<Line> = vec![];
    let mut line_index = 0usize;

    if history.is_empty() {
        lines.push(Line::from(Span::styled(
            "  No history yet — generate something first.",
            Style::default().fg(Theme::TEXT_MUTED),
        )));
    } else {
        for (i, item) in history.iter().enumerate() {
            if i > 0 {
                lines.push(Line::from(Span::styled(
                    "  ─────────────────────────────────────",
                    Style::default().fg(Theme::BORDER),
                )));
                line_index += 1;
            }

            for (field_index, (label, value, color)) in [
                ("  date     : ", item.date.as_str(),     Theme::TEXT),
                ("  branch   : ", item.branch.as_str(),   Theme::HISTORY_BRANCH),
                ("  commit   : ", item.commit.as_str(),   Theme::HISTORY_COMMIT),
                ("  pr_title : ", item.pr_title.as_str(), Theme::AMBER),
            ].iter().enumerate() {
                let is_selected = line_index == state.history_selected_line;
                lines.push(Line::from(vec![
                    Span::styled(*label, Style::default().fg(Theme::TEXT_MUTED)),
                    Span::styled(*value, if is_selected {
                        Style::default().fg(Theme::BG).bg(*color).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(*color)
                    }),
                ]));
                line_index += 1;
            }
        }
    }

    let visible_height = content_area.height.saturating_sub(2) as usize;
    let scroll_offset = if state.history_selected_line < visible_height {
        0
    } else {
        state.history_selected_line - visible_height + 1
    };

    let mut scrollbar_state = ScrollbarState::new(lines.len())
        .position(scroll_offset);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .border_style(Style::default().fg(Theme::BORDER))
        .title(Span::styled(" History ", Style::default().fg(Theme::ACCENT)))
        .style(Style::default().bg(Theme::BG));

    let paragraph = Paragraph::new(Text::from(lines))
        .block(block)
        .scroll((scroll_offset as u16, 0));

    frame.render_widget(paragraph, content_area);
    frame.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .style(Style::default().fg(Theme::BORDER)),
        content_area,
        &mut scrollbar_state,
    );

    let default_status = Paragraph::new(Line::from(vec![
        Span::styled(" ↑↓ ", Style::default().fg(Theme::ACCENT)),
        Span::styled("Navigate  ", Style::default().fg(Theme::TEXT_MUTED)),
        Span::styled("Tab ", Style::default().fg(Theme::ACCENT)),
        Span::styled("Next  ", Style::default().fg(Theme::TEXT_MUTED)),
        Span::styled("BackTab ", Style::default().fg(Theme::ACCENT)),
        Span::styled("Prev  ", Style::default().fg(Theme::TEXT_MUTED)),
        Span::styled("c ", Style::default().fg(Theme::ACCENT)),
        Span::styled("Copy  ", Style::default().fg(Theme::TEXT_MUTED)),
        Span::styled("Enter ", Style::default().fg(Theme::ACCENT)),
        Span::styled("Checkout  ", Style::default().fg(Theme::TEXT_MUTED)),
        Span::styled("b ", Style::default().fg(Theme::ACCENT)),
        Span::styled("Create  ", Style::default().fg(Theme::TEXT_MUTED)),
        Span::styled("q ", Style::default().fg(Theme::ACCENT)),
        Span::styled("Quit", Style::default().fg(Theme::TEXT_MUTED)),
    ])).style(Style::default().bg(Theme::BG_SURFACE));

    let show_message = state.git_message_time
        .map(|t| t.elapsed().as_secs() < 2)
        .unwrap_or(false);

    let status = if show_message {
        match &state.git_message {
            Some(msg) if msg.starts_with('✓') => Paragraph::new(
                Span::styled(format!(" {}", msg), Style::default().fg(Theme::GREEN))
            ).style(Style::default().bg(Theme::BG_SURFACE)),
            Some(msg) => Paragraph::new(
                Span::styled(format!(" {}", msg), Style::default().fg(Theme::AMBER))
            ).style(Style::default().bg(Theme::BG_SURFACE)),
            None => default_status,
        }
    } else {
        default_status
    };

    frame.render_widget(status, status_area);
}