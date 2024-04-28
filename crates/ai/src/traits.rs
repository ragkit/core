use crate::error::Error;
use std::marker::PhantomData;

// ============================================================================
// Processor definition
// ============================================================================

pub trait Processor<Input, Output> {
  fn process(&self, input: Input) -> Result<Output, Error>;
}

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

// ============================================================================
// Pipeline definition
// ============================================================================

pub struct Pipeline<Input, Output, IntermediateOut, IntermediateIn, Curr, Prev>
where
  Curr: Processor<IntermediateIn, Output>,
  Prev: Processor<Input, IntermediateOut>,
{
  processor: Curr,
  prev: Prev,
  phantom: PhantomData<(Input, Output, IntermediateOut, IntermediateIn)>,
}

// Creates a new pipeline. The "prev" processor will always be an
// IdentityProcessor that does nothing.
impl<Input, Output, Curr>
  Pipeline<Input, Output, Input, Input, Curr, IdentityProcessor>
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

impl<Input, Output, IntermediateOut, IntermediateIn, Curr, Prev>
  Pipeline<Input, Output, IntermediateOut, IntermediateIn, Curr, Prev>
where
  Curr: Processor<IntermediateIn, Output>,
  Prev: Processor<Input, IntermediateOut>,
  IntermediateOut: Into<IntermediateIn>,
{
  pub fn chain<NextIn, NextOut, Next>(
    self,
    processor: Next,
  ) -> Pipeline<Input, NextOut, Output, NextIn, Next, Self>
  where
    Next: Processor<NextIn, NextOut>,
    Output: Into<NextIn>,
    Self: Sized,
  {
    Pipeline {
      processor,
      prev: self,
      phantom: PhantomData,
    }
  }

  pub fn run(&self, input: impl Into<Input>) -> Result<Output, Error> {
    self.process(input.into())
  }
}

// Pipelines themselves are also processors for their inputs and outputs.
impl<Input, Output, IntermediateOut, IntermediateIn, Curr, Prev>
  Processor<Input, Output>
  for Pipeline<Input, Output, IntermediateOut, IntermediateIn, Curr, Prev>
where
  Curr: Processor<IntermediateIn, Output>,
  Prev: Processor<Input, IntermediateOut>,
  IntermediateOut: Into<IntermediateIn>,
{
  fn process(&self, input: Input) -> Result<Output, Error> {
    let intermediate = self.prev.process(input)?;
    self.processor.process(intermediate.into())
  }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
  use super::*;

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
  fn it_works() {
    let pipeline = Pipeline::new(Trimmer).chain(ToLowercase);
    let x = pipeline.process(" Hello, World! ").unwrap();
    assert_eq!(x, "hello, world!")
  }
}
