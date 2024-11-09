use rand::Rng;
use regex::Regex;
use std::ffi::OsStr;
use std::fs;
use std::iter;
use std::path::PathBuf;
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
            let filepathopt = file.path().canonicalize();
            if filepathopt.is_err() {
                log::error!(
                    "Failed to make an absolute path for {}",
                    file.path().display()
                );
                return p;
            }
            let filepath = filepathopt.unwrap();
            if filepath.is_file() {
                let e = filepath.extension();
                if e.is_some() {
                    let ext = e.unwrap();
                    let extstring = osstring_to_string(ext).to_ascii_lowercase();
                    if extstring == "png"
                        || extstring == "jpg"
                        || extstring == "jpeg"
                        || extstring == "bmp"
                        || extstring == "gif"
                        || extstring == "webp"
                        || extstring == "tif"
                        || extstring == "tiff"
                    {
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
        .filter_map(|e| e.ok())
    {
        let filepathopt = entry.path().canonicalize();
        if filepathopt.is_err() {
            log::error!(
                "Failed to make an absolute path for {}",
                entry.path().display()
            );
            return p;
        }
        let filepath = filepathopt.unwrap();
        if filepath.is_file() {
            let e = filepath.extension();
            if e.is_some() {
                let ext = e.unwrap();
                let extstring = osstring_to_string(ext).to_ascii_lowercase();
                if extstring == "png"
                    || extstring == "jpg"
                    || extstring == "jpeg"
                    || extstring == "bmp"
                    || extstring == "gif"
                    || extstring == "webp"
                    || extstring == "tif"
                    || extstring == "tiff"
                {
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
            let filepathopt = file.path().canonicalize();
            if filepathopt.is_err() {
                log::error!(
                    "Failed to make an absolute path for {}",
                    file.path().display()
                );
                return p;
            }
            let filepath = filepathopt.unwrap();
            if filepath.is_file() {
                let e = filepath.extension();
                if e.is_some() {
                    let ext = e.unwrap();
                    let extstring = osstring_to_string(ext).to_ascii_lowercase();
                    if extstring == "mkv"
                        || extstring == "mp4"
                        || extstring == "avi"
                        || extstring == "mov"
                        || extstring == "webm"
                    {
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
        .filter_map(|e| e.ok())
    {
        let filepathopt = entry.path().canonicalize();
        if filepathopt.is_err() {
            log::error!(
                "Failed to make an absolute path for {}",
                entry.path().display()
            );
            return p;
        }
        let filepath = filepathopt.unwrap();
        if filepath.is_file() {
            let e = filepath.extension();
            if e.is_some() {
                let ext = e.unwrap();
                let extstring = osstring_to_string(ext).to_ascii_lowercase();
                if extstring == "mkv"
                    || extstring == "mp4"
                    || extstring == "avi"
                    || extstring == "mov"
                    || extstring == "webm"
                {
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

pub fn find_similar_images(
    store: &crate::store::Store,
    id: &str,
    hash: &crate::hash::Hash,
) -> (crate::matches::Matches, String, crate::hash::Hash) {
    let matches = store.query(&hash);
    (matches, id.to_string(), hash.clone())
}

/// presents the two files together renamed to have the same prefix
/// The file containing KEEP is a hard link. You can delete it without removing the original.
/// The file containing REMOVE is the original file. If you delete it, it will be gone!
pub fn present_pairs(
    destination_dir: &std::path::PathBuf,
    remove_candidate: &str,
    keep_candidate: &str,
) {
    if !destination_dir.is_dir() {
        let ret = std::fs::create_dir(destination_dir.as_path());
        if ret.is_err() {
            log::error!(
                "Unable to create the duplicates directory in {}!",
                destination_dir.display()
            );
            std::process::exit(2);
        }
    }
    let prefix = generate_random_string(5);
    let removefile_opt = std::path::Path::new(remove_candidate).file_name();
    let keepfile_opt = std::path::Path::new(remove_candidate).file_name();
    if removefile_opt.is_none() {
        log::error!(
            "Path {} does not contain a valid file name!",
            remove_candidate
        );
        return;
    }
    if keepfile_opt.is_none() {
        log::error!(
            "Path {} does not contain a valid file name!",
            keep_candidate
        );
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
        log::error!(
            "Failed to create a hard link from {} to {}!",
            keep_candidate,
            keep_path.display()
        );
        return;
    }
    let ret = std::fs::rename(remove_candidate, remove_path.clone());
    if ret.is_err() {
        log::error!(
            "Failed to move a file from {} to {}!",
            remove_candidate,
            remove_path.display()
        );
        return;
    }
}

fn prepare_video_table(compare: &Vec<crate::videocandidate::VideoCandidate>) -> String {
    let mut v =
        String::from("<table><thead>\n<tr><th>Video</th><th>Info</th></tr>\n</thead><tbody>\n");
    for i in 0..compare.len() {
        let filepath = std::path::Path::new(&compare[i].id);
        let fileformat;
        let e = filepath.extension();
        if e.is_some() {
            let ext = e.unwrap();
            let extstring = osstring_to_string(ext).to_ascii_lowercase();
            if extstring == "mkv" || extstring == "webm" {
                v = format!(
                    r##"{}<tr><td><video controls width="320" src="{}"</video></td>"##,
                    v, compare[i].id
                );
            } else if extstring == "mp4" || extstring == "mpv" {
                fileformat = "mp4".to_string();
                v = format!(
                    r##"{}<tr><td><video controls width="320"><source src="{}" type="video/{}" />/td>"##,
                    v, compare[i].id, fileformat
                );
            } else {
                v = format!(
                    r##"{}<tr><td><video controls width="320" src="{}"</video></td>"##,
                    v, compare[i].id
                );
            }
            
            v = format!(
                r##"{}<td><a href="{}">{}</a><p>Resolution: {}x{}</p><p>Duration: {}</p></td></tr>"##,
                v,
                compare[i].id,
                compare[i].id,
                compare[i].width,
                compare[i].height,
                compare[i].runtime
            );
        }
        v = format!("{}\n", v);
    }
    v = format!("{}\n</tbody></table>", v);
    v
}

pub fn present_video_matches(
    destination_dir: &std::path::PathBuf,
    compare: &Vec<crate::videocandidate::VideoCandidate>,
) {
    use build_html::*;
    use std::io::Write;
    if compare.len() < 2 {
        return;
    }
    let basefile_opt = std::path::Path::new(&compare[0].id).file_stem();
    if basefile_opt.is_none() {
        return;
    }
    if !destination_dir.is_dir() {
        let ret = std::fs::create_dir(destination_dir.as_path());
        if ret.is_err() {
            log::error!(
                "Unable to create the video matches directory in {}!",
                destination_dir.display()
            );
            std::process::exit(2);
        }
    }

    let basefile = osstring_to_string(&basefile_opt.unwrap());
    let title = format!("Comparing similar videos for {}", basefile);
    let table = prepare_video_table(compare);
    let html = build_html::HtmlPage::new()
        .with_title(&title)
        .with_header(1, &title)
        .with_html(
            HtmlElement::new(HtmlTag::Table)
                .with_attribute("id", "Matches")
                .with_paragraph(&table),
        )
        .to_html_string();
    let storefile = destination_dir.join(format!("{}.html", basefile));
    let path = std::path::Path::new(&storefile);
    let display = path.display();
    // Open a file in write-only mode, returns `io::Result<File>`
    let mut write_file = match std::fs::File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(write_file) => write_file,
    };
    let ret = write_file.write(&html.as_bytes());
    if ret.is_err() {
        log::error!("failed to write html data to file {}!", storefile.display())
    }
    let ret = write_file.flush();
    if ret.is_err() {
        log::error!(
            "failed to empty the write buffer for html data to file {}!",
            &storefile.display()
        )
    }
}

struct VideoMetadata {
    duration: u32,
    width: u32,
    height: u32,
    framerate: f32,
}

impl Default for VideoMetadata {
    fn default() -> VideoMetadata {
        VideoMetadata {
            duration: 0,
            width: 0,
            height: 0,
            framerate: 0.0,
        }
    }
}

pub fn string_to_uint(mystring: &str) -> u32 {
    let u = 0;
    if mystring.trim().len() == 0 {
        return u;
    }
    match u32::from_str_radix(mystring, 10) {
        Ok(ret) => return ret,
        Err(_) => {
            log::warn!("Parsing of {} into Integer failed\n", mystring)
        }
    }
    u
}

pub fn string_to_float(mystring: &str) -> f32 {
    let f = 0.0;
    if mystring.trim().len() == 0 {
        return f;
    }
    match mystring.parse::<f32>() {
        Ok(ret) => return ret,
        Err(_) => {
            log::warn!("Parsing of {} into flaot failed\n", mystring)
        }
    }
    f
}

fn video_metadata(filepath: &str) -> VideoMetadata {
    let mut meta = VideoMetadata {
        ..Default::default()
    };
    let ffmpeg_output = std::process::Command::new("ffmpeg")
        .args(["-i", filepath])
        .output()
        .expect("failed to execute process");
    match String::from_utf8(ffmpeg_output.stderr) {
        Ok(message) => {
            let re_duration = Regex::new(
                r"(?i)\s*Duration:\s+(?P<hours>\d+):(?P<minutes>\d+):(?P<seconds>\d+).\d+\s*,",
            )
            .unwrap();
            if re_duration.is_match(&message) {
                let caps = re_duration.captures(&message).unwrap();
                let hours = string_to_uint(&caps["hours"]);
                let minutes = string_to_uint(&caps["minutes"]);
                let seconds = string_to_uint(&caps["seconds"]);
                meta.duration = hours * 3600 + minutes * 60 + seconds;
            }
            let re_video =
                Regex::new(r"(?i), (?P<width>\d+)x(?P<height>\d+).*, (?P<fps>\d+) fps").unwrap();
            if re_video.is_match(&message) {
                let caps = re_video.captures(&message).unwrap();
                meta.width = string_to_uint(&caps["width"]);
                meta.height = string_to_uint(&caps["height"]);
                meta.framerate = string_to_float(&caps["fps"]);
            }
        }
        Err(error) => log::error!("Error: {}", error),
    }

    meta
}

fn create_screenshots(
    filepath: &str,
    video_id: u32,
    num_videos: u32,
    num_seconds_between_screenshots: u32,
) -> Result<(Vec<crate::videocandidate::Screenshot>, VideoMetadata), image::ImageError> {
    let mut v = Vec::new();

    let meta = video_metadata(filepath);
    let mut screenshot_id: usize = 1;
    let mut timecode = num_seconds_between_screenshots;
    let outputpattern = format!("{}_%03d.jpeg", filepath);
    let output = format!("{}_001.jpeg", filepath);
    let outputpath = std::path::Path::new(&output);
    while timecode < meta.duration {
        let time = timecode_to_ffmpeg_time(timecode);
        if outputpath.is_file() {
            let ret = std::fs::remove_file(&output);
            if ret.is_err() {
                log::error!("could not delete file {}", output);
            }
        }

        let ffmpeg_output = std::process::Command::new("ffmpeg")
            .args([
                "-ss",
                &time,
                "-i",
                filepath,
                "-frames:v",
                "1",
                "-q:v",
                "2",
                &outputpattern,
            ])
            .output()
            .expect("failed to execute process");
        match String::from_utf8(ffmpeg_output.stderr) {
            Ok(_message) => log::warn!(
                "Processing video {} of {} path {} timecode {}",
                video_id + 1,
                num_videos,
                filepath,
                time
            ),
            Err(error) => log::error!("Error: {}", error),
        }
        if !outputpath.is_file() {
            log::error!("Failed to create screenshot: {}", format!("ffmpeg -ss {} -i {} -frames:v 1 -q:v 2 ", time, filepath));
            log::error!("File {} seems to be defective from position {}%", filepath, (timecode / meta.duration) * 100);
            break;
        }
        let res = image::ImageReader::open(outputpath);
        if res.is_ok() {
            let dynimg = res.unwrap().decode();
            if dynimg.is_err() {
                return Ok((v, meta));
            }
            let img = dynimg.unwrap();
            let (hash, _smallimg) = crate::hash::create_hash(&img.into());
            let ss = crate::videocandidate::Screenshot::from(
                filepath,
                video_id as usize,
                screenshot_id,
                timecode,
                &hash,
            );
            v.push(ss);
        }
        timecode += num_seconds_between_screenshots;
        screenshot_id += 1;
    }
    if outputpath.is_file() {
        let ret = std::fs::remove_file(&output);
        if ret.is_err() {
            log::error!("could not delete file {}", output);
        }
    }

    Ok((v, meta))
}

fn timecode_to_ffmpeg_time(timecode: u32) -> String {
    let hours = timecode / 3600;
    let minutes = (timecode - hours * 3600) / 60;
    let seconds = timecode - hours * 3600 - minutes * 60;
    format!("{:02}:{:02}:{:02}.000", hours, minutes, seconds)
}

/// reads a video file, creating screenshots every 10 seconds,
/// creates a hash for each screenshot
/// and compares it with the hashes of screenshots of existing videos.
/// Delivers existing Matches and the new data structure back to the calling program.
pub fn process_video(
    path: &PathBuf,
    video_id: usize,
    num_videos: u32,
    num_seconds_between_screenshots: u32,
) -> crate::videocandidate::VideoCandidate {
    let id = osstring_to_string(path.as_os_str());
    let mut video = crate::videocandidate::VideoCandidate::from(&id, video_id);

    match create_screenshots(&id, video.index, num_videos, num_seconds_between_screenshots) {
        Ok((v, meta)) => {
            video.width = meta.width;
            video.height = meta.height;
            video.runtime = meta.duration;
            video.framerate = meta.framerate;
            for ss in v {
                video.screenshots.push(ss);
            }
            // correct the runtime if the last screenshots could not be read ( possibly defective video )
            let screentime = video.screenshots.len() as u32 * 10;
            if video.runtime - screentime > 10 {
                video.runtime = screentime + 5;
            }
        }
        Err(error) => log::error!("Failed to create screenshots for {}: {}", path.display(), error),
    }
    video
}

pub fn find_similar_videos(
    store: &mut crate::videostore::VideoStore,
    client: &mut rusqlite::Connection,
    id: &str,
    video: &crate::videocandidate::VideoCandidate,
) -> (
    crate::videomatches::VideoMatches,
    String,
    crate::videocandidate::VideoCandidate,
) {
    let matches = store.query(client, video);
    (matches, id.to_string(), video.clone())
}
