use super::time_util;
use soloud::{ audio, AudioExt, LoadExt, Soloud };
use std::fs::File;
use std::io::Write;
use std::thread;
use std::time::Duration;

pub fn write(audio: &[u8], path: &String) -> std::io::Result<()> {
    if path.ends_with(".mp3") || path.ends_with(".wav") {
        let mut file = File::create(path)?;
        file.write_all(&audio)?;
    } else {
        std::fs::create_dir_all(path)?;
        let mut file = File::create(format!("{}/{}.mp3", path, time_util::get_timestamp()))?;
        file.write_all(&audio)?;
    }
    Ok(())
}

pub async fn play(audio: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let sl = Soloud::default()?;
    let mut wav = audio::Wav::default();
    wav.load_mem(audio)?;
    sl.play(&wav);
    while sl.voice_count() > 0 {
        thread::sleep(Duration::from_millis(100));
    }
    Ok(())
}
