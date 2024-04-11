use super::time_util;
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::{Cursor, Write};

pub fn save_audio(audio: &[u8], path: &String) -> std::io::Result<()> {
    let mut file = File::create(format!("{}/{}.mp3", path, time_util::get_timestamp()))?;
    file.write_all(&audio)?;
    Ok(())
}

pub async fn play_audio(audio: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    let cursor = Cursor::new(audio);
    let decoder = Decoder::new(cursor).unwrap();
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let err = stream_handle.play_raw(decoder.convert_samples());
    match err {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}
