use std::collections::HashMap;
use crate::config::FormatConfig;
use crate::state::{FormState, GeneratedResult};

// Le fichier de config
// formats: &FormatConfig {
    // branch:   "{type}/{ticket}-{description}",
    // commit:   "{type}: {description}",
    // pr_title: "[{ticket}] {description}",
    // }

// Ce que le user a entré dans la TUI
// form: &FormState {
    // user_inputs: {
    // "type":        "feat",
    // "ticket":      "42",
    // "description": "add-login-page",
    // }
    // }
pub fn generate_result(form: &FormState, formats: &FormatConfig) -> GeneratedResult{
    let branch = resolve_template(&formats.branch, &form.user_inputs);
    let commit = resolve_template(&formats.commit, &form.user_inputs);
    let pr_title = resolve_template(&formats.pr_title, &form.user_inputs);

    let generated_result = GeneratedResult {
        branch,
        commit,
        pr_title,
    };
    generated_result
}

pub fn resolve_template(template: &str, values: &HashMap<String, String>) -> String {
    let mut result = template.to_string();
    for (key, value) in values {
            let placeholder = format!("{{{}}}", key);
            result = result.replace(&placeholder, &value);
    }
    result
}