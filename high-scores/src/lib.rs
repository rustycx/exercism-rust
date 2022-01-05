#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
    best_score: Option<u32>,
    last_score: Option<u32>,
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        Self {
            scores,
            best_score: scores.iter().copied().max(),
            last_score: scores.last().copied(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.last_score
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.best_score
    }

    pub fn personal_top_three0(&self) -> Vec<u32> {
        self.scores.iter().fold(vec![], |mut v, &score| {
            v.push(score);
            v.sort_by_key(|w| std::cmp::Reverse(*w));
            if v.len() > 3 {
                v[0..v.len() - 1].to_vec()
            } else {
                v
            }
        })
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scores = self.scores.to_vec();
        scores.sort_unstable_by(|a, b| b.cmp(a));
        scores.iter().take(3).cloned().collect::<Vec<u32>>()
        // scores.sort_unstable();
        // scores.into_iter().rev().take(3).collect::<Vec<u32>>()
    }
}
