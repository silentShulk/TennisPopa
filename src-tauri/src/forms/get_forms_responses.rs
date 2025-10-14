use std::{collections::HashMap, error::Error, path::Path, vec};
use chrono::{DateTime, Utc};
use yup_oauth2::{read_application_secret, AccessToken, InstalledFlowAuthenticator, InstalledFlowReturnMethod};
use reqwest::Client;
use rusqlite::{params, Connection, Result};
use rusqlite_struct::{rusqlite_struct_helper::RusqliteStructHelper};
use rusqlite_struct_derive::RusqliteStruct;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::forms::forms_info::FormType;
use crate::players::{Availability, Player, player::VecPlayerFuzzyFinder};

#[derive(Debug, Clone, Serialize, Deserialize, RusqliteStruct)]
pub struct RegistrationFormItems {
    pub id: Option<i32>,
    pub is_primary: bool,
    pub form_url: String,
    pub form_id: String,
    pub question_ids: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, RusqliteStruct)]
pub struct AvailabilityFormItems {
    pub id: Option<i32>,
    pub form_url: String,
    pub form_id: String,
    pub question_ids: String
}

#[tauri::command]
pub async fn main_get_forms_responses(form_type: FormType) -> Result<(), String> {
    let secret = read_application_secret(Path::new("secrets/credentials.json")).await.unwrap();
    let auth = InstalledFlowAuthenticator::builder(secret, InstalledFlowReturnMethod::HTTPRedirect)
        .persist_tokens_to_disk("secrets/token.json")
        .build()
        .await
        .unwrap();

    let token = auth.token(&[
        "https://www.googleapis.com/auth/forms.responses.readonly"
    ]).await.unwrap();
    let client = Client::new();

    match form_type {
        FormType::Registration => save_registration_responses(&client, &token).await.unwrap(),
        FormType::Availability => save_availability_responses(&client, &token).await.unwrap()
    }

    Ok(())
}

async fn save_registration_responses(client: &Client, token: &AccessToken) -> Result<(), String> {
    println!("regi");
    
    let forms_conn = Connection::open("databases/forms.db").unwrap();
    
    let registration_forms = forms_conn.get_from_table_struct::<RegistrationFormItems>().unwrap(); 
    let primary_registration_form_opt = registration_forms.iter().find(|&x| x.is_primary);

    if primary_registration_form_opt.is_none() {
        return Err("Cannot find a primary RegistrationFormItem.".into());
    }
    let primary_registration_form = primary_registration_form_opt.unwrap();

    let form_id = &primary_registration_form.form_id;
    let question_ids: Vec<&str> = primary_registration_form.question_ids.split("~").collect();

    let url = format!("https://forms.googleapis.com/v1/forms/{}/responses", form_id);
    let res = client
        .get(&url)
        .bearer_auth(token.as_str())
        .send()
        .await
        .unwrap();
    let data: Value = res.json().await.unwrap();

    // Get players from form
    // ---------------------
    let mut player_registrations: Vec<Player> = vec![];

    if let Some(responses) = data["responses"].as_array() {
        for r in responses {
            let category_str = r["answers"][&question_ids[1]]["textAnswers"]["answers"][0]["value"].as_str().unwrap_or("").trim();
            let size_str     = r["answers"][&question_ids[3]]["textAnswers"]["answers"][0]["value"].as_str().unwrap_or("").trim();

            let category = category_str.parse();
            if category.is_err() {
                return Err("Failed to convert to Category.".into());
            }

            let size = size_str.parse();
            if size.is_err() {
                return Err("Failed to convert Size.".into());
            }

            player_registrations.push(
                Player {
                    id:               None,
                    name:             r["answers"][&question_ids[0]]["textAnswers"]["answers"][0]["value"].as_str().unwrap_or("").trim().to_string(),
                    category:         category.unwrap(),
                    phone_number:     r["answers"][&question_ids[2]]["textAnswers"]["answers"][0]["value"].as_str().unwrap_or("").trim().to_string(),
                    email:            r["respondentEmail"].as_str().unwrap_or("").to_string(),
                    date_of_creation: r["createTime"].as_str().unwrap_or("").to_string(),
                    availability:     Availability::all(),
                    size:             size.unwrap(),
                    id_group:         None
                }
            );
        }
    }

    // Filter out duplicated players (same email) based on time of registration
    // ------------------------------------------------------------------------
    let mut latest: HashMap<String, Player> = HashMap::new();

    // Filter
    for r in player_registrations.iter() {
        latest
            .entry(r.email.clone())
            .and_modify(|existing| {
                let r_time = DateTime::parse_from_rfc3339(&r.date_of_creation).unwrap().with_timezone(&Utc);
                let e_time = DateTime::parse_from_rfc3339(&existing.date_of_creation).unwrap().with_timezone(&Utc);
                if r_time > e_time {
                    *existing = r.clone();
                }
            })
            .or_insert(r.clone());
    }
    player_registrations = latest.into_values().collect();
    
    for (i, p) in player_registrations.search_by_name("Eric Bona".to_string()).iter().enumerate() {
        println!("{i}: {:?}", p.name);
    }

    // Push into DB
    let players_conn = Connection::open("databases/players.db").unwrap();
    for pr in player_registrations.iter() {
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
            params![pr.name, pr.email, pr.phone_number, pr.category, pr.date_of_creation, pr.size, pr.id_group],
        ).unwrap();
    }

    Ok(())
     
}

