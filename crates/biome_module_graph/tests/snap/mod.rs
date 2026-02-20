use std::collections::BTreeSet;

use biome_fs::MemoryFileSystem;
use biome_js_formatter::context::JsFormatOptions;
use biome_js_formatter::format_node;
use biome_js_parser::{JsParserOptions, parse};
use biome_js_syntax::JsFileSource;
use biome_module_graph::{JsExport, JsOwnExport, ModuleGraph, ModuleInfo, ModuleResolver};
use biome_resolver::ResolvedPath;
use biome_rowan::AstNode;
use biome_test_utils::{dump_registered_module_types, dump_registered_types};
use camino::Utf8PathBuf;

/// Returns `true` for file extensions that are handled as JS/TS by the
/// snapshot helper. CSS and HTML files are handled separately.
fn is_js_like_extension(ext: &str) -> bool {
    matches!(
        ext,
        "js" | "jsx" | "ts" | "tsx" | "mjs" | "cjs" | "mts" | "cts" | "d.ts"
    )
}

pub struct ModuleGraphSnapshot<'a> {
    module_graph: &'a ModuleGraph,
    fs: &'a MemoryFileSystem,
    resolver: Option<&'a ModuleResolver>,
}

impl<'a> ModuleGraphSnapshot<'a> {
    pub fn new(module_graph: &'a ModuleGraph, fs: &'a MemoryFileSystem) -> Self {
        Self {
            module_graph,
            fs,
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

            if is_js_like_extension(extension) {
                let source_type: JsFileSource = file_name.as_path().try_into().unwrap();
                let tree = parse(
                    source_code.as_str(),
                    source_type,
                    JsParserOptions::default(),
                );
                let formatted =
                    format_node(JsFormatOptions::default(), tree.tree().syntax(), false)
                        .unwrap()
                        .print()
                        .unwrap();
                content.push_str(formatted.as_code().trim());
            } else {
                // For CSS/HTML just show the raw source trimmed.
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
                                let resolved = path
                                    .as_path()
                                    .map(|p| p.as_str().replace('\\', "/"))
                                    .unwrap_or_else(|| "<unresolved>".to_string());
                                content.push_str(&format!("\n  \"{specifier}\" => {resolved},"));
                            }
                            content.push_str("\n]");
                        }

                        // Show referenced CSS classes from JSX/HTML className attributes.
                        if !data.referenced_classes.is_empty() {
                            let mut classes: Vec<_> = data
                                .referenced_classes
                                .iter()
                                .map(|c| c.text().to_string())
                                .collect();
                            classes.sort();
                            content.push_str("\nReferenced classes: [");
                            for class in &classes {
                                content.push_str(&format!("\n  {class},"));
                            }
                            content.push_str("\n]");
                        }

                        content.push_str("\n```\n\n");

                        let exported_binding_ids: BTreeSet<_> = data
                            .exports
                            .values()
                            .filter_map(JsExport::as_own_export)
                            .filter_map(|export| match export {
                                JsOwnExport::Binding(binding_id) => Some(*binding_id),
                                JsOwnExport::Type(_) => None,
                            })
                            .collect();
                        if !exported_binding_ids.is_empty() {
                            content.push_str("## Exported Bindings\n\n");
                            content.push_str("```");
                            for binding_id in exported_binding_ids {
                                content.push_str(&format!(
                                    "\n{binding_id:?} => {}\n",
                                    data.binding(binding_id)
                                ));
                            }
                            content.push_str("```\n\n");
                        }

                        dump_registered_module_types(&mut content, &data.types());
                    }
                    ModuleInfo::Css(css_data) => {
                        content.push_str("```\n");
                        content.push_str("classes: [");
                        let mut classes: Vec<_> = css_data
                            .classes
                            .iter()
                            .map(|c| c.text().to_string())
                            .collect();
                        classes.sort();
                        if classes.is_empty() {
                            content.push(']');
                        } else {
                            content.push('\n');
                            for class in &classes {
                                content.push_str(&format!("  {class},\n"));
                            }
                            content.push(']');
                        }
                        content.push('\n');
                        content.push_str("imports: [");
                        let mut imports: Vec<_> = css_data
                            .imports
                            .values()
                            .map(|i| i.specifier.to_string())
                            .collect();
                        imports.sort();
                        if imports.is_empty() {
                            content.push(']');
                        } else {
                            content.push('\n');
                            for import in &imports {
                                content.push_str(&format!("  {import},\n"));
                            }
                            content.push(']');
                        }
                        content.push('\n');
                        content.push_str("```\n\n");
                    }
                    ModuleInfo::Html(html_data) => {
                        content.push_str("```\n");
                        content.push_str("style_classes: [");
                        let mut style_classes: Vec<_> = html_data
                            .style_classes
                            .iter()
                            .map(|c| c.text().to_string())
                            .collect();
                        style_classes.sort();
                        if style_classes.is_empty() {
                            content.push(']');
                        } else {
                            content.push('\n');
                            for class in &style_classes {
                                content.push_str(&format!("  {class},\n"));
                            }
                            content.push(']');
                        }
                        content.push('\n');
                        content.push_str("referenced_classes: [");
                        let mut ref_classes: Vec<_> = html_data
                            .referenced_classes
                            .iter()
                            .map(|c| c.text().to_string())
                            .collect();
                        ref_classes.sort();
                        if ref_classes.is_empty() {
                            content.push(']');
                        } else {
                            content.push('\n');
                            for class in &ref_classes {
                                content.push_str(&format!("  {class},\n"));
                            }
                            content.push(']');
                        }
                        content.push('\n');
                        content.push_str("```\n\n");
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
