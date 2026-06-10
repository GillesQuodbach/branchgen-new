use indexmap::IndexMap;
use crate::config::{FieldConfig, FormatConfig, Normalize};
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
pub fn generate_result(form: &FormState, formats: &FormatConfig, fields: &[FieldConfig]) -> GeneratedResult{

    let mut normalized_inputs = form.user_inputs.clone();

    for field in fields {
        if let Some(value) = normalized_inputs.get_mut(&field.key){
            *value = match field.normalize {
                Normalize::None => value.clone(),
                Normalize::Spaces => value.trim().replace(' ', "-"),
                Normalize::Uppercase => value.trim().to_uppercase(),
                Normalize::Lowercase => value.trim().to_lowercase(),
                Normalize::LowercaseFirstLetter => {
                    let mut chars = value.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(c) => c.to_lowercase().to_string() + chars.as_str(),
                    }
                }
                Normalize::UppercaseFirstLetter => {
                    let mut chars = value.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(c) => c.to_uppercase().to_string() + chars.as_str(),
                    }
                }
            }
        }
    }

    let branch = resolve_template(&formats.branch, &normalized_inputs);
    let commit = resolve_template(&formats.commit, &normalized_inputs);
    let pr_title = resolve_template(&formats.pr_title, &normalized_inputs);

    let generated_result = GeneratedResult {
        branch,
        commit,
        pr_title,
    };
    generated_result
}

pub fn resolve_template(template: &str, values: &IndexMap<String, String>) -> String {
    let mut result = template.to_string();
    for (key, value) in values {
            let placeholder = format!("{{{}}}", key);
            result = result.replace(&placeholder, &value);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{FieldType, Normalize};

    #[test]
    fn test_resolve_template() {
        let mut values = IndexMap::new();
        values.insert("type".to_string(), "feat".to_string());
        values.insert("ticket".to_string(), "42".to_string());
        let result = resolve_template("{type}/{ticket}", &values);
        assert_eq!(result, "feat/42");
    }

    #[test]
    fn test_resolve_template_missing_key() {
        let values = IndexMap::new();
        let result = resolve_template("{type}/test", &values);
        assert_eq!(result, "{type}/test");
    }

    #[test]
    fn test_resolve_template_empty() {
        let values = IndexMap::new();
        let result = resolve_template("", &values);
        assert_eq!(result, "");
    }

    #[test]
    fn test_generate_result_without_normalize() {
        let mut user_inputs = IndexMap::new();
        user_inputs.insert("type".to_string(), "feat".to_string());
        user_inputs.insert("ticket".to_string(), "42".to_string());
        user_inputs.insert("description".to_string(), "add-login".to_string());

        let form = FormState {
            user_inputs,
            selected_field: 0,
            select_input_position: 0,
            cursor_position: 0,
        };

        let formats = FormatConfig {
            branch:   "{type}/{ticket}-{description}".to_string(),
            commit:   "{type}: {description}".to_string(),
            pr_title: "[{ticket}] {description}".to_string(),
        };

        let fields = vec![
            FieldConfig { key: "type".to_string(),        label: "Type".to_string(),        field_type: FieldType::Select, required: true,  values: None, normalize: Normalize::None, persistent: false },
            FieldConfig { key: "ticket".to_string(),      label: "Ticket".to_string(),      field_type: FieldType::Number, required: true,  values: None, normalize: Normalize::None, persistent: false },
            FieldConfig { key: "description".to_string(), label: "Description".to_string(), field_type: FieldType::Text,   required: true,  values: None, normalize: Normalize::None, persistent: false },
        ];

        let result = generate_result(&form, &formats, &fields);
        assert_eq!(result.branch,   "feat/42-add-login");
        assert_eq!(result.commit,   "feat: add-login");
        assert_eq!(result.pr_title, "[42] add-login");
    }

    #[test]
    fn test_generate_result_with_normalize() {
        let mut user_inputs = IndexMap::new();
        user_inputs.insert("type".to_string(), "feat".to_string());
        user_inputs.insert("title".to_string(), "My New Feature".to_string());

        let form = FormState {
            user_inputs,
            selected_field: 0,
            select_input_position: 0,
            cursor_position: 0,
        };

        let formats = FormatConfig {
            branch:   "{type}/{title}".to_string(),
            commit:   "{type}: {title}".to_string(),
            pr_title: "{type}: {title}".to_string(),
        };

        let fields = vec![
            FieldConfig { key: "type".to_string(),  label: "Type".to_string(),  field_type: FieldType::Select, required: true, values: None, normalize: Normalize::None, persistent: false },
            FieldConfig { key: "title".to_string(), label: "Title".to_string(), field_type: FieldType::Text,   required: true, values: None, normalize: Normalize::Spaces, persistent: false },
        ];

        let result = generate_result(&form, &formats, &fields);
        assert_eq!(result.branch,   "feat/My-New-Feature");
        assert_eq!(result.commit,   "feat: My-New-Feature");
        assert_eq!(result.pr_title, "feat: My-New-Feature");
    }
}