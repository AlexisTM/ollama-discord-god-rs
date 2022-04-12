
#[derive(Debug)]
pub enum KirbyError {
  RequestError(reqwest::Error),
  SerdeError(serde_json::Error),
}
impl std::fmt::Display for KirbyError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
          KirbyError::RequestError(parse_int_error) =>
              write!(f, "{}", parse_int_error),
          KirbyError::SerdeError(io_error) =>
              write!(f, "{}", io_error),
      }
  }
}
impl std::error::Error for KirbyError {}
impl From<reqwest::Error> for KirbyError {
  fn from(err: reqwest::Error) -> Self {
      KirbyError::RequestError(err)
  }
}

impl From<serde_json::Error> for KirbyError {
  fn from(err: serde_json::Error) -> Self {
      KirbyError::SerdeError(err)
  }
}
