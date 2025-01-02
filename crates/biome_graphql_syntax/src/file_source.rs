use biome_rowan::FileSourceError;
use biome_string_case::StrLikeExtension;
use camino::Utf8Path;

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(
    Debug, Clone, Default, Copy, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct GraphqlFileSource {
    variant: GraphqlVariant,
}

/// The style of GraphQL contained in the file.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(
    Debug, Clone, Default, Copy, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "camelCase")]
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
    pub fn try_from_well_known(_: &Utf8Path) -> Result<Self, FileSourceError> {
        // TODO: to be implemented
        Err(FileSourceError::UnknownFileName)
    }

    /// Try to return the GraphQL file source corresponding to this file extension
    pub fn try_from_extension(extension: &str) -> Result<Self, FileSourceError> {
        // We assume the file extension is normalized to lowercase
        match extension {
            "graphql" | "gql" => Ok(Self::default()),
            _ => Err(FileSourceError::UnknownExtension),
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
            _ => Err(FileSourceError::UnknownLanguageId),
        }
    }
}

impl TryFrom<&Utf8Path> for GraphqlFileSource {
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
