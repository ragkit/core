use crate::{
  error::Error,
  loc::Loc,
};

pub mod simple;

#[derive(Clone, Debug, PartialEq)]
pub enum Chunk<'a> {
  Simple(SimpleChunk<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct SimpleChunk<'a> {
  pub content: &'a str,
  pub loc: Loc,
}

pub trait Chunker<'a> {
  type Input;
  type Output;
  fn chunk(&self, input: Self::Input) -> Result<Vec<Self::Output>, Error>;
}
