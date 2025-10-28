use std::collections::VecDeque;
use rusqlite::{params, Connection, Result};

use crate::get_resource;
use crate::players::{Availability, Category, Player, Size};
use crate::group::Group;


#[derive(Debug, Clone)]
pub struct GroupMatch {
    p1: Player,
    p2: Player,
    category: Category
}

#[tauri::command]
pub fn test_matching() {

    // 0: SatAft, 1: SunMor, 2: SunAft, 3: SunAft, 4: SatAft, 5: SunMor, 6: SatAft
    // 0 e 6 hanno già giocato
    let group = Group {
        category: Category::A1,
        players: vec![
            Player {
                id: Some(0),
                name: "a".into(),
                email: "a".into(),
                phone_number: "a".into(),
                category: Category::A1,
                date_of_creation: "a".into(),
                availability: Availability::SatAft,
                size: Size::XL,
                id_group: None,
            },
            Player {
                id: Some(1),
                name: "a".into(),
                email: "a".into(),
                phone_number: "a".into(),
                category: Category::A1,
                date_of_creation: "a".into(),
                availability: Availability::SunMor,
                size: Size::XL,
                id_group: None,
            },
            Player {
                id: Some(2),
                name: "a".into(),
                email: "a".into(),
                phone_number: "a".into(),
                category: Category::A1,
                date_of_creation: "a".into(),
                availability: Availability::SunAft,
                size: Size::XL,
                id_group: None,
            },
            Player {
                id: Some(3),
                name: "a".into(),
                email: "a".into(),
                phone_number: "a".into(),
                category: Category::A1,
                date_of_creation: "a".into(),
                availability: Availability::SunAft,
                size: Size::XL,
                id_group: None,
            },
            Player {
                id: Some(4),
                name: "a".into(),
                email: "a".into(),
                phone_number: "a".into(),
                category: Category::A1,
                date_of_creation: "a".into(),
                availability: Availability::SatAft,
                size: Size::XL,
                id_group: None,
            },
            Player {
                id: Some(5),
                name: "a".into(),
                email: "a".into(),
                phone_number: "a".into(),
                category: Category::A1,
                date_of_creation: "a".into(),
                availability: Availability::SunMor,
                size: Size::XL,
                id_group: None,
            },
            Player {
                id: Some(6),
                name: "a".into(),
                email: "a".into(),
                phone_number: "a".into(),
                category: Category::A1,
                date_of_creation: "a".into(),
                availability: Availability::SatAft,
                size: Size::XL,
                id_group: None,
            },
        ]
    };


    let m = group.create_matches();
    for ma in m {
        println!("({},{})", ma.p1.id.unwrap(), ma.p2.id.unwrap());
    }

}
impl Group {
    fn is_available(&self, p: &(i32, i32), conn: &Connection) -> bool {
        let mut p1_id = self.players[p.0 as usize].id.unwrap();
        let mut p2_id = self.players[p.1 as usize].id.unwrap();

        // p1_id must be less than p2_id
        if p2_id < p1_id {
            std::mem::swap(&mut p1_id, &mut p2_id);
        }

        // Check if they have already played against.
        let mut stmt = conn
            .prepare("SELECT 1 FROM PlayerMatch WHERE player_1 = ?1 AND player_2 = ?2 LIMIT 1")
            .unwrap();
        let have_played = stmt.exists([p1_id, p2_id]).unwrap();

        // Check if they have compatible availabilities.
        let have_compatible_availabilities = 
            (self.players[p.0 as usize].availability & self.players[p.1 as usize].availability).bits() != 0;

        return !have_played && have_compatible_availabilities;
    }

    pub fn create_matches(&self) -> Vec<GroupMatch> {
        let conn = Connection::open(get_resource("databases/players.db")).expect("Couldn't open players.db");

        let mut possible_pairings: Vec<(i32, i32)> = vec![];
        for i in 0..self.players.len() {
            for j in (i+1)..self.players.len() {
                possible_pairings.push((i as i32, j as i32));
            }
        }

        let mut best_selection: Vec<(i32, i32)> = vec![];
        for start_idx in 0..possible_pairings.len() {
            let mut pairs: VecDeque<(i32, i32)> = possible_pairings.clone().into();
            let curr = pairs.remove(start_idx).unwrap();

            let mut selected = vec![];
            if self.is_available(&curr, &conn) {
                selected.push(curr);
            }

            // Remove all pairs with elements of curr. 
            pairs.retain(|p| p.0 != curr.0 && p.1 != curr.0 && p.0 != curr.1 && p.1 != curr.1);

            while let Some(curr) = pairs.pop_front() {
                if !self.is_available(&curr, &conn) {
                    continue;
                }

                selected.push(curr);
                // Remove all pairs with elements of curr. 
                pairs.retain(|p| p.0 != curr.0 && p.1 != curr.0 && p.0 != curr.1 && p.1 != curr.1);
            }

            if selected.len() > best_selection.len() {
                best_selection = selected;
            }
        }
        
        return best_selection
                .iter()
                .map(|(i, j)| GroupMatch { p1: self.players[*i as usize].clone(), p2: self.players[*j as usize].clone(), category: self.category})
                .collect()
    }
}