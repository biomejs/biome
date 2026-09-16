use biome_service::projects::ProjectKey;

/// Represents an open [`textDocument`]. Can be cheaply cloned.
///
/// [`textDocument`]: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocumentItem
#[derive(Debug, Clone)]
pub(crate) struct Document {
    pub(crate) project_key: ProjectKey,
    pub(crate) version: i32,
}

impl Document {
    pub(crate) fn new(project_key: ProjectKey, version: i32) -> Self {
        Self {
            project_key,
            version,
        }
    }
}
