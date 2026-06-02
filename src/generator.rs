use std::collections::HashMap;
use crate::config::FormatConfig;
use crate::state::{FormState, GeneratedResult};

pub fn generate_result(form: &FormState, formats: &FormatConfig) -> GeneratedResult{}

pub fn resolve_template(template: &str, values: &HashMap<String, String>) -> String {
    let mut result = template.to_string();
    for (key, value) in values {
            let placeholder = format!("{{{}}}", key);
            result = result.replace(&placeholder, &value);
    }
    result
}