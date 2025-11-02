use std::collections::HashMap;

use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use serde_rusqlite::{from_rows};
use rusqlite_struct::rusqlite_struct_helper::RusqliteStructHelper;

use crate::players::{Category, Player};
use crate::get_resource;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub category: Category,
    pub players:  Vec<Player>
}

impl Group {
    pub const MAX_PLAYERS_IN_GROUP: usize = 4;
}

impl Group {
    pub fn get_from_category(category: Category) -> Vec<Group> {
        let conn = Connection::open(get_resource("databases/players.db")).expect(format!("Couldn't open \"{:?}\"", get_resource("databases/players.db")).as_str());
        //let conn = Connection::open("databases/players.db").expect("Couldn't open \"databases/players.db\".");
        
        let mut stmt = conn.prepare(
            "SELECT id, category FROM PlayerGroup;"
        ).unwrap();
        let rows = stmt.query([]).expect("Coudln't query from PlayerGroup TABLE.");

        // Collect all player groups.
        let all_player_groups = from_rows::<(i32, Category)>(rows)
            .map(|r| r.map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e))))
            .collect::<rusqlite::Result<Vec<_>>>().expect("Couldn't extract PlayerGroups from query.");
        
        let player_groups_ids_in_category: Vec<i32> = all_player_groups
            .iter()
            .filter(|pg| pg.1 == category)
            .map(|pg| pg.0)
            .collect();

        for i in player_groups_ids_in_category.iter(){
            println!("{}", i);
        }

        let mut groups_in_category = vec![];
        let players = conn.get_from_table_struct::<Player>().expect("Couldn't query Player from DB.");
        for pg in player_groups_ids_in_category {
            // Filter players with id_group == pg
            //TODO Fare che i giocatori che hanno l'id group a none non vengono considerati (fare anche che quando ottieni i giocatori li assegna automaticamente ?)
            let players_in_group = players.iter().filter(|p| {
                if p.id_group.is_none() {
                    println!("ID GROUP NONE: {:?}", p); return false;
                }

                p.id_group.unwrap() == pg
            }).cloned().collect();
            groups_in_category.push(Group { category: category, players: players_in_group });
        }

        for g in groups_in_category.iter(){
            println!("category: {:?}", g.category);
            for p in g.players.iter(){
                println!("\t{} {:?}", p.name, p.category);
            }
        }

        return groups_in_category;
    }
}

#[tauri::command]
pub fn create_groups() { 
    let conn = Connection::open(get_resource("databases/players.db")).unwrap();
    let players = conn.get_from_table_struct::<Player>().unwrap();

    let mut players_by_category: HashMap<Category, Vec<Player>> = HashMap::new();
    players.iter()
        .for_each(|p|
             players_by_category
                .entry(p.category)
                .or_insert_with(|| Vec::new())
                .push(p.clone())
    );

    conn.execute("DELETE FROM PlayerGroup;", []).expect("Couldn't clear PlayerGroup");

    let mut player_group_id = 0;
    for (category, players_in_category) in players_by_category.iter_mut() {
        let mut players_in_group = 1;
        for p in players_in_category {
            // Create new group, overwrite if it already exist.
            if players_in_group == 1 {
                player_group_id += 1;
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
            if players_in_group == Group::MAX_PLAYERS_IN_GROUP {
                players_in_group = 1;
            } else {
                players_in_group += 1;
            }
        }

        // Go to next group.
        player_group_id += 1;
    }

    let mut players_edited: Vec<Player> = vec![];
    players_by_category.values().into_iter().for_each(|players| {
        players.iter().for_each(|p| players_edited.push(p.clone()));
    });

    println!("{}", players_edited.len());
    let mut i = 0;
    for p in players_edited {

        println!("{}", i);
        i +=1;
        conn.execute(
                "UPDATE Player
                 SET id_group = ?1
                 WHERE id  = ?2",
            params![p.id_group, p.id.unwrap()],
        ).unwrap();
    }
}

#[tauri::command]
pub fn swap_group_for_players(p1_id: i32, p2_id: i32) -> Result<(), &'static str> {
    println!("sigma");
    if p1_id == p2_id {
        return Err("Non è possibile swappare lo stesso giocatore.");
    }

    let conn = Connection::open(get_resource("databases/players.db")).unwrap();
    let players = conn.get_from_table_struct::<Player>().unwrap();

    let p1_idx = players.iter().position(|p| p.id == Some(p1_id)).ok_or("Giocatore con p1_id non trovato")?;
    let p2_idx = players.iter().position(|p| p.id == Some(p2_id)).ok_or("Giocatore con p2_id non trovato")?;

    if players[p1_idx].category != players[p2_idx].category {
        println!("not sigma");
        return Err("Non è possibile swappare id girone di due giocatori in categorie differenti.");
    }

    conn.execute(
        "UPDATE Player
            SET id_group = ?1
            WHERE id = ?2",
        params![players[p2_idx].id_group, p1_id]
    ).expect("Couldn't update Player.");

    conn.execute(
        "UPDATE Player
            SET id_group = ?1
            WHERE id = ?2",
        params![players[p1_idx].id_group, p2_id]
    ).expect("Couldn't update Player.");
    
    Ok(())
}

#[tauri::command]
pub fn groups_in_category(category: Category) -> Vec<Group> {
    Group::get_from_category(category)
}  
