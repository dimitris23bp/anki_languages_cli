use std::fmt::Debug;

use async_recursion::async_recursion;
use log::{debug, error, info, warn};
use reqwest::Body;
use serde_json::Value;

use crate::models::add_notes::AddNotes;
use crate::models::create_deck::CreateDeck;
use crate::models::deck_name::DeckName;
use crate::models::find_notes::FindNotes;
use crate::traits::to_string_json::ToStringJson;

#[async_recursion]
pub async fn create_notes(add_cards: AddNotes, base_url: &str) {
    let deck_name = &add_cards.params.notes.get(0).unwrap().deck_name;
    let note_front_value = add_cards.params.notes.get(0).unwrap().fields.front.as_str();

    create_deck_if_not_exist(deck_name, base_url).await;
    if !note_exists(deck_name, note_front_value, base_url).await {
        debug!("Note doesn't already exist");
        match send_post(add_cards.to_string_json(), add_cards.action.as_str(), base_url).await {
            Ok(response) => {
                if already_exists(&response) {
                    info!("Note already exists in another deck.");
                    if let Ok(value) = add_space_to_note(add_cards) {
                        info!("Re-creation of the note will take place now.");
                        create_notes(value, base_url).await;
                    } else {
                        error!("Too many similar notes in different decks, cannot create another one");
                    }
                } else {
                    info!("Notes were created successfully");
                }
                response
            }
            Err(err) => {
                error!("Failed to create notes with error: {}", err);
                return;
            }
        };
    } else {
        warn!("Note with value: {} already exists", note_front_value);
    }
}

fn add_space_to_note(mut add_note: AddNotes) -> Result<AddNotes, String> {
    let front_value: &str = &add_note.params.notes.get(0).unwrap().fields.front;
    let trimmed_value = front_value.trim_end();
    if front_value.len() - trimmed_value.len() > 5 {
        return Err(String::from("Too many instances"));
    }

    let upcoming_value = String::from(front_value) + " ";
    add_note.params.notes.get_mut(0).unwrap().fields.front = String::from(upcoming_value);
    Ok(add_note)
}

fn already_exists(response: &Value) -> bool {
    response.get("result").unwrap().as_array().unwrap().get(0).unwrap().is_null()
}

async fn create_deck_if_not_exist(deck_name: &str, base_url: &str) {
    let body = DeckName::new();
    if !deck_exists(body, deck_name, base_url).await {
        create_deck(deck_name, base_url).await;
        info!("Deck is created");
        return;
    }
    debug!("Deck already exists");
}

async fn create_deck(deck_name: &str, base_url: &str) {
    let body = CreateDeck::new(deck_name.to_string());
    send_post(body.to_string_json(), body.action.as_str(), base_url).await.unwrap();
}

async fn deck_exists(body: DeckName, deck_name: &str, base_url: &str) -> bool {
    let response = match send_post(body.to_string_json(), body.action.as_str(), base_url).await {
        Ok(response) => response,
        Err(err) => {
            error!("Failed to find decks: {}", err);
            return false;
        }
    };
    response.get("result").unwrap().as_object().unwrap().get(deck_name).is_some()
}

async fn note_exists(deck_name: &str, note: &str, base_url: &str) -> bool {
    let query = format!("deck:{} {}", deck_name, note);
    let find_notes = FindNotes::new(query.as_str());
    let response = match send_post(find_notes.to_string_json(), find_notes.action.as_str(), base_url).await {
        Ok(response) => response,
        Err(err) => {
            error!("Failed to query notes: {}", err);
            return false;
        }
    };
    !response.get("result").unwrap().as_array().unwrap().is_empty()
}

async fn send_post<T: Into<Body> + Debug>(body: T, action: &str, base_url: &str) -> Result<Value, reqwest::Error> {
    debug!("Body is :{:#?}", body);
    let client = reqwest::Client::new();
    let response = client.post(base_url).body(body).send().await?.json::<Value>().await?;
    debug!("Response is: {:#?}", response);

    if let Some(error) = response.get("error").filter(|&error| !error.is_null()) {
        error!("There was an error with {}: {}", action, error);
    }
    Ok(response)
}

#[cfg(test)]
mod tests {
    use std::sync::Once;

    use mockito::{Matcher, Mock, Server};
    use serde_json::json;

    use super::*;

