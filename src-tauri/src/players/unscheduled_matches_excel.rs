use crate::{get_resource, groups::{courts::CourtSlots, group_scheduler::{ScheduleMatchFrontend, UnscheduledMatch, get_all_scheduled_matches, get_all_unscheduled_matches}}};
use rusqlite::Connection;
use rusqlite_struct::rusqlite_struct_helper::RusqliteStructHelper;
use rust_xlsxwriter::*;
use rusqlite_struct_derive::RusqliteStruct;

#[tauri::command]
pub fn unscheduled_matches_excel(path: String){

    let unscheduled_matches = get_all_unscheduled_matches();

    let normal_text_format_light_gray = Format::new()
        .set_bold()
        .set_text_wrap()
        .set_align(FormatAlign::VerticalCenter)
        .set_font_name("Aptos Narrow")
        .set_background_color(Color::RGB(0xd0d0d0 as u32))
        .set_border(FormatBorder::Thin)
        .set_align(FormatAlign::Center);

    let normal_text_format_white = Format::new()
        .set_bold()
        .set_text_wrap()
        .set_align(FormatAlign::VerticalCenter)
        .set_font_name("Aptos Narrow")
        .set_border(FormatBorder::Thin)
        .set_align(FormatAlign::Center);

    let mut files_excel = Workbook::new();

    let file = files_excel.add_worksheet().set_name("Partite di riserva").expect("Error to create a file");

    file.set_column_width(0, 50).expect("Error to write in the file");
    file.set_column_width(1, 20).expect("Error to write in the file");

    for (i, matche) in unscheduled_matches.iter().enumerate(){
        if i%2 == 0{
            file.write_with_format(i as u32 + 5, 0, format!("{} \n {}", matche.player_1.name, matche.player_2.name), &normal_text_format_white).expect("Error to write in the file");
            file.write_with_format(i as u32 + 5, 1, format!("{:?}", matche.category), &normal_text_format_white).expect("Error to write in the file");
        }
        if i%2 != 0{
            file.write_with_format(i as u32 + 5, 0, format!("{} \n {}", matche.player_1.name, matche.player_2.name), &normal_text_format_light_gray).expect("Error to write in the file");
            file.write_with_format(i as u32 + 5, 1, format!("{:?}", matche.category), &normal_text_format_light_gray).expect("Error to write in the file");
        }

        file.set_row_height(i as u32, 30).expect("Error to write in the file");
    }

    files_excel.save(path).expect("Error to save the file");
}