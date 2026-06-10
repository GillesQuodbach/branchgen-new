use std::io::stdout;
use std::time::Duration;
use crate::config::{AppConfig, FieldType};
use crate::error::AppError;
use crate::state::{AppState, FormState, Step};
use crossterm::{event, execute, terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use indexmap::IndexMap;
use ratatui::{Terminal, backend::CrosstermBackend };

use crate::actions::{update, Action};
use crate::{storage, ui};

pub struct App {
    pub state: AppState,
}

impl App {
    pub fn new(config: AppConfig) -> Self {
        let persistent_data = storage::load_persistent();
        let mut user_inputs = IndexMap::new();

        for field in &config.fields {
            if field.field_type == FieldType::Select {
                if let Some(values) = &field.values{
                    if let Some(first) = values.first(){
                        user_inputs.insert(field.key.clone(), first.clone());
                    }
                }
            }
        }

        for field in &config.fields {
            if field.persistent {
                if let Some(value) = persistent_data.get(&field.key){
                    user_inputs.insert(field.key.clone(), value.clone());
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
                form_error: None,
                result: None,
                config,
                should_quit: false,
                history_scroll: 0,
                history_scroll_limitation: 0,
                git_message: None,
                git_message_time: None,
                result_selected_line: 0,
                history_selected_line: 0,
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
            if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = crossterm::event::read()? {
                // Windows envoie Press + Release, on filtre pour n'avoir que Press
                // sinon bug de navigation
                if key.kind == KeyEventKind::Press {
                let action = handle_key(key, &self.state.step);
                update(&mut self.state, action);
                }
            }}

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
        KeyCode::Char('b') if *step == Step::History => Action::CreateBranchFromHistory,
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::Quit,
        KeyCode::Char('q')      => Action::Quit,
        KeyCode::Char('b') if *step == Step::ShowResults        => Action::CreateBranch,
        KeyCode::Char('c') if *step == Step::ShowResults        => Action::CopyLineFromResults,
        KeyCode::Char('c') if *step == Step::History            => Action::CopyLineFromHistory,
        KeyCode::Char('r') if key.modifiers.contains(KeyModifiers::CONTROL)
        && *step == Step::FillFields
        => Action::ResetForm,
        KeyCode::Up             => Action::MoveUp,
        KeyCode::Down           => Action::MoveDown,
        KeyCode::Left           => Action::MoveLeft,
        KeyCode::Right          => Action::MoveRight,
        KeyCode::Enter          => Action::Enter,
        KeyCode::Enter if *step == Step::History => Action::CheckoutFromHistory,
        KeyCode::Backspace      => Action::Backspace,
        KeyCode::Delete         => Action::Delete,
        KeyCode::Char(c)   => Action::InputCharacter(c),
        KeyCode::Tab            => Action::NextTab,
        KeyCode::BackTab        => Action::PrevTab,
        _                       => Action::None,
    }
}