use rust_xlsxwriter::*;
use crate::groups::group_scheduler::PlayerMatch;
use crate::{get_resource, players::*};
use crate::groups::group::*;
use rusqlite::{Connection, OptionalExtension, Result as SqlResult, params};

#[tauri::command]
pub fn create_excel_group(path: String){

    let mut files_excel = Workbook::new();

    let permanet_name: [String; 3] = [
        "vittorie".to_string(),
        "punti".to_string(),
        "calssifica".to_string()
    ];

    let cols_per_print = 21;

    for category in Category::all_playable_categories(){
        let groups = groups_in_category(category);

        let header_format = Format::new()
            .set_bold()
            .set_font_size(72)
            .set_font_color(Color::RGB(0x275317 as u32))
            .set_font_name("Aptos Narrow");

        let text_group_format = Format::new()
            .set_bold()
            .set_font_size(15)
            .set_font_color(Color::RGB(0x275317 as u32))
            .set_underline(FormatUnderline::Single)
            .set_font_name("Aptos Narrow");

        let player_format = Format::new()
            .set_bold()
            .set_font_size(11)
            .set_border_bottom(FormatBorder::Thin)
            .set_text_wrap()
            .set_font_name("Aptos Narrow");

        let black_spot = Format::new()
            .set_border(FormatBorder::Thin)
            .set_background_color(Color::Black);

        let table_format = Format::new()
            .set_align(FormatAlign::Center)
            .set_border(FormatBorder::Thin);

        let background_color_format = Format::new();

        let right_table_format = Format::new()
            .set_font_size(9)
            .set_font_name("Aptos Narrow");

        let conn = Connection::open(get_resource("databases/players.db")).unwrap();


        let file = files_excel.add_worksheet().set_name(category.to_string()).expect("Error in creation file");

        let mut row_bias = 1;
        let mut col_bias = 1;

        let mut row_per_print = ((groups.len() / 4) * 19) as u32;
        
        if groups.len() % 4 != 0{
            row_per_print += 19;
        }

        for j in 0..row_per_print{
            for y in 0..cols_per_print{
                file.write_with_format(j, y, "", &background_color_format).expect("Error to write in the file");
            }
        }
        
        for (i, group) in groups.iter().enumerate(){

            if group.players.len() == 4{
                file.set_column_width(col_bias - 1, 3).expect("Error to write in the file");

                if i % 4 == 0{
                    file.write_with_format(row_bias, 7, format!("{} {}", "Serie ", category.to_string()), &header_format).expect("Error to write in the file");
                    file.set_row_height(row_bias, 90).expect("Error to write in the file");

                    row_bias += 2;
                }
                
                file.write_with_format(row_bias, col_bias, format!("{} {}", "Girone", i), &text_group_format).expect("Error to write in the file");
                file.set_row_height(row_bias, 19).expect("Error to write in the file");

                row_bias += 2;

                for (y, round_player) in group.players.iter().enumerate(){

                    file.write_with_format(row_bias + (y as u32), col_bias, round_player.name.replace(" ", "\n"), &player_format).expect("Error to write in the file");
                    file.set_row_height(row_bias + (y as u32), 40).expect("Error to write in the file");
                    file.set_column_width(col_bias, 14).expect("Error to write in the file");
                }
                
                for i in 0..4{

                    let player_1 = group.players[i].clone();

                    for y in 0..4{

                        let player_2 = group.players[y].clone();

                        todo!("PLAYER MATCH");
                        //let player_match = ;

                        //file.write_with_format(row_bias + i as u32, col_bias + 1 + y as u16, format!("{}-{} \n {}-{} \n {}-{}", player_match.set_1_p1, player_match.set_1_p2, player_match.set_2_p1, player_match.set_2_p2, player_match.tie_p1, player_match.tie_p2), &table_format).expect("Error to write in the file");
                        file.set_column_width(col_bias + 1 + y as u16, 6).expect("Error to write in the file");

                        if i as u32 == y as u32{
                            file.write_with_format(row_bias + i as u32, col_bias + 1 + y as u16, "", &black_spot).expect("Error to write in the file");
                        }

                    }
                }

                col_bias += 6;
                row_bias -= 1;
                file.set_column_width(col_bias - 1, 3).expect("Error to write in the file");

                for y in 0..3{
                    file.write_with_format(row_bias, col_bias + y, permanet_name[y as usize].clone(), &right_table_format).expect("Error to write in the file");
                    file.set_column_width(col_bias + y, 5).expect("Error to write in the file");
                }

                for i in 0..3{
                    for y in 0..4{
                        file.write_with_format(row_bias + 1 + y, col_bias + i, "", &table_format).expect("Error to write in the file");
                    }
                }
                
                if ((i + 1) as u16)%4 == 0{
                    row_bias += 7;
                    col_bias = 1;
                }
                else if (i as u16)%2 == 0{
                    col_bias += 4;
                    row_bias -= 1;
                }
                else{
                    row_bias += 7;
                    col_bias = 1;
                }
            }
        }
    }
    files_excel.save(path).expect("Error to save the file");
}