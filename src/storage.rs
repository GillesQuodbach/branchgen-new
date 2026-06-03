use std::fs;
use std::path::PathBuf;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct History {
    pub id: usize,
    pub date: String,
    pub branch: String,
    pub commit: String,
    pub pr_title: String,
}

fn build_history_path() -> Result<PathBuf, AppError> {
    // On récupère les chemins standards de l'application "branchgen" suivant l'OS
    // (ex: ~/.config/branchgen sur Linux, ~/Library/Application Support/branchgen sur macOS)
    let config_dir = ProjectDirs::from("", "", "branchgen")
        .ok_or(AppError::Config("Impossible de trouver le dossier config".into()))?;

    Ok(config_dir.config_dir().join("history.json"))
}

pub fn load_history() -> Result<History, AppError>  {

    let file_path = build_history_path()?;;
    // lit le contenu du fichier = cat config.toml
    let content = fs::read_to_string(&file_path)?;
    // donne le contenu a serde pour le transformer en mes struct AppConfig, FieldConfig
    let config: History = toml::from_str(&content)?;

    Ok(config)
}