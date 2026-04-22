#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
    top_three: Vec<u32>,
}

impl HighScores {
    #[must_use]
    pub fn new(scores: &[u32]) -> Self {
        let mut top_three = scores.to_vec();
        top_three.sort_unstable();
        top_three.reverse();
        let top_three = top_three.into_iter().take(3).collect::<Vec<u32>>();
        HighScores {
            scores: scores.to_vec(),
            top_three,
        }
    }

    #[must_use]
    pub fn scores(&self) -> &[u32] {
        self.scores.as_slice()
    }

    #[must_use]
    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    #[must_use]
    pub fn personal_best(&self) -> Option<u32> {
        self.top_three.first().copied()
    }

    #[must_use]
    pub fn personal_top_three(&self) -> Vec<u32> {
        self.top_three.clone()
    }
}
