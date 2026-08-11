use crate::utils::get_current_timestamp;
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
pub async fn create_new_entity(
    app: AppHandle,
    entity_name: String,
    entity_category: String,
    world_id: String,
) -> Result<(), String> {
    let entity = Entity {
        id: uuid::Uuid::new_v4().to_string(),
        name: entity_name,
        category: entity_category,
        thumbnail_path: None,
        metadata: HashMap::new(),
        content: "".to_string(),
        tags: Vec::new(),
        created_at: get_current_timestamp(),
        modified_at: get_current_timestamp(),
    };

    let dir = worlds_dir(&app)?;
    let entity_dir = dir.join(world_id).join("entities").join(&entity.category);
    fs::create_dir_all(&entity_dir).map_err(|e| e.to_string())?;
    let file_path = entity_dir.join(format!("{}.json", entity.id));
    let json_data = serde_json::to_string_pretty(&entity).map_err(|e| e.to_string())?;
    fs::write(file_path, json_data).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn list_entity_categories(
    app: AppHandle,
    world_id: String,
) -> Result<Vec<String>, String> {
    let dir = worlds_dir(&app)?;
    let entity_dir = dir.join(world_id).join("entities");

    let entries = fs::read_dir(&entity_dir).map_err(|e| e.to_string())?;

    let categories = entries
        .filter_map(|entry| entry.ok())
        // Optional: filter to only include directories
        .filter(|entry| entry.path().is_dir())
        .filter_map(|entry| entry.file_name().into_string().ok())
        .collect();

    Ok(categories)
}

#[tauri::command]
pub async fn list_entities_in_category(
    app: AppHandle,
    world_id: String,
    category: String,
) -> Result<Vec<Entity>, String> {
    let dir = worlds_dir(&app)?;
    let entity_dir = dir.join(world_id).join("entities").join(category);

    let entries = fs::read_dir(&entity_dir).map_err(|e| e.to_string())?;

    let mut entities = Vec::new();
    for entry in entries {
        if let Ok(entry) = entry {
            if entry.path().is_file() {
                if let Ok(json_data) = fs::read_to_string(entry.path()) {
                    if let Ok(entity) = serde_json::from_str::<Entity>(&json_data) {
                        entities.push(entity);
                    }
                }
            }
        }
    }

    Ok(entities)
}