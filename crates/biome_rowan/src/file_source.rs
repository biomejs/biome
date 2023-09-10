use crate::Language;
use std::any::TypeId;
use std::fmt::Display;
use std::path::{Path, PathBuf};

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

/// Generic trait that provides a method to safely create
/// a file source that can be send across thread boundaries.
///
/// When applying this trait, the [TryFrom] trait for [Path] must be implemented.
///
/// ```
/// use std::path::Path;
/// use biome_rowan::{FileSource, FileSourceError};
/// use biome_rowan::raw_language::RawLanguage;
/// struct UnknownFileSource {}
///
/// impl TryFrom<&Path> for UnknownFileSource {
///  type Error = FileSourceError;
///
///  fn try_from(value: &Path) -> Result<Self, Self::Error> {
///    Ok(UnknownFileSource {})
///   }
/// }
/// impl<'a> FileSource<'a, RawLanguage> for UnknownFileSource {}
/// ```
pub trait FileSource<'a, L: Language + 'static>:
    TryFrom<&'a Path, Error = FileSourceError>
{
    fn as_any_file_source(&self) -> AnyFileSource {
        AnyFileSource {
            file_source: TypeId::of::<L>(),
        }
    }
}

/// Generic file source that can be send across thread boundaries
#[derive(Clone)]
pub struct AnyFileSource {
    pub(crate) file_source: TypeId,
}

impl AnyFileSource {
    /// Attempts to retrieve the original file source of the file
    ///
    /// ## Errors
    ///
    /// The function will panic if:
    /// - the original type and given type mismatch
    /// - it's possible to retrieve the correct [FileSource] from the given [Path]
    pub fn unwrap_cast_from_path<'a, F, L>(&self, path: &'a Path) -> Result<F, FileSourceError>
    where
        F: FileSource<'a, L> + 'static,
        L: Language + 'static,
    {
        let file_source = TypeId::of::<L>();
        if file_source == self.file_source {
            F::try_from(path)
        } else {
            Err(FileSourceError::MissingFileExtension(PathBuf::from(path)))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::file_source::FileSourceError;
    use crate::raw_language::RawLanguage;
    use crate::FileSource;
    use std::path::{Path, PathBuf};

    #[test]
    fn should_cast_file_source() {
        #[derive(Debug, Eq, PartialEq)]
        struct Test {
            path: String,
        }

        impl TryFrom<&Path> for Test {
            type Error = FileSourceError;

            fn try_from(value: &Path) -> Result<Self, Self::Error> {
                Ok(Test {
                    path: value.display().to_string(),
                })
            }
        }

        impl<'a> FileSource<'a, RawLanguage> for Test {}

        let path = PathBuf::from("test");
        let first_test = Test {
            path: path.display().to_string(),
        };
        let send_first = first_test.as_any_file_source();

        let cast = send_first.unwrap_cast_from_path::<Test, RawLanguage>(path.as_path());

        assert!(cast.is_ok());

        let cast = cast.unwrap();

        assert_eq!(cast, first_test)
    }
}
