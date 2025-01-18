//use array2d::{Array2D, Error};
use log;
use std::io::{Read, Write};
use rusqlite::{Connection, Result, params};

static WEIGHTS: [[f64; 6]; 3] = [
    [5.00_f64, 0.83, 1.01, 0.52, 0.47, 0.30],
    [19.21, 1.26, 0.44, 0.53, 0.28, 0.14],
    [34.37, 0.36, 0.45, 0.14, 0.18, 0.27],
];
pub const IMAGESCALE: u32 = 128;
pub const INDICESMAX: u32 = 98400;
pub static TOPCOEFS: i32 = 40;
static WEIGHTSUMS: [f64; 6] = [58.58 as f64, 2.45, 1.9, 1.19, 0.93, 0.71];
pub static CTRL_C_PRESSED: bool = false;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CandidateCache {
    pub map: std::collections::BTreeMap<u32, crate::videocandidate::VideoCandidate>,
    pub fifo: std::collections::LinkedList<u32>,
    pub max_candidates: usize,
}

impl Default for CandidateCache {
    fn default() -> CandidateCache {
        CandidateCache {
            map: std::collections::BTreeMap::new(),
            fifo: std::collections::LinkedList::new(),
            max_candidates: 0,
        }
    }
}

impl CandidateCache {
    pub fn new(max_candidates: usize) -> Self {
        let mut v = CandidateCache {
            ..Default::default()
        };
        v.max_candidates = max_candidates;
        v
    }

    pub fn contains(&self, video_id: u32) -> bool {
        if self.map.contains_key(&video_id) {
            return true;
        }
        false
    }

    pub fn add(&mut self, video: crate::videocandidate::VideoCandidate) {
        if self.max_candidates == 0 {
            return;
        }
        if self.fifo.len() == self.max_candidates {
            // drop the oldest entry
            let drop_id = self.fifo.pop_front();
            if drop_id.is_some() {
                self.map.remove(&drop_id.unwrap());
            }
        }
        self.fifo.push_back(video.index);
        self.map.insert(video.index, video);
    }

    // encode the data structure to binary stream
    pub fn encode(&self, to: &mut Vec<u8>) {
        crate::marshal::store_usize(self.map.len(), to);
        for id in self.fifo.iter() {
            self.map[id].encode(to);
        }
        crate::marshal::store_usize(self.fifo.len(), to);
        for id in self.fifo.iter() {
            crate::marshal::store_u32(*id, to);
        }
        crate::marshal::store_usize(self.max_candidates, to);
    }

