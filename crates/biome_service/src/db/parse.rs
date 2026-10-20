//! Parses the texts that the workspace publishes in the [WorkspaceDb].

use super::WorkspaceDb;
use crate::file_handlers::{Features, ParseEmbeddedParams, ParseResult};
use crate::settings::{SettingsEditorState, SettingsHandle, SettingsQuery};
use biome_db::{ParsedSource, SnippetParse, SourceParse, SourceText};
use biome_fs::BiomePath;
use biome_languages::{DocumentFileSource, LanguageDb};
use biome_rowan::NodeCache;
use parking_lot::Mutex;

/// Parses `text`, the content of `source`, and the snippets embedded in it.
///
/// The context of `text` holds the settings query of the request that
/// published the text. The project settings it selects are read through
/// Salsa, so changing them parses the text again. The inline configuration is
/// the one of that request.
///
/// `node_cache` is the cache kept for the path of `source`, if there is one.
///
/// ## Panics
///
/// Panics if the context of `text` isn't a [SettingsQuery], or if its file
/// source can't be parsed. The workspace checks both before it publishes a
/// text.
pub(super) fn parse_source_text(
    db: &WorkspaceDb,
    source: ParsedSource,
    text: &SourceText,
    node_cache: Option<&Mutex<NodeCache>>,
) -> SourceParse {
    let query = text
        .context
        .get::<SettingsQuery>()
        .expect("the workspace publishes texts with the settings query of their request");
    let project_settings = query.selection().selected_settings(db, query.project());
    let settings = SettingsHandle::new(
        project_settings.as_ref(),
        SettingsEditorState::new(query.clone()),
    );
    let path = BiomePath::new(source.path(db));
    let file_source = db
        .source_from_index(text.source_index)
        .expect("the source index of a published text points to an inserted file source");
    let features = Features::new();
    let parse = features
        .get_deprecated_capabilities(file_source)
        .parser
        .parse
        .expect("the workspace publishes texts only for file sources that can be parsed");

    let ParseResult {
        any_parse,
        language,
    } = match node_cache {
        Some(node_cache) => parse(
            &path,
            file_source,
            &text.text,
            &settings,
            &mut node_cache.lock(),
        ),
        None => parse(
            &path,
            file_source,
            &text.text,
            &settings,
            &mut NodeCache::default(),
        ),
    };
    let file_source = language.unwrap_or(file_source);

    let data = db.data();
    let parse_embedded_nodes = features
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
                data.insert_source(source),
            )
        })
        .collect()
    });

    SourceParse::new(any_parse, data.insert_source(file_source)).with_snippets(snippets)
}
