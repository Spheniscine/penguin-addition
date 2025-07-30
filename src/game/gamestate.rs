use arrayvec::ArrayVec;
use dioxus::logger::tracing;
use rand::{rng, seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};

use crate::Route;

use super::{difficulty, Difficulty, Equation, Feedback, FeedbackImpl, Operator, SettingsState, NUM_BUCKETS};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScreenState {
    Game, Settings, Help
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub route: Route,
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
    pub fn test_generate(route: Route) -> Self {
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
            route,
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

    pub fn apply_settings(&mut self, settings: SettingsState) {
        self.feedback.set_audio_state(settings.audio_state as f64 / 100.);

        let Some(difficulty) = Difficulty::from_map(&settings.difficulty_options) else {
            tracing::error!("error parsing settings difficulty map");
            return
        };
        if self.difficulty != difficulty || settings.reset_level {
            self.generate(difficulty);
        }
    }

    pub fn generate(&mut self, difficulty: Difficulty) {
        self.difficulty = difficulty;
        self.assignment = [None; NUM_BUCKETS];
        self.selected_ball = None;
        self.is_won = false;
        self.settings_cancelable = true;

        let rng = &mut rng();
        
        let mut answers = ArrayVec::<i32, NUM_BUCKETS>::new();
        let mut equations = ArrayVec::<Equation, NUM_BUCKETS>::new();
        while !equations.is_full() {
            let b = rng.random_range(difficulty.min_addend ..= difficulty.max_addend);
            let a = rng.random_range(1..=difficulty.max_result - b);
            let c = a + b;

            let answer = if difficulty.operator == Operator::Plus { c } else { a };
            if answers.contains(&answer) { continue; }
            answers.push(answer);

            let question = if difficulty.operator == Operator::Plus { 
                format!("{} + {}", a, b)
            } else { 
                format!("{} - {}", c, b)
            };
            equations.push(Equation { 
                question,
                answer: answer.to_string()
            })
        }

        let mut permutation = (0..NUM_BUCKETS).collect::<ArrayVec<usize, NUM_BUCKETS>>();
        permutation.shuffle(rng);

        self.equations = equations.into_inner().unwrap();
        self.permutation = permutation.into_inner().unwrap();
    }

    pub fn toggle_audio(&mut self) {
        self.feedback.toggle_audio();
        // LocalStorage.save_game_state(&self);
    }
}