use super::param::Params;
use reqwest::Client;
use std::env;
use std::error::Error;

pub async fn speak(params: &Params) -> Result<Vec<u8>, Box<dyn Error>> {
    let subscription = env::var("AZURE_KEY").expect("AZURE_KEY must be set");
    let region = env::var("AZURE_REGION").expect("AZURE_REGION must be set");
    let endpoint = env::var("AZURE_ENDPOINT").expect("AZURE_ENDPOINT must be set");
    let url = endpoint.replace("{region}", &region);
    let client = Client::new();
    let xml = params.ssml();
    let res = client
        .post(url)
        .header("Ocp-Apim-Subscription-Key", subscription)
        .header("Content-Type", "application/ssml+xml")
        .header(
            "X-Microsoft-OutputFormat",
            "audio-24khz-160kbitrate-mono-mp3",
        )
        .header("User-Agent", "curl")
        .body(xml)
        .send()
        .await?
        .bytes()
        .await?;
    Ok(res.to_vec())
}
