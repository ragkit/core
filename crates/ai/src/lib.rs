pub fn hello() -> &'static str {
  "Hello, world!"
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    assert_eq!("Hello, world!", hello())
  }
}
