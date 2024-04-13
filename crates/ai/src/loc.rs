/// The location in a corresponding string. Inclusive of `start`, exclusive of
/// `end`.
#[derive(Clone, Debug, PartialEq)]
pub struct Loc {
  pub start: usize,
  pub end: usize,
}
