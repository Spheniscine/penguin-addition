use arrayvec::ArrayVec;
use rand::{rng, seq::SliceRandom};
use serde::{Deserialize, Serialize};

use super::{Equation, NUM_BUCKETS};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    equations: [Equation; NUM_BUCKETS], // questions for the buckets
    permutation: [usize; NUM_BUCKETS], // the displayed order of the balls
    assignment: [Option<usize>; NUM_BUCKETS], // which balls are in each bucket
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
        }
    }
}