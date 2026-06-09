use std::collections::HashMap;
use std::io::stdout;
use crate::config::{AppConfig, FieldType};
use crate::error::AppError;
use crate::state::{AppState, FormState, Step};
use crossterm::{execute, terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::{Terminal, backend::CrosstermBackend };
use crate::actions::{update, Action};
use crate::ui;

pub struct App {
    pub state: AppState,
}

impl App {
    pub fn new(config: AppConfig) -> Self {
        let mut user_inputs = HashMap::new();
        for field in &config.fields {
            if field.field_type == FieldType::Select {
                if let Some(values) = &field.values{
                    if let Some(first) = values.first(){
                        user_inputs.insert(field.key.clone(), first.clone());
                    }
                }
            }
        }
        App {
            state: AppState {
                step: Step::FillFields,
                form: FormState {
                    user_inputs,
                    selected_field: 0,
                    select_input_position: 0,
                    cursor_position: 0,
                },
                result: None,
                config,
                should_quit: false,
                history_scroll: 0,
                history_scroll_limitation: 0,
                git_message: None,
                result_selected_line: 0,
            }
        }
    }



    pub fn run(&mut self) -> Result<(), AppError> {
        //init
        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        //boucle principale
        loop {
            terminal.draw(|f| { ui::render(f, &self.state)
            })?;

            if let Event::Key(key) = crossterm::event::read()? {
                let action = handle_key(key, &self.state.step);
                update(&mut self.state, action);
            }

            if self.state.should_quit {
                break;
            }
        }

        // nettoyage du terminal
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        Ok(())
    }


}

pub fn handle_key(key: KeyEvent, step: &Step) -> Action {
    match key.code {
        KeyCode::Char('q')      => Action::Quit,
        KeyCode::Char('b') if *step == Step::ShowResults        => Action::CreateBranch,
        KeyCode::Char('c') if *step == Step::ShowResults        => Action::CopyLineToClipboard,
        KeyCode::Up             => Action::MoveUp,
        KeyCode::Down           => Action::MoveDown,
        KeyCode::Left           => Action::MoveLeft,
        KeyCode::Right          => Action::MoveRight,
        KeyCode::Enter          => Action::Enter,
        KeyCode::Backspace      => Action::Backspace,
        KeyCode::Delete         => Action::Delete,
        KeyCode::Char(c)   => Action::InputCharacter(c),
        KeyCode::Tab            => tab_action(),
        KeyCode::BackTab        => backtab_action(),
        _                       => Action::None,
    }
}

fn tab_action() -> Action {
    #[cfg(target_os = "windows")]
    return Action::PrevTab;
    #[cfg(not(target_os = "windows"))]
    return Action::NextTab;
}

fn backtab_action() -> Action {
    #[cfg(target_os = "windows")]
    return Action::NextTab;
    #[cfg(not(target_os = "windows"))]
    return Action::PrevTab;
}