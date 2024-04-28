use crate::{
  error::Error,
  loc::Loc,
  tag::Tag,
};
use serde::{
  Deserialize,
  Serialize,
};
use std::collections::HashMap;

pub mod recursive;
pub mod simple;

pub trait Chunker<'a> {
  type Input;
  fn chunk(&self, input: Self::Input) -> Result<Vec<Chunk<'a>>, Error>;
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Chunk<'a> {
  #[serde(borrow)]
  Simple(SimpleChunk<'a>),
}

impl<'a> Chunk<'a> {
  pub fn content(&'a self) -> &'a str {
    match self {
      Chunk::Simple(simple) => simple.content,
    }
  }

  pub fn loc(&'a self) -> &'a Loc {
    match self {
      Chunk::Simple(simple) => &simple.loc,
    }
  }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SimpleChunk<'a> {
  pub content: &'a str,
  pub loc: Loc,
  pub tags: HashMap<&'a str, Tag<'a>>,
}

impl<'a> SimpleChunk<'a> {
  pub fn as_chunk(self) -> Chunk<'a> {
    Chunk::Simple(self)
  }
}
