use crate::config::Config;
use super::param::Params;
use reqwest::Client;
use std::error::Error;

pub async fn speak(params: &Params, config: &Config) -> Result<Vec<u8>, Box<dyn Error>> {
    let subscription = config.get_key().expect("AZURE_KEY must be set");
    let endpoint = config.get_endpoint().expect("AZURE_ENDPOINT must be set");
    let client = Client::new();
    let xml = params.ssml();
    let res = client
        .post(endpoint)
        .header("Ocp-Apim-Subscription-Key", subscription)
        .header("Content-Type", "application/ssml+xml")
        .header("X-Microsoft-OutputFormat", "audio-24khz-160kbitrate-mono-mp3")
        .header("User-Agent", "curl")
        .body(xml)
        .send().await?
        .bytes().await?;
    Ok(res.to_vec())
}
