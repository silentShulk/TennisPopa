use crate::{get_resource, groups::{courts::CourtSlots, group_scheduler::{ScheduleMatchFrontend, get_all_scheduled_matches}}, players::Player};
use chrono::*;
use rusqlite::Connection;
use rusqlite_struct::rusqlite_struct_helper::RusqliteStructHelper;
use rust_xlsxwriter::*;
use rusqlite_struct_derive::RusqliteStruct;

#[tauri::command]
pub fn create_excel_for_player(path: String){

    let write_header_format = Format::new()
        .set_bold()
        .set_font_size(16)
        .set_font_name("Aptos Narrow");

    let normal_format = Format::new()
        .set_bold()
        .set_font_name("Aptos Narrow");

    let mut files_excel = Workbook::new();
    let file = files_excel.add_worksheet().set_name("Anagrafiche").expect("Error to create file");

    let conn = Connection::open(get_resource("databases/players.db")).unwrap();
    let mut players = conn.get_from_table_struct::<Player>().unwrap();

    players.sort_by(|a, b| {
        a.name.to_lowercase().cmp(&b.name.to_lowercase())
    });

    let header: [String; 5] = [
        "NAME".to_string(),
        "CATEGORIA".to_string(),
        "NUMERO DI TELEFONO".to_string(),
        "EMAIL".to_string(),
        "TAGLIA".to_string()
    ];

    for i in 0..header.len(){
        file.write_with_format(0, i as u16, header[i].clone(), &write_header_format).expect("Error to write in the file");

        file.set_column_width(i as u16, 50).expect("Error to write in the file");
    }

    for (i,p) in players.iter().enumerate(){

        file.write_with_format(i as u32 + 3, 0, p.name.clone(), &normal_format).expect("Error to write in the file");
        file.write_with_format(i as u32 + 3, 1, format!("{:?}", p.category.clone()), &normal_format).expect("Error to write in the file");
        file.write_with_format(i as u32 + 3, 2, p.phone_number.clone(), &normal_format).expect("Error to write in the file");
        file.write_with_format(i as u32 + 3, 3, p.email.clone(), &normal_format).expect("Error to write in the file");
        file.write_with_format(i as u32 + 3, 4, format!("{:?}", p.size.clone()), &normal_format).expect("Error to write in the file");
    }

    files_excel.save(path).expect("Error to save the file");

}

