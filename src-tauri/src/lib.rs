mod forms;
mod players;
mod groups;

use tauri::Manager;
use tauri::path::BaseDirectory;
use std::path::PathBuf;
use std::sync::Mutex;
use once_cell::sync::Lazy;

use forms::create_forms::create_form;
use forms::forms_info::*;
use forms::get_forms_responses::main_get_forms_responses;
use crate::groups::group::{create_groups, groups_in_category, swap_group_for_players};
use crate::groups::group_scheduler::{get_all_scheduled_matches, schedule_matches_for_all_players};
use crate::groups::courts::save_availability_court;
use crate::groups::*;
use crate::players::player::*;

static RESOURCE_DIR: Lazy<Mutex<Option<PathBuf>>> = Lazy::new(|| Mutex::new(None));
fn init_resource_dir(app: &tauri::App) -> Result<(), String> {
    let dir = app
        .path()
        .resolve(".", BaseDirectory::Resource)
        .map_err(|e| e.to_string())?;
        
        *RESOURCE_DIR.lock().unwrap() = Some(dir);
        Ok(())
}

pub fn get_resource(relative_path: &str) -> PathBuf {
    let base = RESOURCE_DIR
        .lock()
        .map_err(|e| e.to_string()).unwrap()
        .clone()
        .ok_or("RESOURCE_DIR not initialized.").unwrap();

    base.join(relative_path)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            init_resource_dir(app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![create_form,
                                                get_registration_form,
                                                get_availability_form,
                                                main_get_forms_responses,
                                                get_registration_form_type,
                                                get_availability_form_type,
                                                find_player,
                                                update_spec_player,
                                                create_groups,
                                                save_availability_court,
                                                groups_in_category,
                                                create_excel_group,
                                                save_match_result,
                                                swap_group_for_players,
                                                schedule_matches_for_all_players,
                                                get_all_scheduled_matches])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}
