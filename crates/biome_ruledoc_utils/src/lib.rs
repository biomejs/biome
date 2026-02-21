mod codeblock;

use std::collections::HashMap;
use std::hash::BuildHasher;
use std::sync::Arc;

use biome_fs::{BiomePath, MemoryFileSystem};
use biome_js_analyze::JsAnalyzerServices;
use biome_js_parser::JsFileSource;
use biome_json_parser::{JsonParserOptions, parse_json};
use biome_module_graph::ModuleGraph;
use biome_project_layout::ProjectLayout;
use biome_service::workspace::DocumentFileSource;
use biome_test_utils::{get_added_js_paths, get_css_added_paths};
use camino::Utf8PathBuf;

pub use codeblock::*;

/// Builder that can be used for constructing analyzer services.
///
/// The builder can be reused to create cheap instances of analyzer services
/// for multiple code blocks.
pub struct AnalyzerServicesBuilder {
    module_graph: Arc<ModuleGraph>,
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
            return Self {
                module_graph: Default::default(),
                project_layout: Default::default(),
            };
        }

        let fs = MemoryFileSystem::default();
        let layout = ProjectLayout::default();

        let mut js_paths = Vec::new();
        let mut css_paths = Vec::new();

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
            } else if DocumentFileSource::from_well_known(&path_buf, false).is_javascript_like() {
                js_paths.push(biome_path);
            } else if DocumentFileSource::from_well_known(&path_buf, false).is_css_like() {
                css_paths.push(biome_path);
            }
            // Note: HTML files are not yet supported in rustdoc tests because they
            // require embedded snippet parsing which isn't available in this context.

            fs.insert(path_buf, src);
        }

        let module_graph = ModuleGraph::default();

        // Populate JS files
        let js_added_paths = get_added_js_paths(&fs, &js_paths);
        module_graph.update_graph_for_js_paths(&fs, &layout, &js_added_paths, true);

        // Populate CSS files
        let css_added_paths = get_css_added_paths(&fs, &css_paths);
        module_graph.update_graph_for_css_paths(&fs, &layout, &css_added_paths, None);

        Self {
            module_graph: Arc::new(module_graph),
            project_layout: Arc::new(layout),
        }
    }

    pub fn build_for_js_file_source(&self, file_source: JsFileSource) -> JsAnalyzerServices {
        JsAnalyzerServices::from((
            self.module_graph.clone(),
            self.project_layout.clone(),
            file_source,
        ))
    }
}
