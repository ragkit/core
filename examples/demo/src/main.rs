use ragkit_ai::{
  error::Error,
  traits::{
    Pipeline,
    Processor,
  },
};

#[allow(dead_code)]
struct Named<'a> {
  name: &'a str,
}

impl<'a> Named<'a> {
  fn new(name: &'a str) -> Self {
    Self { name }
  }
}

impl<'a> Processor<&'a str, &'a str> for Named<'a> {
  fn process(&self, input: &'a str) -> Result<&'a str, Error> {
    Ok(input)
  }
}

#[tokio::main]
async fn main() {
  println!("== Demo ==");
  run_pipeline("input").expect("Shouldn't fail");
}

fn run_pipeline(s: &str) -> Result<&str, Error> {
  Pipeline::new(Named::new("From prompt, find potentiall relevant wikis"))
    .chain(Named::new("Fetch wikipedia data"))
    .chain(Named::new(""))
    .run(s)
}
