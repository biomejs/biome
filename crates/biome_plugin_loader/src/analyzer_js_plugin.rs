use std::fmt::{Debug, Formatter};
use std::ops::DerefMut;
use std::sync::Arc;

use boa_engine::class::{Class, ClassBuilder};
use boa_engine::object::builtins::{JsArray, JsFunction};
use boa_engine::property::Attribute;
use boa_engine::{
    Context, Finalize, JsData, JsNativeError, JsResult, JsString, JsValue, NativeFunction, Trace,
    js_string,
};
use camino::{Utf8Path, Utf8PathBuf};

use biome_analyze::{
    AnalyzerPlugin, PluginDiagnosticEntry, PluginEvalResult, PluginTargetLanguage, RuleDiagnostic,
};
use biome_console::markup;
use biome_diagnostics::category;
use biome_glob::NormalizedGlob;
use biome_js_runtime::JsExecContext;
use biome_js_syntax::{AnyJsRoot, JsLanguage, JsSyntaxNode, JsSyntaxToken};
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

#[derive(Debug, JsData)]
struct JsAstNode {
    data: JsAstNodeData,
}

impl Finalize for JsAstNode {}

// SAFETY: `JsAstNode` only contains Rowan data and no values managed by Boa's garbage collector.
unsafe impl Trace for JsAstNode {
    boa_engine::gc::empty_trace!();
}

impl JsAstNode {
    fn from_syntax(node: JsSyntaxNode) -> JsResult<Self> {
        let Some(data) = JsAstNodeData::cast(node) else {
            return Err(JsNativeError::typ()
                .with_message("Unsupported JavaScript AST node")
                .into());
        };

        Ok(Self { data })
    }

    fn from_this(this: &JsValue) -> JsResult<JsSyntaxNode> {
        let Some(object) = this.as_object() else {
            return Err(JsNativeError::typ()
                .with_message("AST getter called with an invalid receiver")
                .into());
        };
        let Some(node) = object.downcast_ref::<Self>() else {
            return Err(JsNativeError::typ()
                .with_message("AST getter called with an invalid receiver")
                .into());
        };

        Ok(node.data.syntax().clone())
    }

    fn resolve_field(this: &JsValue, field: &str, context: &mut Context) -> JsResult<JsValue> {
        let Some(object) = this.as_object() else {
            return Err(JsNativeError::typ()
                .with_message("AST getter called with an invalid receiver")
                .into());
        };
        let Some(node) = object.downcast_ref::<Self>() else {
            return Err(JsNativeError::typ()
                .with_message("AST getter called with an invalid receiver")
                .into());
        };

        node.data.resolve_field(field, context)
    }

    fn text_range(value: &JsValue) -> Option<TextRange> {
        let object = value.as_object()?;
        let node = object.downcast_ref::<Self>()?;
        Some(node.data.syntax().text_trimmed_range())
    }

    fn get_kind(this: &JsValue, _args: &[JsValue], _context: &mut Context) -> JsResult<JsValue> {
        let node = Self::from_this(this)?;
        Ok(JsString::from(format!("{:?}", node.kind())).into())
    }

    fn get_text(this: &JsValue, _args: &[JsValue], _context: &mut Context) -> JsResult<JsValue> {
        let node = Self::from_this(this)?;
        Ok(JsString::from(node.text_trimmed().to_string()).into())
    }

    fn wrap_optional_node<N>(node: Option<N>, context: &mut Context) -> JsResult<JsValue>
    where
        N: AstNode<Language = JsLanguage>,
    {
        match node {
            Some(node) => Self::from_syntax(node.into_syntax())
                .and_then(|node| Self::from_data(node, context))
                .map(Into::into),
            None => Ok(JsValue::undefined()),
        }
    }

    fn wrap_node_list<I, N>(nodes: I, context: &mut Context) -> JsResult<JsValue>
    where
        I: IntoIterator<Item = N>,
        N: AstNode<Language = JsLanguage>,
    {
        let nodes = nodes
            .into_iter()
            .map(|node| {
                Self::from_syntax(node.into_syntax())
                    .and_then(|node| Self::from_data(node, context))
                    .map(Into::into)
            })
            .collect::<JsResult<Vec<_>>>()?;

        Ok(JsArray::from_iter(nodes, context).into())
    }

    fn wrap_token(token: Option<JsSyntaxToken>) -> JsValue {
        token.map_or_else(JsValue::undefined, |token| {
            JsString::from(token.text_trimmed().to_string()).into()
        })
    }
}

include!("generated/js_ast.rs");

impl Class for JsAstNode {
    const NAME: &'static str = "__BiomeJsAstNode";

    fn init(class: &mut ClassBuilder<'_>) -> JsResult<()> {
        let kind =
            NativeFunction::from_fn_ptr(Self::get_kind).to_js_function(class.context().realm());
        let text =
            NativeFunction::from_fn_ptr(Self::get_text).to_js_function(class.context().realm());
        class
            .accessor(js_string!("kind"), Some(kind), None, Attribute::ENUMERABLE)
            .accessor(js_string!("text"), Some(text), None, Attribute::ENUMERABLE);

        Self::init_generated(class);

        Ok(())
    }

    fn data_constructor(
        _new_target: &JsValue,
        _args: &[JsValue],
        _context: &mut Context,
    ) -> JsResult<Self> {
        Err(JsNativeError::typ()
            .with_message("AST nodes cannot be constructed from JavaScript")
            .into())
    }
}

fn load_plugin(fs: Arc<dyn FsWithResolverProxy>, path: &Utf8Path) -> JsResult<LoadedPlugin> {
    let mut ctx = JsExecContext::new(fs)?;
    ctx.set_diagnostic_range_resolver(JsAstNode::text_range);
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

        let ast = match JsAstNode::from_syntax(node)
            .and_then(|node| plugin.ctx.create_class_instance(node))
        {
            Ok(ast) => ast,
            Err(err) => {
                return PluginEvalResult {
                    entries: vec![PluginDiagnosticEntry {
                        diagnostic: RuleDiagnostic::new(
                            category!("plugin"),
                            None::<TextRange>,
                            markup!("Could not pass the AST to the plugin: "<Error>{err.to_string()}</Error>),
                        ),
                        action: None,
                    }],
                };
            }
        };

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
        let content = result
            .entries
            .into_iter()
            .map(|entry| print_diagnostic_to_string(&Error::from(entry.diagnostic)))
            .collect::<String>();

        assert!(content.contains("/file.js|JS_MODULE|function|false|false"));
    }

    #[test]
    fn reports_top_level_var_declarations_using_ast_fields() {
        let plugin = load_test_plugin_from_source(
            r#"import { registerDiagnostic } from "@biomejs/plugin-api";
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
            }"#,
            None,
        );
        let parse = biome_js_parser::parse(
            "var legacy = 1; const modern = 2;",
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );

        let result = plugin.evaluate(parse.syntax().into(), "/file.js".into());
        assert_eq!(result.entries.len(), 1);
        assert_eq!(
            result.entries[0].diagnostic.span(),
            Some(TextRange::new(0.into(), 15.into()))
        );
        let content = result
            .entries
            .into_iter()
            .map(|entry| print_diagnostic_to_string(&Error::from(entry.diagnostic)))
            .collect::<String>();

        assert!(content.contains("Use let or const instead of a top-level var declaration."));
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
