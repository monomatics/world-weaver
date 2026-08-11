use crate::utils::get_current_timestamp;
use crate::utils::worlds_dir;
use serde::{Deserialize, Serialize};
use std::fs;
use tauri::AppHandle;

#[derive(Serialize, Deserialize, Clone)]
pub struct World {
    id: String,
    name: String,
    created_at: u64,
    updated_at: u64,
}

#[tauri::command]
pub async fn create_new_world(app: AppHandle, name: String) -> Result<(), String> {
    let dir = worlds_dir(&app)?;
    let id = uuid::Uuid::new_v4().to_string();
    let world_dir = dir.join(&id);
    fs::create_dir(&world_dir).map_err(|e| e.to_string())?;

    let world = World {
        id,
        name,
        created_at: get_current_timestamp(),
        updated_at: get_current_timestamp(),
    };

    let meta = world_dir.join("world.json");
    let json = serde_json::to_string_pretty(&world).map_err(|e| e.to_string())?;
    fs::write(meta, json).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn list_all_worlds(app: AppHandle) -> Result<Vec<World>, String> {
    let dir = worlds_dir(&app)?;
    let mut worlds = Vec::new();

    for entry in fs::read_dir(&dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let meta = entry.path().join("world.json");
        if meta.exists() {
            let json = fs::read_to_string(meta).map_err(|e| e.to_string())?;
            let world: World = serde_json::from_str(&json).map_err(|e| e.to_string())?;
            worlds.push(world);
        }
    }

    worlds.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    Ok(worlds)
}
