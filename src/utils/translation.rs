use std::collections::HashMap;
use log::debug;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::{json, Value};
use url::Url;

pub async fn translate(word: &str) -> String {
    let url = get_url();
    debug!("URL is: {}", url);
    let headers = get_headers();
    debug!("Headers are: {:?}", headers);
    let body = get_body(word);
    debug!("Body is: {:#?}", body);

    let client = reqwest::Client::new();
    let response = client.post(url).headers(headers).body(body)
        .send().await.unwrap().json::<Value>().await.unwrap();
    debug!("Response from translate is: {}", response);
    response.get("translatedText").unwrap().as_str().unwrap().to_owned()
}

fn get_url() -> Url {
    Url::parse("https://libretranslate.org/translate").unwrap()
}

fn get_body(word: &str) -> String {
    let mut body = HashMap::new();
    body.insert("q", word);
    body.insert("source", "en");
    body.insert("target", "el");
    body.insert("format", "text");
    body.insert("api_key", "");
    json!(body).to_string()
}

fn get_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("content-type", HeaderValue::from_static("application/json"));
    headers
}