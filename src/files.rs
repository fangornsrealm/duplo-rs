use rand::Rng;
use std::fs;
use std::iter;
use std::path::PathBuf;
use std::ffi::OsStr;
use walkdir::WalkDir;

const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

fn generate_random_string(len: usize) -> String {
    let mut rng = rand::thread_rng();
    let one_char = || CHARSET[rng.gen_range(0..CHARSET.len())] as char;
    iter::repeat_with(one_char).take(len).collect()
}

pub fn osstring_to_string(input: &OsStr) -> String {
    let ret = input.to_str();
    if ret.is_none() {
        return String::new();
    }
    let s = ret.unwrap();
    s.to_string()
}

pub fn walk_dir_images(dirpath: &str) -> Vec<PathBuf> {
    let mut p = Vec::new();
    let ret = fs::read_dir(dirpath);
    if ret.is_err() {
        return p;
    }
    let dir = ret.unwrap();
    for f in dir {
        if f.is_ok() {
            let file = f.unwrap(); 
            let filepath = file.path();
            if filepath.is_file() {
                let e = filepath.extension();
                if e.is_some() {
                    let ext = e.unwrap();
                    let extstring = osstring_to_string(ext).to_ascii_lowercase();
                    if extstring == "png" || extstring == "jpg" || extstring == "jpeg" || extstring == "bmp"
                    || extstring == "gif" || extstring == "webp" || extstring == "tif" || extstring == "tiff" {
                        p.push(filepath);
                    }
                }
            }
        }
    }
    p
}

pub fn walk_tree_images(dirpath: &str) -> Vec<PathBuf> {
    let mut p = Vec::new();
    for entry in WalkDir::new(dirpath)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok()) {
        let filepath = entry.path();
        if filepath.is_file() {
            let e = filepath.extension();
            if e.is_some() {
                let ext = e.unwrap();
                let extstring = osstring_to_string(ext).to_ascii_lowercase();
                if extstring == "png" || extstring == "jpg" || extstring == "jpeg" || extstring == "bmp"
                || extstring == "gif" || extstring == "webp" || extstring == "tif" || extstring == "tiff" {
                p.push(filepath.into());
                }
            }
        }
    }
    p
}

pub fn walk_dir_videos(dirpath: &str) -> Vec<PathBuf> {
    let mut p = Vec::new();
    let ret = fs::read_dir(dirpath);
    if ret.is_err() {
        return p;
    }
    let dir = ret.unwrap();
    for f in dir {
        if f.is_ok() {
            let file = f.unwrap(); 
            let filepath = file.path();
            if filepath.is_file() {
                let e = filepath.extension();
                if e.is_some() {
                    let ext = e.unwrap();
                    let extstring = osstring_to_string(ext).to_ascii_lowercase();
                    if extstring == "mkv" || extstring == "mp4" || extstring == "avi" || extstring == "mov"
                        || extstring == "webm" {
                        p.push(filepath);
                    }
                }
            }
        }
    }
    p
}

pub fn walk_tree_videos(dirpath: &str) -> Vec<PathBuf> {
    let mut p = Vec::new();
    for entry in WalkDir::new(dirpath)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok()) {
        let filepath = entry.path();
        if filepath.is_file() {
            let e = filepath.extension();
            if e.is_some() {
                let ext = e.unwrap();
                let extstring = osstring_to_string(ext).to_ascii_lowercase();
                if extstring == "mkv" || extstring == "mp4" || extstring == "avi" || extstring == "mov"
                    || extstring == "webm" {
                    p.push(filepath.into());
                }
            }
        }
    }
    p
}

/// reads an image, creates a hash and compares it with the existing hashes.
/// Delivers existing Matches and the hash back to the calling program.
pub fn process_image(p: &PathBuf) -> crate::hash::Hash {
    let res = image::ImageReader::open(p);
    if res.is_ok() {
        let dynimg = res.unwrap().decode();
        if dynimg.is_err() {
            return crate::hash::Hash::new();
        }
        let img = dynimg.unwrap();
        let (hash, _smallimg) = crate::hash::create_hash(&img.into());
        return hash;
    }
    crate::hash::Hash::new()
}

pub fn find_similar_images(store: &crate::store::Store, id: &str, hash: &crate::hash::Hash) 
-> (crate::matches::Matches, String, crate::hash::Hash)
{
    let matches = store.query(&hash);
    (matches, id.to_string(), hash.clone())
}

