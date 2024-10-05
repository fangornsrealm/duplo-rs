//use array2d::{Array2D, Error};
use std::io::{Read, Write};
use log;

static WEIGHTS: [[f64;6];3] = [[5.00_f64, 0.83, 1.01, 0.52, 0.47, 0.30], 
                [19.21, 1.26, 0.44, 0.53, 0.28, 0.14],
                [34.37, 0.36, 0.45, 0.14, 0.18, 0.27]];
pub const IMAGESCALE: u32 = 128;
pub const INDICESMAX: u32 = 98400;
pub static TOPCOEFS: i32 = 40;
static WEIGHTSUMS: [f64;6] = [58.58 as f64, 2.45, 1.9, 1.19, 0.93, 0.71];
pub static CTRL_C_PRESSED: bool = false;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ScreenshotIndex {
    pub video_id: u32,       // index of this video
    pub screenshot_id: u32, // index of screenshot in this video
    timecode: u32,           // time in seconds
}
impl Default for ScreenshotIndex {
    fn default() -> ScreenshotIndex {
        ScreenshotIndex {
            video_id: 0,
            screenshot_id: 0,
            timecode: 0,
       }
    }
}

impl ScreenshotIndex {
    pub fn new() -> Self {
        ScreenshotIndex {..Default::default()}
    }

    pub fn from(video_id: u32, screenshot_id: u32, timecode: u32) -> Self {
        let mut v = ScreenshotIndex {..Default::default()};
        v.video_id = video_id;
        v.screenshot_id = screenshot_id;
        v.timecode = timecode;
        v
    }

    // encode the data structure to binary stream
    pub fn encode(&self, to: &mut Vec<u8>) {
        crate::marshal::store_u32(self.video_id, to);
        crate::marshal::store_u32(self.screenshot_id, to);
        crate::marshal::store_u32(self.timecode, to);
    }

    // decode data structure from binary stream
    pub fn decode(&mut self, from: &mut std::io::Cursor<Vec<u8>>) {
        self.video_id = crate::marshal::restore_u32(from);
        self.screenshot_id = crate::marshal::restore_u32(from);
        self.timecode = crate::marshal::restore_u32(from);
    }
}


pub struct Sequence {
    pub video_id: u32,       // index of this video
    pub last_timecode: u32,  // time in seconds
    pub sequence: Vec<u32>,
}
impl Default for Sequence {
    fn default() -> Sequence {
        Sequence {
            video_id: 0,
            last_timecode: 0,
            sequence: Vec::new(),
       }
    }
}

impl Sequence {
    pub fn new() -> Self {
        Sequence {..Default::default()}
    }

    pub fn from(video_id: u32, timecode: u32, screenshot_id: u32) -> Self {
        let mut v = Sequence {..Default::default()};
        v.video_id = video_id;
        v.last_timecode = timecode;
        v.sequence.push(screenshot_id);
        v
    }
}

/// Store is a data structure that holds references to images. It holds visual
/// hashes and references to the images but the images themselves are not held
/// in the data structure.
///
/// A general limit to the store is that it can hold no more than 4,294,967,295
/// images. This is to save RAM space but may be easy to extend by modifying its
/// data structures to hold uint64 indices instead of uint32 indices.
///
/// candidates contains all images in the store or, rather, the candidates for a query.
/// 
/// All IDs in the store, mapping to candidate indices.
/// 
/// indices  contains references to the images in the store. It is a slice
/// of slices which contains image indices (into the "candidates" slice).
/// Use the following formula to access an index slice:
///
///	store.indices[sign*ImageScale*ImageScale*haar.ColourChannels + coefIdx*haar.ColourChannels + channel]
///
/// where the variables are as follows:
///
///	sign: Either 0 (positive) or 1 (negative)
///	coefIdx: The index of the coefficient (from 0 to (ImageScale*ImageScale)-1)
///	channel: The colour channel (from 0 to haar.ColourChannels-1)
///     
/// sentivity (Hamming distance threshold) for the perceptual hashes. 
/// Larger values will allow more images to be seen as *similar*
/// 
/// modified tells Whether this store was modified since it was loaded/created.
///
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct VideoStore {
	//sync.RWMutex,

	pub candidates: Vec<crate::videocandidate::VideoCandidate>,
    pub num_images: u32,

	pub ids: std::collections::BTreeMap<String, usize>,

	pub indices: Vec<Vec<ScreenshotIndex>>,

    sensitivity: f64,

    pub modified: bool,
}

