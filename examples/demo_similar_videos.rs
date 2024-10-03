use clap::{command, Arg, ArgAction};
use log::{error, warn, LevelFilter};
use pbr::ProgressBar;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode, WriteLogger};
use std::fs::File;


pub fn main() {
    let mut logfile = "demo_similar_videos.txt".to_string();
    let mut recursive = false;
    let mut sensitivity: f64 = -60.0;
    let curdir = std::env::current_dir().unwrap().as_os_str().to_owned();
    let mut directory = duplo_rs::files::osstring_to_string(&curdir);
    let matches = command!() // requires `cargo` feature
        .arg(Arg::new("logfile").short('l').long("log"))
        .arg(Arg::new("directory").short('d').long("directory"))
        .arg(Arg::new("sensitivity").short('s').long("sensitivity"))
        .arg(
            Arg::new("recursive")
                .short('r')
                .long("recursive")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    if let Some(ret) = matches.get_one::<String>("logfile") {
        logfile = ret.clone();
    }
    if let Some(ret) = matches.get_one::<String>("directory") {
        directory = ret.clone();
    }
    if let Some(ret) = matches.get_one::<String>("sensitivity") {
        let ret = i64::from_str_radix(ret, 10);
        if ret.is_ok() {
            sensitivity -= ret.unwrap() as f64;
        }
    }
    if let Some(ret) = matches.get_one::<bool>("recursive") {
        recursive = *ret;
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
    let dir = directory.clone();
    let p = std::path::Path::new(&dir);
    if !p.is_dir() {
        log::error!("Directory {} does not exist. Specify an existing directory to search!", directory);
        std::process::exit(1);
    }
    // create the directory where the user can compare the similar image pairs
    let dst: std::path::PathBuf = p.join("duplicates");

    // get the list of files to process
    let filelist;
    if recursive {
        // Consider all subdirectories --> will take a long time!
        filelist = duplo_rs::files::walk_tree_videos(&directory);
    } else {
        filelist = duplo_rs::files::walk_dir_videos(&directory);
    }
    let mut progressbar = ProgressBar::new(filelist.len() as u64);
    let mut store = duplo_rs::videostore::VideoStore::new(sensitivity);

    // process the files
    for file in filelist.iter() {
        let video = duplo_rs::files::process_video(file, &store);
        let filepath = duplo_rs::files::osstring_to_string(file.as_os_str());
        let (matches, failedid, failedhash) = 
            duplo_rs::files::find_similar_videos(&store, &filepath, &video);
        progressbar.inc();
        for i in 0..matches.m.len() {
            log::warn!("Match {} is similar to {}.", matches.m[i].id, filepath);
            let retmatch = imagesize::size(matches.m[i].id.clone());
            let retsource = imagesize::size(filepath.clone());
            if retmatch.is_err() {
                log::error!("Failed to read the size of the image {}!", matches.m[i].id);
                continue;
            }
            if retsource.is_err() {
                log::error!("Failed to read the size of the image {}!", filepath);
                continue;
            }
            let matchsize = retmatch.unwrap();
            let sourcesize = retsource.unwrap();
            if matchsize.width * matchsize.height > sourcesize.width * sourcesize.height {
                // match is the *better* image, drop the new hash
                duplo_rs::files::present_pairs(&dst, &filepath, &matches.m[i].id);
            } else {
                // source is the *better* image, remove match from store, add the source and drop the rest of the matches
                duplo_rs::files::present_pairs(&dst, &matches.m[i].id, &filepath);
                store.delete(&matches.m[i].id);
                store.add(&filepath, &video, video.runtime);
                break;
            }
        }
        // add the current file to the store
        if matches.m.len() == 0 {
            store.add(&failedid, &video, video.runtime);
        }
    }
}