use std::fmt::{Debug, Formatter};
use std::ops::DerefMut;
use std::sync::Arc;

use boa_engine::{JsNativeError, JsResult, JsValue};
use camino::{Utf8Path, Utf8PathBuf};

use biome_analyze::{
    AnalyzerPlugin, PluginDiagnosticEntry, PluginEvalResult, PluginTargetLanguage, RuleDiagnostic,
};
use biome_console::markup;
use biome_diagnostics::category;
use biome_glob::NormalizedGlob;
use biome_js_runtime::{JsExecContext, JsPluginRule};
use biome_js_syntax::JsSyntaxNode;
use biome_resolver::FsWithResolverProxy;
use biome_rowan::{AnySyntaxNode, RawSyntaxKind, SyntaxKind};
use biome_text_size::TextRange;

use crate::PluginDiagnostic;
use crate::file_matches_includes;
use crate::thread_local::ThreadLocalCell;

/// Already loaded plugin in a thread.
/// These values can't be shared with another threads.
struct LoadedPlugin {
    ctx: JsExecContext,
    rules: Vec<JsPluginRule>,
}

fn load_plugin(fs: Arc<dyn FsWithResolverProxy>, path: &Utf8Path) -> JsResult<LoadedPlugin> {
    let mut ctx = JsExecContext::new(fs)?;
    let module = ctx.import_module(path)?;
    let rules = ctx.load_rules(&module)?;

    if rules.is_empty() {
        return Err(JsNativeError::typ()
            .with_message("The plugin must export at least one rule created with defineRule()")
            .into());
    }

    Ok(LoadedPlugin { ctx, rules })
}

/// A JS analyzer plugin.
/// As the JS engine is intended to run in single thread, plugins are lazily loaded in each thread
/// just before executing it.
pub struct AnalyzerJsPlugin {
    fs: Arc<dyn FsWithResolverProxy>,
    path: Utf8PathBuf,
    loaded: ThreadLocalCell<LoadedPlugin>,

    /// The union of the syntax kinds queried by the rules of the plugin.
    /// Extracted once at load time, since `query()` can be called from threads
    /// that haven't loaded the plugin yet.
    kinds: Vec<RawSyntaxKind>,

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
        // Load the plugin in the main thread here to catch errors while loading,
        // and to extract the queried kinds.
        let plugin = load_plugin(fs.clone(), path)?;

        let mut kinds: Vec<RawSyntaxKind> = plugin
            .rules
            .iter()
            .flat_map(|rule| &rule.kinds)
            .map(|kind| kind.to_raw())
            .collect();
        kinds.sort_unstable_by_key(|kind| kind.0);
        kinds.dedup();

