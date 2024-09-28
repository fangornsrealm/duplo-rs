use clap::{command, Arg, ArgAction};
use log::LevelFilter;
use pbr::ProgressBar;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode, WriteLogger};
use std::fs::File;

/// Searches for similar images inside a directory or optionally also all it's subdirectories
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
    let mut sensitivity: i32 = -60;
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
        let ret = i32::from_str_radix(ret, 10);
        if ret.is_ok() {
            sensitivity -= ret.unwrap();
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
    let dst = p.join("duplicates");
    if !dst.is_dir() {
        let ret = std::fs::create_dir(dst);
        if ret.is_err() {
            log::error!("Unable to create the duplicates directory in {}!", dir);
            std::process::exit(2);
        }
    }

    // get the list of files to process
    let filelist;
    if recursive {
        // Consider all subdirectories --> will take a long time!
        filelist = duplo_rs::files::walk_tree_images(&directory);
    } else {
        filelist = duplo_rs::files::walk_dir_images(&directory);
    }
    let mut progressbar = ProgressBar::new(filelist.len() as u64);
    let mut store = duplo_rs::store::Store::new();

    // process the files
    for file in filelist.iter() {
        let (matches, hash) = duplo_rs::files::process_image(file, &store);
        progressbar.inc();
        let filepath = duplo_rs::files::osstring_to_string(file.as_os_str());
        for i in 0..matches.m.len() {
            log::warn!("Match {} has a Hamming distance of {} to {}.", matches.m[i].id, matches.m[i].dhash_distance, filepath);
            if matches.m[i].dhash_distance > sensitivity {
                break;
            }
            

        }
        // add the current file to the store
        store.add(&filepath, &hash);
    }
}