impl Default for VideoStore {
    fn default() -> VideoStore {
        VideoStore {
            candidates: Vec::new(),
            num_images: 0,
            ids: std::collections::BTreeMap::new(),
            indices: Vec::new(),
            sensitivity: -100.0,
            modified: false,
       }
    }
}

impl VideoStore {
    pub fn new(sensitivity: f64) -> Self {
        let mut v = VideoStore {..Default::default()};
        v.sensitivity = sensitivity;
        for _ in 0..INDICESMAX {
            // prefill the outer Vec so we can directly access the inner Vec
            v.indices.push(Vec::new()); 
        } 
        v
    }

    pub fn has(&self, id: &str) -> bool {
        if self.ids.contains_key(id) {
            return true;
        }
        false
    }

    /// Add adds an image (via its hash) to the store. 
    /// The provided ID of the video and the index of the screenshot is the value
    /// that will be returned as the result of a similarity query. If an ID is
    /// already in the store, it is not added again.
    pub fn add(&mut self, id: &str, video: &crate::videocandidate::VideoCandidate, _runtime: u32) {
        if self.ids.contains_key(id) {
            return;
        }
        let index;
        if !self.ids.contains_key(id) {
            index = self.candidates.len();
            self.candidates.push(video.clone());
            self.candidates[index].index = index as u32;
            self.ids.insert(id.to_string(), index);
        }
        for i in 0..video.screenshots.len() {
            let hash = &video.screenshots[i].hash;
            if hash.matrix.coefs.len() < 2 {
                return;
            }
            for coefindex in 1..hash.matrix.coefs.len() {
                let coef = &hash.matrix.coefs[coefindex];
                for colorindex in 0..coef.c.len() {
                    let colorcoef = coef.c[colorindex];
                    if colorcoef.abs() < hash.thresholds.c[colorindex] {
                        continue;
                    }
                    let mut sign = 0;
                    if colorcoef < 0.0 {
                        sign = 1;
                    }
                    let location = sign * IMAGESCALE *IMAGESCALE * crate::haar::COLOURCHANNELS 
                                        + coefindex as u32 * crate::haar::COLOURCHANNELS + colorindex as u32;
                    self.indices[location as usize].push(
                                    ScreenshotIndex::from(
                                            video.screenshots[i].video_id, 
                                            video.screenshots[i].screenshot_id, 
                                            video.runtime));
                }
            }
        }
        self.modified = true;
    }

    pub fn ids(&self) -> Vec<String> {
        let mut v = Vec::new();
        for (id, _) in self.ids.iter() {
            v.push(id.clone());
        }
        v
    }

    /// Delete removes an image from the store so it will not be returned during a
    /// query anymore. Note that the candidate slot still remains occupied but its
    /// index will be removed from all index lists. This also means that Size() will
    /// not decrease. This is an expensive operation. If the provided ID could not be
    /// found, nothing happens.
    pub fn delete(&mut self, id: &str) {
        if !self.ids.contains_key(id) {
            return;
        }
        // Get the index.
        let index = self.ids[id];
        self.modified = true;
        // clear the entry in the candidates list without deleting it
        self.num_images -= self.candidates[index].screenshots.len() as u32;
        self.candidates[index] = crate::videocandidate::VideoCandidate::new();
        
        self.ids.remove(id);
        // Remove from all index lists.
        for containerindex in 0..self.indices.len() {
            for imageindex in (0..self.indices[containerindex].len()).rev() {
                if self.indices[containerindex][imageindex].video_id == index as u32 {
                    self.indices[containerindex].remove(imageindex);
                }
            }
        }
    }

