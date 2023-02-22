use crate::query::types::BssidPayload;
use reqwest::Client;
use serde_json::json;

pub async fn make_request(
    hostname: &String,
    url: &str,
    payload: Vec<BssidPayload>,
    client: &Client,
) -> anyhow::Result<()> {
    // simply make a post request with the json body
    match client
        .post(url)
        .body(json!(payload).to_string())
        .header("Content-Type", "application/json")
        .header("X-Device-ID", hostname.trim())
        .send()
        .await
    {
        Ok(_) => {}
        Err(e) => {
            // if we need some sort of alert that the request hasn't gone through,
            // we can add it in here.
            eprintln!("Error with sending payload: {:?}", e)
        }
    };

    Ok(())
}
