mod forms;
mod player;
mod availability;
mod round;

use forms::create_forms::create_form;
use forms::forms_info::*;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![create_form,
                                                get_registration_form,
                                                get_availability_form])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
