use crate::players::{self, player, Availability, Category, Size};
use rusqlite_struct::rusqlite_struct_helper::RusqliteStructHelper;
use rusqlite_struct_derive::RusqliteStruct;
use serde::{Deserialize, Serialize};
use strsim::jaro;
use rusqlite::{params, Connection, Result};
use std::{collections::HashMap, vec};

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

    let players_conn = Connection::open("databases/players.db").unwrap();
    
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

#[tauri::command]
pub fn create_groups() { 
    let conn = Connection::open("databases/players.db").unwrap();
    let players = conn.get_from_table_struct::<Player>().unwrap();

    let mut players_by_category: HashMap<Category, Vec<Player>> = HashMap::new();
    players.iter()
        .for_each(|p|
             players_by_category
                .entry(p.category)
                .or_insert_with(|| Vec::new())
                .push(p.clone())
    );    

    const MAX_PLAYERS_IN_GROUP: i32 = 4;

    let mut player_group_id = 1;
    for (category, players_in_category) in players_by_category.iter_mut() {
        let mut players_in_group = 1;
        for p in players_in_category {
            // Create new group, overwrite if it already exist.
            if players_in_group == 1 {
                conn.execute(
                    "INSERT INTO PlayerGroup (id, category)
                        VALUES (?1, ?2)
                        ON CONFLICT(id) DO UPDATE SET
                            category = excluded.category",
                            params![player_group_id, category]
                ).unwrap();
            }

            // Set current player's group id.
            p.id_group = Some(player_group_id);

            // Go to next group id if group is full.
            if players_in_group == MAX_PLAYERS_IN_GROUP {
                players_in_group = 1;
                player_group_id += 1;
            } else {
                players_in_group += 1;
            }
        }

        // Go to next group only if this group wasn't filled, to avoid different categories in same group.
        if players_in_group != 1 {
            player_group_id += 1;
        }
    }

    let mut players_edited: Vec<Player> = vec![];
    players_by_category.values().into_iter().for_each(|players| {
        players.iter().for_each(|p| players_edited.push(p.clone()));
    });

    for p in players_edited {
        conn.execute(
                "UPDATE Player
                 SET id_group = ?1
                 WHERE id  = ?2",
            params![p.id_group, p.id.unwrap()],
        ).unwrap();
    }
}


#[derive(Debug, Clone, Copy, Serialize, Deserialize, RusqliteStruct)]
struct Courts {
    id: Option<i32>,
    c1_sat: u32,
    c1_sun: u32,
    c2_sat: u32,
    c2_sun: u32,
    c3_sat: u32,
    c3_sun: u32,
    c4_sat: u32,
    c4_sun: u32,
    c6_sat: u32,
    c6_sun: u32,    
    cg1_sat: u32,
    cg1_sun: u32,
    cg2_sat: u32,
    cg2_sun: u32,
}

#[tauri::command]
pub fn save_availability_court(c1: (u32, u32), c2: (u32, u32), c3: (u32, u32), c4: (u32, u32), c6: (u32, u32), cg1: (u32, u32), cg2: (u32, u32)) {
    let courts = Courts {
        id: None,
        c1_sat: c1.0,
        c1_sun: c1.1,
        c2_sat: c2.0,
        c2_sun: c2.1,
        c3_sat: c3.0,
        c3_sun: c3.1,
        c4_sat: c4.0,
        c4_sun: c4.1,
        c6_sat: c6.0,
        c6_sun: c6.1,
        cg1_sat: cg1.0,
        cg1_sun: cg1.1,
        cg2_sat: cg2.0,
        cg2_sun: cg2.1,
    };

    let conn = Connection::open("databases/courts.db").expect("Couldn't open database at \"databases/courts.db\".");
    conn.insert_into_table_struct::<Courts>(&courts).expect("Couldn't insert Courts inside of DB.");
}