use serde::{Deserialize, Serialize};
use crate::traits::to_string_json::ToStringJson;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateDeck {
    pub action: String,
    version: u32,
    pub params: DeckName,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeckName {
    pub deck: String
}

impl CreateDeck {
    pub fn new(deck: String) -> Self {
        CreateDeck {
            action: "createDeck".to_string(),
            version: 6,
            params: DeckName {
                deck
            }
        }
    }
}

impl ToStringJson for CreateDeck {}
