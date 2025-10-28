
use rusqlite::{Connection, Result};
use rusqlite_struct::{rusqlite_struct_helper::RusqliteStructHelper};
use yup_oauth2::{read_application_secret, InstalledFlowAuthenticator, InstalledFlowReturnMethod};
use reqwest::Client;
use serde_json::json;

use crate::get_resource;
use crate::forms::forms_info::*;

#[tauri::command]
pub async fn create_form(form_info: FormInfo) -> Result<String, String> {
    let secret = read_application_secret(get_resource("secrets/credentials.json")).await.unwrap();
    //let secret = read_application_secret(Path::new("secrets/credentials.json")).await.unwrap();
    let auth = InstalledFlowAuthenticator::builder(secret, InstalledFlowReturnMethod::HTTPRedirect)
        //.persist_tokens_to_disk("secrets/token.json")
        .persist_tokens_to_disk(get_resource("secrets/token.json"))
        .build()
        .await
        .unwrap();

    let token = auth.token(&[
        "https://www.googleapis.com/auth/forms.body"
    ]).await.unwrap();
    //println!("✅ Access token obtained: {}", token.as_str());
    let client = Client::new();

    // Create form
    // -----------
    let form_payload = json!({
        "info": {
            "title": form_info.title
        }
    });

    let res = client
        .post("https://forms.googleapis.com/v1/forms")
        .bearer_auth(token.as_str())
        .json(&form_payload)
        .send()
        .await
        .unwrap();
    let body: serde_json::Value = res.json().await.unwrap();

    // Aggiunta delle domande obbligatorie tramite batchUpdate
    let update_payload: serde_json::Value = serde_json::from_str(&form_info.content)
        .map_err(|e| format!("Failed to parse form content as JSON: {}", e))?;

    let form_id = body.get("formId").and_then(|id| id.as_str()).unwrap();
    let res_update = client
        .post(&format!("https://forms.googleapis.com/v1/forms/{}:batchUpdate", form_id))
        .bearer_auth(token.as_str())
        .json(&update_payload)
        .send()
        .await
        .unwrap();
    

    let body_update: serde_json::Value = res_update.json().await.unwrap();

    let replies = body_update["replies"]
        .as_array()
        .ok_or("Replies non è un array nella risposta di batchUpdate")?;

    // Zippa richieste e risposte; salva solo quelle con createItem/questionId
    let mut question_ids = vec![];
    for rep in replies.iter() {
        if let Some(rep_ci) = rep.get("createItem") {
            if let Some(qids) = rep_ci.get("questionId").and_then(|v| v.as_array()) {
                for qid in qids {
                    if let Some(qid_str) = qid.as_str() {
                        question_ids.push(qid_str);
                    }
                }
            }
        }
    }

    let form_url: String;
    if let Some(url) = body.get("responderUri").and_then(|v| v.as_str()) {
        form_url = url.to_string();
    } else {
        return Err("Errore nell'ottenere link del form.".into());
    }

    let question_ids_joined = question_ids.join("~");
    
    let conn = Connection::open("databases/forms.db").unwrap();
    match form_info.form_type {
        FormType::Registration => {
            conn.insert_into_table_struct(
                &RegistrationFormItems {
                    id: None,
                    is_primary: false,
                    form_url: form_url.clone(),
                    form_id: form_id.to_string(),
                    question_ids: question_ids_joined
                }
            ).unwrap();
        },
        FormType::Availability => {
            conn.insert_into_table_struct(
                &AvailabilityFormItems {
                    id: None,
                    form_url: form_url.clone(),
                    form_id: form_id.to_string(),
                    question_ids: question_ids_joined
                }
            ).unwrap();
        },
    };
    
    return Ok(form_url);
}