use super::{
  simple::SimpleChunkerBuilder,
  Chunk,
  Chunker,
  SimpleChunk,
};
use crate::{
  error::Error,
  loc::Loc,
};
use derive_builder::Builder;

/// Recursive chunking algorithm. Splits based on the first separator, then
/// recurses with the next separator. Useful for splitting into logical units,
/// e.g. split by paragraphs and then sentences.
#[derive(Default, Builder, Debug)]
#[builder(setter(into))]
#[builder(build_fn(error = "crate::error::Error"))]
pub struct RecursiveChunker<'sep> {
  /// How large each chunk should be.
  chunk_size: u32,
  separators: Vec<&'sep str>,
}

impl<'a> Chunker<'a> for RecursiveChunker<'a> {
  type Input = &'a str;

  fn chunk(&self, input: Self::Input) -> Result<Vec<Chunk<'a>>, Error> {
    let chunk_size = self.chunk_size as usize;
    if chunk_size == 0 {
      return Err(Error::InvalidChunkSize(chunk_size as u32));
    }

    let mut parts = vec![Part::String(input)];
    for sep in &self.separators {
      parts = parts
        .iter()
        .flat_map(|p| {
          match p {
            Part::String(s) => {
              s.split(sep)
                .flat_map(|p| vec![Part::Sep(sep), Part::String(p)])
                // Remove the first separator; it's always a fake one.
                .skip(1)
                .collect::<Vec<Part<'a>>>()
            }
            Part::Sep(s) => vec![Part::Sep(s)],
          }
        })
        .collect::<Vec<_>>();
    }

    let mut chunks = vec![];
    let mut start = 0;
    for part in parts {
      match part {
        // If we encounter a separator, we just need to increment start for
        // bookkeeping purposes. Necessary to generate correct `Loc`s.
        Part::Sep(s) => start += s.len(),
        // If we have real content, check if it fits into the chunk size. If
        // not do some simple chunking.
        Part::String(s) => {
          if s.is_empty() {
            continue;
          }

          let end = start + s.len();
          if s.len() > chunk_size {
            // We need to further chunk the string.
            let simple_chunker = SimpleChunkerBuilder::default()
              .chunk_size(chunk_size as u32)
              .loc_offset(start)
              .build()?;
            chunks.extend(simple_chunker.chunk(s)?)
          } else {
            chunks.push(Chunk::Simple(SimpleChunk {
              content: &input[start..end],
              loc: Loc { start, end },
            }))
          }
          start = end;
        }
      }
    }

    Ok(chunks)
  }
}

#[derive(Clone, Debug)]
enum Part<'a> {
  String(&'a str),
  Sep(&'a str),
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
}
