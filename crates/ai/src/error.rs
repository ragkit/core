use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum Error {
  #[error("Invalid chunk size: {0}")]
  InvalidChunkSize(u32),

  #[error("Uninitialized field: {0}")]
  UninitializedField(&'static str),
}

impl From<derive_builder::UninitializedFieldError> for Error {
  fn from(value: derive_builder::UninitializedFieldError) -> Self {
    Error::UninitializedField(value.field_name())
  }
}
