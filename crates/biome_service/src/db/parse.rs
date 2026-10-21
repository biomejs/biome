//! Parses the texts that the workspace publishes in the [WorkspaceDb].

use super::{DefaultProject, ProjectsGeneration, WorkspaceDb};
use crate::file_handlers::{Features, ParseEmbeddedParams, ParseResult};
use crate::projects::ProjectDb;
use crate::settings::{SettingsEditorState, SettingsHandle, SettingsQuery, SettingsSelectionKey};
use biome_db::Source;
use biome_fs::BiomePath;
use biome_json_parser::JsonParserOptions;
use biome_languages::{DocumentFileSource, LanguageDb};
use biome_parser::db::{ParseDb, SnippetParse, SourceParse};
use biome_rowan::NodeCache;
use tracing::error;

#[salsa::db]
impl ParseDb for WorkspaceDb {
    /// Parses the text with the parse capability of its file source, and the
    /// settings of the project whose root contains the path of the text, with
    /// the inline configuration of the client that published it on top. A
    /// path outside every project is parsed with the default settings.
    ///
    /// The workspace publishes only texts whose file source has a parse
    /// capability. Otherwise, the result is an empty document of an unknown
    /// file source.
    fn parse_source(&self, source: Source) -> SourceParse {
        let data = self.data();
        let path = BiomePath::new(source.path(self));

        // The project of the path is looked up in a map that Salsa doesn't
        // track, so the parse reads the generation of the map to run again
        // when the lookup can find another project.
        let _ = ProjectsGeneration::try_get(self).map(|generation| generation.value(self));
        let (project, selection, project_settings) = match self
            .find_project_for_path(&path)
            .and_then(|project_key| self.get_settings_context_for_path(project_key, &path))
        {
            Some((project, selection, _, settings)) => (project, selection, settings),
            None => {
                let Some(default_project) = DefaultProject::try_get(self) else {
                    error!(%path, "The database has no default project");
                    return self.empty_parse();
                };
                let project = default_project.project(self);
                (
                    project,
                    SettingsSelectionKey::Root,
                    project.root_settings(self),
                )
            }
        };
        let inline_settings = source
            .configuration_index(self)
            .and_then(|index| self.inline_configuration(index))
            .map(|configuration| {
                project_settings
                    .as_ref()
                    .with_inline_configuration(configuration.clone())
            });
        let query = SettingsQuery::new(
            project,
            selection,
            project_settings.as_ref(),
            inline_settings,
            &path,
        );
        let settings =
            SettingsHandle::new(project_settings.as_ref(), SettingsEditorState::new(query));
        let effective_settings = settings
            .query()
            .inline_settings()
            .map_or(project_settings.as_ref(), AsRef::as_ref);

        let features = Features::new();
        let file_source = effective_settings.file_source_with_parser_variants(
            self.source_from_index(source.file_source_index(self))
                .unwrap_or_default(),
        );
        let Some(parse) = features
            .get_deprecated_capabilities(file_source)
            .parser
            .parse
        else {
            error!(%path, ?file_source, "Parsing a text whose file source can't be parsed");
            return self.empty_parse();
        };

        let text = source.text(self);
        let node_cache = data.node_caches.pin().get(path.as_path()).cloned();
        let ParseResult {
            any_parse,
            language,
        } = match node_cache {
            Some(node_cache) => parse(&path, file_source, text, &settings, &mut node_cache.lock()),
            None => parse(
                &path,
                file_source,
                text,
                &settings,
                &mut NodeCache::default(),
            ),
        };
        let file_source = language.unwrap_or(file_source);

        let parse_embedded_nodes = features
            .get_deprecated_capabilities(file_source)
            .parser
            .parse_embedded_nodes
            .filter(|_| {
                DocumentFileSource::can_contain_embeds(
                    path.as_path(),
                    effective_settings.experimental_full_html_support_enabled(),
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
}

impl WorkspaceDb {
    /// Returns the parse of an empty document of an unknown file source, for a
    /// text that can't be parsed.
    fn empty_parse(&self) -> SourceParse {
        let parse = biome_json_parser::parse_json("", JsonParserOptions::default());
        SourceParse::new(
            parse.into(),
            self.data().insert_source(DocumentFileSource::Unknown),
        )
    }
}
