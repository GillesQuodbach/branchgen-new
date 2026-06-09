use chrono::Local;
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
    NextTab,
    PrevTab,
    HistoryLoaded(usize),
    CopyLineFromResults,
    CopyLineFromHistory,
}

fn compute_history_total(len: usize) -> usize {
    if len == 0 { 0 } else { len * 4 + (len - 1) }
}

pub fn update(state: &mut AppState, action: Action) {
    match action {
        Action::Quit => {
            state.should_quit = true;
        }

        Action::MoveUp => {
            match state.step {
                Step::History => {
                    if state.history_selected_line > 0 {
                        state.history_selected_line -= 1;
                    }
                }
                Step::ShowResults => {
                    if state.result_selected_line > 0 {
                        state.result_selected_line -= 1;
                    }
                }
                _ => {
                    if state.form.selected_field > 0 {
                        state.form.selected_field -= 1;
                        state.form.cursor_position = 0;
                    }
                }
            }
        }

        Action::MoveDown => {
            match state.step {
                Step::History => {
                    if state.history_selected_line < state.history_scroll_limitation {
                        state.history_selected_line += 1;
                    }
                }
                Step::ShowResults => {
                    if state.result_selected_line < 2 {
                        state.result_selected_line += 1;
                    }
                }
                _ => {
                    let nb_fields = state.config.fields.len();
                    if state.form.selected_field < nb_fields {
                        state.form.selected_field += 1;
                        state.form.cursor_position = 0;
                        state.form.select_input_position = 0;
                    }
                }
            }
        }

        Action::MoveLeft => {
            if state.form.selected_field < state.config.fields.len() {
            let field = &state.config.fields[state.form.selected_field];
            match field.field_type {
                FieldType::Select => {
                    let len = field.values.as_ref().map(|v| v.len()).unwrap_or(0);
                    state.form.select_input_position = (state.form.select_input_position + 1) % len;
                    let value = field.values.as_ref().unwrap()[state.form.select_input_position].clone();
                    state.form.user_inputs.insert(field.key.clone(), value);
                }
                _ => {}
            }
        }
    }

        Action::MoveRight => {
            if state.form.selected_field < state.config.fields.len() {
                let field = &state.config.fields[state.form.selected_field];
                match field.field_type {
                    FieldType::Select => {
                        let len = field.values.as_ref().map(|v| v.len()).unwrap_or(0);
                        state.form.select_input_position = (state.form.select_input_position + len - 1) % len;
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
        }

        Action::InputCharacter(character) => {
            if state.form.selected_field < state.config.fields.len() {
                let field = &state.config.fields[state.form.selected_field];
                match field.field_type {
                    FieldType::Select => {}
                    FieldType::Number => {
                        if character.is_ascii_digit() {
                            let key = field.key.clone();
                            let value = state.form.user_inputs.entry(key).or_insert(String::new());
                            value.push(character);
                        }
                    }
                    FieldType::Text => {
                        let key = field.key.clone();
                        let value = state.form.user_inputs.entry(key).or_insert(String::new());
                        value.push(character);
                    }
                }
            }
        }

        Action::Backspace => {
            if state.form.selected_field < state.config.fields.len() {
                let key = &state.config.fields[state.form.selected_field].key;
                if let Some(value) = state.form.user_inputs.get_mut(key) {
                    value.pop();
                }
            }
        }

        Action::Delete => {
            if state.form.selected_field < state.config.fields.len() {
                let key = &state.config.fields[state.form.selected_field].key;
                if let Some(value) = state.form.user_inputs.get_mut(key) {
                    if state.form.cursor_position < value.len() {
                        value.remove(state.form.cursor_position);
                    }
                }
            }
        }

        Action::Generate => {
            let result = generate_result(&state.form, &state.config.formats);
            state.result = Some(result);
        }

        Action::CreateBranch => {
            if let Some(result) = &state.result {
                match create_branch(&result.branch) {
                    Ok(_)  => state.git_message = Some(format!("✓ Branch '{}' created", result.branch)),
                    Err(e) => state.git_message = Some(format!("✗ Error: {}", e)),
                }
            }
        }

        Action::ChangeStep(step) => {
            state.step = step;
        }

        Action::Enter => {
            match state.step {
                Step::FillFields => {
                    let last_field = state.config.fields.len();
                    if state.form.selected_field == last_field {
                        let result = generate_result(&state.form, &state.config.formats);
                        let date = Local::now().format("%d-%m-%Y").to_string();
                        let entry = History {
                            date,
                            branch: result.branch.clone(),
                            commit: result.commit.clone(),
                            pr_title: result.pr_title.clone(),
                        };
                        let _ = save_history(&entry);
                        let history = load_history().unwrap_or_default();
                        state.history_scroll_limitation = compute_history_total(history.len());
                        state.history_scroll = 0;
                        state.result = Some(result);
                        state.step = Step::ShowResults;
                    } else {
                        state.form.selected_field += 1;
                        state.form.cursor_position = 0;
                    }
                }
                _ => {}
            }
        }

        Action::NextTab => {
            state.step = match state.step {
                Step::FillFields  => Step::ShowResults,
                Step::ShowResults => Step::History,
                Step::History     => Step::FillFields,
            };
            if state.step == Step::History {
                let history = load_history().unwrap_or_default();
                state.history_scroll_limitation = compute_history_total(history.len());
                state.history_scroll = 0;
            }
        }

        Action::PrevTab => {
            state.step = match state.step {
                Step::FillFields  => Step::History,
                Step::History     => Step::ShowResults,
                Step::ShowResults => Step::FillFields,
            };
            if state.step == Step::History {
                let history = load_history().unwrap_or_default();
                state.history_scroll_limitation = compute_history_total(history.len());
                state.history_scroll = 0;
            }
        }

        Action::HistoryLoaded(total) => {
            state.history_scroll_limitation = total;
            state.history_scroll = 0;
        }

        Action::CopyLineFromResults => {
            if let Some(result) = &state.result {
                let text = match state.result_selected_line {
                    0 => &result.branch,
                    1 => &result.commit,
                    _ => &result.pr_title,
                };
                let _ = cli_clipboard::set_contents(text.clone());
                state.git_message = Some(format!("✓ Copied: {}", text));
            }
        }
        Action::CopyLineFromHistory => {
            let history = load_history().unwrap_or_default();
            if history.is_empty(){return}
            let line_per_entry = 5;
            let entry_index = state.history_selected_line / line_per_entry;
            let line_in_entry = state.history_selected_line % line_per_entry;
            if let Some(entry) = &history.get(entry_index) {
                let text = match line_in_entry {
                    0 => entry.date.clone(),
                    1 => entry.branch.clone(),
                    2 => entry.commit.clone(),
                    3 => entry.pr_title.clone(),
                    _ => return,
                };
                let _ = cli_clipboard::set_contents(text.clone());
                state.git_message = Some(format!("✓ Copied: {}", text));
            }
        }

        Action::None => {}
    }
}