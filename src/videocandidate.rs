#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Screenshot {
    pub id: String,
    pub video_id: u32,
    pub screenshot_id: u32,
    pub timecode: u32,
    pub hash: crate::hash::Hash,
}

impl Screenshot {
    pub fn new() -> Self {
        let v: Screenshot = Screenshot {
            ..Default::default()
        };
        v
    }
    pub fn from(
        id: &str,
        video_id: usize,
        screenshot_id: usize,
        timecode: u32,
        hash: &crate::hash::Hash,
    ) -> Self {
        let mut v: Screenshot = Screenshot {
            ..Default::default()
        };
        v.id = id.to_string();
        v.video_id = video_id as u32;
        v.screenshot_id = screenshot_id as u32;
        v.timecode = timecode as u32;
        v.hash = hash.clone();
        v
    }

    pub fn encode(&self, to: &mut Vec<u8>) {
        crate::marshal::store_string(&self.id, to);
        crate::marshal::store_u32(self.video_id, to);
        crate::marshal::store_u32(self.screenshot_id, to);
        crate::marshal::store_u32(self.timecode, to);
        self.hash.encode(to);
    }

    /// Compact encoding: everything except the large Haar matrix.
    pub fn encode_compact(&self, to: &mut Vec<u8>) {
        crate::marshal::store_string(&self.id, to);
        crate::marshal::store_u32(self.video_id, to);
        crate::marshal::store_u32(self.screenshot_id, to);
        crate::marshal::store_u32(self.timecode, to);
        self.hash.encode_compact(to);
    }

    pub fn decode(&mut self, from: &mut std::io::Cursor<Vec<u8>>) {
        self.id = crate::marshal::restore_string(from);
        if let Some(r) = crate::marshal::restore_u32(from) {
            self.video_id = r;
        } else {
            return;
        }
        if let Some(r) = crate::marshal::restore_u32(from) {
            self.screenshot_id = r;
        } else {
            return;
        }
        if let Some(r) = crate::marshal::restore_u32(from) {
            self.timecode = r;
        } else {
            return;
        }
        self.hash.decode(from);
    }

    /// Compact decode: reads the hash without the matrix.
    pub fn decode_compact(&mut self, from: &mut std::io::Cursor<Vec<u8>>) {
        self.id = crate::marshal::restore_string(from);
        if let Some(r) = crate::marshal::restore_u32(from) {
            self.video_id = r;
        } else {
            return;
        }
        if let Some(r) = crate::marshal::restore_u32(from) {
            self.screenshot_id = r;
        } else {
            return;
        }
        if let Some(r) = crate::marshal::restore_u32(from) {
            self.timecode = r;
        } else {
            return;
        }
        self.hash.decode_compact(from);
    }
}

