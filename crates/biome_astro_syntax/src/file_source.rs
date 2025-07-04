use biome_rowan::FileSourceError;
use std::ffi::OsStr;
use std::path::Path;

#[derive(
    Debug, Clone, Default, Copy, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct AstroFileSource {
    variant: AstroVariant,
}

#[derive(
    Debug, Clone, Default, Copy, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum AstroVariant {
    #[default]
    Standard,
}

impl AstroFileSource {
    pub fn astro() -> Self {
        Self {
            variant: AstroVariant::Standard,
        }
    }

    pub fn variant(&self) -> AstroVariant {
        self.variant
    }

    pub fn with_variant(mut self, variant: AstroVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Try to return the Astro file source corresponding to this file name from well-known files
    pub fn try_from_well_known(file_name: &str) -> Result<Self, FileSourceError> {
        // No well-known Astro files currently
        Err(FileSourceError::UnknownFileName(file_name.into()))
    }

    /// Try to return the Astro file source corresponding to this file extension
    pub fn try_from_extension(extension: &OsStr) -> Result<Self, FileSourceError> {
        match extension.to_str() {
            Some("astro") => Ok(Self::astro()),
            _ => Err(FileSourceError::UnknownExtension(
                extension.to_string_lossy().into(),
            )),
        }
    }

    /// Try to return the Astro file source corresponding to this language ID
    ///
    /// See the [Language Identifiers] section on the VS Code docs for more information.
    ///
    /// [Language Identifiers]: https://code.visualstudio.com/docs/languages/identifiers
    pub fn try_from_language_id(language_id: &str) -> Result<Self, FileSourceError> {
        match language_id {
            "astro" => Ok(Self::astro()),
            _ => Err(FileSourceError::UnknownLanguageId(language_id.into())),
        }
    }
}

impl TryFrom<&Path> for AstroFileSource {
    type Error = FileSourceError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let file_name = path
            .file_name()
            .ok_or_else(|| FileSourceError::MissingFileName(path.into()))?
            .to_string_lossy();

        Self::try_from_well_known(&file_name)
            .or_else(|_| {
                path.extension()
                    .ok_or_else(|| FileSourceError::MissingFileExtension(path.into()))
                    .and_then(Self::try_from_extension)
            })
    }
}

impl AstroFileSource {
    pub fn to_extension(&self) -> &'static str {
        "astro"
    }
}