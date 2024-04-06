use biome_rowan::FileSourceError;
use std::{ffi::OsStr, path::Path};

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(
    Debug, Clone, Default, Copy, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct CssFileSource {
    // Unused until we potentially support postcss/less/sass
    #[allow(unused)]
    variant: CssVariant,
}

/// The style of CSS contained in the file.
///
/// Currently, Biome only supports plain CSS, and aims to be compatible with
/// the latest Recommendation level standards.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(
    Debug, Clone, Default, Copy, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize,
)]
enum CssVariant {
    #[default]
    Standard,
}

impl CssFileSource {
    pub fn css() -> Self {
        Self {
            variant: CssVariant::Standard,
        }
    }

    pub fn try_from_extension(extension: &str) -> Result<Self, FileSourceError> {
        match extension {
            "css" => Ok(Self::css()),
            _ => Err(FileSourceError::UnknownExtension(
                Default::default(),
                extension.into(),
            )),
        }
    }

    pub fn try_from_well_known(file_name: &str) -> Result<Self, FileSourceError> {
        // TODO: to be implemented
        Err(FileSourceError::UnknownFileName(file_name.into()))
    }

    pub fn try_from_language_id(language_id: &str) -> Result<Self, FileSourceError> {
        match language_id {
            "css" => Ok(Self::css()),
            _ => Err(FileSourceError::UnknownLanguageId(language_id.into())),
        }
    }
}

impl TryFrom<&Path> for CssFileSource {
    type Error = FileSourceError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let file_name = path
            .file_name()
            .and_then(OsStr::to_str)
            .ok_or_else(|| FileSourceError::MissingFileName(path.into()))?;

        if let Ok(file_source) = Self::try_from_well_known(file_name) {
            return Ok(file_source);
        }

        let extension = &path
            .extension()
            .and_then(OsStr::to_str)
            .map(str::to_lowercase)
            .ok_or_else(|| FileSourceError::MissingFileExtension(path.into()))?;

        Self::try_from_extension(extension)
    }
}
