use crate::{get_resource, groups::{group::{Group, groups_in_category}, group_match::GroupMatch}, players::Category};
use rusqlite::{Connection, ToSql, types::{FromSql, FromSqlResult, ToSqlOutput, Value, ValueRef}};
use rusqlite_struct::rusqlite_struct_helper::RusqliteStructHelper;
use rusqlite_struct_derive::RusqliteStruct;
use serde::{Deserialize, Serialize};
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
    pub struct CourtSlots: i32 {
        const SATH13 = 1 << 0; // 13:00
        const SATH14 = 1 << 1; // 14:00
        const SATH15 = 1 << 2; // 15:00
        const SATH16 = 1 << 3; // 16:00
        const SATH17 = 1 << 4; // 17:00
        const SATH18 = 1 << 5; // 18:00
        const SATH19 = 1 << 6; // 19:00

        const SUNH08 = 1 << 7;
        const SUNH09 = 1 << 8;
        const SUNH10 = 1 << 9;
        const SUNH11 = 1 << 10;
        const SUNH12 = 1 << 11;
        
        const SUNH13 = 1 << 12;
        const SUNH14 = 1 << 13;
        const SUNH15 = 1 << 14;
        const SUNH16 = 1 << 15;
        const SUNH17 = 1 << 16;
        const SUNH18 = 1 << 17;
        const SUNH19 = 1 << 18;
    }
}

impl CourtSlots {
    /// All defined slots (Saturday + Sunday)
    pub fn all_slots() -> [CourtSlots; 19] {
        [
            Self::SATH13, Self::SATH14, Self::SATH15, Self::SATH16, Self::SATH17, Self::SATH18, Self::SATH19,
            Self::SUNH08, Self::SUNH09, Self::SUNH10, Self::SUNH11, Self::SUNH12, Self::SUNH13, Self::SUNH14,
            Self::SUNH15, Self::SUNH16, Self::SUNH17, Self::SUNH18, Self::SUNH19,
        ]
    }
}

impl Courts {
    /// Iterate active slots for a given court
    pub fn iter_slots(&self, court_value: i32) -> impl Iterator<Item = CourtSlots> {
        let bits = CourtSlots::from_bits_truncate(court_value);
        CourtSlots::all_slots()
            .into_iter()
            .filter(move |slot| bits.contains(*slot))
    }
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, RusqliteStruct)]
pub struct Courts {
    pub id: Option<i32>,
    pub c1: i32,
    pub c2: i32,
    pub c3: i32,
    pub c4: i32,
    pub c6: i32,
    pub cg1: i32,
    pub cg2: i32,
}

impl Courts {
    pub fn get_courts_availabilities() -> Courts {
        let conn = Connection::open(get_resource("databases/courts.db")).unwrap();
        return *conn.get_from_table_struct::<Courts>().unwrap_or_default().last().unwrap();
    }
}

#[tauri::command]
pub fn save_availability_court(c1: i32, c2: i32, c3: i32, c4: i32, c6: i32, cg1: i32, cg2: i32) {
    let courts = Courts {
        id: None,
        c1,
        c2,
        c3,
        c4,
        c6,
        cg1,
        cg2
    };

    let conn = Connection::open(get_resource("databases/courts.db")).expect(format!("Couldn't open database at \"{:?}\".", get_resource("databases/courts.db")).as_str());
    conn.insert_into_table_struct::<Courts>(&courts).expect("Couldn't insert Courts inside of DB.");
}