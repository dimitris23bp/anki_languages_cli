use serde::{Deserialize, Serialize};
use crate::traits::to_string_json::ToStringJson;

#[derive(Debug, Deserialize, Serialize)]
pub struct DeckName {
    pub action: String,
    version: u32,
}

impl DeckName {
   pub fn new() -> Self {
       DeckName {
           action: "deckNamesAndIds".to_string(),
           version: 6
       }
   }
}

impl ToStringJson for DeckName {}
