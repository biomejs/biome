use biome_rowan::FileSourceError;
use std::{ffi::OsStr, path::Path};

#[derive(Debug, Default, Clone)]
pub struct HtmlFileSource {
    #[allow(unused)]
    variant: HtmlVariant,
}

#[derive(Debug, Default, Clone)]
enum HtmlVariant {
    #[default]
    Standard,
    Astro,
}

impl HtmlFileSource {
    pub fn html() -> Self {
        Self {
            variant: HtmlVariant::Standard,
        }
    }
    pub fn astro() -> Self {
        Self {
            variant: HtmlVariant::Astro,
        }
    }

    /// Try to return the HTML file source corresponding to this file name from well-known files
    pub fn try_from_well_known(file_name: &str) -> Result<Self, FileSourceError> {
        // TODO: to be implemented
        Err(FileSourceError::UnknownFileName(file_name.into()))
    }

    /// Try to return the HTML file source corresponding to this file extension
    pub fn try_from_extension(extension: &str) -> Result<Self, FileSourceError> {
        match extension {
            "html" => Ok(Self::html()),
            "astro" => Ok(Self::astro()),
            _ => Err(FileSourceError::UnknownExtension(
                Default::default(),
                extension.into(),
            )),
        }
    }

    /// Try to return the HTML file source corresponding to this language ID
    ///
    /// See the [LSP spec] and [VS Code spec] for a list of language identifiers
    ///
    /// The language ID for Astro is registered by its [VS Code extension]
    ///
    /// [LSP spec]: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocumentItem
    /// [VS Code spec]: https://code.visualstudio.com/docs/languages/identifiers
    /// [VS Code extension]: https://github.com/withastro/language-tools/blob/0503392b80765c8a1292ddc9c063a1187425c187/packages/vscode/package.json#L140
    pub fn try_from_language_id(language_id: &str) -> Result<Self, FileSourceError> {
        match language_id {
            "html" => Ok(Self::html()),
            "astro" => Ok(Self::astro()),
            _ => Err(FileSourceError::UnknownLanguageId(language_id.into())),
        }
    }
}

impl TryFrom<&Path> for HtmlFileSource {
    type Error = FileSourceError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let file_name = path
            .file_name()
            .and_then(OsStr::to_str)
            .ok_or_else(|| FileSourceError::MissingFileName(path.into()))?;

        if let Ok(file_source) = Self::try_from_well_known(file_name) {
            return Ok(file_source);
        }

        // We assume the file extensions are case-insensitive
        // and we use the lowercase form of them for pattern matching
        let extension = &path
            .extension()
            .and_then(OsStr::to_str)
            .map(str::to_lowercase)
            .ok_or_else(|| FileSourceError::MissingFileExtension(path.into()))?;

        Self::try_from_extension(extension)
    }
}
