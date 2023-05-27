use serde::{Deserialize, Serialize};
// use crate::models::anki_base_model::{AnkiModel, Param};
use crate::traits::to_string_json::ToStringJson;

#[derive(Debug, Serialize, Deserialize)]
pub struct FindNotes<'a> {
    pub action: String,
    version: u32,
    #[serde(borrow)]
    pub params: QueryParams<'a>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryParams<'a> {
    pub query: &'a str,
}

impl<'a> FindNotes<'a> {
    pub(crate) fn new(query: &'a str) -> Self {
        FindNotes {
            action: "findNotes".to_owned(),
            version: 6,
            params: QueryParams { query },
        }
    }
}

// impl<'a> QueryParams<'a> {
//     pub fn new(query: &'a str) -> AnkiModel {
//         AnkiModel {
//             action: "findNotes".to_owned(),
//             version: 6,
//             params: Param::FindNotes(QueryParams { query})
//         }
//     }
//
// }
//
// impl ToStringJson for QueryParams<'_> {}
impl ToStringJson for FindNotes<'_> {}
