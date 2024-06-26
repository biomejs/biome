use biome_rowan::FileSourceError;
use std::ffi::OsStr;
use std::path::Path;

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(
    Debug, Clone, Default, Copy, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct GraphqlFileSource {
    #[allow(unused)]
    variant: GraphqlVariant,
}

/// The style of GraphQL contained in the file.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(
    Debug, Clone, Default, Copy, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize,
)]
enum GraphqlVariant {
    #[default]
    Standard,
}

impl GraphqlFileSource {
    pub fn graphql() -> Self {
        Self {
            variant: GraphqlVariant::Standard,
        }
    }

    /// Try to return the GraphQL file source corresponding to this file name from well-known files
    pub fn try_from_well_known(file_name: &str) -> Result<Self, FileSourceError> {
        // TODO: to be implemented
        Err(FileSourceError::UnknownFileName(file_name.into()))
    }

    /// Try to return the GraphQL file source corresponding to this file extension
    pub fn try_from_extension(extension: &str) -> Result<Self, FileSourceError> {
        match extension {
            "graphql" | "gql" => Ok(Self::default()),
            _ => Err(FileSourceError::UnknownExtension(
                Default::default(),
                extension.into(),
            )),
        }
    }

    /// Try to return the GraphQL file source corresponding to this language ID
    ///
    /// See the [LSP spec] and [VS Code spec] for a list of language identifiers
    ///
    /// [LSP spec]: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocumentItem
    /// [VS Code spec]: https://code.visualstudio.com/docs/languages/identifiers
    pub fn try_from_language_id(language_id: &str) -> Result<Self, FileSourceError> {
        match language_id {
            "graphql" | "gql" => Ok(Self::default()),
            _ => Err(FileSourceError::UnknownLanguageId(language_id.into())),
        }
    }
}

impl TryFrom<&Path> for GraphqlFileSource {
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
