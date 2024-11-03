use clap::{command, Arg, ArgAction};
use log::LevelFilter;
use pbr::ProgressBar;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode, WriteLogger};
use std::fs::File;

use std::sync::mpsc;
use std::thread;

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
/// The derived data from all the screenshots is too large to keep all the data in RAM.
/// Depending on the length of the videos even 96 GB of RAM will be filled with around 1000 videos.
/// The data blobs and the indices are stored in a SQLite database.
/// 1 TB of storage will suffice for 10 000 to 15 000 videos, depending on the length.
/// 
/// The search for similar videos in the database has to be done one by one.
/// The speed is severely limited. Each new video will be compared screenshot by screenshot. 
/// A rough estimate is thirty seconds per screenshot or fifteen minutes for five minutes of video!
/// This is a trade-off between the running time and the quality of similar video detection.
/// This demo app rests solely on the precision side of the evaluation.
/// Outside of Music or TikTok videos scenes are longer than 10 seconds. So the probability to find a similar frame in a modified video is good.
/// A screenshot every five minutes would be much faster and use 30x less resources, but only find the same file that has been cut in the end.
/// 
/// You can stop a scan at any time. Any video that has already been parsed will not be parsed again.
/// It's data will be deleted if the file is no longer available.
///
/// It can happen that it finds similarities that you will not see as such.
/// If you want to decrease the sensitivity you can specify a value between 0 and 100 which will decrease the number of found similar images.
///
/// To enable you to check it creates a directory "similar_videos" that contains a HTML file per video for which similar videos were found.
/// Each HTML file contains a preview of the new file on top and any previous read videos who have at least a minute of similar video.
/// The file path and video metadata are given for each file. The HTML file works for Chromium based browsers. MP4 files also work in Firefox.
///
pub fn main() {
    let mut logfile = "demo_similar_videos.txt".to_string();
    let mut recursive = false;
    let mut sensitivity: f64 = -60.0;

    let curdir = std::env::current_dir().unwrap().as_os_str().to_owned();
    let mut directory = duplo_rs::files::osstring_to_string(&curdir);
    let num_threads = 1;
    let matches = command!() // requires `cargo` feature
        .arg(Arg::new("logfile").short('l').long("log"))
        .arg(Arg::new("directory").short('d').long("directory"))
        .arg(Arg::new("sensitivity").short('s').long("sensitivity"))
        //.arg(Arg::new("num_threads").short('t').long("num_threads"))
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
    //if let Some(ret) = matches.get_one::<String>("num_threads") {
    //    let ret = i64::from_str_radix(ret, 10);
    //    if ret.is_ok() {
    //        num_threads = ret.unwrap() as u32;
    //    }
    //}
    if let Some(ret) = matches.get_one::<bool>("recursive") {
        recursive = *ret;
    }

    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Error,
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
        log::error!(
            "Directory {} does not exist. Specify an existing directory to search!",
            directory
        );
        std::process::exit(1);
    }
    // create the directory where the user can compare the similar image pairs
    let dst: std::path::PathBuf = p.join("similar_videos");
    // get the list of files to process
    let filelist;
    if recursive {
        // Consider all subdirectories --> will take a long time!
        filelist = duplo_rs::files::walk_tree_videos(&directory);
    } else {
        filelist = duplo_rs::files::walk_dir_videos(&directory);
    }
    let mut progressbar = ProgressBar::new(filelist.len() as u64);
    let homedir_opt = dirs::home_dir();
    if homedir_opt.is_none() {
        log::error!("Could not determine the home directory!");
        return;
    }
    let dbpath = homedir_opt.unwrap().join("similar_videos.sqlite3");
    let dbpathstr = duplo_rs::files::osstring_to_string(dbpath.as_os_str());
    let sql_client_opt = duplo_rs::videostore::connect(&dbpathstr);
    if sql_client_opt.is_ok() {
        let mut sql_client = sql_client_opt.unwrap();
        let mut store = duplo_rs::videostore::VideoStore::new(
            &mut sql_client,
            sensitivity,
            &directory,
            num_threads,
        );
        progressbar.add(store.num_candidates as u64);
        let num_videos = filelist.len() as u32;
        //let prev_videos = store.num_candidates;
        let mut video_id_counter = store.num_candidates + 1;
        let mut filepos = 0;
        'outer: loop {
            let mut handles = vec![];
            let (tx, rx) = mpsc::channel();
            
            for _ in 0..store.num_threads {
                if filepos + 1 > num_videos {
                    progressbar.finish();
                    break 'outer;
                }

                let video_id = video_id_counter;
                let filepath = filelist[filepos as usize].clone();
                let filestring = duplo_rs::files::osstring_to_string(&filepath.as_os_str());
                if store.ids.contains_key(&filestring) {
                    filepos += 1;
                    continue;
                }
                video_id_counter += 1;
                let tx1 = mpsc::Sender::clone(&tx);
                let handle = thread::spawn(move || {
                    // call function with
                    // sender as parameter
                    parallel_processor(tx1, &filepath, video_id, num_videos);
                });
                filepos += 1;
                handles.push(handle);
            }
            for _ in 0..handles.len() {
                let video_opt = rx.recv();
                if video_opt.is_err() {
                    log::error!("Failed to receive video data.");
                    continue;
                }
                let video = video_opt.unwrap();

                let (matches, _failedid, _failedhash) = duplo_rs::files::find_similar_videos(
                    &store,
                    &mut sql_client,
                    &video.id,
                    &video,
                );
                progressbar.inc();
                let mut compare: Vec<duplo_rs::videocandidate::VideoCandidate> = Vec::new();
                compare.push(video.clone());
                for i in 0..matches.m.len() {
                    log::warn!("Match {} is similar to {}.", matches.m[i].id, video.id);
                    let index = store.ids[&matches.m[i].id];
                    let (_, candidate) = store.return_candidate(&mut sql_client, index as u32);
                    compare.push(candidate);
                }
                // add the current file to the store
                store.add(&mut sql_client, &video.id, &video, video.runtime);
                duplo_rs::files::present_video_matches(&dst, &compare);
                for handlepos in (0..handles.len()).rev() {
                    if handles[handlepos].is_finished() {
                        handles.remove(handlepos);
                    }
                }
            }
        }
    }
    progressbar.finish();
}

fn parallel_processor(
    a: mpsc::Sender<duplo_rs::videocandidate::VideoCandidate>,
    filepath: &std::path::PathBuf,
    video_id: u32,
    num_videos: u32,
) {
    let video = duplo_rs::files::process_video(filepath, video_id as usize, num_videos);
    // send value
    a.send(video).unwrap();
    return;
}
