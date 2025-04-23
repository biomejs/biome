use biome_fs::MemoryFileSystem;
use biome_js_formatter::context::JsFormatOptions;
use biome_js_formatter::format_node;
use biome_js_parser::{JsParserOptions, parse};
use biome_js_syntax::JsFileSource;
use biome_module_graph::ModuleGraph;
use biome_rowan::AstNode;
use camino::Utf8PathBuf;

pub struct ModuleGraphSnapshot<'a> {
    module_graph: &'a ModuleGraph,
    fs: &'a MemoryFileSystem,
}

impl<'a> ModuleGraphSnapshot<'a> {
    pub fn new(module_graph: &'a ModuleGraph, fs: &'a MemoryFileSystem) -> Self {
        Self { module_graph, fs }
    }

    pub fn assert_snapshot(&self, test_name: &str) {
        let mut content = String::new();
        let files: Vec<_> = self
            .fs
            .files
            .read()
            .iter()
            .map(|(file, entry)| {
                let content = entry.lock();
                let content = std::str::from_utf8(content.as_slice()).unwrap();
                (file.as_str().to_string(), String::from(content))
            })
            .collect();

        let dependency_data = self.module_graph.data();
        for (file_name, source_code) in &files {
            let file_name = Utf8PathBuf::from(file_name.as_str());
            let source_type: JsFileSource = file_name.as_path().try_into().unwrap();
            let extension = file_name.extension().unwrap_or_default();
            let tree = parse(
                source_code.as_str(),
                source_type,
                JsParserOptions::default(),
            );
            let formatted = format_node(JsFormatOptions::default(), tree.tree().syntax())
                .unwrap()
                .print()
                .unwrap();

            content.push_str("## ");
            content.push_str(file_name.as_str());
            content.push_str("\n\n");
            content.push_str("```");
            content.push_str(extension);
            content.push('\n');
            content.push_str(formatted.as_code());
            content.push_str("\n```");

            let data = dependency_data.get(file_name.as_path()).unwrap().clone();

            content.push_str("\n\n");
            content.push_str("```\n");
            content.push_str(&data.to_string());
            content.push_str("\n```\n\n");
        }

        insta::with_settings!({
            snapshot_path => "../snapshots",
            prepend_module_to_snapshot => false,
        }, {
            insta::assert_snapshot!(test_name, content);
        });
    }
}
