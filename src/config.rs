use std::fs;
use std::path::PathBuf;
use directories::ProjectDirs;
use serde::Deserialize;
use crate::error::AppError;
use crate::config_template::DEFAULT_CONFIG;

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

fn build_config_path() -> Result<PathBuf, AppError> {
    // On récupère les chemins standards de l'application "branchgen" suivant l'OS
    // (ex: ~/.config/branchgen sur Linux, ~/Library/Application Support/branchgen sur macOS)
    let config_dir = ProjectDirs::from("", "", "branchgen")
        .ok_or(AppError::Config("Impossible de trouver le dossier config".into()))?;

    Ok(config_dir.config_dir().join("config.toml"))
}


fn get_config_path() -> Result<PathBuf, AppError> {
    // On récupère les chemins standards de l'application "branchgen" suivant l'OS
    // (ex: ~/.config/branchgen sur Linux, ~/Library/Application Support/branchgen sur macOS)
    let file_path = build_config_path()?;

    if !file_path.exists() {
        // On crée physiquement le fichier
        return Err(AppError::Config("Config file not found. Run branchgen --init".into()))
    }
    Ok(file_path)
}

// construit mes structs a partir du fichier toml
pub fn load_config() -> Result<AppConfig, AppError>  {

    let file_path = get_config_path()?;
    // lit le contenu du fichier = cat config.toml
    let content = fs::read_to_string(&file_path)?;
    // donne le contenu a serde pour le transformer en mes struct AppConfig, FieldConfig
    let config: AppConfig = toml::from_str(&content)?;

    Ok(config)
}

pub fn generate_default_config() -> Result<(), AppError> {

    let file_path = build_config_path()?;

    if file_path.exists() {
        return Err(AppError::Config("Config déja existante".into()))
    }

    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(&file_path, DEFAULT_CONFIG)?;
    println!("Generated default config file: {:?}", file_path);
    Ok(())
}