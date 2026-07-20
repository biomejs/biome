use biome_css_formatter::context::CssFormatOptions;
use biome_css_parser::CssParserOptions;
use biome_fs::MemoryFileSystem;
use biome_html_formatter::HtmlFormatOptions;
use biome_html_parser::HtmlParserOptions;
use biome_js_formatter::context::JsFormatOptions;
use biome_js_formatter::format_node;
use biome_js_parser::{JsParserOptions, parse};
use biome_languages::{CssFileSource, HtmlFileSource, JsFileSource};
use biome_module_graph::{JsExport, ModuleDb, ModuleInfoKind};
use biome_rowan::AstNode;
use camino::Utf8PathBuf;
use std::collections::BTreeMap;

pub struct ModuleGraphSnapshot<'a> {
    module_db: &'a dyn ModuleDb,
    files: BTreeMap<String, String>,
}

impl<'a> ModuleGraphSnapshot<'a> {
    pub fn new(module_db: &'a dyn ModuleDb, fs: &'a MemoryFileSystem) -> Self {
        let files = source_files_from_memory_fs(fs);
        Self { module_db, files }
    }

    /// Build a snapshot from a pre-collected list of `(path, source)` pairs.
    ///
    /// Use this when the [`MemoryFileSystem`] has been moved into a
    /// [`WorkspaceServer`] and is no longer directly accessible.
    pub fn from_files(module_db: &'a dyn ModuleDb, files: BTreeMap<String, String>) -> Self {
        Self { module_db, files }
    }

    pub fn assert_snapshot(&self, test_name: &str) {
        let mut content = String::new();
        let files = self.files.clone();
        for (file_name, source_code) in &files {
            let file_name = Utf8PathBuf::from(file_name.as_str());
            write_source_file(&mut content, &file_name, source_code);

            if let Some(data) = self.module_db.module_info_for_path(file_name.as_path()) {
                content.push_str("\n\n## Module Info\n\n");
                match data {
                    ModuleInfoKind::Js(data) => {
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
                    }
                    ModuleInfoKind::Css(css_data) => {
                        content.push_str("```\n");
                        content.push_str(&css_data.to_string());
                        content.push_str("\n```\n\n");
                    }
                    ModuleInfoKind::Html(html_data) => {
                        content.push_str("```\n");
                        content.push_str(&html_data.to_string());
                        content.push_str("\n```\n\n");
                    }
                }
            }
        }

        insta::with_settings!({
            snapshot_path => "../snapshots",
            prepend_module_to_snapshot => false,
        }, {
            insta::assert_snapshot!(test_name, content);
        });
    }
}

pub fn source_files_from_memory_fs(fs: &MemoryFileSystem) -> BTreeMap<String, String> {
    fs.files
        .read()
        .iter()
        .map(|(file, entry)| {
            let content = entry.lock();
            let content = String::from_utf8_lossy(content.as_slice()).into_owned();
            (file.as_str().to_string(), content)
        })
        .collect()
}

pub fn write_source_file(content: &mut String, file_name: &Utf8PathBuf, source_code: &str) {
    let extension = file_name.extension().unwrap_or_default();

    content.push_str("\n# `");
    content.push_str(file_name.as_str());
    content.push('`');
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
            let tree =
                biome_html_parser::parse_html(source_code, HtmlParserOptions::from(&file_source));
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
        let tree =
            biome_css_parser::parse_css(source_code, file_source, CssParserOptions::default());
        let formatted =
            biome_css_formatter::format_node(CssFormatOptions::default(), tree.tree().syntax())
                .unwrap()
                .print()
                .unwrap();
        content.push_str(formatted.as_code().trim());
    } else if let Ok(file_source) = JsFileSource::try_from(file_name.as_path()) {
        let tree = parse(source_code, file_source, JsParserOptions::default());
        let formatted = format_node(JsFormatOptions::default(), tree.tree().syntax(), Vec::new())
            .unwrap()
            .print()
            .unwrap();
        content.push_str(formatted.as_code().trim());
    } else {
        content.push_str(source_code.trim());
    }

    content.push_str("\n```");
}
