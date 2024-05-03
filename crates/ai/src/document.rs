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
  content: &'a str,
}

impl<'a> TextDocument<'a> {
  pub fn new(content: &'a str) -> Self {
    Self { content }
  }
}

impl<'a> From<TextDocument<'a>> for &'a str {
  fn from(val: TextDocument<'a>) -> Self {
    val.content
  }
}
