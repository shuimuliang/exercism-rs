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
        let last_value = self.scores.last();
        match last_value {
            Some(&x)  => Some(x),
            None => None,
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        let max_value = self.scores.iter().max();
        match max_value {
            Some(&x) => Some(x) ,
            None => None,
        }
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
