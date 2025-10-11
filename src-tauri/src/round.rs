use crate::availability::*;
use crate::player::*;
use std::{collections::HashMap, vec};
use rust_xlsxwriter::*;
use chrono::{Local, DateTime, Utc};

#[derive(Debug, PartialEq, Clone, Copy)]
struct PlayerID(u64);

#[derive(Debug, Clone)]
pub struct RoundPlayer {
    pub player: Player,
    pub id: PlayerID,
    pub to_match_against: u64 // TODO: Aggiungere la logica per sto robo con i bit che significano l'aver giocato con un tipo
}

#[derive(Debug, Clone)]
pub struct Match {
    pub player1: PlayerID,
    pub player2: PlayerID,
    pub timeslot: Availability
}

#[derive(Debug, Clone)]
pub struct MatchingResult {
    pub valid_matches: Vec<Match>,
    pub unassignable_players: Vec<PlayerID>
}

#[derive(Debug, Clone)]
pub struct Round {
    players: Vec<RoundPlayer>,
    category: Category
}

impl RoundPlayer {
    pub fn are_matchable(&self, other: &RoundPlayer) -> bool {
        let PlayerID(p1_id) = self.id;
        let PlayerID(p2_id) = other.id;

        let p1_vs_p2 = self.to_match_against  & (1u64 << p2_id) == 0;
        let p2_vs_p1 = other.to_match_against & (1u64 << p1_id) == 0;

        p1_vs_p2 && p2_vs_p1
    }
}

impl Round {
    pub fn new(players: &Vec<Player>, category: Category) -> Self {
        let round_players: Vec<RoundPlayer> = players
            .iter()
            .enumerate()
            .map(|(i, p)| RoundPlayer{player: p.clone(), id: PlayerID(i as u64), to_match_against: 2u64.pow(i as u32)})
            .collect();

        Round {category, players: round_players}
    }

    pub fn create_rounds(rounds_per_category: &HashMap<Category, Vec<Vec<Player>>>) -> HashMap<Category, Vec<Round>> {
        let mut all_round: HashMap<Category, Vec<Round>> = HashMap::new();
        
        for (category, rounds) in rounds_per_category.iter() {
            let mut r = vec![];

            for round in rounds {
                r.push(Self::new(round, *category));
            }

            all_round.insert(category.clone(), r);
        }

        return all_round;
    }

    //pub fn setup_round_from_save

    /// Returns assigned players for a match and a vector of players who aren't assigned.
    pub fn find_best_matching(&self) -> MatchingResult {
        let mut players_per_timeslot: HashMap<Availability, Vec<RoundPlayer>> = HashMap::new();
        let mut best = MatchingResult {
            valid_matches: vec![],
            unassignable_players: vec![]
        };

        self.match_players(0, &mut players_per_timeslot, &mut best);
        best
    } 

    pub fn set_already_played_against_by_match(&mut self, match_result: &MatchingResult) {
        for m in &match_result.valid_matches {
            let PlayerID(p1_id) = m.player1;
            let PlayerID(p2_id) = m.player2;

            self.players[p1_id as usize].to_match_against |= 1u64 << p2_id;
            self.players[p2_id as usize].to_match_against |= 1u64 << p1_id;
        }
    }

    fn match_players(
        &self, i: usize, players_per_timeslot: &mut HashMap<Availability, Vec<RoundPlayer>>, best: &mut MatchingResult
    ) {
        // Base case.
        if i == self.players.len() {
            let result = Self::evaluate_matching(players_per_timeslot);
            if result.valid_matches.len() > best.valid_matches.len() {
                *best = result;
            }
            return;
        }

        let mut assigned_any_slot = false;
        for slot in self.players[i].player.availability.slots() {
            assigned_any_slot = true;

            // Scopes to avoid borrow checker being angry
            {
                let vec = players_per_timeslot.entry(slot).or_insert_with(Vec::new);
                vec.push(self.players[i].clone());
            }

            self.match_players(i+1, players_per_timeslot, best);

            // Backtrack
            {
                let vec = players_per_timeslot.get_mut(&slot).expect("Slot must exist");
                vec.pop();
            }
        }

        if !assigned_any_slot {
            self.match_players(i+1, players_per_timeslot, best);
        }
    }

    fn max_pairs_in_slot(players: &[RoundPlayer], slot: Availability) -> (Vec<Match>, Vec<PlayerID>) {
        fn helper(players: &[RoundPlayer], used: &mut Vec<bool>, slot: Availability) -> Vec<Match> {
            // Find first unused player
            if let Some(i) = (0..players.len()).find(|&i| !used[i]) {
                let mut best = vec![];

                // Case 1: Leave unmatched
                used[i] = true;
                let res = helper(players, used, slot);
                if res.len() > best.len() {
                    best = res;
                }
                used[i] = false;

                // Case 2: Pair with any other player
                for j in (i+1)..players.len() {
                    if !used[j] && players[i].are_matchable(&players[j]) {
                        used[i] = true;
                        used[j] = true;

                        let mut res = vec![Match {
                            player1: players[i].id,
                            player2: players[j].id,
                            timeslot: slot
                        }];
                        res.extend(helper(players, used, slot));

                        if res.len() > best.len() {
                            best = res;
                        }

                        used[i] = false;
                        used[j] = false;
                    }
                }
                best
            } else {
                vec![]
            }
        }

        let mut used = vec![false; players.len()];
        let matches = helper(players, &mut used, slot);

        let assigned: Vec<PlayerID> = matches.iter()
            .flat_map(|m| [m.player1, m.player2])
            .collect();
        let mut unassigned = vec![];
        for p in players {
            if !assigned.contains(&p.id) {
                unassigned.push(p.id);
            }
        }

        (matches, unassigned)
    }

