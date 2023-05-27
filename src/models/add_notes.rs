use serde::{Deserialize, Serialize};
use crate::traits::to_string_json::ToStringJson;

#[derive(Debug, Deserialize, Serialize)]
pub struct AddNotes {
    pub action: String,
    version: u32,
    pub params: NotesParams,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NotesParams {
    pub notes: Vec<Note>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Note {
    #[serde(rename = "deckName")]
    pub deck_name: String,
    #[serde(rename = "modelName")]
    model_name: String,
    pub fields: NoteFields,
    tags: Option<Vec<String>>,
    audio: Option<Vec<Audio>>,
    video: Option<Vec<Video>>,
    picture: Option<Vec<Picture>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NoteFields {
    #[serde(rename = "Front")]
    pub front: String,
    #[serde(rename = "Back")]
    pub back: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Audio {
    url: String,
    filename: String,
    #[serde(rename = "skipHash")]
    skip_hash: String,
    fields: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Video {
    url: String,
    filename: String,
    #[serde(rename = "skipHash")]
    skip_hash: String,
    fields: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Picture {
    url: String,
    filename: String,
    #[serde(rename = "skipHash")]
    skip_hash: String,
    fields: Vec<String>,
}

impl AddNotes {
    pub fn new(deck_name: String, front: String, back: String) -> Self {
        AddNotes {
            action: "addNotes".to_owned(),
            version: 6,
            params: NotesParams {
                notes: vec![Note {
                    deck_name,
                    model_name: "Basic".to_owned(),
                    fields: NoteFields {
                        front,
                        back
                    },
                    tags: None,
                    audio: None,
                    video: None,
                    picture: None
                }],
            },
        }
    }
}

impl ToStringJson for AddNotes {}