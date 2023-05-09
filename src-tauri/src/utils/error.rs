#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    ReadingStream(#[from] std::io::Error),

    #[error(transparent)]
    ParsingData(#[from] Box<bincode::ErrorKind>),

    #[error("Invalid status code")]
    InvalidStatusCode,

    #[error("{0}")]
    Connection(String),
}

// manually implement serde::Serialize
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