    fn evaluate_matching(players_per_timeslot: &HashMap<Availability, Vec<RoundPlayer>>) -> MatchingResult {
        let mut result = MatchingResult {
            valid_matches: vec![],
            unassignable_players: vec![]
        };

        for (slot, players) in players_per_timeslot {
            if players.is_empty() {
                continue;
            }
            let (matches, unassigned) = Self::max_pairs_in_slot(players, *slot);
            result.valid_matches.extend(matches.into_iter());
            result.unassignable_players.extend(unassigned.into_iter());
        }

        result
    }

    pub fn create_excel_file (path: String, rounds_per_category: &HashMap<Category, Vec<Round>>) {
        let mut files_excel = Workbook::new();

        let colors: [u32; 7] = [
            0xffff99,
            0xdae9f8,
            0xf7c7ac,
            0xc1f0c8,
            0xf2ceef,
            0xcaedfb,
            0xdaf2d0
        ];

        let permanet_name: [String; 3] = [
            "vittorie".to_string(),
            "punti".to_string(),
            "calssifica".to_string()
        ];

        let mut k = 0;
        let cols_per_print = 21;

        for (category, rounds) in rounds_per_category {

            let header_format = Format::new()
                .set_bold()
                .set_font_size(72)
                .set_background_color(Color::RGB(colors[k]))
                .set_font_color(Color::RGB(0x275317 as u32))
                .set_font_name("Aptos Narrow");

            let text_group_format = Format::new()
                .set_bold()
                .set_font_size(15)
                .set_font_color(Color::RGB(0x275317 as u32))
                .set_background_color(Color::RGB(colors[k]))
                .set_underline(FormatUnderline::Single)
                .set_font_name("Aptos Narrow");

            let player_format = Format::new()
                .set_bold()
                .set_font_size(11)
                .set_border_bottom(FormatBorder::Thin)
                .set_background_color(Color::RGB(colors[k]))
                .set_text_wrap()
                .set_font_name("Aptos Narrow");

            let black_spot = Format::new()
                .set_border(FormatBorder::Thin)
                .set_background_color(Color::Black);

            let table_format = Format::new()
                .set_border(FormatBorder::Thin)
                .set_background_color(Color::RGB(colors[k]));

            let background_color_format = Format::new()
                .set_background_color(Color::RGB(colors[k]));

            let right_table_format = Format::new()
                .set_font_size(9)
                .set_font_name("Aptos Narrow")
                .set_background_color(Color::RGB(colors[k]));


            let file = files_excel.add_worksheet().set_name(category.to_string()).expect("Error in creation file");

            let mut row_bias = 1;
            let mut col_bias = 1;

            let mut row_per_print = ((rounds.len() / 4) * 19) as u32;
            
            if rounds.len() % 4 != 0{
                row_per_print += 19;
            }

            for j in 0..row_per_print{
                for y in 0..cols_per_print{
                    file.write_with_format(j, y, "", &background_color_format).expect("Error to write in the file");
                }
            }
            
            for (i, round) in rounds.iter().enumerate(){

                file.set_column_width(col_bias - 1, 3).expect("Error to write in the file");

                if i % 4 == 0{
                    file.write_with_format(row_bias, 7, format!("{} {}", "Serie ", category.to_string()), &header_format).expect("Error to write in the file");
                    file.set_row_height(row_bias, 90).expect("Error to write in the file");

                    row_bias += 2;
                }
                
                file.write_with_format(row_bias, col_bias, format!("{} {}", "Girone", i), &text_group_format).expect("Error to write in the file");
                file.set_row_height(row_bias, 19).expect("Error to write in the file");

                row_bias += 2;

                for (y, round_player) in round.players.iter().enumerate(){

                    file.write_with_format(row_bias + (y as u32), col_bias, round_player.player.name.replace(" ", "\n"), &player_format).expect("Error to write in the file");
                    file.set_row_height(row_bias + (y as u32), 38).expect("Error to write in the file");
                    file.set_column_width(col_bias, 14).expect("Error to write in the file");
                }
                
                for i in 0..4{
                    for y in 0..4{
                        file.write_with_format(row_bias + i, col_bias + 1 + y, "", &table_format).expect("Error to write in the file");
                        file.set_column_width(col_bias + 1 + y, 6).expect("Error to write in the file");

                        if i == y as u32{
                            file.write_with_format(row_bias + i, col_bias + 1 + y, "", &black_spot).expect("Error to write in the file");
                        }

                    }
                }

                col_bias += 6;
                row_bias -= 1;
                file.set_column_width(col_bias - 1, 3).expect("Error to write in the file");

                for y in 0..3{
                    file.write_with_format(row_bias, col_bias + y, permanet_name[y as usize].clone(), &right_table_format).expect("Error to write in the file");
                    file.set_column_width(col_bias + y, 5).expect("Error to write in the file");
                }

                for i in 0..3{
                    for y in 0..4{
                        file.write_with_format(row_bias + 1 + y, col_bias + i, "", &table_format).expect("Error to write in the file");
                    }
                }
                
                if ((i + 1) as u16)%4 == 0{
                    row_bias += 7;
                    col_bias = 1;
                }
                else if (i as u16)%2 == 0{
                    col_bias += 4;
                    row_bias -= 1;
                }
                else{
                    row_bias += 7;
                    col_bias = 1;
                }
            }

            k += 1;
        }

        files_excel.save(path).expect("Error to save the file");
    }
}