        Ok(Self {
            fs,
            path: path.to_owned(),
            loaded: ThreadLocalCell::new(),
            kinds,
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
        self.kinds.clone()
    }

    fn evaluate(&self, node: AnySyntaxNode, _path: Utf8PathBuf) -> PluginEvalResult {
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

        let LoadedPlugin { ctx, rules } = plugin.deref_mut();

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

        let kind = node.kind();
        let ast = ctx.create_js_ast(node);
        let mut entries = Vec::new();

        for rule in rules.iter().filter(|rule| rule.kinds.contains(&kind)) {
            let result =
                ctx.call_function(&rule.run, &JsValue::undefined(), std::slice::from_ref(&ast));

            // Drain the diagnostics even on errors, so a failed rule can't leak
            // its diagnostics into the next one.
            let mut diagnostics = ctx.pull_diagnostics();

            if let Err(err) = result {
                diagnostics.push(RuleDiagnostic::new(
                    category!("plugin"),
                    None::<TextRange>,
                    markup!("Rule "<Emphasis>{rule.name}</Emphasis>" errored: "<Error>{err.to_string()}</Error>),
                ));
            }

            entries.extend(
                diagnostics
                    .into_iter()
                    .map(|diagnostic| PluginDiagnosticEntry {
                        diagnostic: diagnostic.subcategory(rule.name.clone()),
                        action: None,
                    }),
            );
        }

        PluginEvalResult { entries }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_diagnostics::{DiagnosticExt, Error, PrintDescription, print_diagnostic_to_string};
    use biome_fs::MemoryFileSystem;
    use biome_js_parser::JsParserOptions;
    use biome_js_syntax::JsSyntaxKind;
    use biome_languages::JsFileSource;

    /// Renders the diagnostics of a single evaluation the same way the CLI does, by attaching the
    /// path and the content of the analyzed file so the code frame can be printed.
    fn render_diagnostics(path: &str, source: &str, result: PluginEvalResult) -> String {
        result
            .entries
            .into_iter()
            .map(|entry| {
                print_diagnostic_to_string(
                    &Error::from(entry.diagnostic)
                        .with_file_path(path)
                        .with_file_source_code(source.to_string()),
                )
            })
            .collect()
    }

    fn snap_diagnostics(test_name: &str, content: String) {
        // Normalize Windows paths...
        let content = content.replace('\\', "/");

        insta::with_settings!({
            prepend_module_to_snapshot => false,
        }, {
            insta::assert_snapshot!(test_name, content);
        });
    }

    fn load_test_plugin_from_source(
        path: &str,
        source: &str,
        includes: Option<&[NormalizedGlob]>,
    ) -> AnalyzerJsPlugin {
        let fs = MemoryFileSystem::default();
        fs.insert(path.into(), source);
        let fs = Arc::new(fs) as Arc<dyn FsWithResolverProxy>;
        AnalyzerJsPlugin::load(fs, path.into(), includes).unwrap()
    }

    fn load_test_plugin(includes: Option<&[NormalizedGlob]>) -> AnalyzerJsPlugin {
        load_test_plugin_from_source(
            "/plugin.js",
            r#"import { ast, defineRule, registerDiagnostic } from "@biomejs/plugin-api";
            export const useMyPlugin = defineRule({
                query: ast("JS_MODULE"),
                run(root) {
                    registerDiagnostic(root, "information", "Hello, world!");
                },
            });"#,
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

    /// The AST is exposed through lazy getters installed on the prototype of each kind, so the
    /// fields are only cast when the plugin accesses them.
    #[test]
    fn passes_the_matched_node_to_run() {
        let plugin = load_test_plugin_from_source(
            "/plugin.js",
            r#"import { ast, defineRule, registerDiagnostic } from "@biomejs/plugin-api";
            export const useMyPlugin = defineRule({
                query: ast("JS_MODULE"),
                run(root) {
                    const descriptor = Object.getOwnPropertyDescriptor(
                        Object.getPrototypeOf(root),
                        "items",
                    );
                    const hasChildNodes = "childNodes" in root;
                    registerDiagnostic(
                        root,
                        "information",
                        `${root.kind}|${typeof descriptor.get}|${Object.prototype.hasOwnProperty.call(root, "items")}|${hasChildNodes}`,
                    );
                },
            });"#,
            None,
        );
        let parse = biome_js_parser::parse(
            "let foo;",
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );

        let result = plugin.evaluate(parse.syntax().into(), "/file.js".into());

        let [entry] = result.entries.as_slice() else {
            panic!("expected a single diagnostic, got {result:?}");
        };

        assert_eq!(
            PrintDescription(&entry.diagnostic).to_string(),
            // kind | the `items` field is a getter | it isn't an own property | unknown fields
            // aren't exposed
            "JS_MODULE|function|false|false"
        );
    }

    #[test]
    fn queries_the_kinds_declared_by_the_rules() {
        let plugin = load_test_plugin_from_source(
            "/plugin.js",
            r#"import { ast, defineRule } from "@biomejs/plugin-api";
            export const rule1 = defineRule({
                query: ast("JS_VARIABLE_STATEMENT", "JS_CALL_EXPRESSION"),
                run(node) {},
            });
            export const rule2 = defineRule({
                query: ast("JS_CALL_EXPRESSION"),
                run(node) {},
            });
            export const notARule = 42;"#,
            None,
        );

        let mut expected = vec![
            JsSyntaxKind::JS_VARIABLE_STATEMENT.to_raw(),
            JsSyntaxKind::JS_CALL_EXPRESSION.to_raw(),
        ];
        expected.sort_unstable_by_key(|kind| kind.0);

        assert_eq!(plugin.query(), expected);
    }

    #[test]
    fn rejects_a_plugin_without_rules() {
        let fs = MemoryFileSystem::default();
        fs.insert("/plugin.js".into(), "export const helper = () => {};");
        let fs = Arc::new(fs) as Arc<dyn FsWithResolverProxy>;

        let error = AnalyzerJsPlugin::load(fs, "/plugin.js".into(), None).unwrap_err();

        assert!(
            PrintDescription(&error)
                .to_string()
                .contains("at least one rule"),
            "unexpected error: {error:?}"
        );
    }

    #[test]
    fn rejects_a_query_with_an_unknown_kind() {
        let fs = MemoryFileSystem::default();
        fs.insert(
            "/plugin.js".into(),
            r#"import { ast, defineRule } from "@biomejs/plugin-api";
            export const myRule = defineRule({
                query: ast("NOT_A_KIND"),
                run(node) {},
            });"#,
        );
        let fs = Arc::new(fs) as Arc<dyn FsWithResolverProxy>;

        let error = AnalyzerJsPlugin::load(fs, "/plugin.js".into(), None).unwrap_err();

        assert!(
            PrintDescription(&error)
                .to_string()
                .contains("Unknown syntax kind"),
            "unexpected error: {error:?}"
        );
    }

    #[test]
    fn reports_top_level_var_declarations_using_ast_fields() {
        let source = r#"import { ast, defineRule, registerDiagnostic } from "@biomejs/plugin-api";
            export const noTopLevelVar = defineRule({
                query: ast("JS_MODULE"),
                run(root) {
                    for (const statement of root.items) {
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
                },
            });"#;
        let content = "var legacy = 1; const modern = 2;";
        let parse = biome_js_parser::parse(
            content,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );

        let plugin = load_test_plugin_from_source("/plugin.js", source, None);
        let result = plugin.evaluate(parse.syntax().into(), "/file.js".into());

        snap_diagnostics(
            "reports_top_level_var_declarations_using_ast_fields",
            render_diagnostics("/file.js", content, result),
        );
    }

    /// Rules only run on the nodes matching their query, and every diagnostic is tagged with the
    /// name of the rule that registered it.
    #[test]
    fn dispatches_nodes_to_the_matching_rules() {
        let source = r#"import { ast, defineRule, registerDiagnostic } from "@biomejs/plugin-api";
            export const noVar = defineRule({
                query: ast("JS_VARIABLE_STATEMENT"),
                run(statement) {
                    if (statement.declaration?.kindToken === "var") {
                        registerDiagnostic(statement, "warning", "Use let or const instead.");
                    }
                },
            });
            export const noFoo = defineRule({
                query: ast("JS_VARIABLE_STATEMENT", "JS_CALL_EXPRESSION"),
                run(node) {
                    registerDiagnostic(node, "information", `Seen: ${node.kind}`);
                },
            });"#;
        let content = "var legacy = foo();";
        let parse = biome_js_parser::parse(
            content,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );

        let plugin = load_test_plugin_from_source("/plugin.js", source, None);
        let content_rendered: String = parse
            .syntax()
            .descendants()
            .filter(|node| plugin.query().contains(&node.kind().to_raw()))
            .map(|node| {
                render_diagnostics(
                    "/file.js",
                    content,
                    plugin.evaluate(node.into(), "/file.js".into()),
                )
            })
            .collect();

        snap_diagnostics("dispatches_nodes_to_the_matching_rules", content_rendered);
    }

    /// Plugins can be written in TypeScript: the types are erased before the module is
    /// evaluated by the engine.
    #[test]
    fn evaluate_typescript_plugin() {
        let plugin = load_test_plugin_from_source(
            "/plugin.ts",
            r#"import { ast, defineRule, registerDiagnostic } from "@biomejs/plugin-api";
            import type { AnyJsRoot, Severity } from "@biomejs/plugin-api";
            export const useMyPlugin = defineRule({
                query: ast("JS_MODULE"),
                run(root: AnyJsRoot): void {
                    registerDiagnostic(root, "information" satisfies Severity, "Hello, TypeScript!");
                },
            });"#,
            None,
        );
        let parse = biome_js_parser::parse(
            "let foo;",
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );

        let result = plugin.evaluate(parse.syntax().into(), "/file.js".into());

        let [entry] = result.entries.as_slice() else {
            panic!("expected a single diagnostic, got {result:?}");
        };
        assert_eq!(
            PrintDescription(&entry.diagnostic).to_string(),
            "Hello, TypeScript!"
        );
    }

    /// TypeScript syntax that generates runtime code can't be erased, so loading fails instead
    /// of silently evaluating broken code.
    #[test]
    fn reject_typescript_plugin_with_unerasable_syntax() {
        let fs = MemoryFileSystem::default();
        fs.insert(
            "/plugin.ts".into(),
            r#"enum Severity { Information }
            export default function useMyPlugin() {}"#,
        );

        let fs = Arc::new(fs) as Arc<dyn FsWithResolverProxy>;
        let error = AnalyzerJsPlugin::load(fs, "/plugin.ts".into(), None)
            .expect_err("`enum` can't be erased");

        snap_diagnostics(
            "reject_typescript_plugin_with_unerasable_syntax",
            print_diagnostic_to_string(&Error::from(error)),
        );
    }

    #[test]
    fn evaluate_in_worker_threads() {
        let fs = MemoryFileSystem::default();
        fs.insert("/foo.js".into(), "let foo;");
        fs.insert("/bar.js".into(), "let bar;");
        fs.insert(
            "/plugin.js".into(),
            r#"import { ast, defineRule, registerDiagnostic } from "@biomejs/plugin-api";
            export const useMyPlugin = defineRule({
                query: ast("JS_MODULE"),
                run(root) {
                    registerDiagnostic(root, "information", "Hello, world!");
                },
            });"#,
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

        assert_eq!(result1.entries.len(), 1);
        assert_eq!(result2.entries.len(), 1);

        let content = render_diagnostics("/foo.js", "let foo;", result1)
            + &render_diagnostics("/bar.js", "let bar;", result2);

        snap_diagnostics("evaluate_in_worker_threads", content);
    }
}
