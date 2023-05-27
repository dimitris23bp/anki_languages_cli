use log::{debug, error, info, warn};
use reqwest::Body;
use serde_json::Value;
use crate::models::add_notes::AddNotes;
use crate::models::find_notes::FindNotes;
use crate::traits::to_string_json::ToStringJson;

const BASE_URL: &str = "http://localhost:8765";

pub async fn create_notes(add_cards: AddNotes) {
    let note_front_value = add_cards.params.notes.get(0).unwrap().fields.front.as_str();
    if !does_exist(note_front_value).await {
        let response = match send_post(add_cards.to_string_json(), add_cards.action.as_str()).await {
            Ok(response) => response,
            Err(err) => {
                error!("Failed to create notes with error: {}", err);
                return
            }
        };

        debug!("Response is: {:#?}", response);
        if let Some(error) = response.get("error") {
            error!("There was an error with the creation of the notes: {}", error);
        } else {
            info!("Notes were created successfully");
        }
    } else {
        warn!("Note with value: {} already exists", note_front_value);
    }
}


async fn does_exist(query: &str) -> bool {
    let find_notes = FindNotes::new(query);
    let response = match send_post(find_notes.to_string_json(), find_notes.action.as_str()).await {
        Ok(response) => response,
        Err(err) => {
            error!("Failed to query notes: {}", err);
            return false
        }
    };
    debug!("Response from find_notes is: {:#?}", response);
    !response.get("result").unwrap().as_array().unwrap().is_empty()
}

async fn send_post<T: Into<Body>>(body: T, action: &str) -> Result<Value, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client.post(BASE_URL).body(body).send().await?.json::<Value>().await?;

    if let Some(error) = response.get("error") {
        error!("There was an error with {}: {}", action, error);
    } else {
        info!("Notes were created successfully");
    }
    Ok(response)
}