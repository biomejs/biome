use biome_rowan::FileSourceError;
use biome_string_case::StrLikeExtension;
use camino::Utf8Path;

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(
    Debug, Clone, Default, Copy, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize,
)]
enum MarkdownVariant {
    #[default]
    Standard,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(
    Debug, Clone, Default, Copy, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct MdFileSource {
    variant: MarkdownVariant,
}

impl MdFileSource {
    pub fn markdown() -> Self {
        Self {
            variant: MarkdownVariant::Standard,
        }
    }

    /// Try to return the Markdown file source corresponding to this file name from well-known files
    pub fn try_from_well_known(_: &Utf8Path) -> Result<Self, FileSourceError> {
        Err(FileSourceError::UnknownFileName)
    }

    pub fn try_from_extension(extension: &str) -> Result<Self, FileSourceError> {
        match extension {
            #[cfg(feature = "experimental-markdown")]
            "md" | "markdown" => Ok(Self::markdown()),
            _ => Err(FileSourceError::UnknownExtension),
        }
    }

    pub fn try_from_language_id(language_id: &str) -> Result<Self, FileSourceError> {
        match language_id {
            #[cfg(feature = "experimental-markdown")]
            "markdown" => Ok(Self::markdown()),
            _ => Err(FileSourceError::UnknownLanguageId),
        }
    }
}

impl TryFrom<&Utf8Path> for MdFileSource {
    type Error = FileSourceError;

    fn try_from(path: &Utf8Path) -> Result<Self, Self::Error> {
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
