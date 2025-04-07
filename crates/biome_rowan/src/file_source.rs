use std::fmt::Display;

/// Errors around the construct of the source type
#[derive(Debug)]
pub enum FileSourceError {
    /// The path has no file name
    MissingFileName,
    /// The path has no file extension
    MissingFileExtension,
    /// The source type is unknown
    UnknownExtension,
    /// The file name is unknown (not a well-known file name)
    UnknownFileName,
    /// The language id is unknown
    UnknownLanguageId,
}

impl std::error::Error for FileSourceError {}

impl Display for FileSourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingFileName => {
                write!(f, "The path has no file name")
            }
            Self::MissingFileExtension => {
                write!(f, "The path has no file extension")
            }
            Self::UnknownExtension => {
                write!(f, "The parser can't parse the extension yet")
            }
            Self::UnknownFileName => {
                write!(f, "The parser doesn't recognize the file yet")
            }
            Self::UnknownLanguageId => {
                write!(f, "The parser can't parse the language yet")
            }
        }
    }
}
