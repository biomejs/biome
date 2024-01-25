use crate::PhpLanguage;
use biome_rowan::{FileSource, FileSourceError};
use std::path::Path;

#[derive(Debug, Default, Clone)]
pub struct PhpFileSource {
    #[allow(unused)]
    variant: PhpVariant,
}

#[derive(Debug, Default, Clone)]
enum PhpVariant {
    #[default]
    Standard,
}

impl PhpFileSource {
    pub fn php() -> Self {
        Self {
            variant: PhpVariant::Standard,
        }
    }
}

impl<'a> FileSource<'a, PhpLanguage> for PhpFileSource {}

impl TryFrom<&Path> for PhpFileSource {
    type Error = FileSourceError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let file_name = path
            .file_name()
            .ok_or_else(|| FileSourceError::MissingFileName(path.into()))?
            .to_str()
            .ok_or_else(|| FileSourceError::MissingFileName(path.into()))?;

        let extension = path
            .extension()
            .ok_or_else(|| FileSourceError::MissingFileExtension(path.into()))?
            .to_str()
            .ok_or_else(|| FileSourceError::MissingFileExtension(path.into()))?;

        compute_source_type_from_path_or_extension(file_name, extension)
    }
}

/// Deduces the [PhpFileSource] from the file name and its extension
fn compute_source_type_from_path_or_extension(
    file_name: &str,
    extension: &str,
) -> Result<PhpFileSource, FileSourceError> {
    Ok(match extension {
        "php" => PhpFileSource::php(),
        _ => {
            return Err(FileSourceError::UnknownExtension(
                file_name.into(),
                extension.into(),
            ));
        }
    })
}
