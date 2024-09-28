use std::fs;
use std::path::PathBuf;
use std::ffi::OsStr;
use walkdir::WalkDir;

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

/// reads an image, creates a hash and compares it with the existing hashes.
/// Delivers existing Matches and the hash back to the calling program.
pub fn process_image(p: &PathBuf, store: &crate::store::Store) -> (crate::matches::Matches, crate::hash::Hash) {
    let res = image::ImageReader::open(p);
    if res.is_ok() {
        let dynimg = res.unwrap().decode();
        if dynimg.is_err() {
            return (crate::matches::Matches::new(), crate::hash::Hash::new());
        }
        let img = dynimg.unwrap();
        let (hash, _smallimg) = crate::hash::create_hash(&img.into());
        let matches = store.query(&hash);
        return (matches, hash);
    }
    (crate::matches::Matches::new(), crate::hash::Hash::new())
}

