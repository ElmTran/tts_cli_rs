mod tts;
mod utils;

use clap::{Arg, ArgAction, ArgMatches, Command};
use dotenv::dotenv;
use std::fs::File;
use std::io::Write;
use tts::azure;
use tts::param;
use utils::time_util;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let args = get_args();
    let params = param::Params::new(
        // "Hello, world!".to_string(),
        // "en-US-AriaNeural".to_string(),
        // "en-US".to_string(),
        // "chat".to_string(),
        // "0%".to_string(),
        // "0%".to_string(),
        args.get_one::<String>("text").unwrap().clone(),
        args.get_one::<String>("speaker").unwrap().clone(),
        args.get_one::<String>("language").unwrap().clone(),
        args.get_one::<String>("style").unwrap().clone(),
        args.get_one::<String>("rate").unwrap().clone(),
        args.get_one::<String>("pitch").unwrap().clone(),
    );
    let res = azure::request(&params).await.unwrap();
    save_audio(&res).unwrap();
}

fn get_args() -> ArgMatches {
    let matches = Command::new("tts-cli")
        .about("Text to speech command line interface")
        .version("0.0.1")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("speak")
                .short_flag('S')
                .long_flag("speak")
                .about("Speak text")
                .arg(
                    Arg::new("text")
                        .short('t')
                        .long("text")
                        .help("Text to speak")
                        .action(ArgAction::Set)
                        .num_args(1..),
                )
                .arg(
                    Arg::new("language")
                        .short('l')
                        .long("language")
                        .help("Language to speak")
                        .action(ArgAction::Set)
                        .num_args(1..),
                )
                .arg(
                    Arg::new("style")
                        .short('y')
                        .long("style")
                        .help("Style to speak")
                        .action(ArgAction::Set)
                        .num_args(1..),
                )
                .arg(
                    Arg::new("speaker")
                        .short('s')
                        .long("speaker")
                        .help("Speaker to speak")
                        .action(ArgAction::Set)
                        .num_args(1..),
                )
                .arg(
                    Arg::new("rate")
                        .short('r')
                        .long("rate")
                        .help("Rate to speak")
                        .action(ArgAction::Set)
                        .num_args(1..),
                )
                .arg(
                    Arg::new("pitch")
                        .short('p')
                        .long("pitch")
                        .help("Pitch to speak")
                        .action(ArgAction::Set)
                        .num_args(1..),
                ),
        )
        .get_matches();
    matches
}

fn save_audio(audio: &[u8]) -> std::io::Result<()> {
    let mut file = File::create(format!("assets/audio/{}.wav", time_util::get_timestamp()))?;
    file.write_all(audio)?;
    file.flush()?;
    Ok(())
}
