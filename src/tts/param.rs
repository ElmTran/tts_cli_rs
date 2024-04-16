pub struct Params {
    text: String,
    speaker: String,
    language: String,
    style: String,
    rate: String,
    pitch: String,
}

impl Params {
    pub fn new(
        text: String,
        speaker: String,
        language: String,
        style: String,
        rate: String,
        pitch: String
    ) -> Self {
        Self {
            text,
            speaker,
            language,
            style,
            rate,
            pitch,
        }
    }

    pub fn ssml(&self) -> String {
        format!(
            r#"
            <speak version="1.0"
                    xmlns="http://www.w3.org/2001/10/synthesis"
                    xmlns:mstts="http://www.w3.org/2001/mstts"
                    xmlns:emo="http://www.w3.org/2009/10/emotionml"
                    xml:lang="{}"
                >
                <voice name="{}">
                    <mstts:express-as style="{}">
                        <prosody rate="{}" pitch="{}">
                            {}
                        </prosody>
                    </mstts:express-as>
                </voice>
            </speak>
        "#,
            self.language,
            self.speaker,
            self.style,
            self.rate,
            self.pitch,
            self.text
        )
    }
}