    const POST: &str = "POST";
    const BASE_PATH: &str = "/";
    const WORD: &str = "application";
    const TRANSLATION: &str = "εφαρμογή";
    const EXISTING_DECK: &str = "DeckTesting";
    const NEW_DECK: &str = "DifferentDeck";
    static INIT: Once = Once::new();

    pub fn initialize() {
        INIT.call_once(|| {
            env_logger::init();
        });
    }

    #[tokio::test]
    async fn existing_deck_new_note() {
        initialize();
        let mut server = Server::new();

        let deck_exists_mock = get_deck_exists_mock(&mut server);
        let find_notes_mock = get_find_notes_mock(&mut server, EXISTING_DECK);
        let add_notes_mock = get_add_notes_mock(&mut server, EXISTING_DECK);

        // Find existing deck, don't find existing word
        let add_note = AddNotes::new(String::from(EXISTING_DECK), String::from("application"), String::from("εφαρμογή"));
        create_notes(add_note, &server.url()).await;

        // Verify
        deck_exists_mock.assert();
        find_notes_mock.assert();
        add_notes_mock.assert();
    }

    #[tokio::test]
    async fn new_deck_new_note() {
        initialize();
        let mut server = Server::new();

        let deck_exists_mock = get_deck_exists_mock(&mut server);
        let create_deck_mock = get_create_deck_mock(&mut server, NEW_DECK);
        let find_notes_mock = get_find_notes_mock(&mut server, NEW_DECK);
        let add_notes_mock = get_add_notes_mock(&mut server, NEW_DECK);

        // Don't find existing deck, don't find existing word
        let add_note = AddNotes::new(String::from(NEW_DECK), String::from("application"), String::from("εφαρμογή"));
        create_notes(add_note, &server.url()).await;

        // Verify
        deck_exists_mock.assert();
        create_deck_mock.assert();
        find_notes_mock.assert();
        add_notes_mock.assert();
    }

    fn get_add_notes_mock(server: &mut Server, deck_name: &str) -> Mock {
        let add_notes_request = create_request_body("addNotes", Some(json!({
            "notes": [
                {
                    "deckName": deck_name,
                    "modelName": "Basic",
                    "fields": {
                        "Front": WORD,
                        "Back": TRANSLATION,
                    },
                    "tags": null,
                    "audio": null,
                    "video": null,
                    "picture": null
                }
            ]
        })));
        let add_notes_response = create_response_body(Some(json!([1692010871488 as u64])), None);
        create_mock(server, add_notes_request, add_notes_response)
    }

    fn get_find_notes_mock(server: &mut Server, deck_name: &str) -> Mock {
        let query = format!("deck:{} {}", deck_name, WORD);
        let find_notes_request = create_request_body("findNotes", Some(json!({
            "query": query
        })));
        let find_notes_response = create_response_body(Some(json!([])), None);
        create_mock(server, find_notes_request, find_notes_response)
    }

    fn get_create_deck_mock(server: &mut Server, deck_name: &str) -> Mock {
        let create_deck_request = create_request_body("createDeck", Some(json!({
            "deck": deck_name
        })));
        let create_deck_response = create_response_body(Some(json!(1692010870564 as u64)), None);
        create_mock(server, create_deck_request, create_deck_response)
    }

    fn get_deck_exists_mock(server: &mut Server) -> Mock {
        let deck_exists_request = create_request_body("deckNamesAndIds", None);
        let deck_exists_response = create_response_body(Some(json!({
            "Default": [1496198395707 as u64],
            "English": [1676819787730 as u64],
            EXISTING_DECK: [1690315099005 as u64]
        })), None);
        create_mock(server, deck_exists_request, deck_exists_response)
    }

    fn create_request_body(action: &str, params: Option<Value>) -> Value {
        let mut request_body = json!({
            "action": action,
            "version": 6
        });
        if let Some(params_value) = params {
            request_body["params"] = params_value;
        }
        request_body
    }

    fn create_response_body(result: Option<Value>, error: Option<Value>) -> Value {
        json!({
            "error": error.unwrap_or(json!(null)),
            "result": result.unwrap_or(json!(null))
        })
    }

    fn create_mock(server: &mut Server, request: Value, response: Value) -> Mock {
        server.mock(POST, BASE_PATH)
            .match_body(Matcher::Json(request))
            .with_body(response.to_string())
            .create()
    }
}