use serde::de::DeserializeOwned;

pub fn json_value_parser<T: DeserializeOwned>(s: &str) -> serde_json::Result<T> {
    serde_json::from_str(s)
}