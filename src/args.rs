use clap::{Arg, ArgAction, ArgMatches, Command};

pub fn get_args() -> ArgMatches {
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
                        .num_args(1..)
                        .help("Text to speak"),
                )
                .arg(
                    Arg::new("language")
                        .short('l')
                        .long("language")
                        .help("Language to speak")
                        .action(ArgAction::Set)
                        .num_args(1..)
                        .default_value("en-US")
                        .help("Language to speak"),
                )
                .arg(
                    Arg::new("style")
                        .short('y')
                        .long("style")
                        .help("Style to speak")
                        .action(ArgAction::Set)
                        .num_args(1..)
                        .default_value("chat")
                        .help("Style to speak"),
                )
                .arg(
                    Arg::new("speaker")
                        .short('s')
                        .long("speaker")
                        .help("Speaker to speak")
                        .action(ArgAction::Set)
                        .num_args(1..)
                        .default_value("en-US-AriaNeural")
                        .help("Speaker to speak"),
                )
                .arg(
                    Arg::new("rate")
                        .short('r')
                        .long("rate")
                        .help("Rate to speak")
                        .action(ArgAction::Set)
                        .num_args(1..)
                        .default_value("0%")
                        .help("Rate to speak"),
                )
                .arg(
                    Arg::new("pitch")
                        .short('p')
                        .long("pitch")
                        .help("Pitch to speak")
                        .action(ArgAction::Set)
                        .num_args(1..)
                        .default_value("0%")
                        .help("Pitch to speak"),
                ),
        )
        .get_matches();
    matches
}
