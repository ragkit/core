use super::{
  recursive::RecursiveChunkerBuilder,
  Chunk,
  Chunker,
  SimpleChunk,
};
use crate::{
  error::Error,
  loc::Loc,
};
use derive_builder::Builder;
use regex::Regex;

/// Preconfigured chunker for markdown that adds additional context to each
/// chunk using the `Tag` api. Available tags include:
/// - `h1`
/// - `h2`
/// - `h3`
/// - `h4`
#[derive(Default, Builder, Debug)]
#[builder(setter(into))]
#[builder(build_fn(error = "crate::error::Error"))]
pub struct MarkdownChunker {
  /// How large each chunk should be.
  chunk_size: u32,
}

impl<'a> Chunker<'a> for MarkdownChunker {
  type Input = &'a str;

  fn chunk(&self, input: Self::Input) -> Result<Vec<Chunk<'a>>, Error> {
    let chunker = RecursiveChunkerBuilder::default()
      .chunk_size(self.chunk_size)
      .separators(vec!["\n\n", "\n", " "])
      .build()
      .unwrap();

    let chunks = chunker.chunk(input)?;
    // Reconstruct the chunks, adding additional context in the form of tags.
    let mut result: Vec<Chunk<'a>> = vec![];

    let mut h1: Option<&Chunk<'a>> = None;
    let mut h2: Option<&Chunk<'a>> = None;
    let mut h3: Option<&Chunk<'a>> = None;
    let mut h4: Option<&Chunk<'a>> = None;
    // TODO: Write a function to attach all the headers to a chunk.

    for chunk in chunks {
      if chunk.is_h1() {
      } else if chunk.is_h2() {
      } else if chunk.is_h3() {
      } else if chunk.is_h4() {
      } else {
        // TODO: Add context.
        result.push(chunk);
      }
    }

    Ok(result)
  }
}

impl<'a> Chunk<'a> {
  fn is_h1(&self) -> bool {
    let re = Regex::new(r"^#[^#]").unwrap();
    re.is_match_at(self.content(), 0)
  }

  fn is_h2(&self) -> bool {
    let re = Regex::new(r"^##[^#]").unwrap();
    re.is_match_at(self.content(), 0)
  }

  fn is_h3(&self) -> bool {
    let re = Regex::new(r"^###[^#]").unwrap();
    re.is_match_at(self.content(), 0)
  }

  fn is_h4(&self) -> bool {
    let re = Regex::new(r"^####[^#]").unwrap();
    re.is_match_at(self.content(), 0)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn basic() {
    let chunker = RecursiveChunkerBuilder::default()
      .chunk_size(5u32)
      .separators(vec![" "])
      .build()
      .unwrap();

    let chunks = chunker.chunk("this is a test").unwrap();
    let content = chunks.iter().map(|c| c.content()).collect::<Vec<_>>();
    assert_eq!(vec!["this", "is", "a", "test"], content);
  }

  #[test]
  fn needs_simple() {
    let chunker = RecursiveChunkerBuilder::default()
      .chunk_size(3u32)
      .separators(vec![" "])
      .build()
      .unwrap();

    let chunks = chunker.chunk("this is a test").unwrap();
    let content = chunks.iter().map(|c| c.content()).collect::<Vec<_>>();
    assert_eq!(vec!["thi", "s", "is", "a", "tes", "t"], content);
  }

  #[test]
  fn multi_sep() {
    let chunker = RecursiveChunkerBuilder::default()
      .chunk_size(3u32)
      .separators(vec!["  ", " "])
      .build()
      .unwrap();

    // Indices:                 01234567890123456
    let chunks = chunker.chunk("000 111  222  333").unwrap();
    let content = chunks.iter().map(|c| c.content()).collect::<Vec<_>>();
    assert_eq!(vec!["000", "111", "222", "333"], content);

    let locs = chunks
      .iter()
      .map(|c| c.loc().as_tuple())
      .collect::<Vec<_>>();
    assert_eq!(vec![(0, 3), (4, 7), (9, 12), (14, 17)], locs);
  }

  #[test]
  fn multi_sep_2() {
    let chunker = RecursiveChunkerBuilder::default()
      .chunk_size(10u32)
      .separators(vec!["  ", " "])
      .build()
      .unwrap();

    let chunks = chunker.chunk("000 111  222  333").unwrap();
    let content = chunks.iter().map(|c| c.content()).collect::<Vec<_>>();
    assert_eq!(vec!["000 111", "222", "333"], content);
  }
}
