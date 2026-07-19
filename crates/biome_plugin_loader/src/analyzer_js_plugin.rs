use std::fmt::{Debug, Formatter};
use std::ops::DerefMut;
use std::sync::Arc;

use boa_engine::object::builtins::JsFunction;
use boa_engine::{JsNativeError, JsResult, JsString, JsValue};
use camino::{Utf8Path, Utf8PathBuf};

use biome_analyze::{
    AnalyzerPlugin, PluginDiagnosticEntry, PluginEvalResult, PluginTargetLanguage, RuleDiagnostic,
};
use biome_console::markup;
use biome_diagnostics::category;
use biome_glob::NormalizedGlob;
use biome_js_runtime::JsExecContext;
use biome_js_syntax::{AnyJsRoot, JsSyntaxNode};
use biome_resolver::FsWithResolverProxy;
use biome_rowan::{AnySyntaxNode, AstNode, RawSyntaxKind, SyntaxKind};
use biome_text_size::TextRange;

use crate::PluginDiagnostic;
use crate::file_matches_includes;
use crate::thread_local::ThreadLocalCell;

/// Already loaded plugin in a thread.
/// These values can't be shared with another threads.
struct LoadedPlugin {
    ctx: JsExecContext,
    entrypoint: JsFunction,
}

fn load_plugin(fs: Arc<dyn FsWithResolverProxy>, path: &Utf8Path) -> JsResult<LoadedPlugin> {
    let mut ctx = JsExecContext::new(fs)?;
    let module = ctx.import_module(path)?;
    let entrypoint = ctx.get_default_export(&module)?;

    let Some(entrypoint) = entrypoint.as_function() else {
        return Err(JsNativeError::typ()
            .with_message("The plugin entrypoint must be a function")
            .into());
    };

    Ok(LoadedPlugin { ctx, entrypoint })
}

/// A JS analyzer plugin.
/// As the JS engine is intended to run in single thread, plugins are lazily loaded in each thread
/// just before executing it.
pub struct AnalyzerJsPlugin {
    fs: Arc<dyn FsWithResolverProxy>,
    path: Utf8PathBuf,
    loaded: ThreadLocalCell<LoadedPlugin>,

    /// Glob patterns that restrict which files this plugin runs on.
    /// `None` means the plugin runs on all files.
    /// `Some(&[])` (an empty list) means the plugin never runs on any file.
    includes: Option<Box<[NormalizedGlob]>>,
}

impl Debug for AnalyzerJsPlugin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnalyzerJsPlugin")
            .field("path", &self.path)
            .finish_non_exhaustive()
    }
}

impl AnalyzerJsPlugin {
    pub fn load(
        fs: Arc<dyn FsWithResolverProxy>,
        path: &Utf8Path,
        includes: Option<&[NormalizedGlob]>,
    ) -> Result<Self, PluginDiagnostic> {
        // Load the plugin in the main thread here to catch errors while loading.
        load_plugin(fs.clone(), path)?;

        Ok(Self {
            fs,
            path: path.to_owned(),
            loaded: ThreadLocalCell::new(),
            includes: includes.map(Into::into),
        })
    }
}

impl AnalyzerPlugin for AnalyzerJsPlugin {
    fn name(&self) -> &str {
        // JS plugins don't declare a name; fall back to the plugin file stem.
        self.path.file_stem().unwrap_or("anonymous")
    }

    fn language(&self) -> PluginTargetLanguage {
        PluginTargetLanguage::JavaScript
    }

    fn applies_to_file(&self, path: &Utf8Path) -> bool {
        file_matches_includes(self.includes.as_deref(), path)
    }

    fn query(&self) -> Vec<RawSyntaxKind> {
        // TODO: Support granular query defined in the JS plugin.
        AnyJsRoot::KIND_SET
            .iter()
            .map(|kind| kind.to_raw())
            .collect()
    }

