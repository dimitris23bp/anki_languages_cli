use serde::Serialize;

pub trait ToStringJson {
    fn to_string_json(&self) -> String where Self: Serialize {
        serde_json::to_string(self).unwrap()
    }
}