use crate::players::{ Availability, Category, Size};
use rusqlite_struct::rusqlite_struct_helper::RusqliteStructHelper;
use rusqlite_struct_derive::RusqliteStruct;
use serde::{Deserialize, Serialize};
use strsim::jaro;
use rusqlite::{params, Connection};
use tauri::async_runtime::set;

use crate::get_resource;

pub trait VecPlayerFuzzyFinder {
    fn search_by_name(&self, to_search: String) -> Vec<Player>;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, RusqliteStruct)]
pub struct Player {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
    pub phone_number: String,
    pub category: Category,
    pub date_of_creation: String,
    pub availability: Availability,
    pub size: Size,
    pub id_group: Option<i32>
}

impl VecPlayerFuzzyFinder for Vec<Player> {
    fn search_by_name(&self, to_search: String) -> Vec<Player> {
        let mut scored: Vec<(f64, Player)> = self
            .iter()
            .map(|p| (jaro(&p.name.trim().to_lowercase(), &to_search.trim().to_lowercase()), p.clone()))
            .collect();

        scored.sort_by(|a, b| b.0.total_cmp(&a.0));
        scored.into_iter().map(|(_, p)| p).collect()
    }
}

#[tauri::command]
pub fn find_player(name: String) -> Vec<Player>{

    let players_conn = Connection::open(get_resource("databases/players.db")).unwrap();
    //let players_conn = Connection::open("databases/players.db").unwrap();

    let all_player = players_conn.get_from_table_struct::<Player>().unwrap();

    return all_player.search_by_name(name);
}

#[tauri::command]
pub fn update_spec_player(update_player: Player){

    let players_conn = Connection::open(get_resource("databases/players.db")).unwrap();
    //let players_conn = Connection::open("databases/players.db").unwrap();
    
    players_conn.execute(
            "INSERT INTO Player (name, email, phone_number, category, date_of_creation, availability, size, id_group)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
             ON CONFLICT(email) DO UPDATE SET
                name = excluded.name,
                phone_number = excluded.phone_number,
                category = excluded.category,
                date_of_creation = excluded.date_of_creation,
                availability = excluded.availability,
                size = excluded.size,
                id_group = excluded.id_group;",
        params![update_player.name, update_player.email, update_player.phone_number, update_player.category, update_player.date_of_creation, update_player.availability, update_player.size, update_player.id_group],
    ).unwrap();   
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, RusqliteStruct)]
struct PlayerMatch {
    player_1: i32,
    player_2: i32,
    set_1_p1: i32,
    set_1_p2: i32,
    set_2_p1: i32,
    set_2_p2: i32,
    tie_p1: i32,
    tie_p2: i32,
}

#[tauri::command]
pub fn save_match_result(p1_id: i32, p2_id: i32, set1: (i32, i32), set2: (i32, i32), tie: (i32, i32)){
    println!("p1:{} p2:{} {}-{} {}-{} {}-{}", p1_id, p2_id, set1.0, set1.1, set2.0, set2.1, tie.0, tie.1);

    let mut p1 = p1_id;
    let mut p2 = p2_id;

    let (mut set1_p1, mut set1_p2) = set1;
    let (mut set2_p1, mut set2_p2) = set2;
    let (mut tie_p1, mut tie_p2) = tie;

    if p1 == p2 {
        println!("ERRORE: p1_id == p2_id");
    }

    if p2 < p1 {
        std::mem::swap(&mut p1, &mut p2);
        std::mem::swap(&mut set1_p1, &mut set1_p2);
        std::mem::swap(&mut set2_p1, &mut set2_p2);
        std::mem::swap(&mut tie_p1, &mut tie_p2);
    }

    /*let player_match = PlayerMatch {
        player_1: p1, player_2: p2,
        set_1_p1: set1_p1, set_1_p2: set1_p2,
        set_2_p1: set2_p1, set_2_p2: set2_p2,
        tie_p1: tie_p1, tie_p2: tie_p2
    };*/

    let conn = Connection::open(get_resource("databases/players.db")).unwrap();
    conn.execute(
        "INSERT INTO PlayerMatch 
        (player_1, player_2, set_1_p1, set_1_p2, set_2_p1, set_2_p2, tie_p1, tie_p2)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
        ON CONFLICT(player_1, player_2) DO UPDATE SET
            set_1_p1 = excluded.set_1_p1,
            set_1_p2 = excluded.set_1_p2,
            set_2_p1 = excluded.set_2_p1,
            set_2_p2 = excluded.set_2_p2,
            tie_p1 = excluded.tie_p1,
            tie_p2 = excluded.tie_p2;
        ",
        params![p1, p2, set1_p1, set1_p2, set2_p1, set2_p2, tie_p1, tie_p2]
    ).expect("Couldn't write PlayerMatch in to databases");
}