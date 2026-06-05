use std::cmp::PartialEq;
use crate::config::FieldType;
use crate::generator::generate_result;
use crate::git::create_branch;
use crate::state::{AppState, Step};
use crate::storage::{load_history, save_history, History};

pub enum Action {
    Quit,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Enter,
    Backspace,
    Delete,
    ChangeStep(Step),
    Generate,
    CreateBranch,
    InputCharacter(char),
    None,
    TabPress,
    HistoryLoaded(usize),
}

pub fn update(state: &mut AppState, action:Action) {
    match action {
        Action::Quit => {
            state.should_quit = true;
        }
        Action::MoveUp => {
            match state.step {
                Step::History => {
                    if state.history_scroll > 0 {
                        state.history_scroll -= 1;
                    }
                }
                _ => {
                    if state.form.selected_field > 0{
                        state.form.selected_field -= 1;
                        state.form.cursor_position = 0;
                    }
                }
            }
        }
        Action::MoveDown => {
            match state.step {
                Step::History => {
                    if (state.history_scroll as usize) < state.history_scroll_limitation{
                        state.history_scroll += 1;
                    }
                }
                _ => {
                    let nb_fields = state.config.fields.len();
                    if state.form.selected_field < nb_fields - 1 {
                        state.form.selected_field += 1;
                        state.form.cursor_position = 0;
                        state.form.select_input_position = 0;
                    }
                }
            }
        }
        Action::MoveLeft => {
            let field = &state.config.fields[state.form.selected_field];
            match field.field_type {
                FieldType::Select => {
                    // on navigue dans les valeurs
                    let len = field.values.as_ref().map(|v| v.len()).unwrap_or(0);

                        state.form.select_input_position = (state.form.select_input_position + 1) % len;
                        // on stock la valeur sélectionnée dans user_inputs
                        let value = field.values.as_ref().unwrap()[state.form.select_input_position].clone();
                        state.form.user_inputs.insert(field.key.clone(), value);

                }
                _ => {
                    state.form.selected_field += 1;
                    state.form.cursor_position = 0;

                }
            }
        }
        Action::MoveRight => {
            let field = &state.config.fields[state.form.selected_field];
            match field.field_type {
                FieldType::Select => {
                    let len = field.values.as_ref().map(|v| v.len()).unwrap_or(0);

                        state.form.select_input_position = (state.form.select_input_position + len -1) % len;

                    let value = field.values.as_ref().unwrap()[state.form.select_input_position].clone();
                    state.form.user_inputs.insert(field.key.clone(), value);
                }
                _ => {
                    if state.form.cursor_position > 0 {
                        state.form.cursor_position -= 1;
                    }
                }
            }
        }
        Action::InputCharacter(character) => {
            // on recupère le champ actif depuis la config
            let field = &state.config.fields[state.form.selected_field];
            match field.field_type {
                FieldType::Select => {},
                FieldType::Number => {
                    if character.is_ascii_digit() {
                        let key = field.key.clone();
                        let value = state.form.user_inputs.entry(key).or_insert(String::new());
                        value.push(character);
                    }
                },
                FieldType::Text => {
                    let key = field.key.clone();
                    let value = state.form.user_inputs.entry(key).or_insert(String::new());
                    value.push(character);
                }
            }
        }
        Action::Backspace => {
            // on recupere le champ actif depuis la config
            let key = &state.config.fields[state.form.selected_field].key;
            if let Some(value) = state.form.user_inputs.get_mut(key){
                value.pop();
            }
        }
        Action::Generate => {
            let result = generate_result(&state.form, &state.config.formats);
            state.result = Some(result);
        }
        Action::CreateBranch => {
            if let Some(result) = &state.result {
                let _ = create_branch(&result.branch);
            }
        }
        Action::Delete => {
            // on recupere le champ actif depuis la config
            let key = &state.config.fields[state.form.selected_field].key;
            if let Some(value) = state.form.user_inputs.get_mut(key){
                if state.form.cursor_position < value.len() {
                    value.remove(state.form.cursor_position);
                }
            }
        }
        Action::ChangeStep(step) => {
            state.step = step;
        }
        Action::Enter => {
            match state.step {
                Step::FillFields => {
                    // c'est le dernier champ ?
                    let last_field = state.config.fields.len()-1;
                    if state.form.selected_field == last_field {
                        let result = generate_result(&state.form, &state.config.formats);
                        state.result = Some(result);
                        state.step = Step::ShowResults;
                    } else {
                        state.form.selected_field += 1;
                        state.form.cursor_position = 0;
                    }
                }
                Step::ShowResults => {
                    if let Some(result) = &state.result {
                        let entry = History {
                            id:0,
                            date: "30-06-2026".to_string(),
                            branch: result.branch.clone(),
                            commit: result.commit.clone(),
                            pr_title: result.pr_title.clone(),
                        };
                        let _ = save_history(&entry);
                        let history = load_history().unwrap_or_default();
                        state.history_scroll_limitation = history.len() * 5;
                        state.history_scroll = 0;
                        state.step = Step::History;
                    }
                }
                _ => {}
            }
        }
        Action::None => {}
        Action::TabPress => {
            state.step = match state.step {
                Step::History => Step::ShowResults,
                Step::ShowResults => Step::FillFields,
                Step::FillFields => Step::FillFields,
            }
        }
        Action::HistoryLoaded(total) => {
            state.history_scroll_limitation = total;
            state.history_scroll = 0;
        }
    }
}