    fn evaluate(&self, node: AnySyntaxNode, path: Utf8PathBuf) -> PluginEvalResult {
        let mut plugin = match self
            .loaded
            .get_mut_or_try_init(|| load_plugin(self.fs.clone(), &self.path))
        {
            Ok(plugin) => plugin,
            Err(err) => {
                return PluginEvalResult {
                    entries: vec![PluginDiagnosticEntry {
                        diagnostic: RuleDiagnostic::new(
                            category!("plugin"),
                            None::<TextRange>,
                            markup!("Could not load the plugin: "<Error>{err.to_string()}</Error>),
                        ),
                        action: None,
                    }],
                };
            }
        };

        let plugin = plugin.deref_mut();

        let Some(node) = node.downcast_ref::<JsSyntaxNode>().cloned() else {
            return PluginEvalResult {
                entries: vec![PluginDiagnosticEntry {
                    diagnostic: RuleDiagnostic::new(
                        category!("plugin"),
                        None::<TextRange>,
                        markup!("Could not pass the AST to the plugin"),
                    ),
                    action: None,
                }],
            };
        };

        let ast = plugin.ctx.create_js_ast(node);
        let diagnostics = plugin
            .ctx
            .call_function(
                &plugin.entrypoint,
                &JsValue::undefined(),
                &[JsValue::from(JsString::from(path.as_str())), ast],
            )
            .map_or_else(
                |err| {
                    vec![RuleDiagnostic::new(
                        category!("plugin"),
                        None::<TextRange>,
                        markup!("Plugin errored: "<Error>{err.to_string()}</Error>),
                    )]
                },
                |_| plugin.ctx.pull_diagnostics(),
            );

        let entries = diagnostics
            .into_iter()
            .map(|diagnostic| PluginDiagnosticEntry {
                diagnostic,
                action: None,
            })
            .collect();

        PluginEvalResult { entries }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_diagnostics::{Error, print_diagnostic_to_string};
    use biome_fs::MemoryFileSystem;
    use biome_js_parser::JsParserOptions;
    use biome_languages::JsFileSource;

    fn snap_diagnostics(test_name: &str, diagnostics: Vec<Error>) {
        let content = diagnostics
            .iter()
            .map(print_diagnostic_to_string)
            .collect::<String>();

        // Normalize Windows paths...
        let content = content.replace('\\', "/");

        insta::with_settings!({
            prepend_module_to_snapshot => false,
        }, {
            insta::assert_snapshot!(test_name, content);
        });
    }

    fn load_test_plugin_from_source(
        source: &str,
        includes: Option<&[NormalizedGlob]>,
    ) -> AnalyzerJsPlugin {
        let fs = MemoryFileSystem::default();
        fs.insert("/plugin.js".into(), source);
        let fs = Arc::new(fs) as Arc<dyn FsWithResolverProxy>;
        AnalyzerJsPlugin::load(fs, "/plugin.js".into(), includes).unwrap()
    }

    fn load_test_plugin(includes: Option<&[NormalizedGlob]>) -> AnalyzerJsPlugin {
        load_test_plugin_from_source(
            r#"import { registerDiagnostic } from "@biomejs/plugin-api";
            export default function useMyPlugin(_path, root) {
                registerDiagnostic(root, "information", "Hello, world!");
            }"#,
            includes,
        )
    }

    #[test]
    fn name_is_derived_from_the_plugin_file() {
        let plugin = load_test_plugin(None);
        assert_eq!(plugin.name(), "plugin");
    }

    #[test]
    fn applies_to_all_files_without_includes() {
        let plugin = load_test_plugin(None);
        assert!(plugin.applies_to_file(Utf8Path::new("src/main.ts")));
        assert!(plugin.applies_to_file(Utf8Path::new("test/foo.js")));
    }

    #[test]
    fn applies_to_matching_files_with_includes() {
        let globs: Vec<NormalizedGlob> = vec!["src/**/*.ts".parse().unwrap()];
        let plugin = load_test_plugin(Some(&globs));
        assert!(plugin.applies_to_file(Utf8Path::new("src/main.ts")));
        assert!(plugin.applies_to_file(Utf8Path::new("src/nested/file.ts")));
    }

    #[test]
    fn rejects_non_matching_files_with_includes() {
        let globs: Vec<NormalizedGlob> = vec!["src/**/*.ts".parse().unwrap()];
        let plugin = load_test_plugin(Some(&globs));
        assert!(!plugin.applies_to_file(Utf8Path::new("test/foo.ts")));
        assert!(!plugin.applies_to_file(Utf8Path::new("src/main.js")));
    }

    #[test]
    fn passes_ast_as_the_second_argument() {
        let plugin = load_test_plugin_from_source(
            r#"import { registerDiagnostic } from "@biomejs/plugin-api";
            export default function useMyPlugin(path, root) {
                const descriptor = Object.getOwnPropertyDescriptor(
                    Object.getPrototypeOf(root),
                    "items",
                );
                const hasChildNodes = "childNodes" in root;
                registerDiagnostic(
                    root,
                    "information",
                    `${path}|${root.kind}|${typeof descriptor.get}|${Object.prototype.hasOwnProperty.call(root, "items")}|${hasChildNodes}`,
                );
            }"#,
            None,
        );
        let parse = biome_js_parser::parse(
            "let foo;",
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );

        let result = plugin.evaluate(parse.syntax().into(), "/file.js".into());

        snap_diagnostics(
            "passes_ast_as_the_second_argument",
            result
                .entries
                .into_iter()
                .map(|entry| entry.diagnostic.into())
                .collect(),
        );
    }

    #[test]
    fn reports_top_level_var_declarations_using_ast_fields() {
        let source = r#"import { registerDiagnostic } from "@biomejs/plugin-api";
            export default function noTopLevelVar(_path, root) {
                const statements = root.kind === "JS_MODULE" ? root.items : [];
                for (const statement of statements) {
                    if (
                        statement.kind === "JS_VARIABLE_STATEMENT" &&
                        statement.declaration?.kindToken === "var"
                    ) {
                        registerDiagnostic(
                            statement,
                            "warning",
                            "Use let or const instead of a top-level var declaration.",
                        );
                    }
                }
            }"#;
        let parse = biome_js_parser::parse(
            "var legacy = 1; const modern = 2;",
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );

        let plugin = load_test_plugin_from_source(source, None);
        let result = plugin.evaluate(parse.syntax().into(), "/file.js".into());

        snap_diagnostics(
            "reports_top_level_var_declarations_using_ast_fields",
            result
                .entries
                .into_iter()
                .map(|entry| entry.diagnostic.into())
                .collect(),
        );
    }

    #[test]
    fn evaluate_in_worker_threads() {
        let fs = MemoryFileSystem::default();
        fs.insert("/foo.js".into(), "let foo;");
        fs.insert("/bar.js".into(), "let bar;");
        fs.insert(
            "/plugin.js".into(),
            r#"import { registerDiagnostic } from "@biomejs/plugin-api";
            export default function useMyPlugin(_path, root) {
                registerDiagnostic(root, "information", "Hello, world!");
            }"#,
        );

        let fs = Arc::new(fs) as Arc<dyn FsWithResolverProxy>;
        let plugin =
            Arc::new(AnalyzerJsPlugin::load(fs.clone(), "/plugin.js".into(), None).unwrap());

        let worker1 = {
            let plugin = plugin.clone();

            std::thread::spawn(move || {
                let parse = biome_js_parser::parse(
                    "let foo;",
                    JsFileSource::js_module(),
                    JsParserOptions::default(),
                );

                plugin.evaluate(parse.syntax().into(), "/foo.js".into())
            })
        };

        let worker2 = {
            let plugin = plugin.clone();

            std::thread::spawn(move || {
                let parse = biome_js_parser::parse(
                    "let bar;",
                    JsFileSource::js_module(),
                    JsParserOptions::default(),
                );

                plugin.evaluate(parse.syntax().into(), "/bar.js".into())
            })
        };

        let result1 = worker1.join().unwrap();
        let result2 = worker2.join().unwrap();
        let mut diagnostics: Vec<_> = result1.entries.into_iter().map(|e| e.diagnostic).collect();
        diagnostics.extend(result2.entries.into_iter().map(|e| e.diagnostic));

        assert_eq!(diagnostics.len(), 2);
        snap_diagnostics(
            "evaluate_in_worker_threads",
            diagnostics.into_iter().map(|diag| diag.into()).collect(),
        );
    }
}
