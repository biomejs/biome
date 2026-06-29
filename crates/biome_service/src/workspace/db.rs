use crate::module_graph::PathInfoCache;
use crate::{WorkspaceError, embed::EmbedContent};
use biome_db::{ParsedSnippet, ParsedSource};
use biome_parser::AnyParse;
use biome_rowan::SendNode;
use biome_workspace_db::WorkspaceDb;
use camino::Utf8Path;

/// Represents the state of the database in the workspace.
#[derive(Default)]
pub(crate) struct DbState {
    db: WorkspaceDb,
    pub(crate) path_info_cache: PathInfoCache,
}

impl DbState {
    pub(crate) fn clone_db(&self) -> WorkspaceDb {
        self.db.clone()
    }

    pub(crate) fn add_source(
        &mut self,
        document_file_source: biome_languages::DocumentFileSource,
    ) -> usize {
        self.db.insert_source(document_file_source)
    }

    pub(crate) fn update_parsed_file(
        &mut self,
        path: &Utf8Path,
        parsed: AnyParse,
        language_index: usize,
        snippets: Vec<(AnyParse, EmbedContent, usize)>,
    ) -> ParsedSource {
        let parsed_snippets = snippets
            .into_iter()
            .map(|(parse, content, index)| {
                ParsedSnippet::new(
                    &self.db,
                    parse,
                    content.element_range,
                    content.content_range,
                    content.content_offset,
                    index,
                )
            })
            .collect();

        self.db
            .insert_or_update_file(path, parsed, language_index, parsed_snippets)
    }

    pub(crate) fn update_parsed_root(
        &mut self,
        path: &Utf8Path,
        new_root: SendNode,
    ) -> Result<ParsedSource, WorkspaceError> {
        self.db
            .update_parsed_root(path, new_root)
            .ok_or_else(|| WorkspaceError::not_found(path.to_string()))
    }

    pub(crate) fn unload_path(&self, path: &Utf8Path) {
        self.db.unload_path(path);
    }
}
