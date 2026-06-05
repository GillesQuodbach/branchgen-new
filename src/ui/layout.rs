
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Line, Stylize};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Tabs};
use crate::state::{AppState, Step};

pub fn base_layout(frame: &mut Frame) -> (Rect, Rect, Rect) {
    let chunks = Layout::default().direction(Direction::Vertical).constraints([
        Constraint::Length(3), // tab
        Constraint::Min(0), // contenu
        Constraint::Length(3), //statusbar
    ])
        .split(frame.area());
    (chunks[0], chunks[1], chunks[2])
}

pub fn render_tabs(frame: &mut Frame, area: Rect, step: &Step) {
    let titles = vec![
        Line::from("Form"),
        Line::from("Results"),
        Line::from("History"),
    ];

    let selected = match step {
        Step::FillFields                    => 0,
        Step::ShowResults                   => 1,
        Step::History                       => 2,
    };

    let tabs = Tabs::new(titles)
        .select(selected)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(Style::default().fg(Color::White))
        .style(Style::default().fg(Color::DarkGray));

    frame.render_widget(tabs, area);
}
pub fn render_layout(frame: &mut Frame, state: Rect) {
    // TODO
}