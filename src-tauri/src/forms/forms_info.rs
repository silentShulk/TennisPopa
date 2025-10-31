use rusqlite_struct_derive::RusqliteStruct;
use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;

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

#[derive(Deserialize, Serialize, Clone)]
pub enum FormType {
    Registration, Availability
}

#[derive(Deserialize, Serialize, Clone)]
pub struct FormInfo {
    pub title: String,
    pub content: String,
    pub form_type: FormType
}

#[tauri::command]
pub async fn get_registration_form() -> FormInfo {
    REGISTRATION_FORM_INFO.clone()
}
#[tauri::command]
pub async fn get_availability_form() -> FormInfo {
    AVAILABILITY_FORM_INFO.clone()
}

#[tauri::command]
pub async fn get_registration_form_type() -> FormType {
    FormType::Registration
}
#[tauri::command]
pub async fn get_availability_form_type() -> FormType {
    FormType::Availability
}

lazy_static! {
    pub static ref REGISTRATION_FORM_INFO: FormInfo = FormInfo {
        title: "Registrazione Torneo Sociale".to_string(),
        content: r#"
        {
        "requests": [
            {
                "createItem": {
                    "item": {
                        "textItem": {},
                        "title": "ATTENZIONE",
                        "description": "L'email inserita verrà utilizzata per identificare univocamente un solo giocatore, se si invia con la stessa email più volte il form, come registrazione verrà presa quella più recente.\nSE PIÙ PERSONE DELLO STESSO NUCLEO FAMIGLIARE VOGLIONO PARTECIPARE UTILIZZARE EMAIL DIVERSE"
                    },
                    "location": { "index": 0 }
                }
            },
            {
                "createItem": {
                    "item": {
                        "title": "Nome e Cognome",
                        "questionItem": {
                            "question": {
                                "required": true,
                                "textQuestion": {
                                    "paragraph": false
                                }
                            }
                        }
                    },
                    "location": { "index": 1 }
                }
            },
            {
                "createItem": {
                    "item": {
                        "title": "Categoria (SE NON CONOSCIUTA SELEZIONARE SCONOSCIUTA)",
                        "questionItem": {
                            "question": {
                                "required": true,
                                "choiceQuestion": {
                                    "type": "RADIO",
                                    "options": [
                                        { "value": "A1" },
                                        { "value": "A2" },
                                        { "value": "B1" },
                                        { "value": "B2" },
                                        { "value": "C" },
                                        { "value": "D" },
                                        { "value": "E" },
                                        { "value": "Sconosciuta"}
                                    ]
                                }
                            }
                        }
                    },
                    "location": { "index": 2 }
                }
            },
            {
                "createItem": {
                    "item": {
                        "title": "Numero di telefono",
                        "questionItem": {
                            "question": {
                                "required": true,
                                "textQuestion": {
                                    "paragraph": false
                                }
                            }
                        }
                    },
                    "location": { "index": 3 }
                }
            },
            {
                "updateSettings": 
                {
                    "settings": 
                    {
                        "emailCollectionType": "VERIFIED"
                    },
                    "updateMask": "emailCollectionType"
                }
            },
            {
                "createItem": {
                    "item": {
                        "title": "Taglia di maglietta",
                        "questionItem": {
                            "question": {
                                "required": true,
                                "choiceQuestion": {
                                    "type": "RADIO",
                                    "options": [
                                        { "value": "XL" },
                                        { "value": "L" },
                                        { "value": "M" },
                                        { "value": "S" },
                                        { "value": "XS" }
                                    ]
                                }
                            }
                        }
                    },
                    "location": { "index": 4 }
                }
            },
            {
                "createItem": {
                    "item": {
                        "textItem": {},
                        "title": "ATTENZIONE",
                        "description": "Entrate nel gruppo WhatsApp per restare aggiornato sugli orari di gioco:\nhttps://chat.whatsapp.com/GO1f8o1PjyB1gnVqfMALQE?mode=wwc"
                    },
                    "location": { "index": 5 }
                }
            }
        ]
    }"#.to_string(),
        form_type: FormType::Registration
    };

    static ref AVAILABILITY_FORM_INFO: FormInfo = FormInfo {
        title: "Preferenze Orarie".to_string(),
        content: r#"
        {
            "requests": [
                {
                    "createItem": {
                        "item": {
                            "title": "Preferenza oraria",
                            "questionItem": {
                                "question": {
                                    "required": false,
                                    "choiceQuestion": {
                                        "type": "CHECKBOX",
                                        "options": [
                                            { "value": "Sabato pomeriggio" },
                                            { "value": "Domenica mattina" },
                                            { "value": "Domenica pomeriggio" },
                                            { "value": "NON DISPONIBILE" }
                                        ]
                                    }
                                }
                            }
                        },
                        "location": { "index": 0 }
                    }
                },
                {
                    "updateSettings": 
                    {
                        "settings": 
                        {
                            "emailCollectionType": "VERIFIED"
                        },
                        "updateMask": "emailCollectionType"
                    }
                }
            ]
        }"#.to_string(),
        form_type: FormType::Availability
    };
}

