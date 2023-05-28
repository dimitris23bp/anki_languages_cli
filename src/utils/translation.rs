use std::collections::HashMap;
use log::debug;
use reqwest::Body;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::Value;

pub async fn translate(word: &str) -> String {
    let headers = get_headers();
    let mut body = HashMap::new();
    body.insert("source", "en");
    body.insert("target", "el");
    body.insert("q", word);
    let encoded_body = serde_urlencoded::to_string(body).unwrap();
    debug!("Encoded body is: {}", encoded_body);
    let body = Body::from(serde_json::to_string(&encoded_body).unwrap());

    let client = reqwest::Client::new();
    let response = client.post("https://google-translate1.p.rapidapi.com/language/translate/v2").headers(headers)
        .body(body).send().await.unwrap().json::<Value>().await.unwrap();
    debug!("Response from translate is: {}", response);
    response.get("data").unwrap().get("translations").unwrap().as_array().unwrap().get(0).unwrap().get("translatedText").unwrap().as_str().unwrap().to_string()
}

fn get_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("content-type", HeaderValue::from_static("application/x-www-form-urlencoded"));
    headers.insert("Accept-Encoding", HeaderValue::from_static("application/gzip"));
    headers.insert("X-RapidAPI-Key", HeaderValue::from_static("06b8a8e9a8mshd4e7da5a5b1f799p1ba048jsn80e9d4cb8989"));
    headers.insert("X-RapidAPI-Host", HeaderValue::from_static("google-translate1.p.rapidapi.com"));
    headers
}