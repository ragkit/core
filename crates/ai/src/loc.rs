use serde::{
  Deserialize,
  Serialize,
};

/// The location in a corresponding string. Inclusive of `start`, exclusive of
/// `end`.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Loc {
  pub start: usize,
  pub end: usize,
}

impl Loc {
  pub fn as_tuple(&self) -> (usize, usize) {
    (self.start, self.end)
  }
}
