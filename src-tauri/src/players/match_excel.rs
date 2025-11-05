use crate::{groups::{courts::CourtSlots, group_scheduler::{ScheduleMatchFrontend, get_all_scheduled_matches}}};
use chrono::*;
use rust_xlsxwriter::*;



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
        .set_font_name("Aptos Narrow")
        .set_font_color(Color::Blue);

    let header_table_format = Format::new()
        .set_background_color(Color::RGB(0xDAE9F8 as u32))
        .set_font_name("Aptos Narrow")
        .set_border(FormatBorder::Thin)
        .set_bold();

    let white_line_format = Format::new()
        .set_border(FormatBorder::Thin)
        .set_font_name("Aptos Narrow")
        .set_bold()
        .set_font_color(Color::Red)
        .set_align(FormatAlign::VerticalCenter)
        .set_align(FormatAlign::Center);

    let light_gray_line_format = Format::new()
        .set_border(FormatBorder::Thin)
        .set_background_color(Color::RGB(0xd0d0d0 as u32))
        .set_font_name("Aptos Narrow")
        .set_bold()
        .set_font_color(Color::Red)
        .set_align(FormatAlign::VerticalCenter)
        .set_align(FormatAlign::Center);

    let normal_text_format_white = Format::new()
        .set_bold()
        .set_text_wrap()
        .set_align(FormatAlign::VerticalCenter)
        .set_font_name("Aptos Narrow")
        .set_border(FormatBorder::Thin)
        .set_align(FormatAlign::Center);

    let normal_text_format_light_gray = Format::new()
        .set_bold()
        .set_text_wrap()
        .set_align(FormatAlign::VerticalCenter)
        .set_font_name("Aptos Narrow")
        .set_background_color(Color::RGB(0xd0d0d0 as u32))
        .set_border(FormatBorder::Thin)
        .set_align(FormatAlign::Center);


    let mut files_excel = Workbook::new();

    let (saturday, sunday) = the_next_weekend();

    //G1 matches.

    let mut row_bias = 2;

    let file = files_excel.add_worksheet().set_name("G1").expect("Error in creation file");

    file.write_with_format(row_bias, 2, format!("Sabato: {} - G1", saturday), &h1_format).expect("Error to write in the file");

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

    file.write_with_format(row_bias - 1, 2, format!("Domenica: {} - G1", sunday), &h1_format).expect("Error to write in the file");

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

            let name_p1 = shorten_name(matche.player_1.name.clone());
            let name_p2 = shorten_name(matche.player_2.name.clone());

            if row % 2 == 1{
                file.write_with_format(row, col - 1, format!("{} \n {}", matche.player_1.name.clone(), matche.player_2.name), &normal_text_format_white).expect("Error to write in the file");
                file.write_with_format(row, col, format!("{:?}", matche.player_1.category), &normal_text_format_white).expect("Error to write in the file");
            }
            else{
                file.write_with_format(row, col - 1, format!("{} \n {}", matche.player_1.name.clone(), matche.player_2.name), &normal_text_format_light_gray).expect("Error to write in the file");
                file.write_with_format(row, col, format!("{:?}", matche.player_1.category), &normal_text_format_light_gray).expect("Error to write in the file");
            }
            
        }

    }

    //Circolo matches

    let file = files_excel.add_worksheet().set_name("Circolo").expect("Error in creation file");

    let mut row_bias = 2;


    file.write_with_format(row_bias, 5, format!("Sabato: {} - Circolo", saturday), &h1_format).expect("Error to write in the file");

    row_bias +=2;

    for i in 0..5{
        file.write_with_format(row_bias, 1 +(i * 3), "ORA", &header_table_format).expect("Error to write in the file");

        if i >= 4{
            file.write_with_format(row_bias, 2 + (i * 3), format!("CAMPO {}", i + 2), &header_table_format).expect("Error to write in the file");
        }else{
            file.write_with_format(row_bias, 2 + (i * 3), format!("CAMPO {}", i + 1), &header_table_format).expect("Error to write in the file");
        }
        
        file.write_with_format(row_bias, 3 + (i * 3), "TAB.", &header_table_format).expect("Error to write in the file");

        file.set_column_width(1 + (i * 3), 8).expect("Error to write in the file");
        file.set_column_width(2 + (i * 3), 20).expect("Error to write in the file");
        file.set_column_width(3 + (i * 3), 8).expect("Error to write in the file");
    }
    row_bias += 1;

    for i in 0..saturday_time.len(){

        for y in 0..15{
            file.set_row_height(i as u32 + row_bias, 35).expect("Error to write in the file");

            if y%3 == 0 && i%2 == 0{
                file.write_with_format(row_bias + i as u32, y + 1, saturday_time[i].clone(), &white_line_format).expect("Error to write in the file");
            }
            else if y%3 == 0 && i%2 != 0{
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

    file.write_with_format(row_bias - 1, 5, format!("Domenica: {} - Circolo", sunday), &h1_format).expect("Error to write in the file");

    row_bias += 1;

    for i in 0..5{

        file.write_with_format(row_bias, 1 +(i * 3), "ORA", &header_table_format).expect("Error to write in the file");
        
        if i >= 4{
            file.write_with_format(row_bias, 2 + (i * 3), format!("CAMPO {}", i + 2), &header_table_format).expect("Error to write in the file");
        }else{
            file.write_with_format(row_bias, 2 + (i * 3), format!("CAMPO {}", i + 1), &header_table_format).expect("Error to write in the file");
        }
        
        file.write_with_format(row_bias, 3 + (i * 3), "TAB.", &header_table_format).expect("Error to write in the file");
    }

    row_bias += 1;

    for i in 0..sunday_time.len(){
        for y in 0..15{
            file.set_row_height(i as u32 + row_bias, 35).expect("Error to write in the file");

            if y%3 == 0 && i%2 == 0{
                file.write_with_format(row_bias + i as u32, y + 1, sunday_time[i].clone(), &white_line_format).expect("Error to write in the file");
            }
            else if y%3 == 0 && i%2 != 0{
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

    for matche in circolo_matches.iter(){
        
        if let Some((row, col)) = get_match_position(matche, saturday_start_row, sunday_start_row){

            let name_p1 = shorten_name(matche.player_1.name.clone());
            let name_p2 = shorten_name(matche.player_2.name.clone());

            if row % 2 == 1{
                file.write_with_format(row, col - 1, format!("{} \n {}", matche.player_1.name.clone(), matche.player_2.name), &normal_text_format_white).expect("Error to write in the file");
                file.write_with_format(row, col, format!("{:?}", matche.player_1.category), &normal_text_format_white).expect("Error to write in the file");
            }
            else{
                file.write_with_format(row, col - 1, format!("{} \n {}", matche.player_1.name.clone(), matche.player_2.name), &normal_text_format_light_gray).expect("Error to write in the file");
                file.write_with_format(row, col, format!("{:?}", matche.player_1.category), &normal_text_format_light_gray).expect("Error to write in the file");
            }
            
        }

    }


    files_excel.save(path).expect("Error to save the file");

}

fn shorten_name(name_player: String) -> String {
    // Dividi in parti separate da spazio
    let mut part = name_player.split_whitespace();

    // Estrai nome e cognome (se esistono)
    let name = part.next().unwrap_or("");
    let surname = part.next().unwrap_or("");

    
    if let Some(initial_character) = surname.chars().next() {
        format!("{} {}.", name, initial_character.to_uppercase())
    } else {
        name.to_string()
    }
}

fn get_match_position(matche: &ScheduleMatchFrontend, saturday_start_row: u32, sunday_start_row: u32,) -> Option<(u32, u16)> {

    let campo_col = if matche.court == 5 || matche.court == 0{
        3u16  
    } else if matche.court == 6 || matche.court == 1{
        6u16  
    } else if matche.court == 2{
        9u16
    }else if matche.court == 3{
        12u16
    }else if matche.court == 4{
        15u16
    }else{
        return None;
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

fn the_next_weekend() -> (String, String) {
    let oggi = Local::now().date_naive();
    let weekday = oggi.weekday();

    // Calcola quanto manca al sabato (Weekday::Sat = 6)
    let giorni_a_sabato = match weekday {
        Weekday::Sat => 0,
        _ => (Weekday::Sat.num_days_from_monday() + 7 - weekday.num_days_from_monday()) % 7,
    };

    let sabato = oggi + Duration::days(giorni_a_sabato as i64);
    let domenica = sabato + Duration::days(1);

    // Crea le stringhe "gg mese"
    let sabato_str = format!("{:02} {}", sabato.day(), month_name(sabato.month()));
    let domenica_str = format!("{:02} {}", domenica.day(), month_name(domenica.month()));

    (sabato_str, domenica_str)
}

fn month_name(mese: u32) -> &'static str {
    match mese {
        1 => "Gennaio",
        2 => "Febbraio",
        3 => "Marzo",
        4 => "Aprile",
        5 => "Maggio",
        6 => "Giugno",
        7 => "Luglio",
        8 => "Agosto",
        9 => "Settembre",
        10 => "Ottobre",
        11 => "Novembre",
        12 => "Dicembre",
        _ => "",
    }
}