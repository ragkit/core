pub mod chunk;
pub mod document;
pub mod element;
pub mod error;
pub mod loc;
pub mod process;
pub mod tag;
pub mod traits;

#[cfg(test)]
mod tests {
  use crate::{
    document::TextDocument,
    error::Error,
    process::splitter::simple::SimpleSplitterBuilder,
    traits::{
      Pipeline,
      Processor,
    },
  };

  pub struct ToLowercase;

  impl<'a> Processor<&'a str, String> for ToLowercase {
    fn process(&self, input: &'a str) -> Result<String, Error> {
      Ok(input.to_ascii_lowercase())
    }
  }

  pub struct Trimmer;

  impl<'a> Processor<&'a str, &'a str> for Trimmer {
    fn process(&self, input: &'a str) -> Result<&'a str, Error> {
      Ok(input.trim())
    }
  }

  #[test]
  fn basic_pipeline() {
    let pipeline = Pipeline::new(Trimmer).chain(ToLowercase);
    let x = pipeline.process(" Hello, World! ").unwrap();
    assert_eq!(x, "hello, world!")
  }

  #[test]
  fn basic_splitting() {
    let doc = TextDocument::new("hello world");

    let splitter = SimpleSplitterBuilder::default()
      .chunk_size(5_u32)
      .build()
      .unwrap();

    let pipeline = Pipeline::new(splitter);

    let elements = pipeline.run(doc).unwrap();
    let strings = elements.iter().map(|el| el.content()).collect::<Vec<_>>();

    assert_eq!(strings, vec!["hello", " worl", "d"])
  }
}
