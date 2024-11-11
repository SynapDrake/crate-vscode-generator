// The result type alias
pub type Result<T> = std::result::Result<T, Error>;

// The crate error
pub enum Error {
    Io(std::io::Error),
    Json(serde_json::Error),

    NameIsRequired,
    PrefixIsRequired,
    BodyIsEmpty,
    IndexOutOfBounds(usize)
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "{e}"),
            Self::Json(e) => write!(f, "{e}"),
            Self::NameIsRequired => write!(f, "Name is required"),
            Self::PrefixIsRequired => write!(f, "Prefix is required"),
            Self::BodyIsEmpty => write!(f, "Body cannot be empty"),
            Self::IndexOutOfBounds(n) => write!(f, "Index '{n}' out of bounds")
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}