    // decode data structure from binary stream
    pub fn decode(&mut self, from: &mut std::io::Cursor<Vec<u8>>) {
        let maplen = crate::marshal::restore_usize(from);
        for _ in 0..maplen {
            let mut candidate = crate::videocandidate::VideoCandidate::new();
            candidate.decode(from);
            self.map.insert(candidate.index, candidate);
        }
        let fifolen = crate::marshal::restore_usize(from);
        for _ in 0..fifolen {
            self.fifo.push_back(crate::marshal::restore_u32(from));
        }
        self.max_candidates = crate::marshal::restore_usize(from);
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ScreenshotIndex {
    pub id: String,
    pub video_id: u32,
    pub screenshot_id: u32,
    pub runtime: u32,
}

impl Default for ScreenshotIndex {
    fn default() -> ScreenshotIndex {
        ScreenshotIndex {
            id: String::new(),
            video_id: 0,
            screenshot_id: 0,
            runtime: 0,
        }
    }
}

impl ScreenshotIndex {
    pub fn new() -> Self {
        ScreenshotIndex {
            ..Default::default()
        }
    }

    pub fn from(filename: &str, video_id: u32, screenshot_id: u32, runtime: u32) -> Self {
        let mut v = ScreenshotIndex {
            ..Default::default()
        };
        v.id = filename.to_string();
        v.video_id = video_id;
        v.screenshot_id = screenshot_id;
        v.runtime = runtime;
        v
    }

    // encode the data structure to binary stream
    pub fn encode(&self, to: &mut Vec<u8>) {
        crate::marshal::store_string(&self.id, to);
        crate::marshal::store_u32(self.video_id, to);
        crate::marshal::store_u32(self.screenshot_id, to);
        crate::marshal::store_u32(self.runtime, to);
    }

    // decode data structure from binary stream
    pub fn decode(&mut self, from: &mut std::io::Cursor<Vec<u8>>) {
        self.id = crate::marshal::restore_string(from);
        self.video_id = crate::marshal::restore_u32(from);
        self.screenshot_id = crate::marshal::restore_u32(from);
        self.runtime = crate::marshal::restore_u32(from);
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Sequence {
    pub video_id: u32,      // index of this video
    pub last_timecode: u32, // time in seconds
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
        Sequence {
            ..Default::default()
        }
    }

    pub fn from(video_id: u32, timecode: u32, screenshot_id: u32) -> Self {
        let mut v = Sequence {
            ..Default::default()
        };
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
/// start_directory the directory the direcotory list or walk started. Also the directory possible matches will be presented in
/// num_threads     number of threads scanning videos
///
/// modified tells Whether this store was modified since it was loaded/created.
/// num_seconds_between screenshots   default 10. Increase for quicker scanning and less resources.
/// min_similar_screenshots_in_sequence  number of similar screenshots in a row that have to match to count as similar video. Default: 6 or 1 minute
///
/// candidate_cache   hold N last used video data in RAM so we don't have to hit the database all the time.
///                   blocks N * <data_size> for the runtime of the program! Where <data_size> is 100 MB or more, depending on the runtime of the video.
///                   This will easlity eat 10 GB or your RAM for 50 < N < 150.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct VideoStore {
    //sync.RWMutex,
    pub num_candidates: u32,
    //pub candidates: Vec<crate::videocandidate::VideoCandidate>,
    pub num_images: u32,

    pub ids: std::collections::BTreeMap<String, usize>,
    pub video_ids: std::collections::BTreeMap<u32, usize>,

    //pub indices: Vec<Vec<ScreenshotIndex>>,
    pub num_indices: std::collections::BTreeMap<u32, usize>,
    pub num_index_values: u32,
    sensitivity: f64,
    pub start_directory: String,
    pub num_threads: u32,

    pub modified: bool,
    pub num_seconds_between_screenshots: u32, 
    pub min_similar_screenshots_in_sequence: u32,
    candidate_cache: CandidateCache,
}

impl Default for VideoStore {
    fn default() -> VideoStore {
        VideoStore {
            //candidates: Vec::new(),
            num_candidates: 0,
            num_images: 0,
            ids: std::collections::BTreeMap::new(),
            video_ids: std::collections::BTreeMap::new(),
            //indices: Vec::new(),
            num_indices: std::collections::BTreeMap::new(),
            num_index_values: 0,
            start_directory: ".".to_string(),
            num_threads: 1,
            sensitivity: -100.0,
            modified: false,
            num_seconds_between_screenshots: 10, 
            min_similar_screenshots_in_sequence: 6,
            candidate_cache: CandidateCache::new(100),
        }
    }
}

impl VideoStore {
    pub fn new(
        connection: &mut rusqlite::Connection,
        sensitivity: f64,
        start_directory: &str,
        num_threads: u32,
        num_seconds_between_screenshots: u32,
        min_similar_screenshots_in_sequence: u32,
        max_candidates_in_cache: usize, 
    ) -> Self {
        let mut v = VideoStore {
            ..Default::default()
        };
        v.sensitivity = sensitivity;
        v.start_directory = start_directory.to_string();
        v.num_threads = num_threads;
        v.num_seconds_between_screenshots = num_seconds_between_screenshots;
        v.min_similar_screenshots_in_sequence = min_similar_screenshots_in_sequence;
        v.candidate_cache.max_candidates = max_candidates_in_cache;
        let query = "SELECT * FROM videostore_parameters WHERE config_id = 1";
        match connection.prepare(query) {
            Ok(mut statement) => {
                match statement.query(params![]) {
                    Ok(mut rows) => {
                        loop { 
                            match rows.next() {
                                Ok(Some(row)) => {
                                    let s_opt = row.get(1);
                                    if s_opt.is_ok() {
                                        v.sensitivity = s_opt.unwrap();
                                    }
                                    let s_opt = row.get(2);
                                    if s_opt.is_ok() {
                                        v.start_directory = s_opt.unwrap();
                                    }
                                    let s_opt = row.get(3);
                                    if s_opt.is_ok() {
                                        v.num_threads = s_opt.unwrap();
                                    }
                                    let s_opt = row.get(4);
                                    if s_opt.is_ok() {
                                        v.num_seconds_between_screenshots = s_opt.unwrap();
                                    }
                                    let s_opt = row.get(5);
                                    if s_opt.is_ok() {
                                        v.min_similar_screenshots_in_sequence = s_opt.unwrap();
                                    }
                                    let s_opt = row.get(6);
                                    if s_opt.is_ok() {
                                        v.candidate_cache.max_candidates = s_opt.unwrap();
                                    }
                                    break;
                                },
                                Ok(None) => {
                                    log::warn!("No data read from parameters.");
                                    break;
                                },
                                Err(error) => {
                                    log::error!("Failed to read a row from parameters: {}", error);
                                    break;
                                }
                            }
                        }
                    },
                    Err(err) => {
                        log::error!("could not read line from parameter database: {}", err);
                    }
                }
                if v.sensitivity != sensitivity 
                || v.num_seconds_between_screenshots != num_seconds_between_screenshots 
                || v.min_similar_screenshots_in_sequence != min_similar_screenshots_in_sequence
                || v.candidate_cache.max_candidates != max_candidates_in_cache {
                    // change the parameters in the database
                    let query_delete = "DELETE FROM videostore_parameters WHERE config_id = 1";
                    match connection.execute(query_delete, params![]) {
                        Ok(retval) => log::warn!("Deleted {} data from parameters.", retval),
                        Err(error) => {
                            log::error!("Failed to delete data from parameter database: {}", error);
                            return v;
                        }
                    }
                    let sensitivity_u32 = v.sensitivity as u32;
                    match connection.execute(
                        "INSERT INTO videostore_parameters (config_id, sensitivity, start_directory, num_threads, num_seconds_between_screenshots, min_similar_screenshots_in_sequence, max_candidates_in_cache) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                        params![&1, &sensitivity_u32, &v.start_directory, &v.num_threads, &v.num_seconds_between_screenshots, &v.min_similar_screenshots_in_sequence, &v.candidate_cache.max_candidates],
                    ) {
                        Ok(retval) => log::warn!("Inserted {} data into parameter.", retval),
                        Err(error) => {
                            log::error!("Failed to insert data into parameter database: {}", error);
                            return v;
                        }
                    }
                    v.sensitivity = sensitivity;
                    v.start_directory = start_directory.to_string();
                    v.num_threads = num_threads;
                    v.num_seconds_between_screenshots = num_seconds_between_screenshots;
                    v.min_similar_screenshots_in_sequence = min_similar_screenshots_in_sequence;
                    v.candidate_cache.max_candidates = max_candidates_in_cache;
                }
            },
            Err(error) => {
                log::error!("Could not prepare statement {}: {}", query, error);
                return v;
            }
        }
        let query_candidates = "SELECT candidate_id, filename, video_id FROM videostore_candidates";
        match connection.prepare(query_candidates) {
            Ok(mut statement) => {
                match statement.query(params![]) {
                    Ok(mut rows) => {
                        loop {
                            match rows.next() {
                                Ok(Some(row)) => {
                                    let mut candidate_id: usize;
                                    let id;
                                    let video_id;
                                    match row.get(0) {
                                        Ok(s) => candidate_id = s,
                                        Err(error) => {
                                            log::error!("Failed to read candiate_id for video: {}", error);
                                            continue;
                                        }
                                    }
                                    match row.get(1) {
                                        Ok(s) => id = s,
                                        Err(error) => {
                                            log::error!("Failed to read id for video: {}", error);
                                            continue;
                                        }
                                    }
                                    match row.get(2) {
                                        Ok(s) => video_id = s,
                                        Err(error) => {
                                            log::error!("Failed to read video_id for video: {}", error);
                                            continue;
                                        }
                                    }
                                    candidate_id -= 1;
                                    v.ids.insert(id, candidate_id as usize);
                                    v.video_ids.insert(video_id, candidate_id as usize);
                                    v.num_candidates += 1;
                                },
                                Ok(None) => {
                                    log::warn!("No data read from candidates.");
                                    break;
                                },
                                Err(error) => {
                                    log::error!("Failed to read a row from candidates: {}", error);
                                    break;
                                }
                            }
                        }
                    },
                    Err(err) => {
                        log::error!("could not read line from candidate database: {}", err);
                    }
                }
            },
            Err(error) => {
                log::error!("Could not prepare statement {}: {}", query_candidates, error);
                return v;
            }
        }
        // fill the num_indices  and number of indices values
        let query_index_location_count = "SELECT location, COUNT(index_id) FROM videostore_indices GROUP BY location";
        match connection.prepare(query_index_location_count) {
            Ok(mut statement) => {
                match statement.query(params![]) {
                    Ok(mut rows) => {
                        loop {
                            match rows.next() {
                                Ok(Some(row)) => {
                                    let location;
                                    let num_entries;
                                    match row.get(0) {
                                        Ok(s) => location = s,
                                        Err(error) => {
                                            log::error!("Failed to read location for indices: {}", error);
                                            continue;
                                        }
                                    }
                                    match row.get(1) {
                                        Ok(s) => num_entries = s,
                                        Err(error) => {
                                            log::error!("Failed to read count for indices: {}", error);
                                            continue;
                                        }
                                    }
                                    v.num_indices.insert(location, num_entries);
                                },
                                Ok(None) => {
                                    log::warn!("No data read from indices.");
                                    break;
                                },
                                Err(error) => {
                                    log::error!("Failed to read a row from indices: {}", error);
                                    break;
                                }
                            }
                        }
                    },
                    Err(err) => {
                        log::error!("could not read line from indices database: {}", err);
                    }
                }
            },
            Err(error) => {
                log::error!("Could not prepare statement {}: {}", query_index_location_count, error);
                return v;
            }
        }

        let query_index_count = "SELECT COUNT(index_id) FROM videostore_indices";
        match connection.prepare(query_index_count) {
            Ok(mut statement) => {
                match statement.query(params![]) {
                    Ok(mut rows) => {
                        loop {
                            match rows.next() {
                                Ok(Some(row)) => {
                                    match row.get(0) {
                                        Ok(s) => v.num_index_values = s,
                                        Err(error) => {
                                            log::error!("Failed to read count for indices: {}", error);
                                            continue;
                                        }
                                    }
                                },
                                Ok(None) => {
                                    log::warn!("No data read from indices.");
                                    break;
                                },
                                Err(error) => {
                                    log::error!("Failed to read a row from indices: {}", error);
                                    break;
                                }
                            }
                        }
                    },
                    Err(err) => {
                        log::error!("could not read line from indices database: {}", err);
                    }
                }
            },
            Err(error) => {
                log::error!("Could not prepare statement {}: {}", query_index_count, error);
                return v;
            }
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
    pub fn add(
        &mut self,
        connection: &mut rusqlite::Connection,
        id: &str,
        video: &crate::videocandidate::VideoCandidate,
        _runtime: u32,
    ) {
        if self.ids.contains_key(id) {
            return;
        }
        let candidate_id;
        if !self.ids.contains_key(id) {
            candidate_id = self.num_candidates + 1;
            let mut blob = Vec::new();
            video.encode(&mut blob);
            log::warn!("Inserting Video information of length {} and data size of {} MegaBytes", video.runtime, blob.len() / 1024 / 1024);
            match connection.execute(
                "INSERT INTO videostore_candidates (candidate_id, filename, video_id, data) VALUES (?1, ?2, ?3, ?4)",
                params![&candidate_id, &video.id, &video.index, &blob],
            ) {
                Ok(_retval) => {}, //log::warn!("Inserted {} video with ID {} and location {} into candidates.", video.id, video.index, candidate_id),
                Err(error) => {
                    log::error!("Failed to insert video {} into  database: {}", video.id, error);
                    return;
                }
            }
            self.num_candidates += 1;
            self.video_ids.insert(video.index, candidate_id as usize);
            self.ids.insert(id.to_string(), candidate_id as usize);
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
                    let location = sign * IMAGESCALE * IMAGESCALE * crate::haar::COLOURCHANNELS
                        + coefindex as u32 * crate::haar::COLOURCHANNELS
                        + colorindex as u32 + 1;
                    if !self.num_indices.contains_key(&location) {
                        self.num_indices.insert(location, 0);
                    }
                    let arrayindex = self.num_indices[&location] as u32 + 1;
                    let index_id = self.num_index_values + 1;
                    match connection.execute(
                        "INSERT INTO videostore_indices (index_id, location, arrayindex, filename, video_id, screenshot_id, runtime) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                        params![&index_id, location, &arrayindex, &video.id, &video.index, &video.screenshots[i].screenshot_id, &video.runtime],
                    ) {
                        Ok(_retval) => {}, //log::warn!("Inserted screenshot {} video_id {} into indices at location {} as {} element.", video.screenshots[i].screenshot_id, video.index, location, arrayindex),
                        Err(error) => {
                            log::error!("Failed to insert video {} into  database: {}", video.id, error);
                            return;
                        }
        
                    }
                    *self.num_indices.get_mut(&location).unwrap() += 1;
                    self.num_index_values += 1;
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
    pub fn delete(&mut self, connection: &mut rusqlite::Connection, id: &str) {
        if !self.ids.contains_key(id) {
            return;
        }
        // Get the index.
        //let index = self.ids[id];
        let query = "SELECT video_id FROM videostore_candidates WHERE filename = ?1";
        match connection.prepare(query) {
            Ok(mut statement) => {
                match statement.query(params![id]) {
                    Ok(mut rows) => {
                        while let Ok(Some(row)) = rows.next() {
                            let s_opt = row.get(1);
                            if s_opt.is_ok() {
                                let video_id = s_opt.unwrap();
                                self.video_ids.remove(&video_id);
                            }
                        }
                    },
                    Err(err) => {
                        log::error!("could not read line from videostore_candidates database: {}", err);
                    }
                }
            },
            Err(error) => {
                log::error!("Failed to get video_id for {} from database: {}", id, error);
                return;
            }
        }
        self.modified = true;
        // clear the entry in the candidates list without deleting it
        let ret = connection.execute(
            "DELETE FROM videostore_candidates WHERE filename = ?1",
            params![&id],
        );
        if ret.is_err() {
            log::error!("Failed to delete candidate {}!", id);
            return;
        }
        self.ids.remove(id);

        // Remove from all index lists.
        let ret = connection.execute(
            "DELETE FROM videostore_indices WHERE filename = ?1",
            params![&id],
        );
        if ret.is_err() {
            log::error!("Failed to delete indices for {}!", id);
            return;
        }
    }

    /// Exchange exchanges the ID of an image for a new one. If the old ID could not
    /// be found, nothing happens. If the new ID already existed prior to the
    /// exchange, the function returns immediately.
    ///
    pub fn exchange(&mut self, connection: &mut rusqlite::Connection, oldid: &str, newid: &str) -> bool {
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
        let ret = connection.execute(
            "UPDATE videostore_candidates filename = ?1 WHERE filename = ?2",
            params![&newid, &oldid],
        );
        if ret.is_err() {
            log::error!("Failed to update candidate {}!", oldid);
            return false;
        }
        true
    }

    /// Find all similar screenshots for a single screenshot
    fn search_matches(
        &mut self,
        client: &mut rusqlite::Connection,
        hash: &crate::hash::Hash,
        video_ids: &std::collections::BTreeMap<u32, usize>,
        screenshot_index_global: usize,
        video_screenshot_to_score_map: &Vec<Vec<usize>>,
        scoreid_to_video_screenshot_map: &Vec<ScreenshotIndex>,
    ) -> crate::videomatches::VideoMatches {
        let mut ms = crate::videomatches::VideoMatches::new();
        // build a mapping of video, screenshot to a global image index
        if self.num_candidates == 0 {
            return ms;
        }
        // prepare the scoring vector where we can rate any existing screenshot
        let mut scores: Vec<f64> = Vec::new();
        scores.reserve(screenshot_index_global);
        for _ in 0..screenshot_index_global {
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
                let location = sign * IMAGESCALE * IMAGESCALE * crate::haar::COLOURCHANNELS
                    + coefindex as u32 * crate::haar::COLOURCHANNELS
                    + colorindex as u32 + 1;
                let arr = &self.return_indice(client, location);
                for i in 0..arr.len() {
                    let matchscreenshot = arr[i].clone();
                    if !video_ids.contains_key(&matchscreenshot.video_id) {
                        continue;
                    }
                    let video_pos = video_ids[&matchscreenshot.video_id];
                    if video_pos >= video_screenshot_to_score_map.len()
                        || video_screenshot_to_score_map[video_pos].len()
                            == 0
                    {
                        log::error!("Search Matches failed to lookup the video position {} by its Index {}", video_pos, matchscreenshot.video_id);
                        return ms;
                    }
                    if matchscreenshot.screenshot_id < 1 {
                        continue;
                    }
                    let screenshot_pos = matchscreenshot.screenshot_id as usize - 1;
                    if screenshot_pos >= video_screenshot_to_score_map[video_pos].len() {
                        continue;
                    }
                    let screenshot_index_global = video_screenshot_to_score_map[video_pos][screenshot_pos];
                    if scores[screenshot_index_global].is_nan() {
                        // calculated initial score
                        let mut score: f64 = 0.0;
                        for colorid in 0..coef.c.len() {
                            score += WEIGHTS[colorid][0];
                        }
                        scores[screenshot_index_global] = score;
                    }
                    // At this point, we have an entry in matches. Simply subtract the
                    // corresponding weight.
                    scores[screenshot_index_global] -= WEIGHTSUMS[bin];
                }
            }
        }
        // Create matches. If the dhash_distance is lower than the sensitivity threshold it is a *valid* match.
        for index in 0..scores.len() {
            if !scores[index].is_nan() {
                if scores[index] > self.sensitivity {
                    continue;
                }
                let mut m = crate::videomatches::VideoMatch::new();
                let video_id = scoreid_to_video_screenshot_map[index].video_id;
                let screenshot_id = scoreid_to_video_screenshot_map[index].screenshot_id;
                let screenshot;
                if self.candidate_cache.max_candidates > 0 {
                    if !self.candidate_cache.contains(video_id) {
                        let (_, candidate) = self.return_candidate(client, video_id);
                        log::warn!("Found Match {}", candidate.id);
                        self.candidate_cache.add(candidate);
                    }
                    m.id = self.candidate_cache.map[&video_id].id.clone();
                    let screenshot_pos = screenshot_id as usize - 1;
                    screenshot = self.candidate_cache.map[&video_id].screenshots[screenshot_pos].clone();
                } else {
                    let (_, candidate) = self.return_candidate(client, video_id);
                    log::warn!("Found Match {}", candidate.id);
                    m.id = candidate.id.clone();
                    let screenshot_pos = screenshot_id as usize - 1;
                    screenshot = candidate.screenshots[screenshot_pos].clone();
                }
                m.video_id = video_id;
                m.screenshot_id = screenshot_id;
                m.timecode = screenshot.timecode;
                m.score = scores[index];
                m.ratio_diff = screenshot.hash.ratio.log(10.0).abs() - hash.ratio.log(10.0);
                m.dhash_distance =
                    crate::hamming::hamming_distance(screenshot.hash.dhash[0], hash.dhash[0])
                        + crate::hamming::hamming_distance(screenshot.hash.dhash[1], hash.dhash[1]);
                m.histogram_distance =
                    crate::hamming::hamming_distance(screenshot.hash.histogram, hash.histogram);
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
    fn rate_match(
        &self,
        client: &mut rusqlite::Connection,
        new_video: &crate::videocandidate::VideoCandidate,
        match_id: u32,
        screenshot_id: u32,
        num_matches: usize,
        time_between_screenshots: u32,
    ) -> crate::videomatches::VideoMatch {
        let mut m = crate::videomatches::VideoMatch::new();
        let (_, matched) = self.return_candidate(client, match_id);
        m.id = matched.id.clone();
        m.video_id = matched.index;
        m.screenshot_id = screenshot_id;
        m.timecode = screenshot_id * time_between_screenshots as u32;
        m.score = -60.0                                                                                // base value
                    - 100.0 * (num_matches as f64 * time_between_screenshots as f64) / matched.runtime as f64 // the longer the similar part, the better the match
                    + ((new_video.width - matched.width) * (new_video.width - matched.width)) as f64;  // if the resolution is higher the match gets better

        m
    }

    /// Query performs a similarity search on the given image hashes and returns
    /// all potential matches. The returned slice will sort the match with the best score as its
    /// first element.
    ///
    /// Videos consist of one screenshot every ten seconds.
    /// A Match contains a portion of at least a minute (six similar screenshots in a row)
    /// The longer the sequence the better the match.
    ///
    pub fn query(
        &mut self,
        client: &mut rusqlite::Connection,
        video: &crate::videocandidate::VideoCandidate,
    ) -> crate::videomatches::VideoMatches {
        let mut ms = crate::videomatches::VideoMatches::new();
        if self.num_candidates == 0 {
            return ms;
        }
        let mut sequences = std::collections::BTreeMap::new();
        let mut active_sequence_counter = std::collections::BTreeMap::new();

        // prepare data structures
        let mut video_screenshot_to_score_map = Vec::new();
        let mut scoreid_to_video_screenshot_map = Vec::new();
        let mut video_ids: std::collections::BTreeMap<u32, usize> = std::collections::BTreeMap::new();
        let mut screenshot_index_global: usize = 0;
        for video_id in 1..self.num_candidates + 1 {
            let (_candidate_id, candidate) = self.return_candidate(client, video_id);
            let video_pos = video_screenshot_to_score_map.len();
            video_ids.insert(candidate.index, video_id  as usize - 1);
            video_screenshot_to_score_map.push(Vec::new());
            for screenshot_pos in 0..candidate.screenshots.len() {
                video_screenshot_to_score_map[video_pos].push(screenshot_index_global);
                scoreid_to_video_screenshot_map.push(ScreenshotIndex::from(
                    &candidate.id,
                    video_id,
                    screenshot_pos as u32 + 1,
                    candidate.runtime,
                ));
                screenshot_index_global += 1;
            }
        }

        // search for each screenshot of the current video in the store
        let report_interval = (5 * 60 / self.num_seconds_between_screenshots) as usize;
        for screenshot_pos in 0..video.screenshots.len() {
            let hash = &video.screenshots[screenshot_pos].hash;
            let matches = self.search_matches(
                                client, 
                                hash, 
                                &video_ids,
                                screenshot_index_global,
                                &video_screenshot_to_score_map,
                                &scoreid_to_video_screenshot_map,
                            );
            let mut previous_videos = std::collections::BTreeSet::new();
            for (key, _) in sequences.iter() {
                previous_videos.insert(*key);
            }
            let mut new_videos= std::collections::BTreeSet::new();
            if screenshot_pos % report_interval == 0 {
                log::warn!("Comparing new video with the databse. Currently at position {} Minutes into the video", 
                            (screenshot_pos as u32 / self.min_similar_screenshots_in_sequence) as u32);
            }
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
                        if x[x.len() - 1] + 1 == screenshot_id {
                            x.push(screenshot_id);
                            break;
                        } else {
                            // broken sequence
                            x.clear();
                            x.push(screenshot_id);
                            break;
                        }
                    }
                }
            }
            // remove the videos from the sequence that were not a match
            let dropped_videos = previous_videos.difference(&new_videos);
            for id in dropped_videos {
                // check if the sequence was longer than a minute amd add the ones long enough
                if sequences[id].len() > 5 {
                    let videomatch =
                        self.rate_match(
                            client, 
                            &video, 
                            *id, 
                            screenshot_pos as u32, 
                            sequences[id].len(),
                            self.num_seconds_between_screenshots);
                    if videomatch.id.len() != 0 {
                        ms.m.push(videomatch);
                    }
                } else {
                    log::error!("Candidate {} had {} matches!", id, sequences[id].len());
                }
                sequences.remove(id);
            }
            new_videos.clear();
        }
        // done parsing, add everything with more than 5 matches in a row to the list
        for (video_id, v) in sequences {
            // check if the sequence was longer than a minute
            if v.len() >= self.min_similar_screenshots_in_sequence as usize {
                let mut m = crate::videomatches::VideoMatch::new();
                let matchedvideo;
                if self.candidate_cache.contains(video_id) {
                    matchedvideo = self.candidate_cache.map[&video_id].clone();
                } else {
                    (_, matchedvideo) = self.return_candidate(client, video_id);
                }
                m.id = matchedvideo.id.clone();
                m.video_id = matchedvideo.index;
                m.screenshot_id = 0;
                m.timecode = 0;
                m.score = -60.0                                                                            // base value
                            - 100.0 * (v.len() as f64 * 10.0) / matchedvideo.runtime as f64                 // the longer the similar part, the better the match
                            + ((video.width - matchedvideo.width) * (video.width - matchedvideo.width)) as f64; // if the resolution is higher the match gets better
                if m.id.len() != 0 {
                    ms.m.push(m);
                }
            }
        }
        ms.sort();
        ms
    }

    pub fn return_candidate(
        &self,
        connection: &mut rusqlite::Connection,
        video_id: u32,
    ) -> (u32, crate::videocandidate::VideoCandidate) {
        let mut v = crate::videocandidate::VideoCandidate::new();
        let mut candidate_id = 0;
        let query = "SELECT candidate_id, data FROM videostore_candidates WHERE video_id = ?1";
        match connection.prepare(query) {
            Ok(mut statement) => {
                match statement.query(params![&video_id]) {
                    Ok(mut rows) => {
                        loop {
                            match rows.next() {
                                Ok(Some(row)) => {
                                    match row.get(0) {
                                        Ok(s) => {
                                            candidate_id = s;
                                        },
                                        Err(error) => {
                                            log::error!("Failed to read candiate_id for video: {}", error);
                                            continue;
                                        }
                                    }
                                    match row.get(1) {
                                        Ok(s) => {
                                            let blob: Vec<u8> = s;
                                            v.decode(&mut std::io::Cursor::new(blob));
                                        },
                                        Err(error) => {
                                            log::error!("Failed to read binary data for video: {}", error);
                                            continue;
                                        }
                                    }
                                },
                                Ok(None) => {
                                    //log::warn!("No data read from candidates.");
                                    break;
                                },
                                Err(error) => {
                                    log::error!("Failed to read a row from candidates: {}", error);
                                    break;
                                }
                            }
                        }
                    },
                    Err(err) => {
                        log::error!("could not read line from videostore_candidates database: {}", err);
                    }
                }
            },
            Err(err) => {
                log::error!("could not prepare SQL statement: {}", err);
            }
        }
        (candidate_id, v)
    }

    pub fn return_indice(
        &self,
        connection: &mut rusqlite::Connection,
        location: u32,
    ) -> Vec<ScreenshotIndex> {
        let mut v = Vec::new();
        let query = "SELECT filename, video_id, screenshot_id, runtime FROM videostore_indices WHERE location = ?1";
        match connection.prepare(query) {
            Ok(mut statement) => {
                match statement.query(params![&location]) {
                    Ok(mut rows) => {
                        loop {
                            match rows.next() {
                                Ok(Some(row)) => {
                                    let mut s = ScreenshotIndex::new();
                                    match row.get(0) {
                                        Ok(val) => s.id = val,
                                        Err(error) => {
                                            log::error!("Failed to read id for video: {}", error);
                                            continue;
                                        }
                                    }
                                    match row.get(1) {
                                        Ok(val) => s.video_id = val,
                                        Err(error) => {
                                            log::error!("Failed to read video_id for video: {}", error);
                                            continue;
                                        }
                                    }
                                    match row.get(2) {
                                        Ok(val) => s.screenshot_id = val,
                                        Err(error) => {
                                            log::error!("Failed to read screenshot_id for video: {}", error);
                                            continue;
                                        }
                                    }
                                    match row.get(3) {
                                        Ok(val) => s.runtime = val,
                                        Err(error) => {
                                            log::error!("Failed to read runtime for video: {}", error);
                                            continue;
                                        }
                                    }
                                    v.push(s);
                                },
                                Ok(None) => {
                                    //log::warn!("No data read from indices.");
                                    break;
                                },
                                Err(error) => {
                                    log::error!("Failed to read a row from indices: {}", error);
                                    break;
                                }
                            }
                        }
                    },
                    Err(err) => {
                        log::error!("could not read line from videostore_indices database: {}", err);
                    }
                }
            },
            Err(err) => {
                log::error!("could not prepare SQL statement: {}", err);
            }
        }
        v
    }

    pub fn size(&self) -> usize {
        self.num_candidates as usize
    }

    pub fn modified(&self) -> bool {
        self.modified
    }

    // encode the data structure to binary stream
    pub fn encode(&self, to: &mut Vec<u8>) {
        crate::marshal::store_u32(self.num_candidates, to);
        crate::marshal::store_hash_string_usize(&self.ids, to);
        crate::marshal::store_hash_u32_usize(&self.video_ids, to);
        crate::marshal::store_hash_u32_usize(&self.num_indices, to);
        crate::marshal::store_u32(self.num_index_values, to);
        crate::marshal::store_f64(self.sensitivity, to);
        crate::marshal::store_bool(self.modified, to);
    }

    // decode data structure from binary stream
    pub fn decode(&mut self, from: &mut std::io::Cursor<Vec<u8>>) {
        self.num_candidates = crate::marshal::restore_u32(from);
        self.ids
            .extend(crate::marshal::restore_hash_string_usize(from));
        self.video_ids
            .extend(crate::marshal::restore_hash_u32_usize(from));
        self.num_indices
            .extend(crate::marshal::restore_hash_u32_usize(from));
        self.num_index_values = crate::marshal::restore_u32(from);
        self.sensitivity = crate::marshal::restore_f64(from);
        self.modified = crate::marshal::restore_bool(from);
        self.modified = false;
    }

    // Write binary stream to file
    pub fn dump_binary(&self, storefile: &str) {
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
    pub fn slurp_binary(&mut self, storefile: &str, client: &mut rusqlite::Connection) {
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
        let mut keys = Vec::new();
        for (key, _val) in self.ids.iter_mut() {
            keys.push(key.to_string());
        }
        for key in keys {
            let filepath = std::path::Path::new(&key);
            if !filepath.is_file() {
                // video has vanished, remove it from the store
                self.delete(client, &key);
            }
        }
    }
}

/// Make a connection to the database
/// This requires a running PostgreSQL server.
/// Also there has to be a valid user and a Database / Schema.
/// 
pub fn connect(
    dbpath: &str,
) -> Result<rusqlite::Connection, rusqlite::Error> {
    let path = std::path::Path::new(dbpath);
    let connection;
    if !path.is_file() {
        connection = Connection::open(dbpath)?;
        println!("{}", connection.is_autocommit());
        match connection.execute(
            "CREATE TABLE videostore_candidates (
                candidate_id UNSIGNED BIG INT PRIMARY KEY NOT NULL,
                filename TEXT NOT NULL unique, 
                video_id UNSIGNED BIG INT NOT NULL, 
                data BLOB
            )", (),
        ) {
            Ok(_ret) => {},
            Err(error) => {
                log::error!("Failed to create table candidates: {}", error);
                return Err(error);
            }
        }
        match connection.execute(
            "CREATE INDEX index_videostore_candidates_filename ON videostore_candidates (filename)", (),
        ) {
            Ok(_ret) => {},
            Err(error) => {
                log::error!("Failed to create index on candidates: {}", error);
                return Err(error);
            }
        }
        match connection.execute(
            "CREATE INDEX index_videostore_candidat_video_id ON videostore_candidates (video_id)", (),
        ) {
            Ok(_) => {},
            Err(error) => {
                log::error!("Failed to create index on candidates: {}", error);
                return Err(error);
            }
        }
        match connection.execute("
            CREATE TABLE videostore_indices (
                index_id UNSIGNED BIG INT PRIMARY KEY NOT NULL, 
                location UNSIGNED BIG INT NOT NULL, 
                arrayindex UNSIGNED BIG INT NOT NULL, 
                filename TEXT NOT NULL, 
                video_id UNSIGNED BIG INT NOT NULL, 
                screenshot_id UNSIGNED BIG INT NOT NULL, 
                runtime UNSIGNED BIG INT NOT NULL
            )", [],
        ) {
            Ok(_ret) => {},
            Err(error) => {
                log::error!("Failed to create table indices: {}", error);
                return Err(error);
            }
        }
        match connection.execute(
            "CREATE INDEX index_videostore_indices_location ON videostore_indices (filename)", (),
        ) {
            Ok(_ret) => {},
            Err(error) => {
                log::error!("Failed to create index on indices: {}", error);
                return Err(error);
            }
        }
        match connection.execute(
            "CREATE INDEX index_videostore_indices_filename ON videostore_indices (filename)", (),
        ) {
            Ok(_ret) => {},
            Err(error) => {
                log::error!("Failed to create index on indices: {}", error);
                return Err(error);
            }
        }
        match connection.execute(
            "CREATE INDEX index_videostore_indices_video_id ON videostore_indices (video_id)", (),
        ) {
            Ok(_ret) => {},
            Err(error) => {
                log::error!("Failed to create index on indices: {}", error);
                return Err(error);
            }
        }
        match connection.execute("
            CREATE TABLE videostore_parameters (
                config_id UNSIGNED BIG INT PRIMARY KEY NOT NULL, 
                sensitivity BIGINT, 
                start_directory TEXT, 
                num_threads UNSIGNED BIG INT,
                num_seconds_between_screenshots UNSIGNED BIG INT,
                min_similar_screenshots_in_sequence UNSIGNED BIG INT,
                max_candidates_in_cache UNSIGNED BIG INT
            )", [],
        ) {
            Ok(_ret) => {},
            Err(error) => {
                log::error!("Failed to create table parameters: {}", error);
                return Err(error);
            }
        }
        match connection.execute(
            "INSERT INTO videostore_parameters (config_id, sensitivity, start_directory, num_threads, num_seconds_between_screenshots, min_similar_screenshots_in_sequence, max_candidates_in_cache) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![&1, &-60, &"./", &2, &10, &6, &100],
        ) {
            Ok(retval) => log::warn!("Inserted {} data into parameter.", retval),
            Err(error) => {
                log::error!("Failed to insert data into parameter database: {}", error);
                return Err(error);
            }
        }

    } else {
        connection = Connection::open(dbpath)?;
    }
    Ok(connection)
}
