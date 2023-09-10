use std::collections::{HashMap, HashSet};

use log::{debug, error, info};
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::{json, Value};
use url::Url;

pub async fn translate(word: &str, source: String, target: String, base_url: &str) -> Result<String, String> {
    info!("base url is: {}", base_url);
    let url = get_url(base_url);
    debug!("URL is: {}", url);
    let headers = get_headers();
    debug!("Headers are: {:?}", headers);
    let body = get_body(word, source, target);
    debug!("Body is: {:#?}", body);

    let client = reqwest::Client::new();
    let result_response = client.post(url).headers(headers).body(body)
        .send().await;
    if let Ok(value) = result_response {
        if value.status().is_success() {
            let response = value.json::<Value>().await.unwrap();
            debug!("Response from translate is: {}", response);
            let translation = response.get("translatedText").unwrap().as_str().unwrap().to_owned();
            info!("Translation before trunc is: {}", translation);
            let translation = truncate_translation(translation);
            info!("Translation after trunc is: {}", translation);
            Ok(translation)
        } else {
            // TODO: status_handler here
            Err(String::from("fd"))
        }
    } else {
        error!("Response from translation API could not be retrieved.");
        Err(String::from("Error retrieving response from translation API."))
    }
}

fn truncate_translation(translation: String) -> String {
    let words = translation.split_ascii_whitespace().fold(HashSet::new(), |mut words, word| {
        words.insert(word);
        words
    });
    if words.len() == 1 {
        return words.iter().cloned().collect::<Vec<&str>>().join(" ");
    }
    translation
}

fn get_url(base_url: &str) -> Url {
    Url::parse(format!("{}/translate", base_url).as_str()).unwrap()
}

fn get_body(word: &str, source: String, target: String) -> String {
    let mut body = HashMap::new();
    body.insert("q", word);
    body.insert("source", source.as_str());
    body.insert("target", target.as_str());
    body.insert("format", "text");
    body.insert("api_key", "");
    json!(body).to_string()
}

fn get_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("content-type", HeaderValue::from_static("application/json"));
    headers
}

#[cfg(test)]
mod tests {
    use mockito::Matcher;
    use serde_json::json;

    use super::*;

    #[tokio::test]
    async fn test_translation() {
        env_logger::init();
        let word = "application";
        let source = String::from("en");
        let target = String::from("el");

        let mut server = mockito::Server::new();

        let request_body = json!({
            "q": word,
            "source": source,
            "target": target,
            "format": "text",
            "api_key": ""
        });

        let response_body = json!({
            "translatedText": "οπα οπα οπα",
        });
        let mock = server.mock("POST", "/translate")
            .match_body(Matcher::Json(request_body))
            .match_header("Content-type", "application/json")
            .with_body(response_body.to_string())
            .with_header("Content-type", "application/json")
            .create();

        let result = translate(word, source, target, &server.url()).await;

        // Verify that the mock server received the request as expected
        mock.assert();

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "οπα");
    }
}