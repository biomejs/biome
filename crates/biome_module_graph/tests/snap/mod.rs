use biome_css_formatter::context::CssFormatOptions;
use biome_css_parser::CssParserOptions;
use biome_css_syntax::CssFileSource;
use biome_fs::MemoryFileSystem;
use biome_html_formatter::HtmlFormatOptions;
use biome_html_parser::HtmlParserOptions;
use biome_html_syntax::HtmlFileSource;
use biome_js_formatter::context::JsFormatOptions;
use biome_js_formatter::format_node;
use biome_js_parser::{JsParserOptions, parse};
use biome_js_syntax::JsFileSource;
use biome_module_graph::{JsExport, JsOwnExport, ModuleGraph, ModuleInfo, ModuleResolver};
use biome_resolver::ResolvedPath;
use biome_rowan::AstNode;
use biome_test_utils::{dump_registered_module_types, dump_registered_types};
use camino::Utf8PathBuf;
use std::collections::BTreeSet;

pub struct ModuleGraphSnapshot<'a> {
    module_graph: &'a ModuleGraph,
    files: Vec<(String, String)>,
    resolver: Option<&'a ModuleResolver>,
}

impl<'a> ModuleGraphSnapshot<'a> {
    pub fn new(module_graph: &'a ModuleGraph, fs: &'a MemoryFileSystem) -> Self {
        let files = fs
            .files
            .read()
            .iter()
            .map(|(file, entry)| {
                let content = entry.lock();
                let content = String::from_utf8_lossy(content.as_slice()).into_owned();
                (file.as_str().to_string(), content)
            })
            .collect();
        Self {
            module_graph,
            files,
            resolver: None,
        }
    }

    /// Build a snapshot from a pre-collected list of `(path, source)` pairs.
    ///
    /// Use this when the [`MemoryFileSystem`] has been moved into a
    /// [`WorkspaceServer`] and is no longer directly accessible.
    pub fn from_files(module_graph: &'a ModuleGraph, files: Vec<(String, String)>) -> Self {
        Self {
            module_graph,
            files,
            resolver: None,
        }
    }

    pub fn with_resolver(self, resolver: &'a ModuleResolver) -> Self {
        Self {
            resolver: Some(resolver),
            ..self
        }
    }

