mod args;
mod config;
mod tts;
mod utils;

use structopt::StructOpt;
use tts::azure;
use tts::param;
use utils::audio_util;

#[tokio::main]
async fn main() {
    let mut config = config::Config::from_toml("conf.toml");
    let matches = args::Opt::from_args();
    if let Some(command) = matches.command {
        handle_command(&command, &mut config);
    } else {
        let params = param::Params::new(
            matches.text.unwrap_or("please provide text to speak".to_string()),
            matches.speaker,
            matches.language,
            matches.style,
            matches.rate,
            matches.pitch
        );
        let res = azure::speak(&params, &config).await;
        match res {
            Ok(_) => {
                let audio = res.unwrap();
                if let Some(output) = matches.output {
                    audio_util::write(&audio, &output).unwrap();
                }
                if let Err(_) = audio_util::play(&audio).await {
                    println!("Playing audio failed, please check your device");
                }
            }
            Err(e) => println!("{}", e),
        }
    }
}

fn handle_command(cmd: &args::Command, config: &mut config::Config) {
    match cmd {
        args::Command::Config(arg) => {
            if let Some(conf) = &arg.set {
                config.set_config(&conf);
            } else if let Some(query) = &arg.get {
                let val = config.get_config(&query);
                match val {
                    Ok(v) => println!("{}", v),
                    Err(e) => println!("{}", e),
                }
            }
        }
    }
}
