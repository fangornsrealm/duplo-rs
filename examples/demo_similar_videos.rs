use clap::{command, Arg, ArgAction};
use log::LevelFilter;
use pbr::ProgressBar;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode, WriteLogger};
use std::fs::File;
//use std::sync::mpsc::{Sender, Receiver};
//use std::sync::mpsc;

/// Searches for similar videos inside a directory or optionally also all it's subdirectories
/// 
/// *Similar* in this definition means that at least a minute should show similar video content
/// ffmpeg is used to generate a screenshot every ten seconds. 
/// The similar image routines of duplo-rs are used to find similar screenshots.
/// If a video has six or more similar screenshots in a row they are considered similar.
/// 
/// Dependencies: 
/// ffmpeg needs to be installed and in the path for executables. No development packages required.
/// 
/// It can happen that it finds similarities that you will not see as such.
/// If you want to decrease the sensitivity you can specify a value between 0 and 100 which will decrease the number of found similar images.
/// 
/// To enable you to check it creates a directory "duplicates" that contains pairs of images. 
/// The file with the "KEEP" in the name will stay, as it is a hardlink (or a copy if your filesystem does not support hardlinks)
/// The file with the "REMOVE" in the name will be gone if you delete it as it was moved there.
/// They have the same random prefix so they are sorted together.
/// The "better" of the two images was picked. The criteria for "better" could be more elaborate.
/// Use any Image browser / file manager to review at your leisure.
///
/// If you are satisfied with the selection, just delete all of the images. 
/// If images were picked that you would like to keep, you have to move at least the files with "REMOVE" in the name somewhere else.
/// 
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
    let directory = "/ssd/src/rust/duplo-rs/testtfiles".to_string();
    let dir = directory.clone();
    let p = std::path::Path::new(&dir);
    if !p.is_dir() {
        log::error!("Directory {} does not exist. Specify an existing directory to search!", directory);
        std::process::exit(1);
    }
    // create the directory where the user can compare the similar image pairs
    let dst: std::path::PathBuf = p.join("duplicates");
    let storepath = p.join("demo_example_videos.store");
    let mut store: duplo_rs::videostore::VideoStore = duplo_rs::videostore::VideoStore::new(sensitivity);;
    if storepath.is_file() {
        let storefile = duplo_rs::files::osstring_to_string(storepath.as_os_str());
        store.slurp_binary(&storefile);
    }
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
    //let (tx, rx): (Sender<duplo_rs::videocandidate::VideoCandidate>, Receiver<duplo_rs::videocandidate::VideoCandidate>) = mpsc::channel();
    let num_videos = filelist.len() as u32;
    for file in filelist.iter() {
        let video_id = store.candidates.len();
    //use rayon::prelude::*;
    //filelist.iter().enumerate().for_each(|(video_id, file)| {
    //    let thread_tx = tx.clone();
        let filepath = duplo_rs::files::osstring_to_string(file.as_os_str());
        if store.ids.contains_key(&filepath) {
            continue;
            //thread_tx.send(duplo_rs::videocandidate::VideoCandidate::new()).unwrap() ; // already parsed
        }
        let video = duplo_rs::files::process_video(file, video_id, num_videos);
        //thread_tx.send(video).unwrap();
    //});
    
    //let mut num_processed = 0;
    //loop {
    //    let video_opt = rx.recv();
    //    if video_opt.is_err() {
    //        continue;
    //    }
    //    let video = video_opt.unwrap();
        let (matches, failedid, _failedhash) = 
            duplo_rs::files::find_similar_videos(&store, &video.id, &video);
        progressbar.inc();
        for i in 0..matches.m.len() {
            log::warn!("Match {} is similar to {}.", matches.m[i].id, video.id);
            let index = store.ids[&matches.m[i].id];
            let matched = &store.candidates[index];
            if matched.width * matched.height > video.width * video.height {
                // match is the *better* image, drop the new hash
                duplo_rs::files::present_pairs(&dst, &video.id, &matches.m[i].id);
            } else {
                // source is the *better* image, remove match from store, add the source and drop the rest of the matches
                duplo_rs::files::present_pairs(&dst, &matches.m[i].id, &video.id);
                store.delete(&matches.m[i].id);
                store.add(&video.id, &video, video.runtime);
                break;
            }
        }
        // add the current file to the store
        if matches.m.len() == 0 {
            store.add(&failedid, &video, video.runtime);
        }
        //num_processed += 1;
        //if store.candidates.len() == filelist.len() {
        //    // all videos processed
        //    break;
        //}
    }
    if storepath.is_file() {
        let ret = std::fs::remove_file(storepath);
        if ret.is_err() {
            log::error!("Failed to delete store file");
        }
    }
    store.dump_binary("demo_example_videos.store");
}