/// IMPORTANT!
/// When calling this function make sure to warn the user that it will reset all availabilities for all players!
async fn save_availability_responses(client: &Client, token: &AccessToken) -> Result<(), Box<dyn Error>> {
    println!("dispo");
    
    let forms_conn = Connection::open("databases/forms.db")?;
    
    let availability_forms = forms_conn.get_from_table_struct::<AvailabilityFormItems>()?; 
    let primary_availability_form_opt = availability_forms.iter().find(|&x| x.id.unwrap_or(0) == availability_forms.len() as i32);

    if primary_availability_form_opt.is_none() {
        return Err("Cannot find a primary AvailabilityFormItem.".into());
    }
    let primary_availability_form = primary_availability_form_opt.unwrap();

    let form_id = &primary_availability_form.form_id;
    let question_ids: Vec<&str> = primary_availability_form.question_ids.split("~").collect();

    let url = format!("https://forms.googleapis.com/v1/forms/{}/responses", form_id);
    let res = client
        .get(&url)
        .bearer_auth(token.as_str())
        .send()
        .await?;
    let data: Value = res.json().await?;

    // Map question text to powers of 2
    let mut options_codes: HashMap<&str, i32> = HashMap::new();
    options_codes.insert("Sabato pomeriggio", 1);
    options_codes.insert("Domenica mattina", 2);
    options_codes.insert("Domenica pomeriggio", 4);

    #[derive(Serialize, Clone)]
    struct TimePreferenceData {
        email: String,
        date_of_creation: String,
        availability: i32
    }

    let mut time_preferences: Vec<TimePreferenceData> = vec![];
    if let Some(responses) = data["responses"].as_array() {
        for r in responses {
            let email = r["respondentEmail"].as_str().unwrap_or("").to_string();
            // Calculate availability.
            let mut avail_code = 0;
            if let Some(multi_answers) = r["answers"][&question_ids[0]]["textAnswers"]["answers"].as_array() {
                for ans in multi_answers {
                    if let Some(val) = ans["value"].as_str() {
                        if let Some(&bit) = options_codes.get(val.trim()) {
                            avail_code |= bit;
                        }
                    }
                }
            }
            let date_of_creation = r["createTime"].as_str().unwrap_or("").to_string();

            time_preferences.push(
                TimePreferenceData { email, availability: avail_code, date_of_creation}
            );
        }
    }

    // Filter out duplicated availabilities (same email) based on time of registration
    // ------------------------------------------------------------------------
    let mut latest: HashMap<String, TimePreferenceData> = HashMap::new();

    // Filter
    for r in time_preferences.iter() {
        latest
            .entry(r.email.clone())
            .and_modify(|existing| {
                let r_time = DateTime::parse_from_rfc3339(&r.date_of_creation).unwrap().with_timezone(&Utc);
                let e_time = DateTime::parse_from_rfc3339(&existing.date_of_creation).unwrap().with_timezone(&Utc);
                if r_time > e_time {
                    *existing = r.clone();
                }
            })
            .or_insert(r.clone());
    }
    time_preferences = latest.into_values().collect();

    // Reset all players' availabilities to default,
    // Then set the availabilities gotten from form for those who sent it.
    // -------------------------------------------------------------------
    let players_conn = Connection::open("databases/players.db")?;
    players_conn.execute("UPDATE Player SET availability = ?1", [Availability::all()])?;

    for tp in time_preferences.iter() {
        players_conn.execute("UPDATE Player SET availability = ?1 WHERE email = ?2", params![tp.availability, tp.email])?;
    }

    for p in players_conn.get_from_table_struct::<Player>().unwrap() {
        println!("{:?}\n", p.availability);
    }
    
    Ok(())
}