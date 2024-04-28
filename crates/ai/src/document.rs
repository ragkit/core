use serde::{
  Deserialize,
  Serialize,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Document<'a> {
  #[serde(borrow)]
  Text(TextDocument<'a>),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TextDocument<'a> {
  pub content: &'a str,
}
