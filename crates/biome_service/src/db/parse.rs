//! Parses the texts that the workspace publishes in the [WorkspaceDb].

use super::{DefaultProject, ProjectsGeneration, WorkspaceDb};
use crate::file_handlers::{Features, ParseEmbeddedParams, ParseResult};
use crate::projects::ProjectDb;
use crate::settings::{
    SettingsEditorState, SettingsHandle, SettingsQuery, SettingsSelectionKey,
    settings_without_overrides,
};
use biome_db::Source;
use biome_json_parser::JsonParserOptions;
use biome_languages::{DocumentFileSource, LanguageDb};
use biome_parser::db::{ParseDb, SnippetOutput, SourceOutput};
use biome_rowan::NodeCache;
use tracing::error;

#[salsa::db]
impl ParseDb for WorkspaceDb {
    /// Parses the text of `source` with the parse capability of its file
    /// source.
    ///
    /// The settings are the ones of the project whose root contains the path
    /// of the text, or the project assigned to a virtual document, with the
    /// inline configuration of the client that published the text on top. A
    /// filesystem path outside every project gets the default settings.
    ///
    /// The workspace publishes only texts whose file source can be parsed. If
    /// it can't, the result is an empty document of an unknown file source.
    fn parse_source(&self, source: Source) -> SourceOutput {
        let data = self.data();
        let path = self.biome_path(source.path(self));

        // Salsa doesn't track the map of projects. Reading this counter makes
        // the parse run again when the project of the path could change.
        let _ = ProjectsGeneration::try_get(self).map(|generation| generation.value(self));
        let is_virtual = path.is_virtual();
        let settings_context = if is_virtual {
            self.project_key_for_source(source)
                .and_then(|project_key| self.get_project(&project_key))
                .map(|project| {
                    (
                        project,
                        SettingsSelectionKey::Root,
                        project.root_settings(self),
                    )
                })
        } else {
            self.find_project_for_path(path.as_path())
                .and_then(|project_key| {
                    self.get_settings_context_for_path(project_key, path.as_path())
                })
                .map(|(project, selection, _, settings)| (project, selection, settings))
        };
        let (project, selection, project_settings) = match settings_context {
            Some(settings_context) => settings_context,
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
        let virtual_project_settings;
        let project_settings = if is_virtual {
            virtual_project_settings = settings_without_overrides(project_settings.as_ref());
            &virtual_project_settings
        } else {
            project_settings.as_ref()
        };
        let inline_settings = source
            .configuration_index(self)
            .and_then(|index| self.inline_configuration(index))
            .map(|configuration| {
                let settings = project_settings.with_inline_configuration(configuration.clone());
                if is_virtual {
                    settings_without_overrides(&settings)
                } else {
                    settings
                }
            });
        let query = if is_virtual {
            SettingsQuery::new_without_overrides(project, selection, inline_settings)
        } else {
            SettingsQuery::new(
                project,
                selection,
                project_settings,
                inline_settings,
                path.as_path(),
            )
        };
        let settings = SettingsHandle::new(project_settings, SettingsEditorState::new(query));
        let effective_settings = settings
            .query()
            .inline_settings()
            .map_or(project_settings, AsRef::as_ref);

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
            .filter(|_| file_source.may_contain_embeds());
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
                SnippetOutput::new(
                    parse,
                    content.element_range,
                    content.content_range,
                    content.content_offset,
                    data.insert_source(source),
                )
            })
            .collect()
        });

        SourceOutput::new(any_parse, data.insert_source(file_source)).with_snippets(snippets)
    }
}

impl WorkspaceDb {
    /// Returns the parse of an empty document of an unknown file source, for a
    /// text that can't be parsed.
    fn empty_parse(&self) -> SourceOutput {
        let parse = biome_json_parser::parse_json("", JsonParserOptions::default());
        SourceOutput::new(
            parse.into(),
            self.data().insert_source(DocumentFileSource::Unknown),
        )
    }
}
