pub mod processors;

use processors::Named;
use ragkit_ai::{
  error::Error,
  traits::Pipeline,
};

#[tokio::main]
async fn main() {
  println!("== Demo ==");
  run_pipeline("input").expect("Shouldn't fail");
  println!("==========");
}

fn run_pipeline(s: &str) -> Result<&str, Error> {
  Pipeline::new(Named::new("From prompt, find potentialy relevant wikis"))
    .chain(Named::new("Fetch wikipedia data"))
    .chain(Named::new(""))
    .run(s)
}