    /// Exchange exchanges the ID of an image for a new one. If the old ID could not
    /// be found, nothing happens. If the new ID already existed prior to the
    /// exchange, the function returns immediately.
    /// 
    pub fn exchange(&mut self, oldid: &str, newid: &str) -> bool {
        if !self.ids.contains_key(oldid) {
            return false;
        }
        if self.ids.contains_key(newid) {
            return false;
        }
        let index = self.ids[oldid];
        // update the ids
        self.ids.remove(oldid);
        self.ids.insert(newid.to_string(), index);

        // Update the candidate.
        self.candidates[index].id = newid.to_string();
        self.modified = true;
        true
    }

    /// Find all similar screenshots for a single screenshot
    fn search_matches(&self, hash: &crate::hash::Hash) -> crate::videomatches::VideoMatches {
        let mut ms = crate::videomatches::VideoMatches::new();
        // build a mapping of video, screenshot to a global image index
        if self.candidates.len() == 0 {
            return ms;
        }
        let mut video_screenshot_to_score_map = Vec::new();
        let mut scoreid_to_video_screenshot_map = Vec::new();
        let mut sindex: usize = 0;
        for video_id in 0..self.candidates.len() {
            video_screenshot_to_score_map.push(Vec::new());
            for screenshot_id in 0..self.candidates[video_id].screenshots.len() {
                video_screenshot_to_score_map[video_id].push(sindex);
                scoreid_to_video_screenshot_map.push(ScreenshotIndex::from(video_id as u32, screenshot_id as u32, 0));
                sindex += 1;
            }
        }
        // prepare the scoring vector where we can rate any existing screenshot
        let mut scores: Vec<f64> = Vec::new();
        scores.reserve(sindex);
        for _ in 0..sindex {
            scores.push(f64::NAN);
        }
        
        // Examine hash buckets.
        for coefindex in 0..hash.matrix.coefs.len() {
            let coef = &hash.matrix.coefs[coefindex];
            if coefindex == 0 {
                continue; // igore scaling function coefficient for now
            }
            // Calculate the weight bin outside the main loop.
            let y = coefindex / hash.matrix.width as usize;
            let x = coefindex % hash.matrix.width as usize;
            let mut bin = y;
            if x > y {
                bin = x;
            }
            if bin > 5 {
                bin = 5;
            }
            for colorindex in 0..coef.c.len() {
                let colorcoef = coef.c[colorindex];
                if colorcoef.abs() < hash.thresholds.c[colorindex] {
                    // Coef is too small. Ignore.
                    continue;
                }
                // At this point, we have a coefficient which we want to look up
                // in the index buckets.
                let mut sign = 0;
                if colorcoef < 0.0 {
                    sign = 1;
                }
                let location = sign * IMAGESCALE *IMAGESCALE * crate::haar::COLOURCHANNELS 
                                    + coefindex as u32 * crate::haar::COLOURCHANNELS + colorindex as u32;
                let arr = &self.indices[location as usize];
                for i in 0..arr.len() {
                    let matchscreenshot = arr[i].clone();
                    if matchscreenshot.video_id as usize > video_screenshot_to_score_map.len() - 1  
                        || video_screenshot_to_score_map[matchscreenshot.video_id as usize].len() == 0 {
                            log::error!("Failed to lookup the video screenshot index");
                            return ms;
                    }
                    let sindex = video_screenshot_to_score_map[matchscreenshot.video_id as usize][matchscreenshot.screenshot_id as usize];
                    if scores[sindex].is_nan() {
                        // calculated initial score
                        let mut score: f64  = 0.0;
                        for colorid in 0..coef.c.len() {
                            score += WEIGHTS[colorid][0];
                        }
                        scores[sindex] = score;
                    }
                    // At this point, we have an entry in matches. Simply subtract the
                    // corresponding weight.
                    scores[sindex as usize] -= WEIGHTSUMS[bin];
                }
            }
        }
        // Create matches. If the dhash_distance is lower than the sensitivity threshold it is a *valid* match.
        for index in 0..scores.len() {
            if !scores[index].is_nan() {
                let mut m = crate::videomatches::VideoMatch::new();
                let video_id = scoreid_to_video_screenshot_map[index].video_id;
                let screenshot_id = scoreid_to_video_screenshot_map[index].screenshot_id;
                m.id = self.candidates[video_id as usize].id.clone();
                m.video_id = video_id;
                m.screenshot_id = screenshot_id;
                let candidate = &self.candidates[video_id as usize].screenshots[screenshot_id as usize];
                m.timecode = candidate.timecode;
                m.score = scores[index];
                m.ratio_diff = candidate.hash.ratio.log(10.0).abs() - hash.ratio.log(10.0);
                m.dhash_distance = crate::hamming::hamming_distance(
                                        candidate.hash.dhash[0], hash.dhash[0])
                                             + crate::hamming::hamming_distance(
                                        candidate.hash.dhash[1], hash.dhash[1]);
                m.histogram_distance = crate::hamming::hamming_distance(
                    candidate.hash.histogram, hash.histogram);
                if m.score < self.sensitivity {
                    ms.m.push(m);                
                }
            }
        }
        // sort the vector so the first match is the one with the lowest value --> the best match
        // The number of matches should be small, so the bubble sort is about the fastest algorithm.
        ms.sort();
        ms    
    }

