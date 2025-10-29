use crate::{get_resource, groups::{courts::Courts, group::{Group, groups_in_category}, group_match::GroupMatch}, players::{Availability, Category}};
use rusqlite::{Connection, ToSql, types::{FromSql, FromSqlResult, ToSqlOutput, Value, ValueRef}};
use rusqlite_struct::rusqlite_struct_helper::RusqliteStructHelper;
use rusqlite_struct_derive::RusqliteStruct;
use serde::{Deserialize, Serialize};
use bitflags::bitflags;
use crate::courts::*;
use crate::players::Player;


#[derive(Debug, Clone, Serialize, Deserialize, RusqliteStruct)]
struct ScheduledMatch {
    player_1: Player,
    player_2: Player,
    scheduled_time: CourtSlots,
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
        let courts = Courts::get_courts_availabilities();
        
        let mut court_availability: Vec<i32> = vec![
            courts.c1, courts.c2, courts.c3, courts.c4, courts.c6, courts.cg1, courts.cg2
        ];
        let mut scheduled_matches: Vec<ScheduledMatch> = vec![];
    
        for gm in all_matches {
            println!("MATCH: {}-{}", gm.p1.id.unwrap(), gm.p2.id.unwrap());

            let match_availability = (gm.p1.availability & gm.p2.availability).bits();

            let match_availability_expanded = 
                GroupScheduler::expand_bits_player_avail(match_availability);


            let mut order = [0, 1, 2, 3, 4, 5, 6];
            if gm.category == Category::E || gm.category == Category::D {
                order = [4, 0, 1, 2, 3, 5, 6];
            }
            for i in order {
                let combined = court_availability[i] & match_availability_expanded;
                if combined == 0 {
                    continue;
                }
                
                let first_available = GroupScheduler::highest_set_bit(combined);
                if first_available == 0 {
                    println!("Error in first available {:032b}", combined);
                    continue;
                }

                let first_slot = CourtSlots::from_bits_truncate(first_available);

                court_availability[i] ^= first_available;

                scheduled_matches.push(ScheduledMatch {
                     player_1: gm.p1.clone(), player_2: gm.p2.clone(), scheduled_time: first_slot, court: i as i32
                }); 
            }
        }

        // TEST!
        for sm in scheduled_matches {
            let a = sm.player_1.availability & sm.player_2.availability;

            let match_availability_expanded = 
                    GroupScheduler::expand_bits_player_avail(a.bits());
            if CourtSlots::from_bits_truncate(match_availability_expanded).contains(sm.scheduled_time) {
                println!("{}) {}-{} {:?} - {:?}", sm.court, sm.player_1.id.unwrap(), sm.player_2.id.unwrap(), CourtSlots::from_bits_truncate(match_availability_expanded), sm.scheduled_time);
            }
        }

        /*let conn = Connection::open(get_resource("databases/players.db")).unwrap();
        conn.execute("DELETE FROM ScheduledMatch;", []).unwrap();

        for sm in scheduled_matches {
            conn.insert_into_table_struct(&sm).unwrap();
        }*/
    }

    fn expand_bits_player_avail(flag: i32) -> i32 {
        let x = (flag as u32) & 0b111; // Keep only lowest 3 bits

        // Extract bits
        let a = ((x >> 2) & 1) * 0b111_1111;        // 7 ones  = 127
        let b = ((x >> 1) & 1) * 0b1_1111;           // 5 ones  = 31
        let c = (x & 1) * 0b111_1111;                // 7 ones  = 127

        // Pack: A at [12..19), B at [7..12), C at [0..7)
        ((a << 12) | (b << 7) | c) as i32
    }

    fn highest_set_bit(v: i32) -> i32 {
    let u = v as u32;
    if u == 0 {
        return 0;
    }
    let pos = 31 - u.leading_zeros();           // position of highest 1
    1i32 << pos                                 // 1 << pos
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