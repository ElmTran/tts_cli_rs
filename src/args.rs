use std::error::Error;
use structopt::StructOpt;
use crate::config;
use crate::tts::param;
use crate::tts::azure;
use crate::utils::audio_util;

#[derive(StructOpt)]
#[structopt(name = "tts_cli_rs")]
pub struct Opt {
    #[structopt(short, long, help = "Text to speak")]
    pub text: Option<String>,
    #[structopt(short, long, help = "Language to speak", default_value = "en-US")]
    pub language: String,
    #[structopt(short = "y", long, help = "Style to speak", default_value = "chat")]
    pub style: String,
    #[structopt(short, long, help = "Speaker to speak", default_value = "en-US-AriaNeural")]
    pub speaker: String,
    #[structopt(short, long, help = "Rate to speak", default_value = "0%")]
    pub rate: String,
    #[structopt(short, long, help = "Pitch to speak", default_value = "0%")]
    pub pitch: String,
    /// Output file
    pub output: Option<String>,
    #[structopt(subcommand)]
    pub command: Option<Command>,
}

#[derive(StructOpt)]
pub enum Command {
    #[structopt(name = "config")] Config(Config),
}
#[derive(StructOpt)]
pub struct Config {
    #[structopt(
        short = "s",
        long,
        help = "Set configuration",
        parse(try_from_str = parse_key_val),
        number_of_values = 1
    )]
    pub set: Option<[String; 2]>,
    #[structopt(short = "g", long, help = "Get configuration")]
    pub get: Option<String>,
}

fn parse_key_val<T>(s: &str) -> Result<[T; 2], Box<dyn Error>>
    where T: std::str::FromStr, T::Err: Error + 'static
{
    let parts = s.find('=').ok_or_else(|| format!("invalid KEY=value: no `=` found in `{}`", s))?;
    Ok([s[..parts].parse()?, s[parts + 1..].parse()?])
}

pub fn handle_command(cmd: &Command, config: &mut config::Config) {
    match cmd {
        Command::Config(arg) => {
            if let Some(conf) = &arg.set {
                if let Err(_) = config.set_config(&conf) {
                    println!("Setting config failed, please check your input");
                }
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

pub async fn handle_option(opt: Opt, config: &config::Config) {
    let params = param::Params::new(
        opt.text.unwrap_or("please provide text to speak".to_string()),
        opt.speaker,
        opt.language,
        opt.style,
        opt.rate,
        opt.pitch
    );
    let res = azure::speak(&params, &config).await;
    match res {
        Ok(_) => {
            let audio = res.unwrap();
            if let Some(output) = opt.output {
                audio_util::write(&audio, &output).unwrap();
            }
            if let Err(_) = audio_util::play(&audio).await {
                println!("Playing audio failed, please check your device");
            }
        }
        Err(e) => println!("{}", e),
    }
}
