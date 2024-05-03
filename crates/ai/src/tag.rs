use crate::loc::Loc;
use serde::{
  Deserialize,
  Serialize,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Tag<'a> {
  pub key: &'a str,
  pub value: &'a str,
  pub loc: Loc,
}
