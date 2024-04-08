use reqwest::Client;
use std::env;
use std::error::Error;

pub async fn speak(text: &str) -> Result<(), Box<dyn Error>> {
    let subscription = env::var("AZURE_KEY").expect("AZURE_KEY must be set");
    let region = env::var("AZURE_REGION").expect("AZURE_REGION must be set");
    let endpoint = env::var("AZURE_ENDPOINT").expect("AZURE_ENDPOINT must be set");
    let url = endpoint.replace("{region}", &region);
    let client = Client::new();
    let xml =
        format!(r#"
        <speak version="1.0" xmlns="http://www.w3.org/2001/10/synthesis" xmlns:mstts="http://www.w3.org/2001/mstts" xmlns:emo="http://www.w3.org/2009/10/emotionml" xml:lang="en-US">
            <voice name="en-US-AriaNeural">
                <mstts:express-as style="chat">
                    <prosody rate="0%" pitch="0%">
                        {}
                    </prosody>
                </mstts:express-as>
            </voice>
        </speak>
    "#, text);
    let res = client
        .post(url)
        .header("Ocp-Apim-Subscription-Key", subscription)
        .header("Content-Type", "application/ssml+xml")
        .header("X-Microsoft-OutputFormat", "audio-24khz-160kbitrate-mono-mp3")
        .header("User-Agent", "curl")
        .body(xml)
        .send().await?
        .text().await?;
    save_audio(&res)?;
    Ok(())
}

fn save_audio(res: &str) -> Result<(), Box<dyn Error>> {
    let path = env::var("AUDIO_PATH").expect("AUDIO_PATH must be set");
    std::fs::write(path, res)?;
    Ok(())
}
