use arrayvec::ArrayVec;
use rand::{rng, seq::SliceRandom};
use serde::{Deserialize, Serialize};

use super::{Equation, NUM_BUCKETS};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub equations: [Equation; NUM_BUCKETS], // questions for the buckets
    pub permutation: [usize; NUM_BUCKETS], // the displayed order of the balls
    pub assignment: [Option<usize>; NUM_BUCKETS], // which balls are in each bucket
    pub selected_ball: Option<usize>, // index of selected ball
}

impl GameState {
    pub fn test_generate() -> Self {
        let rng = &mut rng();
        let mut pool = (1..=9).collect::<Vec<_>>();
        let pool = pool.partial_shuffle(rng, NUM_BUCKETS).0;

        let equations = (0..NUM_BUCKETS).map(|i| {
            let x = pool[i];
            let question = format!("{} + 1", x);
            let answer = (x + 1).to_string();
            Equation {
                question, answer
            }
        }).collect::<ArrayVec<Equation, NUM_BUCKETS>>();

        let mut permutation = (0..NUM_BUCKETS).collect::<ArrayVec<usize, NUM_BUCKETS>>();
        permutation.shuffle(rng);

        Self {
            equations: equations.into_inner().unwrap(),
            permutation: permutation.into_inner().unwrap(),
            assignment: [None; NUM_BUCKETS],
            selected_ball: None,
        }
    }

    pub fn click_ball(&mut self, index: usize) {
        if self.selected_ball == Some(index) {
            self.selected_ball = None;
        } else {
            self.selected_ball = Some(index);
        }
    } 
}