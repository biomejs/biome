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
                    const hasUnknownField = "unknownField" in root;
                    registerDiagnostic(
                        root,
                        "information",
                        `${root.kind}|${typeof descriptor.get}|${Object.prototype.hasOwnProperty.call(root, "items")}|${hasUnknownField}`,
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
    fn traverses_parents_and_ancestors_of_queried_descendants() {
        let plugin = load_test_plugin_from_source(
            "/plugin.js",
            r#"import { ast, defineRule, registerDiagnostic } from "@biomejs/plugin-api";
            export const ancestry = defineRule({
                query: ast("JS_EXPRESSION_STATEMENT", "JS_NUMBER_LITERAL_EXPRESSION", "JS_YIELD_ARGUMENT"),
                run(node) {
                    const ancestors = node.ancestors();
                    if (!Array.isArray(ancestors)) throw new Error("Expected an array");
                    const kinds = ancestors.map(ancestor => ancestor.kind).join(",");
                    let parent = node.parent;
                    for (const ancestor of ancestors) {
                        if (parent?.kind !== ancestor.kind || parent.text !== ancestor.text) {
                            throw new Error("Parent chain differs from ancestors");
                        }
                        parent = parent.parent;
                    }
                    if (parent !== undefined) throw new Error("Parent chain must end at the root");
                    const fresh = node.ancestors();
                    if (fresh === ancestors) throw new Error("Expected a fresh array");
                    ancestors.length = 0;
                    if (fresh.map(ancestor => ancestor.kind).join(",") !== kinds ||
                        node.ancestors().map(ancestor => ancestor.kind).join(",") !== kinds) {
                        throw new Error("Mutating an array changed the ancestry");
                    }
                    registerDiagnostic(node, "information", kinds);
                },
            });"#,
            None,
        );

        for (content, kind, malformed, expected) in [
            (
                "{ call(); }",
                JsSyntaxKind::JS_EXPRESSION_STATEMENT,
                false,
                "JS_STATEMENT_LIST,JS_BLOCK_STATEMENT,JS_MODULE_ITEM_LIST,JS_MODULE",
            ),
            (
                "call(1, 2);",
                JsSyntaxKind::JS_NUMBER_LITERAL_EXPRESSION,
                false,
                "JS_CALL_ARGUMENT_LIST,JS_CALL_ARGUMENTS,JS_CALL_EXPRESSION,JS_EXPRESSION_STATEMENT,JS_MODULE_ITEM_LIST,JS_MODULE",
            ),
            (
                "call((1));",
                JsSyntaxKind::JS_NUMBER_LITERAL_EXPRESSION,
                false,
                "JS_PARENTHESIZED_EXPRESSION,JS_CALL_ARGUMENT_LIST,JS_CALL_ARGUMENTS,JS_CALL_EXPRESSION,JS_EXPRESSION_STATEMENT,JS_MODULE_ITEM_LIST,JS_MODULE",
            ),
            (
                "yield 10;",
                JsSyntaxKind::JS_YIELD_ARGUMENT,
                true,
                "JS_BOGUS_EXPRESSION,JS_EXPRESSION_STATEMENT,JS_MODULE_ITEM_LIST,JS_MODULE",
            ),
        ] {
            let parse = biome_js_parser::parse(
                content,
                JsFileSource::js_module(),
                JsParserOptions::default(),
            );
            assert_eq!(parse.has_errors(), malformed, "{content}");
            let node = parse
                .syntax()
                .descendants()
                .find(|node| node.kind() == kind)
                .unwrap();
            let result = plugin.evaluate(node.into(), "/file.js".into());
            let [entry] = result.entries.as_slice() else {
                panic!("expected a single diagnostic for {content}, got {result:?}");
            };
            assert_eq!(
                PrintDescription(&entry.diagnostic).to_string(),
                expected,
                "{content}"
            );
        }
    }

    #[test]
    fn reports_ancestor_ranges_from_child_access() {
        let plugin = load_test_plugin_from_source(
            "/plugin.js",
            r#"import { ast, defineRule, registerDiagnostic } from "@biomejs/plugin-api";
            export const ancestry = defineRule({
                query: ast("JS_MODULE"),
                run(root) {
                    const node = root.items[0].expression.arguments.args[1].expression;
                    registerDiagnostic(node.parent, "information", node.parent.kind);
                    for (const ancestor of node.ancestors()) {
                        registerDiagnostic(ancestor, "information", ancestor.kind);
                    }
                },
            });"#,
            None,
        );
        let parse = biome_js_parser::parse(
            "  call(1, (2));  ",
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );
        assert!(!parse.has_errors());
        let result = plugin.evaluate(parse.syntax().into(), "/file.js".into());
        let expected = [
            ("JS_PARENTHESIZED_EXPRESSION", 10, 13),
            ("JS_PARENTHESIZED_EXPRESSION", 10, 13),
            ("JS_CALL_ARGUMENT_LIST", 7, 13),
            ("JS_CALL_ARGUMENTS", 6, 14),
            ("JS_CALL_EXPRESSION", 2, 14),
            ("JS_EXPRESSION_STATEMENT", 2, 15),
            ("JS_MODULE_ITEM_LIST", 2, 15),
            ("JS_MODULE", 2, 15),
        ];
        assert_eq!(result.entries.len(), expected.len(), "{result:?}");
        for (entry, (kind, start, end)) in result.entries.iter().zip(expected) {
            assert_eq!(PrintDescription(&entry.diagnostic).to_string(), kind);
            assert_eq!(
                entry.diagnostic.span(),
                Some(TextRange::new(start.into(), end.into())),
                "{kind}"
            );
        }
    }

    #[test]
    fn roots_have_no_parent_or_ancestors() {
        let plugin = load_test_plugin_from_source(
            "/plugin.js",
            r#"import { ast, defineRule, registerDiagnostic } from "@biomejs/plugin-api";
            export const ancestry = defineRule({
                query: ast("JS_MODULE", "JS_SCRIPT", "TS_DECLARATION_MODULE"),
                run(root) {
                    const ancestors = root.ancestors();
                    if (root.parent !== undefined || !Array.isArray(ancestors) || ancestors.length !== 0) {
                        throw new Error("Expected a root without a parent or ancestors");
                    }
                    const fresh = root.ancestors();
                    if (fresh === ancestors) throw new Error("Expected a fresh root array");
                    ancestors.push(root);
                    if (fresh.length !== 0 || root.ancestors().length !== 0) {
                        throw new Error("Mutating an array changed the root ancestry");
                    }
                    const [directives, list] = root.children();
                    const listKind = root.kind === "JS_SCRIPT" ? "JS_STATEMENT_LIST" : "JS_MODULE_ITEM_LIST";
                    if (root.children().length !== 2 || directives.kind !== "JS_DIRECTIVE_LIST" ||
                        directives.children().length !== 0 || list.kind !== listKind) {
                        throw new Error("Expected root list nodes, including empty directives");
                    }
                    for (const child of [directives, list]) {
                        if (Array.isArray(child) || child.parent.kind !== root.kind ||
                            child.ancestors().map(ancestor => ancestor.kind).join(",") !== root.kind) {
                            throw new Error("Expected the root as the list's only ancestor");
                        }
                    }
                    const items = root.items ?? root.statements;
                    const child = items[0];
                    if (!Array.isArray(items) || child.text !== list.children()[0].text ||
                        child.parent.kind !== listKind ||
                        child.ancestors().map(ancestor => ancestor.kind).join(",") !== `${listKind},${root.kind}`) {
                        throw new Error("Expected the list between the child and root");
                    }
                    registerDiagnostic(root, "information", root.kind);
                },
            });"#,
            None,
        );
        for (content, source, expected) in [
            ("let value;", JsFileSource::js_module(), "JS_MODULE"),
            ("let value;", JsFileSource::js_script(), "JS_SCRIPT"),
            (
                "declare const value: number;",
                JsFileSource::d_ts(),
                "TS_DECLARATION_MODULE",
            ),
        ] {
            let parse = biome_js_parser::parse(content, source, JsParserOptions::default());
            assert!(!parse.has_errors(), "{content}");
            let result = plugin.evaluate(parse.syntax().into(), "/file.js".into());
            let [entry] = result.entries.as_slice() else {
                panic!("expected a single diagnostic for {expected}, got {result:?}");
            };
            assert_eq!(PrintDescription(&entry.diagnostic).to_string(), expected);
        }
    }

    #[test]
    fn traversal_members_are_shared_non_enumerable_and_validate_receivers() {
        let plugin = load_test_plugin_from_source(
            "/plugin.js",
            r#"import { ast, defineRule, registerDiagnostic } from "@biomejs/plugin-api";
            export const ancestry = defineRule({
                query: ast("JS_MODULE"),
                run(root) {
                    const child = root.items[0];
                    const list = child.parent;
                    for (const name of ["parent", "ancestors", "children"]) {
                        let owner = Object.getPrototypeOf(root);
                        while (owner && !Object.prototype.hasOwnProperty.call(owner, name)) {
                            owner = Object.getPrototypeOf(owner);
                        }
                        if (!owner || !owner.isPrototypeOf(child) || !owner.isPrototypeOf(list)) {
                            throw new Error(`${name} must belong to a shared prototype`);
                        }
                        const descriptor = Object.getOwnPropertyDescriptor(owner, name);
                        const member = name === "parent" ? descriptor.get : descriptor.value;
                        if (descriptor.enumerable || typeof member !== "function") {
                            throw new Error(`${name} has the wrong descriptor`);
                        }
                        for (const node of [root, child, list]) {
                            if (Object.prototype.hasOwnProperty.call(node, name)) {
                                throw new Error(`${name} must not be an own property`);
                            }
                            for (const key in node) {
                                if (key === name) throw new Error(`${name} must not be enumerable`);
                            }
                        }
                        for (const receiver of [undefined, null, 1, "node", {}, [], Object.create(child)]) {
                            let threw = false;
                            try {
                                member.call(receiver);
                            } catch (error) {
                                if (!(error instanceof TypeError)) throw error;
                                threw = true;
                            }
                            if (!threw) throw new Error(`${name} accepted an invalid receiver`);
                        }
                    }
                    registerDiagnostic(root, "information", "Traversal prototype contract holds");
                },
            });"#,
            None,
        );
        let parse = biome_js_parser::parse(
            "let value;",
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );
        let result = plugin.evaluate(parse.syntax().into(), "/file.js".into());
        let [entry] = result.entries.as_slice() else {
            panic!("expected a single diagnostic, got {result:?}");
        };
        assert_eq!(
            PrintDescription(&entry.diagnostic).to_string(),
            "Traversal prototype contract holds"
        );
    }

    #[test]
    fn children_return_fresh_raw_nodes_in_source_order() {
        let plugin = load_test_plugin_from_source(
            "/plugin.js",
            r#"import { ast, defineRule, registerDiagnostic } from "@biomejs/plugin-api";
            export const children = defineRule({
                query: ast("JS_MODULE", "JS_BLOCK_STATEMENT", "JS_CALL_EXPRESSION", "JS_CALL_ARGUMENTS",
                    "JS_PARENTHESIZED_EXPRESSION", "JS_NUMBER_LITERAL_EXPRESSION", "JS_EXPRESSION_STATEMENT",
                    "JS_BOGUS_EXPRESSION", "JSX_ELEMENT", "JSX_EXPRESSION_CHILD",
                    "JS_DIRECTIVE_LIST", "JS_MODULE_ITEM_LIST", "JS_STATEMENT_LIST", "JS_CALL_ARGUMENT_LIST", "JSX_CHILD_LIST"),
                run(node) {
                    const children = node.children();
                    if (!Array.isArray(children)) throw new Error("Expected an array");
                    const describe = nodes => nodes.map(child => `${child.kind}:${child.text}`).join("|");
                    const description = describe(children);
                    for (const child of children) {
                        if (Array.isArray(child)) throw new Error("Expected a syntax node, not an array");
                        if (child.parent?.kind !== node.kind || child.parent.text !== node.text) {
                            throw new Error("Expected the immediate syntax parent");
                        }
                        if (describe(child.ancestors()) !== describe([node, ...node.ancestors()])) {
                            throw new Error("Expected the raw ancestor chain");
                        }
                        if (!Array.isArray(child.children())) throw new Error("Child cannot traverse children");
                    }
                    if (node.kind === "JSX_ELEMENT") {
                        if (!Array.isArray(node.elements) || children[1].kind !== "JSX_CHILD_LIST" ||
                            describe(node.elements) !== describe(children[1].children()) ||
                            describe([node.openingElement, children[1], node.closingElement]) !== description) {
                            throw new Error("JSX fields must remain separate from generic children");
                        }
                    }
                    const fresh = node.children();
                    if (fresh === children) throw new Error("Expected a fresh array");
                    children.length = 0;
                    children.push(node);
                    if (describe(fresh) !== description || describe(node.children()) !== description) {
                        throw new Error("Mutating an array changed the children");
                    }
                    registerDiagnostic(node, "information", description);
                },
            });"#,
            None,
        );
        for (content, kind, malformed, expected) in [
            (
                "\"use strict\"; \"custom\"; let a; call();",
                JsSyntaxKind::JS_MODULE,
                false,
                "JS_DIRECTIVE_LIST:\"use strict\"; \"custom\";|JS_MODULE_ITEM_LIST:let a; call();",
            ),
            (
                "\"use strict\"; \"custom\"; let a; call();",
                JsSyntaxKind::JS_DIRECTIVE_LIST,
                false,
                "JS_DIRECTIVE:\"use strict\";|JS_DIRECTIVE:\"custom\";",
            ),
            (
                "let a; call();",
                JsSyntaxKind::JS_MODULE_ITEM_LIST,
                false,
                "JS_VARIABLE_STATEMENT:let a;|JS_EXPRESSION_STATEMENT:call();",
            ),
            (
                "{ first(); { second(); } }",
                JsSyntaxKind::JS_BLOCK_STATEMENT,
                false,
                "JS_STATEMENT_LIST:first(); { second(); }",
            ),
            (
                "{ first(); { second(); } }",
                JsSyntaxKind::JS_STATEMENT_LIST,
                false,
                "JS_EXPRESSION_STATEMENT:first();|JS_BLOCK_STATEMENT:{ second(); }",
            ),
            (
                "call(1, (2), 3);",
                JsSyntaxKind::JS_CALL_EXPRESSION,
                false,
                "JS_IDENTIFIER_EXPRESSION:call|JS_CALL_ARGUMENTS:(1, (2), 3)",
            ),
            (
                "call(1, (2), 3);",
                JsSyntaxKind::JS_CALL_ARGUMENTS,
                false,
                "JS_CALL_ARGUMENT_LIST:1, (2), 3",
            ),
            (
                "call(1, (2), 3);",
                JsSyntaxKind::JS_CALL_ARGUMENT_LIST,
                false,
                "JS_NUMBER_LITERAL_EXPRESSION:1|JS_PARENTHESIZED_EXPRESSION:(2)|JS_NUMBER_LITERAL_EXPRESSION:3",
            ),
            (
                "call((2));",
                JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION,
                false,
                "JS_NUMBER_LITERAL_EXPRESSION:2",
            ),
            (
                "<a>hello{value}<b /></a>;",
                JsSyntaxKind::JSX_ELEMENT,
                false,
                "JSX_OPENING_ELEMENT:<a>|JSX_CHILD_LIST:hello{value}<b />|JSX_CLOSING_ELEMENT:</a>",
            ),
            (
                "<a>hello{value}<b /></a>;",
                JsSyntaxKind::JSX_CHILD_LIST,
                false,
                "JSX_TEXT:hello|JSX_EXPRESSION_CHILD:{value}|JSX_SELF_CLOSING_ELEMENT:<b />",
            ),
            ("1;", JsSyntaxKind::JS_NUMBER_LITERAL_EXPRESSION, false, ""),
            (
                "",
                JsSyntaxKind::JS_MODULE,
                false,
                "JS_DIRECTIVE_LIST:|JS_MODULE_ITEM_LIST:",
            ),
            ("", JsSyntaxKind::JS_DIRECTIVE_LIST, false, ""),
            ("", JsSyntaxKind::JS_MODULE_ITEM_LIST, false, ""),
            (
                "{}",
                JsSyntaxKind::JS_BLOCK_STATEMENT,
                false,
                "JS_STATEMENT_LIST:",
            ),
            ("{}", JsSyntaxKind::JS_STATEMENT_LIST, false, ""),
            (
                "call();",
                JsSyntaxKind::JS_CALL_ARGUMENTS,
                false,
                "JS_CALL_ARGUMENT_LIST:",
            ),
            ("call();", JsSyntaxKind::JS_CALL_ARGUMENT_LIST, false, ""),
            (
                "<a></a>;",
                JsSyntaxKind::JSX_ELEMENT,
                false,
                "JSX_OPENING_ELEMENT:<a>|JSX_CHILD_LIST:|JSX_CLOSING_ELEMENT:</a>",
            ),
            ("<a></a>;", JsSyntaxKind::JSX_CHILD_LIST, false, ""),
            ("<a>{}</a>;", JsSyntaxKind::JSX_EXPRESSION_CHILD, false, ""),
            (
                "yield 10;",
                JsSyntaxKind::JS_EXPRESSION_STATEMENT,
                true,
                "JS_BOGUS_EXPRESSION:yield 10",
            ),
            (
                "yield 10;",
                JsSyntaxKind::JS_BOGUS_EXPRESSION,
                true,
                "JS_YIELD_ARGUMENT:10",
            ),
        ] {
            let parse =
                biome_js_parser::parse(content, JsFileSource::jsx(), JsParserOptions::default());
            assert_eq!(parse.has_errors(), malformed, "{content}");
            let node = parse
                .syntax()
                .descendants()
                .find(|node| node.kind() == kind)
                .unwrap();
            let result = plugin.evaluate(node.into(), "/file.jsx".into());
            let [entry] = result.entries.as_slice() else {
                panic!("expected a single diagnostic for {content}, got {result:?}");
            };
            assert_eq!(
                PrintDescription(&entry.diagnostic).to_string(),
                expected,
                "{content}: {kind:?}"
            );
        }
    }

    #[test]
    fn queries_lists_and_reports_diagnostics_on_lists_and_their_children() {
        let plugin = load_test_plugin_from_source(
            "/plugin.js",
            r#"import { ast, defineRule, registerDiagnostic } from "@biomejs/plugin-api";
            export const children = defineRule({
                query: ast("JS_CALL_ARGUMENT_LIST"),
                run(list) {
                    if (Array.isArray(list) || list.kind !== "JS_CALL_ARGUMENT_LIST" || list.text !== "1, 2" ||
                        list.parent.kind !== "JS_CALL_ARGUMENTS" ||
                        list.ancestors().map(node => node.kind).join(",") !==
                            "JS_CALL_ARGUMENTS,JS_CALL_EXPRESSION,JS_EXPRESSION_STATEMENT,JS_MODULE_ITEM_LIST,JS_MODULE") {
                        throw new Error("Expected a queryable list wrapper with ancestry");
                    }
                    registerDiagnostic(list, "information", list.kind);
                    for (const child of list.children()) {
                        if (child.parent.kind !== list.kind || child.parent.text !== list.text) {
                            throw new Error("Expected the list as the argument's parent");
                        }
                        registerDiagnostic(child, "information", child.valueToken);
                    }
                },
            });"#,
            None,
        );
        assert_eq!(
            plugin.query(),
            [JsSyntaxKind::JS_CALL_ARGUMENT_LIST.to_raw()]
        );
        let parse = biome_js_parser::parse(
            "call(1, 2);",
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );
        assert!(!parse.has_errors());
        let list = parse
            .syntax()
            .descendants()
            .find(|node| plugin.query().contains(&node.kind().to_raw()))
            .unwrap();
        let result = plugin.evaluate(list.into(), "/file.js".into());
        let expected = [("JS_CALL_ARGUMENT_LIST", 5, 9), ("1", 5, 6), ("2", 8, 9)];
        assert_eq!(result.entries.len(), expected.len(), "{result:?}");
        for (entry, (message, start, end)) in result.entries.iter().zip(expected) {
            assert_eq!(PrintDescription(&entry.diagnostic).to_string(), message);
            assert_eq!(
                entry.diagnostic.span(),
                Some(TextRange::new(start.into(), end.into())),
                "{message}"
            );
        }
    }

    #[test]
    fn reports_children_ranges_and_exposes_normal_fields() {
        let plugin = load_test_plugin_from_source(
            "/plugin.js",
            r#"import { ast, defineRule, registerDiagnostic } from "@biomejs/plugin-api";
            export const children = defineRule({
                query: ast("JS_MODULE"),
                run(root) {
                    const items = root.children()[1];
                    const statement = items.children()[0];
                    const call = statement.children()[0];
                    const args = call.children()[1];
                    if (statement.expression.text !== call.text || call.arguments.text !== args.text) {
                        throw new Error("Child fields differ from generic traversal");
                    }
                    const list = args.children()[0];
                    const [first, wrapped] = list.children();
                    const second = wrapped.children()[0];
                    if (first.valueToken !== "1" || wrapped.expression.valueToken !== "2" ||
                        second.valueToken !== "2" || !Array.isArray(args.args) || args.args.length !== 2 ||
                        !Array.isArray(root.items) || root.items[0].text !== statement.text ||
                        args.args[0].text !== first.text || args.args[1].text !== wrapped.text) {
                        throw new Error("Expected normal fields on children");
                    }
                    for (const node of [items, statement, call, args, list, first, wrapped, second]) {
                        registerDiagnostic(node, "information", node.kind);
                    }
                },
            });"#,
            None,
        );
        let parse = biome_js_parser::parse(
            "  call(1, (2));  ",
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );
        assert!(!parse.has_errors());
        let result = plugin.evaluate(parse.syntax().into(), "/file.js".into());
        let expected = [
            ("JS_MODULE_ITEM_LIST", 2, 15),
            ("JS_EXPRESSION_STATEMENT", 2, 15),
            ("JS_CALL_EXPRESSION", 2, 14),
            ("JS_CALL_ARGUMENTS", 6, 14),
            ("JS_CALL_ARGUMENT_LIST", 7, 13),
            ("JS_NUMBER_LITERAL_EXPRESSION", 7, 8),
            ("JS_PARENTHESIZED_EXPRESSION", 10, 13),
            ("JS_NUMBER_LITERAL_EXPRESSION", 11, 12),
        ];
        assert_eq!(result.entries.len(), expected.len(), "{result:?}");
        for (entry, (kind, start, end)) in result.entries.iter().zip(expected) {
            assert_eq!(PrintDescription(&entry.diagnostic).to_string(), kind);
            assert_eq!(
                entry.diagnostic.span(),
                Some(TextRange::new(start.into(), end.into())),
                "{kind}"
            );
        }
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
