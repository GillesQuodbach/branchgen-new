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

// use std::path::{Path, PathBuf};
// use directories::ProjectDirs;test_path
// use std::fs;
// use dialoguer::Input;
// // use dialoguer::Input;
// use serde::{Deserialize, Serialize};
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct AppConfig {
//     pub team: String,
//     pub default_pi: Option<u32>,
//     pub default_it: Option<u32>,
// }
//
// impl AppConfig {
//     pub fn new(team: String) -> Self {
//         Self {
//             team,
//             default_pi: None,
//             default_it: None,
//         }
//     }
// }
//
// // load or create config
// pub fn load_or_init_config() -> Result<AppConfig, String> {
//     let config_file_exists = config_file_exists().map_err(|e| format!("Config file exists error: {}", e))?;
//
//     if config_file_exists {
//         // read config file
//         return Ok(read_config_file()?format);
//     }
//
//     let input_team: String = Input::new()
//         .with_prompt("Please enter your team name")
//         .validate_with(|input: &String| {
//             if input.trim().is_empty(){
//                 Err("Team name cannot be empty")
//             } else {
//                 Ok(())
//             }
//         })
//         .interact_text()
//         .map_err(|e| format!("Failed to read team name: {}", e))?;
//
//     let team_name = input_team.trim().to_uppercase();
//
//     let config = create_config_file(team_name)?;
//     Ok(config)
// }
//
// //Ou est le dosssier ?
// pub fn
// get_config_dir() -> Result<PathBuf, String> {
//     let proj_dir = ProjectDirs::from("fr","aps","branchgen-cli")
//         .ok_or("Unable to find config dir")?;
//
//     let config_path = proj_dir.config_dir();
//     fs::create_dir_all(config_path).map_err(|e| format!("Unable to create config dir: {}", e))?;
//     Ok(config_path.to_path_buf())
// }
//
// // quel est le chemin du fichier ?
// pub fn get_config_file_path() -> Result<PathBuf, String> {
//     let config_file = get_config_dir()?.join("config.json");
//     Ok(config_file)
// }
//
// // est-ce qu'il existe ?
// pub fn config_file_exists() -> Result<bool, String> {
//     let config_file = get_config_file_path()?;
//     config_file.try_exists().map_err(|e| format!("Check your configuration file: {}", e))
// }
//
// // creation/ecriture du fichier
// pub fn create_config_file(team: String) -> Result<AppConfig, String> {
//     let config_file_path = get_config_file_path()?;
//
//     let config = AppConfig::new(team);
//
//     let json = serde_json::to_string_pretty(&config).map_err(|e| format!("Unable to serialize config: {}", e))?;
//
//     fs::write(config_file_path, json).map_err(|e| format!("Unable to write config file: {}", e))?;
//
//     Ok(config)
// }
//
// pub fn read_config_file() -> Result<AppConfig, String> {
//     let config_file_path = get_config_file_path()?;
//     let json = fs::read_to_string(config_file_path).map_err(|e| format!("Unable to read config file: {}", e))?;
//     let config: AppConfig = serde_json::from_str(&json).map_err(|e| format!("Unable to deserialize config: {}", e))?;
//     Ok(config)
// }
//
// pub fn save_config_to_path(path: &Path,config: &AppConfig) -> Result<(), String> {
//     let json = serde_json::to_string_pretty(config).map_err(|e| format!("Failed to serialize config: {}", e))?;
//
//     fs::write(path, json).map_err(|e| format!("Failed to write config: {}", e))?;
//
//     Ok(())
// }
//
// pub fn save_config(config: &AppConfig) -> Result<(), String> {
//     let config_file_path = get_config_file_path()?;
//     save_config_to_path(&config_file_path,config)
// }
//
// pub fn update_pi_it_defaults(pi: u32, it: u32) -> Result<AppConfig, String> {
//     let mut config = load_or_init_config()?;
//     config.default_pi = Some(pi);
//     config.default_it = Some(it);
//
//     save_config(&config)?;
//     Ok(config)
// }









