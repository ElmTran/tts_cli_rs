mod args;
mod config;
mod tts;
mod utils;

use args::Opt;
use structopt::StructOpt;
use tts::azure;
use tts::param;
use utils::audio_util;

#[tokio::main]
async fn main() {
    let matches = Opt::from_args();
    if let Some(command) = matches.command {
        match command {
            args::Command::Config(config) => {
                if let Some(set) = config.set {
                    let mut config_new = config::Config::new();
                    config_new.set_config(&set);
                } else if let Some(query) = config.get {
                    let config_new = config::Config::new();
                    config_new.get_config(query);
                }
            }
        }
    } else {
        let params = param::Params::new(
            matches
                .text
                .unwrap_or("please provide text to speak".to_string()),
            matches.speaker,
            matches.language,
            matches.style,
            matches.rate,
            matches.pitch,
        );
        let res = azure::speak(&params).await.unwrap();
        if let Some(output) = matches.output {
            audio_util::save_audio(&res, &output).unwrap();
        }
        audio_util::play_audio(&res).await.unwrap();
    }
}
