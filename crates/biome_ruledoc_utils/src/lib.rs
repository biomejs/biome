mod codeblock;

use biome_db::ParsedSource;
use biome_fs::{BiomePath, MemoryFileSystem};
use biome_js_analyze::JsAnalyzerServices;
use biome_js_semantic::semantic_model_from_source;
use biome_json_parser::{JsonParserOptions, parse_json};
use biome_languages::{DocumentFileSource, JsFileSource};
use biome_module_graph::{
    ModuleInfoKind, PathInfoCache, resolve_css_module, resolve_html_module, resolve_js_module,
};
use biome_project_layout::ProjectLayout;
use biome_test_utils::{get_added_js_paths, get_css_added_paths, get_html_added_paths};
use biome_workspace_db::WorkspaceDb;
use camino::Utf8PathBuf;
pub use codeblock::*;
use std::collections::HashMap;
use std::hash::BuildHasher;
use std::sync::Arc;

/// Builder that can be used for constructing analyzer services.
///
/// The builder can be reused to create cheap instances of analyzer services
/// for multiple code blocks.
pub struct AnalyzerServicesBuilder {
    module_db: WorkspaceDb,
    project_layout: Arc<ProjectLayout>,
}

impl AnalyzerServicesBuilder {
    /// Creates a service builder from a map of `files`.
    ///
    /// Constructs an in-memory file system from the given files and uses it
    /// to initialise a module graph and project layout.
    ///
    /// # Arguments
    ///
    /// * `files` - A map of file paths to their contents.
    pub fn from_files<S: BuildHasher>(files: HashMap<String, String, S>) -> Self {
        if files.is_empty() {
            let db = WorkspaceDb::default();
            return Self {
                module_db: db,
                project_layout: Default::default(),
            };
        }

        let fs = MemoryFileSystem::default();
        let layout = ProjectLayout::default();
        let path_info_cache = PathInfoCache::default();

        let mut js_paths = Vec::new();
        let mut css_paths = Vec::new();
        let mut html_paths = Vec::new();

        for (path, src) in files {
            let path_buf = Utf8PathBuf::from(path);
            let biome_path = BiomePath::new(&path_buf);
            if biome_path.is_manifest() {
                match biome_path.file_name() {
                    Some("package.json") => {
                        let parsed = parse_json(&src, JsonParserOptions::default());
                        layout.insert_serialized_node_manifest(
                            path_buf.parent().unwrap().into(),
                            &parsed.syntax().as_send().unwrap(),
                        );
                    }
                    Some("tsconfig.json") => {
                        let parsed = parse_json(
                            &src,
                            JsonParserOptions::default()
                                .with_allow_comments()
                                .with_allow_trailing_commas(),
                        );
                        layout.insert_serialized_tsconfig(
                            path_buf.parent().unwrap().into(),
                            &parsed.syntax().as_send().unwrap(),
                        );
                    }
                    _ => unimplemented!("Unhandled manifest: {biome_path}"),
                }
            } else {
                let document_file_source = DocumentFileSource::from_path(&path_buf, false);
                match document_file_source {
                    DocumentFileSource::Js(_) => js_paths.push(biome_path),
                    DocumentFileSource::Css(_) => css_paths.push(biome_path),
                    DocumentFileSource::Html(_) => html_paths.push(biome_path),
                    _ => unimplemented!(
                        "Unhandled file type: {biome_path}. Add a new branch once the module graph understands new module types"
                    ),
                }
            }

            fs.insert(path_buf, src);
        }

        let db = WorkspaceDb::default();

        let js_added_paths = get_added_js_paths(&fs, &js_paths);
        for (path, root, semantic_model) in js_added_paths {
            let (module_info, _, _) = resolve_js_module(
                root,
                path,
                &fs,
                &layout,
                semantic_model,
                &path_info_cache,
                true,
            );
            let md = biome_module_graph::ModuleInfo::new(
                &db,
                path.as_path().to_path_buf(),
                ModuleInfoKind::Js(module_info),
            );
            db.insert_module(path.as_path().to_path_buf(), md);
        }

        let css_added_paths = get_css_added_paths(&fs, &css_paths);
        for (path, root) in css_added_paths {
            let (module_info, _, _) =
                resolve_css_module(root, path, &fs, &layout, &path_info_cache);
            let md = biome_module_graph::ModuleInfo::new(
                &db,
                path.as_path().to_path_buf(),
                ModuleInfoKind::Css(module_info),
            );
            db.insert_module(path.as_path().to_path_buf(), md);
        }

        let html_added_paths = get_html_added_paths(&fs, &html_paths);
        for (path, root, embedded_content) in html_added_paths {
            let (module_info, _, _) = resolve_html_module(
                root,
                &embedded_content,
                path,
                &fs,
                &layout,
                &path_info_cache,
            );
            let md = biome_module_graph::ModuleInfo::new(
                &db,
                path.as_path().to_path_buf(),
                ModuleInfoKind::Html(module_info),
            );
            db.insert_module(path.as_path().to_path_buf(), md);
        }

        Self {
            module_db: db,
            project_layout: Arc::new(layout),
        }
    }

    pub fn build_for_js_parse(
        &mut self,
        path: Utf8PathBuf,
        parse: biome_js_parser::Parse<biome_js_parser::AnyJsRoot>,
        file_source: JsFileSource,
    ) -> JsAnalyzerServices<'_> {
        let source_index = self
            .module_db
            .insert_source(DocumentFileSource::Js(file_source));
        let parsed_source = ParsedSource::new(
            &self.module_db,
            path.clone(),
            parse.into(),
            source_index,
            vec![],
        );
        self.module_db.insert_file(&path, parsed_source);

        JsAnalyzerServices::from((
            self.module_db.rc_module_db(),
            self.project_layout.clone(),
            file_source,
        ))
        .with_embedded_db(self.module_db.rc_embedded_db())
        .with_semantic_model(semantic_model_from_source(&self.module_db, parsed_source))
    }
}
