use crate::error::Error;
use std::marker::PhantomData;

// ============================================================================
// Processor definition
// ============================================================================

pub trait Processor<Input, Output> {
  fn process(&self, input: Input) -> Result<Output, Error>;

  // TODO: Not sure how helpful this is, can just handle errors in the process
  // implementation itself.
  fn on_err(&self, err: Error) -> Result<Output, Error> {
    Err(err)
  }

  fn run(&self, input: Input) -> Result<Output, Error> {
    let out = self.process(input);
    match out {
      Ok(o) => Ok(o),
      Err(e) => self.on_err(e),
    }
  }
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
