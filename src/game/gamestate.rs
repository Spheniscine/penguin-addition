use arrayvec::ArrayVec;
use rand::{rng, seq::SliceRandom};
use serde::{Deserialize, Serialize};

use super::{Difficulty, Equation, Feedback, FeedbackImpl, SettingsState, NUM_BUCKETS};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScreenState {
    Game, Settings, Help
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub difficulty: Difficulty,
    pub equations: [Equation; NUM_BUCKETS], // questions for the buckets
    pub permutation: [usize; NUM_BUCKETS], // the displayed order of the balls
    pub assignment: [Option<usize>; NUM_BUCKETS], // which balls are in each bucket
    pub selected_ball: Option<usize>, // index of selected ball
    pub feedback: FeedbackImpl,
    pub is_won: bool,
    pub screen_state: ScreenState,
    pub settings_cancelable: bool,
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
            difficulty: Difficulty::default(),
            equations: equations.into_inner().unwrap(),
            permutation: permutation.into_inner().unwrap(),
            assignment: [None; NUM_BUCKETS],
            selected_ball: None,
            feedback: FeedbackImpl { audio_state: 1., prev_audio_state: 1. },
            is_won: false,
            screen_state: ScreenState::Settings,
            settings_cancelable: false,
        }
    }

    pub fn click_ball(&mut self, index: usize) {
        if self.selected_ball == Some(index) {
            self.selected_ball = None;
        } else {
            self.selected_ball = Some(index);
        }
    } 

    pub fn click_bucket(&mut self, index: usize) {
        if let Some(ball) = self.selected_ball {
            for x in &mut self.assignment {
                if *x == Some(ball) { *x = None; }
            }
            self.assignment[index] = Some(ball);
            self.selected_ball = None;
        }
    }

    pub fn click_ball_slot(&mut self) {
        if let Some(ball) = self.selected_ball {
            for x in &mut self.assignment {
                if *x == Some(ball) { *x = None; }
            }
            self.selected_ball = None;
        }
    }

    pub fn should_show_check_button(&self) -> bool {
        !self.is_won && self.assignment.iter().all(|x| x.is_some())
    }

    pub fn check(&mut self) -> bool {
        let mut ans = true;

        for i in 0..NUM_BUCKETS {
            if self.assignment[i] != Some(i) {
                ans = false;
                self.assignment[i] = None;
            }
        }

        ans
    }

    pub fn get_settings_state(&self) -> SettingsState {
        SettingsState {
            difficulty_options: self.difficulty.to_map(),
            audio_state: (self.feedback.get_audio_state() * 100.).round() as i32,
            reset_level: !self.settings_cancelable,
        }
    }
}