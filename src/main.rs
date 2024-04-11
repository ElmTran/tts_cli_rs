mod args;
mod tts;
mod utils;

use dotenv::dotenv;
use tts::azure;
use tts::param;
use utils::audio_util;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let args = args::get_args();
    match args.subcommand() {
        Some(("speak", speak_matches)) => {
            let params = param::Params::new(
                speak_matches.get_one::<String>("text").unwrap().clone(),
                speak_matches.get_one::<String>("speaker").unwrap().clone(),
                speak_matches.get_one::<String>("language").unwrap().clone(),
                speak_matches.get_one::<String>("style").unwrap().clone(),
                speak_matches.get_one::<String>("rate").unwrap().clone(),
                speak_matches.get_one::<String>("pitch").unwrap().clone()
            );
            let res = azure::speak(&params).await.unwrap();
            if speak_matches.contains_id("output") {
                let path = speak_matches.get_one::<String>("output").unwrap().to_string();
                audio_util::save_audio(&res, &path).unwrap();
            }
            audio_util::play_audio(&res).await.unwrap();
        }
        _ => {
            println!("Invalid command");
        }
    }
}
