use reqwest::Client;
use std::env;
use std::error::Error;

pub async fn speak(text: &str) -> Result<(), Box<dyn Error>> {
    let subscription = env::var("AZURE_KEY").expect("AZURE_KEY must be set");
    let region = env::var("AZURE_REGION").expect("AZURE_REGION must be set");
    let endpoint = env::var("AZURE_ENDPOINT").expect("AZURE_ENDPOINT must be set");
    let url = endpoint.replace("{region}", &region);
    let client = Client::new();
    let response = client
        .post(&url)
        .header("Ocp-Apim-Subscription-Key", subscription)
        .header("Content-Type", "application/ssml+xml")
        .header(
            "X-Microsoft-OutputFormat",
            "audio-16khz-128kbitrate-mono-mp3",
        )
        .body(format!(
            r#"<speak version="1.0" xmlns="http://www.w3.org/2001/10/synthesis" xml:lang="en-US">
                    <voice name="en-US-Guy24kRUS">
                        {}
                    </voice>
                </speak>"#,
            text
        ))
        .send()
        .await?;
    println!("Response: {:?}", response);
    Ok(())
}
