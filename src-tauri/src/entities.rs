use crate::utils::worlds_dir;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use tauri::AppHandle;

#[derive(Serialize, Deserialize)]
pub struct Entity {
    id: String,
    name: String,
    category: String,
    thumbnail_path: Option<String>,
    metadata: HashMap<String, String>,
    content: String,
    tags: Vec<String>,
    created_at: u64,
    modified_at: u64,
}

#[tauri::command]
pub async fn save_entity(app: AppHandle, entity: Entity, world_id: String) -> Result<(), String> {
    let dir = worlds_dir(&app)?;
    let entity_dir = dir.join(world_id).join("entities").join(&entity.category);

    fs::create_dir_all(&entity_dir).map_err(|e| e.to_string())?;
    let file_path = entity_dir.join(format!("{}.json", entity.id));
    let json_data = serde_json::to_string_pretty(&entity).map_err(|e| e.to_string())?;
    fs::write(file_path, json_data).map_err(|e| e.to_string())?;
    Ok(())
}