    /// create a score value for a *similar* video in comparison to duration, resulution. ...
    /// and return a Match
    fn rate_match(&self, 
        matches: &crate::videomatches::VideoMatches, 
        new_video: &crate::videocandidate::VideoCandidate, 
        match_id: u32, 
        num_matches: usize
    ) -> crate::videomatches::VideoMatch 
    {
        let mut m = crate::videomatches::VideoMatch::new();
         for i in 0..matches.m.len() {
            if matches.m[i].video_id == match_id {
                let matchedvideo = matches.m[i].clone();
                m.id = matchedvideo.id.clone();
                m.video_id = matchedvideo.video_id;
                m.screenshot_id = matchedvideo.screenshot_id;
                m.timecode = matchedvideo.timecode;
                let matched = &self.candidates[match_id as usize];
                m.score = -60.0                                                                            // base value
                            - 100.0 * (num_matches as f64 * 10.0) / matched.runtime as f64                 // the longer the similar part, the better the match
                            + ((new_video.width - matched.width) * (new_video.width - matched.width)) as f64 // if the resolution is higher the match gets better
            }
        }

        m
    }

    /// Query performs a similarity search on the given image hashes and returns
    /// all potential matches. The returned slice will sort the match with the best score as its
    /// first element.
    /// 
    /// Videos consist of one screenshot every ten seconds. 
    /// A Match contains a portion of at least a miunte (six similar screenshots in a row)
    /// The longer the sequence the better the match.
    /// 
    pub fn query(&self, video: &crate::videocandidate::VideoCandidate) -> crate::videomatches::VideoMatches {
        let mut ms = crate::videomatches::VideoMatches::new();
        if self.candidates.len() == 0 {
            return ms;
        }
        let mut sequences = std::collections::BTreeMap::new();
        let mut active_sequence_counter = std::collections::BTreeMap::new();
        
        // search for each screenshot of the current video in the store
        for screenshot_id in 0..video.screenshots.len() {
            let hash = &video.screenshots[screenshot_id].hash;
            let matches = self.search_matches(hash);
            let mut previous_videos = std::collections::BTreeSet::new();
            for (key, _) in sequences.iter() {
                previous_videos.insert(*key);
            }
            let mut new_videos = std::collections::BTreeSet::new();
            for i in 0..matches.len() {
                let video_id = matches.m[i].video_id;
                new_videos.insert(video_id);
                let screenshot_id = matches.m[i].screenshot_id;
                if !active_sequence_counter.contains_key(&video_id) {
                    active_sequence_counter.insert(video_id, 1 as u32);
                } else {
                    if let Some(x) = active_sequence_counter.get_mut(&video_id) {
                        *x += 1;
                    }
                }
                if !sequences.contains_key(&video_id) {
                    sequences.insert(video_id, Vec::from([screenshot_id]));
                } else {
                    if let Some(x) = sequences.get_mut(&video_id) {
                        if x[x.len()-1] + 1 == screenshot_id {
                            x.push(screenshot_id);
                        } else {
                            // broken sequence
                            x.clear();
                            x.push(screenshot_id);
                        }
                    }
                }
            }
            // remove the videos from the sequence that were not a match
            let dropped_videos = previous_videos.difference(&new_videos);
            for id in dropped_videos {
                // check if the sequence was longer than a minute
                if sequences[id].len() > 5 {
                    let videomatch = self.rate_match(&matches, &video, *id, sequences[id].len());
                    if videomatch.id.len() != 0 {
                        ms.m.push(videomatch);
                    }
                }
                sequences.remove(id);
            }
            new_videos.clear();
            
        }
        ms
    }

