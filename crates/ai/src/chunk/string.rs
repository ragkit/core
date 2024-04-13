use super::{
  Chunker,
  SimpleChunk,
};
use crate::{
  error::Error,
  loc::Loc,
};
use derive_builder::Builder;

/// Simple chunking algorithm. Splits a string along character boundaries
/// according to the `chunk_size``. This should not be used in practice. It
/// serves as a reference point for more advanced chunking algorithms.
#[derive(Default, Builder, Debug)]
#[builder(setter(into))]
#[builder(build_fn(error = "crate::error::Error"))]
pub struct SimpleChunker {
  /// How large each chunk should be.
  chunk_size: u32,
}

impl<'a> Chunker<'a> for SimpleChunker {
  type Input = &'a str;
  type Output = SimpleChunk<'a>;

  fn chunk(&self, input: Self::Input) -> Result<Vec<Self::Output>, Error> {
    let chunk_size = self.chunk_size as usize;
    if chunk_size == 0 {
      return Err(Error::InvalidChunkSize(chunk_size as u32));
    }

    let estimated_chunks = input.len() / chunk_size + 1;
    let mut chunks: Vec<SimpleChunk> = Vec::with_capacity(estimated_chunks);

    // This always corresponds to the first byte in a valid UTF-8 code point
    // sequence.
    let mut start = 0;
    // This might temporarily point to the midle of a UTF-8 code point sequence.
    let mut end = 0;

    while start < input.len() {
      end = std::cmp::min(input.len(), end + chunk_size);
      // Naively incrementing by `chunk_size` could put us in the middle of a
      // UTF-8 code point sequence. We have to adjust `end` accordingly.
      end = next_boundary(input, end);
      chunks.push(SimpleChunk {
        content: &input[start..end],
        loc: Loc { start, end },
      });
      start = end;
    }

    Ok(chunks)
  }
}

// This finds the next valid character boundary in `string` that is >= `index`.
// Note: it may return `string.len()` which is always considered a valid
// character boundary.
fn next_boundary(string: &str, index: usize) -> usize {
  let mut res = index;
  while !string.is_char_boundary(res) {
    res += 1;
    if res >= string.len() {
      break;
    }
  }
  std::cmp::min(string.len(), res)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn basic() {
    let chunker = SimpleChunkerBuilder::default()
      .chunk_size(5u32)
      .build()
      .unwrap();

    let chunks = chunker.chunk("this is a test").unwrap();
    let content = chunks.iter().map(|c| c.content).collect::<Vec<_>>();
    assert_eq!(vec!["this ", "is a ", "test"], content)
  }

  #[test]
  fn chunk_size_0() {
    let chunker = SimpleChunkerBuilder::default()
      .chunk_size(0u32)
      .build()
      .unwrap();

    let chunks = chunker.chunk("test");
    assert!(chunks.is_err());
  }

  #[test]
  fn chunk_size_1() {
    let chunker = SimpleChunkerBuilder::default()
      .chunk_size(1u32)
      .build()
      .unwrap();

    let chunks = chunker.chunk("test").unwrap();
    let content = chunks.iter().map(|c| c.content).collect::<Vec<_>>();
    assert_eq!(vec!["t", "e", "s", "t"], content)
  }
}
