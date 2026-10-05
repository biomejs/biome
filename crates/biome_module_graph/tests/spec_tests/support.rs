use std::collections::BTreeMap;

use biome_configuration::{Configuration, HtmlConfiguration};
use biome_css_parser::{CssModulesKind, CssParserOptions, parse_css};
use biome_db::{ParsedSnippet, ParsedSource};
use biome_fs::{BiomePath, MemoryFileSystem, OsFileSystem};
use biome_languages::css::{CssEmbeddingKind, EmbeddingHtmlKind, EmbeddingStyleApplicability};
use biome_languages::{CssFileSource, DocumentFileSource, HtmlFileSource, JsonFileSource};
use biome_module_graph::{
    ModuleInfoKind, resolve_css_module, resolve_html_module, resolve_js_module,
};
use biome_parser::AnyParse;
use biome_resolver::FsWithResolverProxy;
use biome_service::Workspace;
use biome_service::db::WorkspaceDb;
use biome_service::settings::ModuleGraphResolutionKind;
use biome_service::test_utils::setup_workspace_and_open_project;
use biome_service::workspace::UpdateSettingsParams;
use biome_test_utils::{get_added_js_paths, get_css_added_paths};
use camino::{Utf8Path, Utf8PathBuf};
use std::sync::Arc;

type HtmlTestFile<'a> = (
    &'a str,
    &'a str,
    HtmlFileSource,
    Vec<(AnyParse, CssFileSource)>,
);

/// A filesystem whose files a test database can read.
pub trait TestFs: FsWithResolverProxy {
    /// Returns a handle to the same files, for the resolver of a database.
    fn share(&self) -> Arc<dyn FsWithResolverProxy>;
}

impl TestFs for MemoryFileSystem {
    fn share(&self) -> Arc<dyn FsWithResolverProxy> {
        Arc::new(Self::from_files(self.files.0.clone()))
    }
}

impl TestFs for OsFileSystem {
    fn share(&self) -> Arc<dyn FsWithResolverProxy> {
        Arc::new(Self {
            working_directory: self.working_directory.clone(),
        })
    }
}

pub fn add_js_modules(
    db: &mut WorkspaceDb,
    fs: &dyn FsWithResolverProxy,
    paths: &[BiomePath],
    infer_types: bool,
) {
    for (path, root, semantic_model) in get_added_js_paths(fs, paths) {
        let (info, _, _) = resolve_js_module(&*db, root, path, semantic_model, infer_types);
        db.update_or_insert_module(path.as_path().to_path_buf(), ModuleInfoKind::Js(info));
    }
}

pub fn add_css_modules(db: &mut WorkspaceDb, fs: &dyn FsWithResolverProxy, paths: &[BiomePath]) {
    for (path, root) in get_css_added_paths(fs, paths) {
        let (info, _, _) = resolve_css_module(&*db, root, path);
        db.update_or_insert_module(path.as_path().to_path_buf(), ModuleInfoKind::Css(info));
    }
}

pub fn build_js_db(fs: &impl TestFs, paths: &[BiomePath], infer_types: bool) -> WorkspaceDb {
    let mut db = WorkspaceDb::new(fs.share());
    add_js_modules(&mut db, fs, paths, infer_types);
    db
}

/// Indexes a JSON document, such as a manifest, as a parsed source.
///
/// This uses setters, so resolutions already computed from the filesystem are
/// invalidated, like when the workspace opens a manifest.
pub fn add_json_file(db: &mut WorkspaceDb, path: &Utf8Path, source: &str) {
    let parse = biome_json_parser::parse_json(source, Default::default());
    let source_index = db.insert_source(DocumentFileSource::Json(JsonFileSource::json()));
    db.upsert_file(path, parse.into(), source_index, Vec::new());
}

pub fn build_css_db(files: &[(&str, &str)]) -> (MemoryFileSystem, WorkspaceDb) {
    let fs = MemoryFileSystem::default();
    for (path, source) in files {
        fs.insert((*path).into(), *source);
    }
    let paths = files
        .iter()
        .map(|(path, _)| BiomePath::new(*path))
        .collect::<Vec<_>>();
    let mut db = WorkspaceDb::new(fs.share());
    add_css_modules(&mut db, &fs, &paths);

    for (path, source) in files {
        let parse = parse_css(source, CssFileSource::css(), CssParserOptions::default());
        let parsed = ParsedSource::new(&db, Utf8PathBuf::from(*path), parse.into(), 0, Vec::new());
        db.insert_file(Utf8Path::new(path), parsed);
    }
    (fs, db)
}

