use crate::groups::{courts::CourtSlots, group_scheduler::{ScheduleMatchFrontend, get_all_scheduled_matches}};
use rust_xlsxwriter::*;
use serde_json::error::Category;



#[tauri::command]
pub fn create_matches_excel(path: String){

    let matches = get_all_scheduled_matches();

    let mut circolo_matches = vec![];
    let mut g1_matches = vec![];

    for matche in matches.iter() {
        if matche.court < 5 {
            circolo_matches.push(matche);
        } else {
            g1_matches.push(matche);
        }
    }

    let saturday_time: [String; 7] = [
        "13:00".to_string(),
        "14:00".to_string(),
        "15:00".to_string(),
        "16:00".to_string(),
        "17:00".to_string(),
        "18:00".to_string(),
        "19:00".to_string()
    ];
    let sunday_time: [String; 12] = [
        "8:00".to_string(),
        "9:00".to_string(),
        "10:00".to_string(),
        "11:00".to_string(),
        "12:00".to_string(),
        "13:00".to_string(),
        "14:00".to_string(),
        "15:00".to_string(),
        "16:00".to_string(),
        "17:00".to_string(),
        "18:00".to_string(),
        "19:00".to_string()
    ];

    let h1_format = Format::new()
        .set_bold()
        .set_font_size(16)
        .set_font_color(Color::Blue);

    let header_table_format = Format::new()
        .set_background_color(Color::RGB(0xDAE9F8 as u32))
        .set_border(FormatBorder::Thin)
        .set_bold();

    let white_line_format = Format::new()
        .set_border(FormatBorder::Thin)
        .set_bold()
        .set_font_color(Color::Red)
        .set_align(FormatAlign::VerticalCenter)
        .set_align(FormatAlign::Center);

    let light_gray_line_format = Format::new()
        .set_border(FormatBorder::Thin)
        .set_background_color(Color::RGB(0xd0d0d0 as u32))
        .set_bold()
        .set_font_color(Color::Red)
        .set_align(FormatAlign::VerticalCenter)
        .set_align(FormatAlign::Center);

    let normal_text_format_white = Format::new()
        .set_bold()
        .set_text_wrap()
        .set_align(FormatAlign::VerticalCenter)
        .set_border(FormatBorder::Thin)
        .set_align(FormatAlign::Center);

    let normal_text_format_light_gray = Format::new()
        .set_bold()
        .set_text_wrap()
        .set_align(FormatAlign::VerticalCenter)
        .set_background_color(Color::RGB(0xd0d0d0 as u32))
        .set_border(FormatBorder::Thin)
        .set_align(FormatAlign::Center);


    let mut files_excel = Workbook::new();

    let mut row_bias = 2;

    let file = files_excel.add_worksheet().set_name("G1").expect("Error in creation file");

    file.write_with_format(row_bias, 2, "Sabato ... - G1", &h1_format).expect("Error to write in the file");

    row_bias +=2;

    for i in 0..2{
        file.write_with_format(row_bias, 1 +(i * 3), "ORA", &header_table_format).expect("Error to write in the file");
        file.write_with_format(row_bias, 2 + (i * 3), format!("CAMPO {}", i + 1), &header_table_format).expect("Error to write in the file");
        file.write_with_format(row_bias, 3 + (i * 3), "TAB.", &header_table_format).expect("Error to write in the file");

        file.set_column_width(1 + (i * 3), 8).expect("Error to write in the file");
        file.set_column_width(2 + (i * 3), 20).expect("Error to write in the file");
        file.set_column_width(3 + (i * 3), 8).expect("Error to write in the file");
    }
    row_bias += 1;

    for i in 0..saturday_time.len(){

        for y in 0..6{
            file.set_row_height(i as u32 + row_bias, 35).expect("Error to write in the file");

            if y == 0 && i%2 == 0{
                file.write_with_format(row_bias + i as u32, y + 1, saturday_time[i].clone(), &white_line_format).expect("Error to write in the file");
            }
            else if y == 0 && i%2 != 0{
                file.write_with_format(row_bias + i as u32, y + 1, saturday_time[i].clone(), &light_gray_line_format).expect("Error to write in the file");
            }
            else if y == 3 && i%2 == 0 {
                file.write_with_format(row_bias + i as u32, y + 1, saturday_time[i].clone(), &white_line_format).expect("Error to write in the file");
            }
            else if y == 3 && i%2 != 0 {
                file.write_with_format(row_bias + i as u32, y + 1, saturday_time[i].clone(), &light_gray_line_format).expect("Error to write in the file");
            }
            else if i%2 == 0{
                file.write_with_format(row_bias + i as u32, y + 1, "", &white_line_format).expect("Error to write in the file");
            }else{
                file.write_with_format(row_bias + i as u32, y + 1, "", &light_gray_line_format).expect("Error to write in the file");
            }
        }
    }

    row_bias = 31;

    file.write_with_format(row_bias - 1, 2, "Domenica ... - G1", &h1_format).expect("Error to write in the file");

    row_bias += 1;

    for i in 0..2{
        file.write_with_format(row_bias, 1 +(i * 3), "ORA", &header_table_format).expect("Error to write in the file");
        file.write_with_format(row_bias, 2 + (i * 3), format!("CAMPO {}", i + 1), &header_table_format).expect("Error to write in the file");
        file.write_with_format(row_bias, 3 + (i * 3), "TAB.", &header_table_format).expect("Error to write in the file");
    }

    row_bias += 1;

    for i in 0..sunday_time.len(){
        for y in 0..6{
            file.set_row_height(i as u32 + row_bias, 35).expect("Error to write in the file");

            if y == 0 && i%2 == 0{
                file.write_with_format(row_bias + i as u32, y + 1, sunday_time[i].clone(), &white_line_format).expect("Error to write in the file");
            }
            else if y == 0 && i%2 != 0{
                file.write_with_format(row_bias + i as u32, y + 1, sunday_time[i].clone(), &light_gray_line_format).expect("Error to write in the file");
            }
            else if y == 3 && i%2 == 0 {
                file.write_with_format(row_bias + i as u32, y + 1, sunday_time[i].clone(), &white_line_format).expect("Error to write in the file");
            }
            else if y == 3 && i%2 != 0 {
                file.write_with_format(row_bias + i as u32, y + 1, sunday_time[i].clone(), &light_gray_line_format).expect("Error to write in the file");
            }
            else if i%2 == 0{
                file.write_with_format(row_bias + i as u32, y + 1, "", &white_line_format).expect("Error to write in the file");
            }else{
                file.write_with_format(row_bias + i as u32, y + 1, "", &light_gray_line_format).expect("Error to write in the file");
            }
        }
    }

    let saturday_start_row = 5;
    let sunday_start_row = 33;

    for matche in g1_matches.iter(){
        
        if let Some((row, col)) = get_match_position(matche, saturday_start_row, sunday_start_row){

            if row % 2 == 1{
                file.write_with_format(row, col - 1, format!("{} \n {}", matche.player_1.name, matche.player_2.name), &normal_text_format_white).expect("Error to write in the file");
                file.write_with_format(row, col, format!("{:?}", matche.player_1.category), &normal_text_format_white).expect("Error to write in the file");
            }
            else{
                file.write_with_format(row, col - 1, format!("{} \n {}", matche.player_1.name, matche.player_2.name), &normal_text_format_light_gray).expect("Error to write in the file");
                file.write_with_format(row, col, format!("{:?}", matche.player_1.category), &normal_text_format_light_gray).expect("Error to write in the file");
            }
            
        }

    }


    files_excel.save(path).expect("Error to save the file");

}

