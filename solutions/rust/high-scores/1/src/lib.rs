#[derive(Debug)]
pub struct HighScores {
    _scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        Self {
            _scores: Vec::<u32>::from(scores),
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self._scores
    }

    pub fn latest(&self) -> Option<u32> {
        self._scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self._scores.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut sorted: Vec<u32> = self._scores.clone();
        if sorted.is_empty() {
            return sorted;
        }
        
        sorted.sort_by(|a, b| b.cmp(a));
        let upper_limit = std::cmp::min(3, sorted.len());
        sorted.get(0..upper_limit).unwrap().to_vec()
    }
}
