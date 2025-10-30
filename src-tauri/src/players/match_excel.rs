use crate::groups::group_scheduler::get_all_scheduled_matches;
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
        .set_font_color(Color::Blue);

    let header_table_format = Format::new()
        .set_background_color(Color::RGB(0xDAE9F8 as u32))
        .set_border(FormatBorder::Thin)
        .set_bold();

    let white_line_format = Format::new()
        .set_border(FormatBorder::Thin);

    let light_gray_line_format = Format::new()
        .set_border(FormatBorder::Thin)
        .set_background_color(Color::RGB(0xd0d0d0 as u32));

    let time_format = Format::new()
        .set_font_color(Color::Red)
        .set_bold();

    let normal_text_format = Format::new()
        .set_bold();


    let mut files_excel = Workbook::new();

    let mut row_bias = 2;

    let file = files_excel.add_worksheet().set_name("G1").expect("Error in creation file");

    file.write_with_format(row_bias, 2, "Sabato ... - G1", &h1_format).expect("Error to write in the file");

    row_bias +=2;

    for i in 0..2{
        file.write_with_format(row_bias, 1 +(i * 3), "ORA", &header_table_format).expect("Error to write in the file");
        file.write_with_format(row_bias, 2 + (i * 3), format!("CAMPO {}", i), &header_table_format).expect("Error to write in the file");
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

            if i%2 == 0{
                file.write_with_format(row_bias + i as u32, y + 1, "", &white_line_format).expect("Error to write in the file");
            }else{
                file.write_with_format(row_bias + i as u32, y + 1, "", &light_gray_line_format).expect("Error to write in the file");
            }
        }
    }

    row_bias = 23;

    file.write_with_format(row_bias - 2, 2, "Domenica ... - G1", &h1_format).expect("Error to write in the file");

    for i in 0..2{
        file.write_with_format(row_bias, 1 +(i * 3), "ORA", &header_table_format).expect("Error to write in the file");
        file.write_with_format(row_bias, 2 + (i * 3), format!("CAMPO {}", i), &header_table_format).expect("Error to write in the file");
        file.write_with_format(row_bias, 3 + (i * 3), "TAB.", &header_table_format).expect("Error to write in the file");
    }

    for i in 0..sunday_time.len(){
        for y in 0..6{
            file.set_row_height(i as u32 + row_bias, 35).expect("Error to write in the file");

            if y == 0 && i%2 == 0{
                file.write_with_format(row_bias + i as u32, y + 1, sunday_time[i].clone(), &white_line_format).expect("Error to write in the file");
            }
            else if y == 0 && i%2 != 0{
                file.write_with_format(row_bias + i as u32, y + 1, sunday_time[i].clone(), &light_gray_line_format).expect("Error to write in the file");
            }
            
            if i%2 == 0{
                file.write_with_format(row_bias + i as u32, y + 1, "", &white_line_format).expect("Error to write in the file");
            }else{
                file.write_with_format(row_bias + i as u32, y + 1, "", &light_gray_line_format).expect("Error to write in the file");
            }
        }
    }

    files_excel.save(path).expect("Error to save the file");

}