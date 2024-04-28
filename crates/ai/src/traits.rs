use crate::error::Error;
use std::marker::PhantomData;

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

impl<Input, Output, T> Processor<Input, Output> for T
where
  T: Fn(Input) -> Result<Output, Error>,
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

pub struct Pipeline<Input, Output, Intermediate, Curr, Prev>
where
  Curr: Processor<Intermediate, Output>,
  Prev: Processor<Input, Intermediate>,
{
  processor: Curr,
  prev: Prev,
  phantom: PhantomData<(Input, Output, Intermediate)>,
}

impl<Input, Output, Curr>
  Pipeline<Input, Output, Input, Curr, IdentityProcessor>
where
  Curr: Processor<Input, Output>,
{
  pub fn new(processor: Curr) -> Self {
    Pipeline {
      processor,
      prev: IdentityProcessor,
      phantom: PhantomData,
    }
  }
}

impl<Input, Output, Intermediate, Curr, Prev>
  Pipeline<Input, Output, Intermediate, Curr, Prev>
where
  Curr: Processor<Intermediate, Output>,
  Prev: Processor<Input, Intermediate>,
{
  pub fn chain<Next, P>(
    self,
    processor: P,
  ) -> Pipeline<Input, Next, Output, P, Self>
  where
    P: Processor<Output, Next>,
    Self: Sized,
  {
    Pipeline {
      processor,
      prev: self,
      phantom: PhantomData,
    }
  }
}

// Pipelines themselves are also processors for their inputs and outputs.
impl<Input, Output, Intermediate, Curr, Prev> Processor<Input, Output>
  for Pipeline<Input, Output, Intermediate, Curr, Prev>
where
  Curr: Processor<Intermediate, Output>,
  Prev: Processor<Input, Intermediate>,
{
  fn process(&self, input: Input) -> Result<Output, Error> {
    let intermediate = self.prev.process(input)?;
    self.processor.process(intermediate)
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
