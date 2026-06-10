
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Line};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::widgets::{Block, BorderType, Borders, Tabs};
use crate::state::{Step};
pub struct Theme;

impl Theme {
    pub const BG:         Color = Color::Rgb(13, 17, 23);
    pub const BG_SURFACE: Color = Color::Rgb(22, 27, 34);
    pub const BORDER:     Color = Color::Rgb(33, 38, 45);
    pub const ACCENT:     Color = Color::Rgb(127, 119, 221);
    pub const TEXT:       Color = Color::Rgb(201, 209, 217);
    pub const TEXT_MUTED: Color = Color::Rgb(110, 118, 129);
    pub const GREEN:      Color = Color::Rgb(93, 202, 165);
    pub const AMBER:      Color = Color::Rgb(239, 159, 39);
    pub const HISTORY_BRANCH: Color = Color::Rgb(56, 189, 248);
    pub const HISTORY_COMMIT: Color = Color::Rgb(232, 121, 249);
    pub const ERROR: Color = Color::Rgb(224, 75, 74);
}
pub fn base_layout(frame: &mut Frame) -> (Rect, Rect, Rect) {
    // fond global
    frame.render_widget(
        Block::default().style(Style::default().bg(Theme::BG)),
        frame.area()
    );
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
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .border_style(Style::default().fg(Theme::BORDER))
                .style(Style::default().bg(Theme::BG_SURFACE))
        )
        .highlight_style(Style::default().fg(Theme::ACCENT))
        .style(Style::default().fg(Theme::TEXT_MUTED)).bg(Theme::BG_SURFACE)
        .divider("")
        .padding(" ", " ");

    frame.render_widget(tabs, area);
}
pub fn render_layout(frame: &mut Frame, state: Rect) {
    // TODO
}

