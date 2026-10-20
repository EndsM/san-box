use std::sync::Mutex;

use crate::map::{WorldMap, demo::demo_world, view::MapSnapshot};

pub mod map;

struct AppState {
    // Import Mutex from std for now, consider tauri one later
    world: Mutex<WorldMap>,
}

#[tauri::command]
fn get_world_snapshot(state: tauri::State<'_, AppState>) -> MapSnapshot {
    let world = state.world.lock().expect("world lock poisoned");
    MapSnapshot::from_world(&world)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            world: Mutex::new(demo_world()),
        })
        .invoke_handler(tauri::generate_handler![get_world_snapshot])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
