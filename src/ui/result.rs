use ratatui::Frame;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use crate::state::AppState;
use crate::ui::layout::{base_layout, render_tabs, Theme};

pub fn render_results(frame: &mut Frame, state: &AppState) {
    let (tabs_area, content_area, status_area) = base_layout(frame);
    render_tabs(frame, tabs_area, &state.step);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .border_style(Style::default().fg(Theme::BORDER))
        .title(Span::styled(" Results ", Style::default().fg(Theme::ACCENT)))
        .style(Style::default().bg(Theme::BG));

    if let Some(result) = &state.result {
        let line_style = |index: usize, color| {
            if state.result_selected_line == index {
                Style::default().fg(Theme::BG).bg(color).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(color)
            }
        };

        let text = Text::from(vec![
            Line::from(format!("  Branch   : {}", result.branch))
                .style(line_style(0, Theme::GREEN)),
            Line::from(format!("  Commit   : {}", result.commit))
                .style(line_style(1, Theme::ACCENT)),
            Line::from(format!("  PR title : {}", result.pr_title))
                .style(line_style(2, Theme::AMBER)),
        ]);

        frame.render_widget(Paragraph::new(text).block(block), content_area);
    } else {
        let text = Paragraph::new(
            Span::styled(
                "  No results yet — fill the form and press Enter.",
                Style::default().fg(Theme::TEXT_MUTED),
            )
        ).block(block);
        frame.render_widget(text, content_area);
    }

    let default_status = Paragraph::new(Line::from(vec![
        Span::styled(" ↑↓ ", Style::default().fg(Theme::ACCENT)),
        Span::styled("Navigate  ", Style::default().fg(Theme::TEXT_MUTED)),
        Span::styled("Tab ", Style::default().fg(Theme::ACCENT)),
        Span::styled("Next  ", Style::default().fg(Theme::TEXT_MUTED)),
        Span::styled("BackTab ", Style::default().fg(Theme::ACCENT)),
        Span::styled("Prev  ", Style::default().fg(Theme::TEXT_MUTED)),
        Span::styled("c ", Style::default().fg(Theme::ACCENT)),
        Span::styled("Copy  ", Style::default().fg(Theme::TEXT_MUTED)),
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