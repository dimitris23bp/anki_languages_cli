use std::fmt::Debug;
use log::{debug, error, info, warn};
use reqwest::Body;
use serde_json::Value;
use async_recursion::async_recursion;
use crate::models::add_notes::AddNotes;
use crate::models::create_deck::CreateDeck;
use crate::models::deck_name::DeckName;
use crate::models::find_notes::FindNotes;
use crate::traits::to_string_json::ToStringJson;

const BASE_URL: &str = "http://localhost:8765";

#[async_recursion]
pub async fn create_notes(add_cards: AddNotes) {
    let deck_name = &add_cards.params.notes.get(0).unwrap().deck_name;
    let note_front_value = add_cards.params.notes.get(0).unwrap().fields.front.as_str();

    create_deck_if_not_exist(deck_name).await;
    if !note_exists(deck_name, note_front_value).await {
        debug!("Note doesn't already exist");
        match send_post(add_cards.to_string_json(), add_cards.action.as_str()).await {
            Ok(response) => {
                if already_exists(&response) {
                    info!("Note already exists in another deck.");
                    if let Ok(value) = add_space_to_note(add_cards) {
                        info!("Re-creation of the note will take place now.");
                        create_notes(value).await;
                    } else {
                        error!("Too many similar notes in different decks, cannot create another one");
                    }
                } else {
                    info!("Notes were created successfully");
                }
                response
            },
            Err(err) => {
                error!("Failed to create notes with error: {}", err);
                return
            }
        };

    } else {
        warn!("Note with value: {} already exists", note_front_value);
    }
}

fn add_space_to_note(mut add_note: AddNotes) -> Result<AddNotes, String>  {
    let front_value: &str = &add_note.params.notes.get(0).unwrap().fields.front;
    let trimmed_value = front_value.trim_end();
    if front_value.len() - trimmed_value.len() > 5 {
        return Err(String::from("Too many instances"))
    }

    let upcoming_value = String::from(front_value) + " ";
    add_note.params.notes.get_mut(0).unwrap().fields.front = String::from(upcoming_value);
    Ok(add_note)
}

fn already_exists(response: &Value) -> bool {
    response.get("result").unwrap().as_array().unwrap().get(0).unwrap().is_null()
}

async fn create_deck_if_not_exist(deck_name: &str) {
    let body = DeckName::new();
    if !deck_exists(body, deck_name).await {
       create_deck(deck_name).await;
        info!("Deck is created");
        return
    }
    debug!("Deck already exists");
}

async fn create_deck(deck_name: &str) {
    let body = CreateDeck::new(deck_name.to_string());
    send_post(body.to_string_json(), body.action.as_str()).await.unwrap();
}

async fn deck_exists(body: DeckName, deck_name: &str) -> bool {
    // TODO: Do them as the rest and add a common function to call with err message
    let response = send_post(body.to_string_json(), body.action.as_str()).await.unwrap();
    response.get("result").unwrap().as_object().unwrap().get(deck_name).is_some()
}

async fn note_exists(deck_name: &str, note: &str) -> bool {
    let query = format!("deck:{} {}", deck_name, note);
    let find_notes = FindNotes::new(query.as_str());
    let response = match send_post(find_notes.to_string_json(), find_notes.action.as_str()).await {
        Ok(response) => response,
        Err(err) => {
            error!("Failed to query notes: {}", err);
            return false
        }
    };
    !response.get("result").unwrap().as_array().unwrap().is_empty()
}

async fn send_post<T: Into<Body> + Debug>(body: T, action: &str) -> Result<Value, reqwest::Error> {
    debug!("Body is :{:#?}", body);
    let client = reqwest::Client::new();
    let response = client.post(BASE_URL).body(body).send().await?.json::<Value>().await?;
    debug!("Response is: {:#?}", response);

    if let Some(error) = response.get("error").filter(|&error| !error.is_null()) {
            error!("There was an error with {}: {}", action, error);
    }
    Ok(response)
}