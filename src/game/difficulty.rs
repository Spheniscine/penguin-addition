use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, EnumIter, Display, EnumString)]
pub enum Operator {
    Plus, Minus
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Difficulty {
    pub max_result: i32,
    pub min_addend: i32,
    pub max_addend: i32,
    pub operator: Operator,
}

impl Difficulty {
    pub const RESULT_MAXES: &[i32] = &[10, 20, 50, 100];
}

impl Default for Difficulty {

    fn default() -> Self {
        Self { max_result: 10, min_addend: 1, max_addend: 1, operator: Operator::Plus }
    }
}