/// presents the two files together renamed to have the same prefix
/// The file containing KEEP is a hard link. You can delete it without removing the original.
/// The file containing REMOVE is the original file. If you delete it, it will be gone!
pub fn present_pairs(destination_dir: &std::path::PathBuf, remove_candidate: &str, keep_candidate: &str) {
    if !destination_dir.is_dir() {
        let ret = std::fs::create_dir(destination_dir.as_path());
        if ret.is_err() {
            log::error!("Unable to create the duplicates directory in {}!", destination_dir.display());
            std::process::exit(2);
        }
    }
    let prefix = generate_random_string(5);
    let removefile_opt = std::path::Path::new(remove_candidate).file_name();
    let keepfile_opt = std::path::Path::new(remove_candidate).file_name();
    if removefile_opt.is_none() {
        log::error!("Path {} does not contain a valid file name!", remove_candidate);
        return;
    }
    if keepfile_opt.is_none() {
        log::error!("Path {} does not contain a valid file name!", keep_candidate);
        return;
    }
    let removefile_opt = removefile_opt.unwrap().to_str();
    let keepfile_opt = keepfile_opt.unwrap().to_str();
    if removefile_opt.is_none() {
        log::error!("Path {} does contain illegal characters!", remove_candidate);
        return;
    }
    if keepfile_opt.is_none() {
        log::error!("Path {} does contain illegal characters!", keep_candidate);
        return;
    }
    let removefile = removefile_opt.unwrap();
    let keepfile = keepfile_opt.unwrap();
    let remove_path = destination_dir.join(format!("{}_{}_{}", prefix, "REMOVE", removefile));
    let keep_path = destination_dir.join(format!("{}_{}_{}", prefix, "KEEP", keepfile));
    let ret = std::fs::hard_link(keep_candidate, keep_path.clone());
    if ret.is_err() {
        log::error!("Failed to create a hard link from {} to {}!", keep_candidate, keep_path.display());
        return;
    }
    let ret = std::fs::rename(remove_candidate, remove_path.clone());
    if ret.is_err() {
        log::error!("Failed to move a file from {} to {}!", remove_candidate, remove_path.display());
        return;
    }
}

/// reads a video file, creating screenshots every 10 seconds, 
/// creates a hash for each screenshot 
/// and compares it with the hashes of screenshots of existing videos.
/// Delivers existing Matches and the new data structure back to the calling program.
pub fn process_video(path: &PathBuf, store: &crate::videostore::VideoStore) 
-> crate::videocandidate::VideoCandidate 
{
    use video_rs;
    video_rs::init().unwrap();
    let location = video_rs::location::Location::File(path.clone());
    let ret = video_rs::Decoder::new(location);
    if ret.is_err() {
        log::error!("Failed to open {} for reading!", path.display());
        return crate::videocandidate::VideoCandidate::new();
    }
    let id = osstring_to_string(path.as_os_str());
    let mut decoder = ret.unwrap();
    let mut video = crate::videocandidate::VideoCandidate::from(
                &id, store.candidates.len());
    (video.width, video.height) = decoder.size();
    let duration_opt = decoder.duration();
    if duration_opt.is_err() {
        log::error!("Failed to read runtime from video file!");
        return crate::videocandidate::VideoCandidate::new();
    }
    video.runtime = duration_opt.unwrap().as_secs() as u32;
    video.framerate = decoder.frame_rate();
    for timestamp in (10..video.runtime).step_by(10) {
        decoder.seek((timestamp * 1000) as i64);
        let frame_opt = decoder.decode();
        if frame_opt.is_err() {
            log::error!("Failed to read frame at {} seconds from video!", timestamp);
            continue;
        }
        let (_timecode, frame) = frame_opt.unwrap();
        
        let rawpixels_opt = frame.as_slice();
        if rawpixels_opt.is_none() {
            log::error!("Failed to convert frame at {} seconds into raw data!", timestamp);
            continue;
        }
        let rawpixels = Vec::from(rawpixels_opt.unwrap());
        let mut rgbapixels = Vec::new();
        let mut index = 0;
        for i in 0..rawpixels.len() {
            if index < 3 {
                rgbapixels.push(rawpixels[i]);
                index += 1;
            } else {
                rgbapixels.push(255);
                index = 0;
            }
        }
        let res = image::RgbaImage::from_raw(video.width, video.height, rgbapixels);
        if res.is_some() {
            let img = res.unwrap();
            let (hash, _smallimg) = crate::hash::create_hash(&img.into());
            let candidate = crate::videocandidate::Screenshot::from(
                                        &id, 
                                        video.index as usize, 
                                        video.screenshots.len(), 
                                        timestamp, 
                                        &hash);
            video.screenshots.push(candidate);
        } else {
            log::error!("Failed to create a hash at {} seconds!", timestamp);
        }
    }
    video
}

pub fn find_similar_videos(store: &crate::videostore::VideoStore, id: &str, video: &crate::videocandidate::VideoCandidate) 
-> (crate::videomatches::VideoMatches, String, crate::videocandidate::VideoCandidate)
{
    let matches = store.query(video);
    (matches, id.to_string(), video.clone())
}

