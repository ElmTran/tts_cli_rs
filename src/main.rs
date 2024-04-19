mod args;
mod config;
mod tts;
mod utils;

use structopt::StructOpt;
use confy;

#[tokio::main]
async fn main() {
    let mut config: config::Config = confy::load("tts_cli_rs", "conf").unwrap();
    let opt = args::Opt::from_args();
    if let Some(command) = opt.command {
        args::handle_command(&command, &mut config);
    } else {
        args::handle_option(opt, &config).await;
    }
}
