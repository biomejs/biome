use biome_rowan::FileSourceError;
use biome_string_case::StrLikeExtension;
use std::{ffi::OsStr, path::Path};
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(
    Debug, Clone, Default, Copy, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct GritFileSource {
    #[allow(unused)]
    variant: GritVariant,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(
    Debug, Clone, Default, Copy, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize,
)]
enum GritVariant {
    #[default]
    Standard,
}

impl GritFileSource {
    pub fn grit() -> Self {
        Self {
            variant: GritVariant::Standard,
        }
    }

    /// Try to return the HTML file source corresponding to this file name from well-known files
    pub fn try_from_well_known(_: &Path) -> Result<Self, FileSourceError> {
        // TODO: to be implemented
        Err(FileSourceError::UnknownFileName)
    }

    pub fn try_from_extension(extension: &OsStr) -> Result<Self, FileSourceError> {
        match extension.as_encoded_bytes() {
            b"grit" => Ok(Self::grit()),
            _ => Err(FileSourceError::UnknownExtension),
        }
    }

    pub fn try_from_language_id(language_id: &str) -> Result<Self, FileSourceError> {
        match language_id {
            "grit" => Ok(Self::grit()),
            _ => Err(FileSourceError::UnknownLanguageId),
        }
    }
}

impl TryFrom<&Path> for GritFileSource {
    type Error = FileSourceError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        if let Ok(file_source) = Self::try_from_well_known(path) {
            return Ok(file_source);
        }

        let Some(extension) = path.extension() else {
            return Err(FileSourceError::MissingFileExtension);
        };
        // We assume the file extensions are case-insensitive
        // and we use the lowercase form of them for pattern matching
        Self::try_from_extension(&extension.to_ascii_lowercase_cow())
    }
}
