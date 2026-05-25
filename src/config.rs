use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub fields: Vec<FieldConfig>,
    pub formats: FormatConfig,
}

#[derive(Debug, Deserialize)]
pub struct FieldConfig {
    pub key: String,
    pub label: String,
    #[serde(rename = "type")]
    pub field_type: FieldType,
    pub required: bool,
}

#[derive(Debug, Deserialize)]
pub struct FormatConfig {
    pub branch: String,
    pub commit: String,
    pub pr_title: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FieldType {
    Text,
    Select,
    Number
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_config() {
        let toml_str = r#"
            [[fields]]
            key = "ticket"
            label = "Numero de ticket"
            type = "number"
            required = true
            [formats]
            branch = "{type}/{ticket}-{description}"
            commit = "{type}: {description}"
            pr_title = "[{ticket}] {description}"
        "#;

        let config: AppConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.fields[0].key, "ticket");
    }
}