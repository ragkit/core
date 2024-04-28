use crate::{
  loc::Loc,
  tag::Tag,
};
use serde::{
  Deserialize,
  Serialize,
};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Element<'a> {
  #[serde(borrow)]
  Simple(SimpleElement<'a>),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SimpleElement<'a> {
  pub content: &'a str,
  pub loc: Loc,
  pub tags: HashMap<&'a str, Tag<'a>>,
}
