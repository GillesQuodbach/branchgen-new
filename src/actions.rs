use crate::generator::generate_result;
use crate::git::create_branch;
use crate::state::{AppState, Step};

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
}



pub fn update(state: &mut AppState, action:Action) {
    match action {
        Action::Quit => {
            state.should_quit = true;
        }
        Action::MoveUp => {
            if state.form.selected_field > 0{
                state.form.selected_field -= 1;
                state.form.cursor_position = 0;
            }
        }
        Action::MoveDown => {
            state.form.selected_field += 1;
            state.form.cursor_position = 0;
        }
        Action::MoveLeft => {
            if state.form.cursor_position > 0 {
                state.form.cursor_position -= 1;
            }
        }
        Action::MoveRight => {
            // on recupère le champ actif depuis la config
            let key = &state.config.fields[state.form.selected_field].key;
            // on calcul la longueur du mot
            let len = state.form.user_inputs.get(key).map(|s| s.len()).unwrap_or(0);
            if state.form.cursor_position < len {
                state.form.cursor_position += 1;
            }
        }
        Action::InputCharacter(character) => {
            // on recupère le champ actif depuis la config
            let key = &state.config.fields[state.form.selected_field].key;
            // on recupere la valeur actuelle et on ajoute un caractere
            let value = state.form.user_inputs.entry(key.clone()).or_insert(String::new());
            value.push(character);
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
                Step::SelectType => {
                    state.step = Step::FillFields;
                }
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
                _ => {}
            }
        }
    }
}