    pub fn assert_snapshot(&self, test_name: &str) {
        let mut content = String::new();
        let files: Vec<_> = self.files.clone();

        let dependency_data = self.module_graph.data();
        for (file_name, source_code) in &files {
            let file_name = Utf8PathBuf::from(file_name.as_str());
            let extension = file_name.extension().unwrap_or_default();

            content.push_str("\n# `");
            content.push_str(file_name.as_str());
            content.push('`');
            if let Some(resolver) = self.resolver {
                content.push_str(" (");
                match resolver
                    .modules_by_path
                    .get(&ResolvedPath::from_path(&file_name))
                {
                    Some(module_id) => {
                        content.push_str("Module ");
                        content.push_str(&module_id.index().to_string());
                    }
                    None => content.push_str("Not imported by resolver"),
                }
                content.push(')');
            }
            content.push_str("\n\n## Source\n\n");
            content.push_str("```");
            content.push_str(extension);
            content.push('\n');

            // Check HtmlFileSource first: .vue/.astro/.svelte are also matched
            // by JsFileSource (legacy embedding path), so we must prioritise the
            // HTML branch to avoid feeding raw SFC source through the JS parser.
            if let Ok(file_source) = HtmlFileSource::try_from(file_name.as_path()) {
                if file_source.is_html() {
                    // Format plain .html files with the HTML formatter.
                    let tree = biome_html_parser::parse_html(
                        source_code.as_str(),
                        HtmlParserOptions::from(&file_source),
                    );
                    let formatted = biome_html_formatter::format_node(
                        HtmlFormatOptions::default(),
                        tree.tree().syntax(),
                        false,
                    )
                    .unwrap()
                    .print()
                    .unwrap();
                    content.push_str(formatted.as_code().trim());
                } else {
                    // Framework files (.vue, .astro, .svelte): display raw source.
                    // The HTML formatter cannot handle their embedded blocks, and the
                    // JS parser would mangle them (closing tags → bogus nodes).
                    content.push_str(source_code.trim());
                }
            } else if let Ok(file_source) = CssFileSource::try_from(file_name.as_path()) {
                let tree = biome_css_parser::parse_css(
                    source_code.as_str(),
                    file_source,
                    CssParserOptions::default(),
                );
                let formatted = biome_css_formatter::format_node(
                    CssFormatOptions::default(),
                    tree.tree().syntax(),
                )
                .unwrap()
                .print()
                .unwrap();
                content.push_str(formatted.as_code().trim());
            } else if let Ok(file_source) = JsFileSource::try_from(file_name.as_path()) {
                let tree = parse(
                    source_code.as_str(),
                    file_source,
                    JsParserOptions::default(),
                );
                let formatted =
                    format_node(JsFormatOptions::default(), tree.tree().syntax(), false)
                        .unwrap()
                        .print()
                        .unwrap();
                content.push_str(formatted.as_code().trim());
            } else {
                content.push_str(source_code.trim());
            }

            content.push_str("\n```");

            if let Some(data) = dependency_data.get(file_name.as_path()) {
                content.push_str("\n\n## Module Info\n\n");
                match data {
                    ModuleInfo::Js(data) => {
                        content.push_str("```\n");
                        content.push_str(&data.to_string());

                        // Show side-effect import paths (e.g. `import "./styles.css"`)
                        // that are not captured in the named `static_imports` map and
                        // are not re-exports.
                        //
                        // We detect true side-effect imports by excluding any specifier
                        // that is referenced by a named import, a blanket re-export, or
                        // a named re-export in the exports map.
                        let reexport_specifiers: std::collections::HashSet<_> = data
                            .blanket_reexports
                            .iter()
                            .map(|r| r.import.specifier.text().to_string())
                            .chain(data.exports.values().filter_map(|e| match e {
                                JsExport::Reexport(r) => {
                                    Some(r.import.specifier.text().to_string())
                                }
                                JsExport::ReexportType(r) => {
                                    Some(r.import.specifier.text().to_string())
                                }
                                _ => None,
                            }))
                            .collect();

                        let named_import_specifiers: std::collections::HashSet<_> = data
                            .static_imports
                            .values()
                            .map(|imp| imp.specifier.text().to_string())
                            .collect();

                        let side_effect_paths: Vec<_> = data
                            .static_import_paths
                            .iter()
                            .filter(|(specifier, _)| {
                                let s = specifier.text().to_string();
                                !named_import_specifiers.contains(&s)
                                    && !reexport_specifiers.contains(&s)
                            })
                            .collect();
                        if !side_effect_paths.is_empty() {
                            content.push_str("\nSide-effect imports: [");
                            for (specifier, path) in &side_effect_paths {
                                let resolved =
                                    path.as_path().map_or("<unresolved>".to_string(), |p| {
                                        p.as_str().replace('\\', "/")
                                    });
                                content.push_str(&format!("\n  \"{specifier}\" => {resolved},"));
                            }
                            content.push_str("\n]");
                        }

                        // Show referenced CSS classes from JSX/HTML className attributes.
                        if !data.referenced_classes.is_empty() {
                            let mut classes: Vec<_> = data
                                .referenced_classes
                                .iter()
                                .flat_map(|r| {
                                    r.token
                                        .text()
                                        .split_ascii_whitespace()
                                        .map(|s| s.to_string())
                                })
                                .collect();
                            classes.sort();
                            content.push_str("\nReferenced classes: [");
                            for class in &classes {
                                content.push_str(&format!("\n  {class},"));
                            }
                            content.push_str("\n]");
                        }

                        content.push_str("\n```\n\n");

                        let exported_binding_ranges: BTreeSet<_> = data
                            .exports
                            .values()
                            .filter_map(JsExport::as_own_export)
                            .filter_map(|export| match export {
                                JsOwnExport::Binding(binding_range) => Some(*binding_range),
                                JsOwnExport::Type(_) | JsOwnExport::Namespace(_) => None,
                            })
                            .collect();
                        if !exported_binding_ranges.is_empty() {
                            content.push_str("## Exported Bindings\n\n");
                            content.push_str("```");
                            for binding_range in exported_binding_ranges {
                                if let Some(type_data) = data.binding_type_data(binding_range) {
                                    // Get the binding name from the semantic model
                                    let binding_name = data
                                        .semantic_model
                                        .all_bindings()
                                        .find(|b| b.syntax().text_trimmed_range() == binding_range)
                                        .and_then(|b| b.tree().name_token().ok())
                                        .map_or_else(|| "<unknown>".to_string(), |b| b.to_string());

                                    content.push_str(&format!(
                                        "\n{} => {}\n",
                                        binding_name, type_data
                                    ));
                                }
                            }
                            content.push_str("```\n\n");
                        }

                        dump_registered_module_types(&mut content, &data.types());
                    }
                    ModuleInfo::Css(css_data) => {
                        content.push_str("```\n");
                        content.push_str(&css_data.to_string());
                        content.push_str("\n```\n\n");
                    }
                    ModuleInfo::Html(html_data) => {
                        content.push_str("```\n");
                        content.push_str(&html_data.to_string());
                        content.push_str("\n```\n\n");
                    }
                }
            }
        }

        if let Some(resolver) = self.resolver {
            content.push_str("\n# Module Resolver\n\n");
            dump_registered_types(&mut content, resolver);
        }

        insta::with_settings!({
            snapshot_path => "../snapshots",
            prepend_module_to_snapshot => false,
        }, {
            insta::assert_snapshot!(test_name, content);
        });
    }
}
