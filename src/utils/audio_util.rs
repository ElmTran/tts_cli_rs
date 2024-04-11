use super::time_util;
use std::fs::File;
use std::io::Write;
use std::thread;
use std::time::Duration;
use soloud::{ audio, AudioExt, LoadExt, Soloud };

pub fn save_audio(audio: &[u8], path: &String) -> std::io::Result<()> {
    let mut file = File::create(format!("{}/{}.mp3", path, time_util::get_timestamp()))?;
    file.write_all(&audio)?;
    Ok(())
}

pub async fn play_audio(audio: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let sl = Soloud::default()?;
    let mut wav = audio::Wav::default();
    wav.load_mem(audio)?;
    sl.play(&wav);
    while sl.voice_count() > 0 {
        thread::sleep(Duration::from_millis(100));
    }
    Ok(())
}
