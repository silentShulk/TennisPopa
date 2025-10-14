use crate::players::{Category, Availability, Size};
use rusqlite_struct_derive::RusqliteStruct;
use serde::{Deserialize, Serialize};
use strsim::jaro;

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