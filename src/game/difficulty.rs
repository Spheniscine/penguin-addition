use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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

impl Default for Difficulty {
    fn default() -> Self {
        Self { max_result: 10, min_addend: 1, max_addend: 1, operator: Operator::Plus }
    }
}