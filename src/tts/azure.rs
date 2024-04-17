use super::param::Params;
use crate::config::Config;
use reqwest::Client;
use std::error::Error;

pub async fn speak(params: &Params, config: &Config) -> Result<Vec<u8>, Box<dyn Error>> {
    let subscription = config.get_key()?;
    let endpoint = config.get_endpoint()?;
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
    if res.is_empty() {
        Err("failed to generate audio, please check your settings or text".into())
    } else {
        Ok(res.to_vec())
    }
}
