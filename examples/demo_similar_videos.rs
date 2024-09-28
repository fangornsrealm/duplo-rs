use clap::{command, Arg, ArgAction};
use log::{error, warn, LevelFilter};
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode, WriteLogger};
use std::fs::File;


pub fn main() {
    let mut logfile = "demo_similar_videos.txt".to_string();
    let matches = command!() // requires `cargo` feature
        .arg(Arg::new("logfile").short('l').long("log"))
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    if let Some(ret) = matches.get_one::<String>("logfile") {
        logfile = ret.clone();
    }
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Warn,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            File::create(&logfile).unwrap(),
        ),
    ])
    .unwrap();


}