pub fn parse_embedded_css(src: &str, file_source: CssFileSource) -> (AnyParse, CssFileSource) {
    let css_modules = match file_source.as_embedding_kind() {
        CssEmbeddingKind::Html(EmbeddingHtmlKind::Vue { .. }) => CssModulesKind::Vue,
        CssEmbeddingKind::Html(
            EmbeddingHtmlKind::Astro { .. } | EmbeddingHtmlKind::Svelte { .. },
        ) => CssModulesKind::Classic,
        _ => CssModulesKind::None,
    };
    let parsed = parse_css(
        src,
        file_source,
        CssParserOptions {
            css_modules,
            ..Default::default()
        },
    );
    (parsed.into(), file_source)
}

pub fn build_html_db(fs: &MemoryFileSystem, files: &[HtmlTestFile<'_>]) -> WorkspaceDb {
    let mut db = WorkspaceDb::new(fs.share());
    for (path, source, file_source, embedded) in files {
        let path = BiomePath::new(*path);
        let snippets = embedded
            .iter()
            .map(|(parsed, file_source)| {
                let source_index = db.insert_source(DocumentFileSource::Css(*file_source));
                ParsedSnippet::new(
                    &db,
                    parsed.clone(),
                    Default::default(),
                    Default::default(),
                    0.into(),
                    source_index,
                )
            })
            .collect();
        let parsed = biome_html_parser::parse_html(source, file_source.into());
        let source_index = db.insert_source(DocumentFileSource::Html(*file_source));
        let parsed_source = ParsedSource::new(
            &db,
            path.as_path().to_path_buf(),
            parsed.into(),
            source_index,
            snippets,
        );
        db.insert_file(path.as_path(), parsed_source);
        let (info, _, _) = resolve_html_module(&db, &path).unwrap();
        db.update_or_insert_module(path.as_path().to_path_buf(), ModuleInfoKind::Html(info));
    }
    db
}

pub fn html_css_source() -> CssFileSource {
    CssFileSource::css().with_embedding_kind(CssEmbeddingKind::Html(EmbeddingHtmlKind::Html))
}

pub fn vue_css_source(applicability: EmbeddingStyleApplicability) -> CssFileSource {
    CssFileSource::css().with_embedding_kind(CssEmbeddingKind::Html(EmbeddingHtmlKind::Vue {
        applicability,
    }))
}

pub fn astro_css_source(applicability: EmbeddingStyleApplicability) -> CssFileSource {
    CssFileSource::css().with_embedding_kind(CssEmbeddingKind::Html(EmbeddingHtmlKind::Astro {
        applicability,
    }))
}

pub fn svelte_css_source() -> CssFileSource {
    CssFileSource::css().with_embedding_kind(CssEmbeddingKind::Html(EmbeddingHtmlKind::Svelte {
        applicability: EmbeddingStyleApplicability::Local,
    }))
}

pub fn build_module_db_via_workspace(files: &[(&str, &str)]) -> WorkspaceDb {
    let fs = MemoryFileSystem::default();
    for (path, source) in files {
        fs.insert(Utf8PathBuf::from(*path), *source);
    }
    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/src");
    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            configuration: Configuration {
                html: Some(HtmlConfiguration {
                    experimental_full_support_enabled: Some(true.into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            workspace_directory: None,
            extended_configurations: vec![],
            module_graph_resolution_kind: ModuleGraphResolutionKind::Modules,
        })
        .unwrap();
    workspace.index_files_for_test(
        project_key,
        files.iter().map(|(path, _)| {
            let path = BiomePath::new(*path);
            let source = DocumentFileSource::from_well_known(path.as_path(), true);
            (path, source)
        }),
    );
    workspace.get_module_db_for_test()
}

pub fn snapshot_files(files: &[(&str, &str)]) -> BTreeMap<String, String> {
    files
        .iter()
        .map(|(path, source)| ((*path).into(), (*source).into()))
        .collect()
}