fn get_match_position(matche: &ScheduleMatchFrontend, saturday_start_row: u32, sunday_start_row: u32,) -> Option<(u32, u16)> {
    // 1. Determina il campo (5 → campo 1, 6 → campo 2)
    let campo_col = if matche.court == 5 {
        3u16  // Colonna "CAMPO 1"
    } else if matche.court == 6 {
        6u16  // Colonna "CAMPO 2"
    } else {
        return None; // Non è G1
    };

    let slot = matche.scheduled_time;

    // 2. Controlla Sabato
    if let Some(idx) = [
        CourtSlots::SATH13, CourtSlots::SATH14, CourtSlots::SATH15,
        CourtSlots::SATH16, CourtSlots::SATH17, CourtSlots::SATH18, CourtSlots::SATH19,
    ].iter().position(|&s| slot.contains(s)) {
        let row = saturday_start_row + idx as u32;
        return Some((row, campo_col));
    }

    // 3. Controlla Domenica
    if let Some(idx) = [
        CourtSlots::SUNH08, CourtSlots::SUNH09, CourtSlots::SUNH10, CourtSlots::SUNH11,
        CourtSlots::SUNH12, CourtSlots::SUNH13, CourtSlots::SUNH14, CourtSlots::SUNH15,
        CourtSlots::SUNH16, CourtSlots::SUNH17, CourtSlots::SUNH18, CourtSlots::SUNH19,
    ].iter().position(|&s| slot.contains(s)) {
        let row = sunday_start_row + idx as u32;
        return Some((row, campo_col));
    }

    None // Orario non trovato
}