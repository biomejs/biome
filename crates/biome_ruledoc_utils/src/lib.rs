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
use biome_test_utils::get_added_paths;
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

        let mut added_paths = Vec::with_capacity(files.len());

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
                added_paths.push(biome_path);
            }

            fs.insert(path_buf, src);
        }

        let module_graph = ModuleGraph::default();
        let added_paths = get_added_paths(&fs, &added_paths);
        module_graph.update_graph_for_js_paths(&fs, &layout, &added_paths, &[]);

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
