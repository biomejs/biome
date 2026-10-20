//! Parses the texts that the workspace publishes in the [WorkspaceDb](super::WorkspaceDb).

use super::WorkspaceDbData;
use crate::file_handlers::{Features, Parse, ParseEmbeddedParams, ParseResult};
use crate::settings::{SettingsEditorState, SettingsHandle, SettingsQuery};
use biome_db::{Db, ParsedSource, SnippetParse, SourceParse, TextParser};
use biome_fs::BiomePath;
use biome_languages::DocumentFileSource;
use biome_rowan::NodeCache;
use std::fmt::Debug;

/// Parses a text that the workspace publishes, with the parse capability of
/// its file source and the settings of the request that published it.
///
/// The project settings that the settings query selects are read through
/// Salsa, so changing them parses the text again. The inline configuration is
/// the one of the request.
pub(crate) struct WorkspaceTextParser {
    parse: Parse,
    file_source: DocumentFileSource,
    query: SettingsQuery,
    data: WorkspaceDbData,
}

impl WorkspaceTextParser {
    pub(crate) fn new(
        parse: Parse,
        file_source: DocumentFileSource,
        query: SettingsQuery,
        data: WorkspaceDbData,
    ) -> Self {
        Self {
            parse,
            file_source,
            query,
            data,
        }
    }
}

impl Debug for WorkspaceTextParser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WorkspaceTextParser")
            .field("file_source", &self.file_source)
            .field("query", &self.query)
            .finish_non_exhaustive()
    }
}

impl TextParser for WorkspaceTextParser {
    fn parse(&self, db: &dyn Db, source: ParsedSource, text: &str) -> SourceParse {
        let project_settings = self
            .query
            .selection()
            .selected_settings(db, self.query.project());
        let settings = SettingsHandle::new(
            project_settings.as_ref(),
            SettingsEditorState::new(self.query.clone()),
        );
        let path = BiomePath::new(source.path(db));

        let node_cache = self.data.node_caches.pin().get(path.as_path()).cloned();
        let ParseResult {
            any_parse,
            language,
        } = match node_cache {
            Some(node_cache) => (self.parse)(
                &path,
                self.file_source,
                text,
                &settings,
                &mut node_cache.lock(),
            ),
            None => (self.parse)(
                &path,
                self.file_source,
                text,
                &settings,
                &mut NodeCache::default(),
            ),
        };
        let file_source = language.unwrap_or(self.file_source);

        let parse_embedded_nodes = Features::new()
            .get_deprecated_capabilities(file_source)
            .parser
            .parse_embedded_nodes
            .filter(|_| {
                DocumentFileSource::can_contain_embeds(
                    path.as_path(),
                    settings.as_ref().experimental_full_html_support_enabled(),
                )
            });
        let snippets = parse_embedded_nodes.map_or_else(Vec::new, |parse_embedded_nodes| {
            parse_embedded_nodes(ParseEmbeddedParams {
                any_parse: &any_parse,
                path: &path,
                file_source: &file_source,
                settings: &settings,
                node_cache: &mut NodeCache::default(),
            })
            .nodes
            .into_iter()
            .map(|(parse, content, source)| {
                SnippetParse::new(
                    parse,
                    content.element_range,
                    content.content_range,
                    content.content_offset,
                    self.data.insert_source(source),
                )
            })
            .collect()
        });

        SourceParse::new(any_parse, self.data.insert_source(file_source)).with_snippets(snippets)
    }
}
