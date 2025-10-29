use crate::{get_resource, groups::{courts::Courts, group::{Group, groups_in_category}, group_match::GroupMatch}, players::Category};
use rusqlite::{Connection, ToSql, types::{FromSql, FromSqlResult, ToSqlOutput, Value, ValueRef}};
use rusqlite_struct::rusqlite_struct_helper::RusqliteStructHelper;
use rusqlite_struct_derive::RusqliteStruct;
use serde::{Deserialize, Serialize};
use bitflags::bitflags;
use crate::courts::*;


#[derive(Debug, Clone, Copy, Serialize, Deserialize, RusqliteStruct)]
struct ScheduledMatch {
    player_1: i32,
    player_2: i32,
    scheduled_time: i32,
    court: i32,
}

struct GroupScheduler();

impl GroupScheduler {
    pub fn schedule_matches_for_all_groups() {
        let mut all_groups: Vec<Group> = vec![];

        for c in Category::all_playable_categories() {
            all_groups.append(&mut Group::get_from_category(c));
        }
        let mut all_matches: Vec<GroupMatch> = vec![];
        for g in all_groups {
            if g.players.len() != Group::MAX_PLAYERS_IN_GROUP {
                continue;
            }

            all_matches.append(&mut g.create_matches());
        }
        let courts_availability = Courts::get_courts_availabilities();
        
        
        // Court busy matrix: 7 courts x 19 timeslots
        let mut court_busy: Vec<Vec<bool>> = vec![vec![false; 19]; 7];
        let mut scheduled_matches: Vec<ScheduledMatch> = vec![];


        let slots = CourtSlots::from_bits_truncate(courts_availability.c1);
        court_busy[0] = CourtSlots::all_slots().into_iter().map(|s| slots.contains(s)).collect::<Vec<bool>>();
        let slots = CourtSlots::from_bits_truncate(courts_availability.c2);
        court_busy[1] = CourtSlots::all_slots().into_iter().map(|s| slots.contains(s)).collect::<Vec<bool>>();
        let slots = CourtSlots::from_bits_truncate(courts_availability.c3);
        court_busy[2] = CourtSlots::all_slots().into_iter().map(|s| slots.contains(s)).collect::<Vec<bool>>();
        let slots = CourtSlots::from_bits_truncate(courts_availability.c4);
        court_busy[3] = CourtSlots::all_slots().into_iter().map(|s| slots.contains(s)).collect::<Vec<bool>>();
        let slots = CourtSlots::from_bits_truncate(courts_availability.c6);
        court_busy[4] = CourtSlots::all_slots().into_iter().map(|s| slots.contains(s)).collect::<Vec<bool>>();
        let slots = CourtSlots::from_bits_truncate(courts_availability.cg1);
        court_busy[5] = CourtSlots::all_slots().into_iter().map(|s| slots.contains(s)).collect::<Vec<bool>>();
        let slots = CourtSlots::from_bits_truncate(courts_availability.cg2);
        court_busy[6] = CourtSlots::all_slots().into_iter().map(|s| slots.contains(s)).collect::<Vec<bool>>();

        for gm in all_matches.iter() {
            let p_avail = gm.p1.availability & gm.p2.availability;
            
            //TODO: do this.

            // Try to insert in court 6.
            if gm.category == Category::E || gm.category == Category::D {
                let mut has_inserted = false;
                for i in 0..court_busy[4].len() {
                    if court_busy[4][i] = true {
                        court_busy[4][i] = false;
                        has_inserted = true;

                        scheduled_matches
                    }
                }
            }
        }
    }
}

#[tauri::command]
pub fn schedule_matches_for_all_players() {
    GroupScheduler::schedule_matches_for_all_groups();
}

#[tauri::command]
pub fn get_all_scheduled_matches() -> Vec<ScheduledMatch>{
    let conn = Connection::open(get_resource("databases/players.db")).unwrap();

    return conn.get_from_table_struct::<ScheduledMatch>().unwrap();
}