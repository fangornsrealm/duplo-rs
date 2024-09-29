#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Match {
    pub id: String,
    pub score: f64, 
    pub ratio_diff: f64,
    pub dhash_distance: i32,
    pub histogram_distance: i32,
}

impl Match {
    pub fn new() -> Self {
        let v = Match{..Default::default()};
        v
    }

    pub fn from(id: &str, score: f64, ratio_diff: f64, dhash_distance: i32, histogram_distance: i32) -> Self {
        let mut v = Match{..Default::default()};
        v.id = id.to_string();
        v.score += score;
        v.ratio_diff += ratio_diff;
        v.dhash_distance += dhash_distance;
        v.histogram_distance += histogram_distance;
        v
    }

    pub fn string(&self) -> String {
        format!("{}: score={:.4}, ratio-diff={:.1}, dHash-dist={}, histDist={}",
                self.id, self.score, self.ratio_diff, self.dhash_distance, self.histogram_distance)
    }

    pub fn encode(&self, to: &mut Vec<u8>) {
        crate::marshal::store_string(&self.id, to);
        crate::marshal::store_f64(self.score, to);
        crate::marshal::store_f64(self.ratio_diff, to);
        crate::marshal::store_i32(self.dhash_distance, to);
        crate::marshal::store_i32(self.histogram_distance, to);
    }

    pub fn decode(&mut self, from: &mut std::io::Cursor<Vec<u8>>) {
        self.id = crate::marshal::restore_string(from);
        self.score = crate::marshal::restore_f64(from);
        self.ratio_diff = crate::marshal::restore_f64(from);
        self.dhash_distance = crate::marshal::restore_i32(from);
        self.histogram_distance = crate::marshal::restore_i32(from);

    }
}

impl Default for Match {
    fn default() -> Match {
        Match {
            id: String::new(),
            score: 0.0,
            ratio_diff: 0.0,
            dhash_distance: 0,
            histogram_distance: 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Matches {
    pub m: Vec<Match>,
}

impl Matches {
    pub fn new() -> Self {
        let v = Matches {..Default::default()};
        v
    }

    pub fn from(m: Vec<Match>) -> Self {
        let mut v = Matches {..Default::default()};
        v.m.extend(m);
        v
    }

    pub fn len(&self) -> usize {self.m.len()}
    
    pub fn swap(&mut self, i: usize, j: usize) {
        let tmp = self.m[i].clone();
        self.m[i] = self.m[j].clone();
        self.m[j] = tmp;
    }

    pub fn less(&self, i: usize, j: usize) -> bool {
        if j >= self.m.len() {
            return false;
        }
        if i >= self.m.len() {
            return false;
        }
        return self.m[i].score < self.m[j].score;
    }

    pub fn sort(&mut self) {
        // we use Bubble sort until someone wants to spend the time
        if self.m.len() == 0 {
            return;
        }
        for i in 0..self.m.len() - 1 {
            for j in i+1..self.len() {
                if self.less(j, i) {
                    self.swap(j, i);
                }
            }
        }
    }

    pub fn encode(&self, to: &mut Vec<u8>) {
        let s = self.m.len();
        crate::marshal::store_usize(s, to);
        for elem in &self.m {
            elem.encode(to);
        }
    }

    pub fn decode(&mut self, from: &mut std::io::Cursor<Vec<u8>>) {
        let s = crate::marshal::restore_usize(from);
        for _i in 0..s {
            let mut elem = Match {
                ..Default::default()
            };
            elem.decode(from);
            self.m.push(elem);
        }
    }
}

impl Default for Matches {
    fn default() -> Matches {
        Matches {
            m: Vec::new(),
        }
    }
}