    pub fn size(&self) -> usize {
        self.candidates.len()
    }

    pub fn modified(&self) -> bool {
        self.modified
    }

    // encode the data structure to binary stream
    pub fn encode(&self, to: &mut Vec<u8>) {
        let s = self.candidates.len();
        crate::marshal::store_usize(s, to);
        for elem in &self.candidates {
            elem.encode(to);
        }
        crate::marshal::store_hash_string_usize(&self.ids, to);
        let s = self.indices.len();
        crate::marshal::store_usize(s, to);
        crate::marshal::store_f64(self.sensitivity, to);
        let s = self.indices.len();
        crate::marshal::store_usize(s, to);
        for v in &self.indices {
            let t = v.len();
            crate::marshal::store_usize(t, to);
            for elem in v {
                elem.encode(to);
            }
        }
        crate::marshal::store_bool(self.modified, to);
    }

    // decode data structure from binary stream
    pub fn decode(&mut self, from: &mut std::io::Cursor<Vec<u8>>) {
        let s = crate::marshal::restore_usize(from);
        for _i in 0..s {
            let mut elem = crate::videocandidate::VideoCandidate::new();
            elem.decode(from);
            self.candidates.push(elem);
        }
        self.ids.extend(crate::marshal::restore_hash_string_usize(from));
        self.sensitivity = crate::marshal::restore_f64(from);
        let s = crate::marshal::restore_usize(from);
        for _i in 0..s {
            let mut v = Vec::new();
            let t = crate::marshal::restore_usize(from);
            for _i in 0..t {
                let mut elem = ScreenshotIndex::new();
                elem.decode(from);
                v.push(elem);
            }
            self.indices.push(v);
        }
        self.modified = crate::marshal::restore_bool(from);
        self.modified = false;
    }

    // Write binary stream to file
    pub fn dump_binary(&mut self, storefile: &str) {
        let mut buffer = Vec::new();
        self.encode(&mut buffer);
        let path = std::path::Path::new(&storefile);
        let display = path.display();
        // Open a file in write-only mode, returns `io::Result<File>`
        let mut write_file = match std::fs::File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(write_file) => write_file,
        };
        let ret = write_file.write(&mut buffer);
        if ret.is_err() {
            log::error!("failed to write binary data to file {}!", &storefile)
        }
        let ret = write_file.flush();
        if ret.is_err() {
            log::error!(
                "failed to empty the write buffer for binary data to file {}!",
                &storefile
            )
        }
    }

    // read binary stream from file
    pub fn slurp_binary(&mut self, storefile: &str) {
        let path = std::path::Path::new(&storefile);
        let display = path.display();
        let mut input_file = match std::fs::File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(read_file) => read_file,
        };
        let mut buf = Vec::new();
        let ret = input_file.read_to_end(&mut buf);
        if ret.is_err() {
            log::error!("Failed to read test file back in.");
            return;
        }
        let mut read_file = std::io::Cursor::new(buf);
        self.decode(&mut read_file);
    }

}

