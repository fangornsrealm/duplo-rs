//use array2d::{Array2D, Error};

static WEIGHTS: [[f64;6];3] = [[5.00_f64, 0.83, 1.01, 0.52, 0.47, 0.30], 
                [19.21, 1.26, 0.44, 0.53, 0.28, 0.14],
                [34.37, 0.36, 0.45, 0.14, 0.18, 0.27]];
pub const IMAGESCALE: u32 = 128;
pub const INDICESMAX: u32 = 49200;
pub static TOPCOEFS: i32 = 40;
static WEIGHTSUMS: [f64;6] = [58.58 as f64, 2.45, 1.9, 1.19, 0.93, 0.71];

/// Store is a data structure that holds references to images. It holds visual
/// hashes and references to the images but the images themselves are not held
/// in the data structure.
///
/// A general limit to the store is that it can hold no more than 4,294,967,295
/// images. This is to save RAM space but may be easy to extend by modifying its
/// data structures to hold uint64 indices instead of uint32 indices.
///
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Store {
	//sync.RWMutex,

	// All images in the store or, rather, the candidates for a query.
	pub candidates: Vec<crate::candidate::Candidate>,

	// All IDs in the store, mapping to candidate indices.
	pub ids: std::collections::BTreeMap<String, usize>,

	// indices  contains references to the images in the store. It is a slice
	// of slices which contains image indices (into the "candidates" slice).
	// Use the following formula to access an index slice:
	//
	//		s := store.indices[sign*ImageScale*ImageScale*haar.ColourChannels + coefIdx*haar.ColourChannels + channel]
	//
	// where the variables are as follows:
	//
	//		* sign: Either 0 (positive) or 1 (negative)
	//		* coefIdx: The index of the coefficient (from 0 to (ImageScale*ImageScale)-1)
	//		* channel: The colour channel (from 0 to haar.ColourChannels-1)
	pub indices: Vec<Vec<u32>>,

	// Whether this store was modified since it was loaded/created.
	pub modified: bool,
}

impl Store {
    pub fn new() -> Self {
        let mut v = Store {
            candidates: Vec::new(),
            ids: std::collections::BTreeMap::new(),
            indices: Vec::new(),
            modified: false,
        };
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

    /// Add adds an image (via its hash) to the store. The provided ID is the value
    /// that will be returned as the result of a similarity query. If an ID is
    /// already in the store, it is not added again.
    pub fn add(&mut self, id: &str, hash: &crate::hash::Hash) {
        if self.ids.contains_key(id) {
            return;
        }
        let index = self.candidates.len();
        self.candidates.push(crate::candidate::Candidate::from(id, hash));
        self.ids.insert(id.to_string(), index);
        for i in 1..hash.matrix.coefs.len() {
            if hash.matrix.coefs[i].c[0].abs() > hash.thresholds.c[0] {
                continue;
            }
            for j in 0..hash.matrix.coefs[i].c.len() {
                let mut sign = 0;
                if hash.matrix.coefs[i].c[j] < 0.0 {
                    sign = 1;
                }
                let location = sign * IMAGESCALE *IMAGESCALE * crate::haar::COLOURCHANNELS 
                                    + j as u32 * crate::haar::COLOURCHANNELS + j as u32;
                self.indices[location as usize].push(index as u32);
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
        self.candidates[index] = crate::candidate::Candidate::new();
        self.ids.remove(id);
        // Remove from all index lists.
        for i in 0..self.indices.len() {
            for j in 0..self.indices[i].len() {
                if self.indices[i][j] == index as u32 {
                    self.indices[i].remove(j);
                    break;
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

    // Query performs a similarity search on the given image hash and returns
    // all potential matches. The returned slice will not be sorted but implements
    // sort.Interface, which will sort it so the match with the best score is its
    // first element.
    pub fn query(&self, hash: &crate::hash::Hash) -> crate::matches::Matches {
        let mut ms = crate::matches::Matches::new();
        if self.candidates.len() == 0 {
            return ms;
        }

        let mut scores: Vec<f64> = Vec::new();
        for _ in 0..self.candidates.len() {
            scores.push(f64::NAN);
        }
        //let mut nummatches: usize;

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
                if hash.matrix.coefs[coefindex].c[colorindex] < 0.0 {
                    sign = 1;
                }
                let location = sign * IMAGESCALE *IMAGESCALE * crate::haar::COLOURCHANNELS 
                                    + colorindex as u32 * crate::haar::COLOURCHANNELS + colorindex as u32;
                for i in 0..self.indices[location as usize].len() {
                    let sindex = self.indices[location as usize][i];
                    if !scores[sindex as usize].is_nan() {
                        // calculated initial score
                        let mut score: f64  = 0.0;
                        for colorid in 0..coef.c.len() {
                            score += WEIGHTS[colorid][0];
                        }
                        scores[sindex as usize] = score;
                    }
                    // At this point, we have an entry in matches. Simply subtract the
                    // corresponding weight.
                    scores[sindex as usize] -= WEIGHTSUMS[bin];
                }
            }
        }
        // Create matches.
        for index in 0..scores.len() {
            if !scores[index].is_nan() {
                let mut m = crate::matches::Match::new();
                m.id = self.candidates[index].id.clone();
                m.score = scores[index];
                m.ratio_diff = self.candidates[index].ratio.log(10.0).abs() - hash.ratio.log(10.0);
                m.dhash_distance = crate::hamming::hamming_distance(
                                    self.candidates[index].dhash[0], hash.dhash[0])
                                    + crate::hamming::hamming_distance(
                                        self.candidates[index].dhash[1], hash.dhash[1]);
                m.histogram_distance = crate::hamming::hamming_distance(
                                            self.candidates[index].histogram, hash.histogram);
                ms.m.push(m);
                
            }
        }
        ms.sort();
        ms
    }

    pub fn size(&self) -> usize {
        self.candidates.len()
    }

    pub fn modified(&self) -> bool {
        self.modified
    }

    pub fn encode(&self, to: &mut Vec<u8>) {
        let s = self.candidates.len();
        crate::marshal::store_usize(s, to);
        for elem in &self.candidates {
            elem.encode(to);
        }
        crate::marshal::store_hash_string_usize(&self.ids, to);
        let s = self.indices.len();
        crate::marshal::store_usize(s, to);
        for elem in &self.indices {
            crate::marshal::store_vec_u32(elem, to);
        }
        crate::marshal::store_bool(self.modified, to);
    }

    pub fn decode(&mut self, from: &mut std::io::Cursor<Vec<u8>>) {
        let s = crate::marshal::restore_usize(from);
        for _i in 0..s {
            let mut elem = crate::candidate::Candidate::new();
            elem.decode(from);
            self.candidates.push(elem);
        }
        self.ids.extend(crate::marshal::restore_hash_string_usize(from));
        let s = crate::marshal::restore_usize(from);
        for _i in 0..s {
            let mut elem = Vec::new();
            elem.extend(crate::marshal::restore_vec_u32(from));
            self.indices.push(elem);
        }
        self.modified = crate::marshal::restore_bool(from);

    }

}

