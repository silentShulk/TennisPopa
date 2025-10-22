use crate::players::{player, Availability, Category, Size};
use rusqlite_struct::rusqlite_struct_helper::RusqliteStructHelper;
use rusqlite_struct_derive::RusqliteStruct;
use serde::{Deserialize, Serialize};
use strsim::jaro;
use rusqlite::{params, Connection, Result};

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
    let players_conn = Connection::open("databases/players.db").unwrap();

    let all_player = players_conn.get_from_table_struct::<Player>().unwrap();

    return all_player.search_by_name(name);
}

#[tauri::command]
pub fn update_spec_player(update_player: Player){
    println!("{:?}", update_player);

    let players_conn = Connection::open("databases/players.db").unwrap();
    /* 
    players_conn.execute(
            "INSERT INTO Player (name, email, phone_number, category, date_of_creation, size, id_group)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
             ON CONFLICT(email) DO UPDATE SET
                name = excluded.name,
                phone_number = excluded.phone_number,
                category = excluded.category,
                date_of_creation = excluded.date_of_creation,
                size = excluded.size,
                id_group = excluded.id_group;",
        params![update_player.name, update_player.email, update_player.phone_number, update_player.category, update_player.date_of_creation, update_player.size, update_player.id_group],
    ).unwrap();
    */
}