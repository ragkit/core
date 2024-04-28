use crate::{
  document::Document,
  element::Element,
  error::Error,
};

pub trait Splitter<'a> {
  fn split(&self, input: Document<'a>) -> Result<Vec<Element<'a>>, Error>;
}

// ============================================================================
// Processor definition
// ============================================================================

pub trait Processor<Input, Output> {
  fn process(&self, input: Input) -> Result<Output, Error>;
}

// ============================================================================
// Processor examples
// ============================================================================

pub struct IdentityProcessor;

impl<Input> Processor<Input, Input> for IdentityProcessor {
  fn process(&self, input: Input) -> Result<Input, Error> {
    Ok(input)
  }
}

impl<Input, Output, T: Fn(Input) -> Result<Output, Error>>
  Processor<Input, Output> for T
{
  fn process(&self, input: Input) -> Result<Output, Error> {
    self(input)
  }
}

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

// ============================================================================
// Pipeline definition
// ============================================================================

pub struct Pipeline<Input, Output> {
  processor: Box<dyn Processor<Input, Output> + 'static>,
}

impl<Input, Output> Pipeline<Input, Output> {
  pub fn new<P>(processor: P) -> Self
  where
    P: Processor<Input, Output> + 'static,
  {
    Pipeline {
      processor: Box::new(processor),
    }
  }

  pub fn chain<Next, NextOutput>(
    self,
    next: Next,
  ) -> Pipeline<Input, NextOutput>
  where
    Next: Processor<Output, NextOutput> + 'static,
    Input: 'static,
    Output: 'static,
  {
    let closure = move |input: Input| -> Result<NextOutput, Error> {
      let intermediate = self.processor.process(input)?;
      next.process(intermediate)
    };
    Pipeline {
      processor: Box::new(closure),
    }
  }
}

// Pipelines themselves are also processors for their inputs and outputs.
impl<Input, Output> Processor<Input, Output> for Pipeline<Input, Output> {
  fn process(&self, input: Input) -> Result<Output, Error> {
    self.processor.process(input)
  }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let pipeline = Pipeline::new(Trimmer).chain(ToLowercase);
    let x = pipeline.process(" Hello, World! ").unwrap();
    assert_eq!(x, "hello, world!")
  }
}
