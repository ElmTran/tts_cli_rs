use std::error::Error;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
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

#[derive(StructOpt, Debug)]
pub enum Command {
    #[structopt(name = "config")] Config(Config),
}
#[derive(StructOpt, Debug)]
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
