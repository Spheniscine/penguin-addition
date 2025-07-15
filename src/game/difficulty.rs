use indexmap::{IndexMap, indexmap};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, EnumIter, Display, EnumString, PartialEq, Eq)]
pub enum Operator {
    Plus, Minus
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Difficulty {
    pub max_result: i32,
    pub min_addend: i32,
    pub max_addend: i32,
    pub operator: Operator,
}

impl Difficulty {
    pub const RESULT_MAXES: &[i32] = &[10, 20, 50, 100];
    pub const STR_OPERATOR: &str = "operator";
    pub const STR_MAX_RESULT: &str = "max_result";
    pub const STR_ADDEND_RANGE: &str = "addend_range";
}

impl Difficulty {
    pub fn to_map(&self) -> IndexMap<String, String> {
        indexmap! {
            Self::STR_OPERATOR.into() => self.operator.to_string(),
            Self::STR_MAX_RESULT.into() => self.max_result.to_string(),
            Self::STR_ADDEND_RANGE.into() => format!("{},{}", self.min_addend, self.max_addend),
        }
    }

    pub fn from_map(map: &IndexMap<String, String>) -> Option<Self> {
        let max_result = map.get(Self::STR_MAX_RESULT)?.parse::<i32>().ok()?;
        let mut addends = map.get(Self::STR_ADDEND_RANGE)?.split(',');
        let min_addend = addends.next()?.parse::<i32>().ok()?;
        let max_addend = addends.next()?.parse::<i32>().ok()?;
        let operator = map.get(Self::STR_OPERATOR)?.parse::<Operator>().ok()?;

        Some(Self {
            max_result, min_addend, max_addend, operator
        })
    }
}

impl Default for Difficulty {
    fn default() -> Self {
        Self { max_result: 10, min_addend: 1, max_addend: 1, operator: Operator::Plus }
    }
}