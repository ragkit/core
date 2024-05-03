use ragkit_ai::{
  error::Error,
  traits::Processor,
};

pub struct Named<'a> {
  #[allow(dead_code)]
  name: &'a str,
}

impl<'a> Named<'a> {
  pub fn new(name: &'a str) -> Self {
    Self { name }
  }
}

impl<'a> Processor<&'a str, &'a str> for Named<'a> {
  fn process(&self, input: &'a str) -> Result<&'a str, Error> {
    Ok(input)
  }
}