impl Default for Screenshot {
    fn default() -> Screenshot {
        Screenshot {
            id: String::new(),
            video_id: 0,
            screenshot_id: 0,
            timecode: 0,
            hash: crate::hash::Hash::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct VideoCandidate {
    pub id: String,
    pub index: u32,
    pub screenshots: Vec<Screenshot>,
    pub width: u32,
    pub height: u32,
    pub runtime: u32,
    pub framerate: f32,
}

impl VideoCandidate {
    pub fn new() -> Self {
        let v: VideoCandidate = VideoCandidate {
            ..Default::default()
        };
        v
    }
    pub fn from(id: &str, index: usize) -> Self {
        let mut v: VideoCandidate = VideoCandidate {
            ..Default::default()
        };
        v.id = id.to_string();
        v.index = index as u32;
        v
    }

    pub fn encode(&self, to: &mut Vec<u8>) {
        crate::marshal::store_string(&self.id, to);
        crate::marshal::store_u32(self.index, to);
        let s = self.screenshots.len();
        crate::marshal::store_usize(s, to);
        for elem in &self.screenshots {
            elem.encode(to);
        }
        crate::marshal::store_u32(self.width, to);
        crate::marshal::store_u32(self.height, to);
        crate::marshal::store_u32(self.runtime, to);
        crate::marshal::store_f32(self.framerate, to);
    }

    /// Compact encoding (version 2): omits the large Haar matrix from each
    /// screenshot. The version byte (2) is written first so `decode_any`
    /// can distinguish this format from the legacy v1 (which has no marker).
    pub fn encode_compact(&self, to: &mut Vec<u8>) {
        to.push(2); // format version
        crate::marshal::store_string(&self.id, to);
        crate::marshal::store_u32(self.index, to);
        let s = self.screenshots.len();
        crate::marshal::store_usize(s, to);
        for elem in &self.screenshots {
            elem.encode_compact(to);
        }
        crate::marshal::store_u32(self.width, to);
        crate::marshal::store_u32(self.height, to);
        crate::marshal::store_u32(self.runtime, to);
        crate::marshal::store_f32(self.framerate, to);
    }

    pub fn decode(&mut self, from: &mut std::io::Cursor<Vec<u8>>) {
        self.id = crate::marshal::restore_string(from);
        if let Some(r) = crate::marshal::restore_u32(from) {
            self.index = r;
        } else {
            return;
        }
        if let Some(s) = crate::marshal::restore_usize(from) {
            for _i in 0..s {
                let mut elem = Screenshot::new();
                elem.decode(from);
                self.screenshots.push(elem);
            }
        }
        if let Some(r) = crate::marshal::restore_u32(from) {
            self.width = r;
        } else {
            return;
        }
        if let Some(r) = crate::marshal::restore_u32(from) {
            self.height = r;
        } else {
            return;
        }
        if let Some(r) = crate::marshal::restore_u32(from) {
            self.runtime = r;
        } else {
            return;
        }
        if let Some(r) = crate::marshal::restore_f32(from) {
            self.framerate = r;
        } else {
            return;
        }
    }

    /// Decode a compact (v2) candidate: screenshots have no matrix.
    /// The caller must have already consumed the version byte.
    pub fn decode_compact(&mut self, from: &mut std::io::Cursor<Vec<u8>>) {
        self.id = crate::marshal::restore_string(from);
        if let Some(r) = crate::marshal::restore_u32(from) {
            self.index = r;
        } else {
            return;
        }
        if let Some(s) = crate::marshal::restore_usize(from) {
            for _i in 0..s {
                let mut elem = Screenshot::new();
                elem.decode_compact(from);
                self.screenshots.push(elem);
            }
        }
        if let Some(r) = crate::marshal::restore_u32(from) {
            self.width = r;
        } else {
            return;
        }
        if let Some(r) = crate::marshal::restore_u32(from) {
            self.height = r;
        } else {
            return;
        }
        if let Some(r) = crate::marshal::restore_u32(from) {
            self.runtime = r;
        } else {
            return;
        }
        if let Some(r) = crate::marshal::restore_f32(from) {
            self.framerate = r;
        } else {
            return;
        }
    }

    /// Auto-detect the blob format (v1 legacy with matrix, or v2 compact
    /// without) and decode accordingly. Safe to call on either format.
    pub fn decode_any(&mut self, from: &mut std::io::Cursor<Vec<u8>>) {
        use std::io::{Read, Seek, SeekFrom};
        let pos = from.position();
        let mut version = [0u8; 1];
        if from.read_exact(&mut version).is_ok() && version[0] == 2 {
            self.decode_compact(from);
            return;
        }
        // Rewind and use the legacy (v1) decoder.
        let _ = from.seek(SeekFrom::Start(pos));
        self.decode(from);
    }
}

impl Default for VideoCandidate {
    fn default() -> VideoCandidate {
        VideoCandidate {
            id: String::new(),
            index: 0,
            screenshots: Vec::new(),
            width: 0,
            height: 0,
            runtime: 0,
            framerate: 0.0,
        }
    }
}
