pub mod chunk;
pub mod document;
pub mod element;
pub mod error;
pub mod loc;
pub mod processors;
pub mod tag;
pub mod traits;

#[cfg(test)]
mod tests {
  use crate::{
    document::TextDocument,
    processors::simple_splitter::SimpleSplitterBuilder,
    traits::Pipeline,
  };

  #[test]
  fn it_works() {
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
