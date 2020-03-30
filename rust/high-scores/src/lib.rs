use std::collections::BinaryHeap;

#[derive(Debug)]
pub struct HighScores<'a> {
    scores : &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self   {
        HighScores {scores: scores}
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().cloned()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().cloned()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut top3: Vec<u32> = Vec::new();
        let mut heap = BinaryHeap::new();
        for i in self.scores {
            heap.push(i)
        }

        for _ in 0..3 {
            if let Some(x) = heap.pop() {
                top3.push(*x);
            }
        }
        top3
    }
}
