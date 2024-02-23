use std::fmt::Display;
use std::path::PathBuf;

/// Errors around the construct of the source type
#[derive(Debug)]
pub enum FileSourceError {
    /// The path has no file name
    MissingFileName(PathBuf),
    /// The path has no file extension
    MissingFileExtension(PathBuf),
    /// The source type is unknown
    UnknownExtension(String, String),
}

impl std::error::Error for FileSourceError {}

impl Display for FileSourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileSourceError::MissingFileName(path) => {
                write!(f, "The path {path:?} has no file name")
            }
            FileSourceError::MissingFileExtension(path) => {
                write!(f, "The path {path:?} has no file extension")
            }
            FileSourceError::UnknownExtension(_, extension) => {
                write!(f, "The parser can't parse the extension '{extension}' yet")
            }
        }
    }
}
