// use clap::{Arg, ArgAction, Command};
use dotenv::dotenv;
use tts_cli_rs::tts::azure;

#[tokio::main]
async fn main() {
    dotenv().ok();
    if let Err(e) = azure::speak("Hello, world!").await {
        eprintln!("Error: {}", e);
    };
    // let matches = Command::new("tts-cli")
    //     .about("Text to speech command line interface")
    //     .version("0.0.1")
    //     .subcommand_required(true)
    //     .arg_required_else_help(true)
    //     .subcommand(
    //         Command::new("speak")
    //             .short_flag('S')
    //             .long_flag("speak")
    //             .about("Speak text")
    //             .arg(
    //                 Arg::new("text")
    //                     .short('t')
    //                     .long("text")
    //                     .help("Text to speak")
    //                     .action(ArgAction::Set)
    //                     .num_args(1..),
    //             )
    //             .arg(
    //                 Arg::new("language")
    //                     .short('l')
    //                     .long("language")
    //                     .help("Language to speak")
    //                     .action(ArgAction::Set)
    //                     .num_args(1..),
    //             )
    //             .arg(
    //                 Arg::new("speaker")
    //                     .short('s')
    //                     .long("speaker")
    //                     .help("Speaker to speak")
    //                     .action(ArgAction::Set)
    //                     .num_args(1..),
    //             )
    //             .arg(
    //                 Arg::new("rate")
    //                     .short('r')
    //                     .long("rate")
    //                     .help("Rate to speak")
    //                     .action(ArgAction::Set)
    //                     .num_args(1..),
    //             ),
    //     )
    //     .get_matches();

    // match matches.subcommand() {
    //     Some(("speak", speak_matches)) => {
    //         if speak_matches.contains_id("text") {
    //             let text: &String = speak_matches.get_one::<String>("text").unwrap();
    //             println!("Text: {}", text);
    //         }
    //     }
    //     _ => (),
    // }
}
