mod entities;
mod utils;
mod worlds;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            worlds::create_new_world,
            worlds::list_all_worlds,
            entities::create_new_entity,
            entities::list_entity_categories,
            entities::list_entities_in_category,
            entities::get_entity,
            entities::update_entity,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
