use std::collections::HashMap;
use std::io::stdout;
use crate::config::AppConfig;
use crate::error::AppError;
use crate::state::{AppState, FormState, Step};
use crossterm::{execute, terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::{Terminal, backend::CrosstermBackend };
use crate::actions::{update, Action};

pub struct App {
    pub state: AppState,
}

impl App {
    pub fn new(config: AppConfig) -> Self {
        App {
            state: AppState {
                step: Step::SelectType,
                form: FormState {
                    user_inputs: HashMap::new(),
                    selected_field: 0,
                    select_input_position: 0,
                    cursor_position: 0,
                },
                result: None,
                config,
                should_quit: false,

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
            terminal.draw(|f| { //TODO: ui::render(f, &self.state)
            });

            if let Event::Key(key) = crossterm::event::read()? {
                let action = handle_key(key);
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

pub fn handle_key(key: KeyEvent) -> Action {
    match key.code {
        KeyCode::Char('q')      => Action::Quit,
        KeyCode::Up             => Action::MoveUp,
        KeyCode::Down           => Action::MoveDown,
        KeyCode::Left           => Action::MoveLeft,
        KeyCode::Right          => Action::MoveRight,
        KeyCode::Enter          => Action::Enter,
        KeyCode::Backspace      => Action::Backspace,
        KeyCode::Delete         => Action::Delete,
        KeyCode::Char(c)   => Action::InputCharacter(c),
        _                       => Action::Quit, // temporaire
    }
}