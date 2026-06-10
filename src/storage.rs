use std::fs;
use std::path::PathBuf;
use directories::ProjectDirs;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct History {
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

fn build_persistent_path() -> Result<PathBuf, AppError> {
    let config_dir = ProjectDirs::from("", "", "branchgen")
        .ok_or(AppError::Config("Impossible de trouver le dossier config".into()))?;

    Ok(config_dir.config_dir().join("persistent.json"))
}

pub fn load_history() -> Result<Vec<History>, AppError>  {

    let file_path = build_history_path()?;
    if !file_path.exists() {
        return Ok(Vec::new());
    }
    // lit le contenu du fichier = cat config.toml
    let content = fs::read_to_string(&file_path)?;
    // donne le contenu a serde pour le transformer en mes struct AppConfig, FieldConfig
    match serde_json::from_str(&content) {
    Ok(entries) => Ok(entries),
        Err(_) => {
            let backup_path = file_path.with_extension("json.bak");
            let _ = fs::copy(&file_path, &backup_path);
            Ok(Vec::new())
        }
    }
}

pub fn load_persistent() -> IndexMap<String, String>  {

    let file_path = match build_persistent_path() {
        Ok(path) => path,
        Err(_) => return IndexMap::new(),
    };
    if !file_path.exists() {
        return IndexMap::new();
    }
    // lit le contenu du fichier = cat config.toml
    let content = match fs::read_to_string(&file_path) {
        Ok(data) => data,
        Err(_) => return IndexMap::new(),
    };
    // donne le contenu a serde pour le transformer en mes struct AppConfig, FieldConfig
    let persistent_data: IndexMap<String, String> = serde_json::from_str(&content).unwrap_or_default();

    persistent_data
}

pub fn save_history(history: &History) -> Result<(), AppError> {
    let file_path = build_history_path()?;

    let mut entries: Vec<History> = if file_path.exists() {
        let content = fs::read_to_string(&file_path)?;
        serde_json::from_str(&content)?
    } else {
        Vec::new()
    };
    entries.push(history.clone());
    let json = serde_json::to_string_pretty(&entries).map_err(|_| AppError::Config("Failed to serialize history".into()))?;
    fs::write(file_path, json).map_err(|_| AppError::Config("Failed to save history".into()))?;
    Ok(())
}

pub fn save_persistent(values: &IndexMap<String, String>) -> Result<(), AppError> {
    let file_path = build_persistent_path()?;
    let json = serde_json::to_string_pretty(values)?;
    fs::write(file_path, json)?;
    Ok(())
}