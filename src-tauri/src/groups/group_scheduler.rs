use crate::{get_resource, groups::{courts::Courts, group::{Group, groups_in_category}, group_match::GroupMatch}, players::Category};
use rusqlite::{Connection, ToSql, types::{FromSql, FromSqlResult, ToSqlOutput, Value, ValueRef}};
use rusqlite_struct::rusqlite_struct_helper::RusqliteStructHelper;
use rusqlite_struct_derive::RusqliteStruct;
use serde::{Deserialize, Serialize};
use bitflags::bitflags;


#[derive(Debug, Clone, Copy, Serialize, Deserialize, RusqliteStruct)]
struct ScheduledMatch {
    player_1: i32,
    player_2: i32,
    scheduled_time: i32,
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

        let court_availabilities = Courts::get_courts_availabilities();
        
        // Court number 6 is primarily for Category E and D.

    }   
}

#[tauri::command]
pub fn schedule_matches_for_all_players() {
    GroupScheduler::schedule_matches_for_all_groups();
}