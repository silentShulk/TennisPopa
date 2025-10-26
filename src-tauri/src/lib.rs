mod forms;
mod players;

use forms::create_forms::create_form;
use forms::forms_info::*;
use forms::get_forms_responses::main_get_forms_responses;
use crate::players::group::*;
use crate::players::player::*;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
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
                                                create_excel_group])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
