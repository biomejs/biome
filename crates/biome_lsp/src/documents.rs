use biome_fs::BiomePath;
use biome_languages::DocumentFileSource;
use biome_line_index::LineIndex;
use biome_service::projects::ProjectKey;

/// Represents an open [`textDocument`]. Can be cheaply cloned.
///
/// [`textDocument`]: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocumentItem
#[derive(Debug, Clone)]
pub(crate) struct Document {
    pub(crate) project_key: ProjectKey,
    /// The workspace key of the document.
    pub(crate) path: BiomePath,
    pub(crate) file_source: DocumentFileSource,
    pub(crate) version: i32,
    pub(crate) line_index: LineIndex,
}

impl Document {
    /// Creates state for an open document.
    pub(crate) fn new(
        project_key: ProjectKey,
        path: BiomePath,
        file_source: DocumentFileSource,
        version: i32,
        text: &str,
    ) -> Self {
        Self {
            project_key,
            path,
            file_source,
            version,
            line_index: LineIndex::new(text),
        }
    }

    pub(crate) fn extension(&self) -> Option<&str> {
        self.path.extension().or_else(|| {
            let extension = match self.file_source {
                DocumentFileSource::Js(source) if source.as_embedding_kind().is_astro() => "astro",
                DocumentFileSource::Js(source) if source.as_embedding_kind().is_vue() => "vue",
                DocumentFileSource::Js(source) if source.as_embedding_kind().is_svelte() => {
                    "svelte"
                }
                _ => self.file_source.file_extension(),
            };
            (!extension.is_empty()).then_some(extension)
        })
    }
}
