#[derive(Debug)]
pub enum GodError {
    RequestError(reqwest::Error),
    SerdeError(serde_json::Error),
}
impl std::fmt::Display for GodError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GodError::RequestError(parse_int_error) => write!(f, "{}", parse_int_error),
            GodError::SerdeError(io_error) => write!(f, "{}", io_error),
        }
    }
}
impl std::error::Error for GodError {}
impl From<reqwest::Error> for GodError {
    fn from(err: reqwest::Error) -> Self {
        GodError::RequestError(err)
    }
}

impl From<serde_json::Error> for GodError {
    fn from(err: serde_json::Error) -> Self {
        GodError::SerdeError(err)
    }
}
