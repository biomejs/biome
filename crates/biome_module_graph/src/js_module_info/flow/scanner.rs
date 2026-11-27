use super::builder::MAX_DEPTH;
use biome_js_control_flow::AnyJsControlFlowRoot;
use biome_js_syntax::{
    AnyJsBinding, AnyTsReturnType, JsConditionalExpression, JsDoWhileStatement, JsForStatement,
    JsIdentifierBinding, JsIfStatement, JsLogicalExpression, JsSyntaxKind, JsSyntaxNode,
    JsWhileStatement, TsDeclareFunctionDeclaration,
};
use biome_rowan::{AstNode, SyntaxKind, declare_node_union};
use std::ops::Range;

/// Limits the syntax nodes, including nested roots, of a root that gets flow.
pub(super) const MAX_ROOT_NODES: usize = 16_384;

declare_node_union! {
    /// Syntax that tests a condition along some path.
    AnyFlowConditionSource = JsIfStatement
        | JsWhileStatement
        | JsDoWhileStatement
        | JsForStatement
        | JsLogicalExpression
        | JsConditionalExpression
}

/// Selects the execution roots that may need flow while the module visitor
/// walks the syntax tree, so collection never walks the module again.
///
/// The visitor must report every node of the module in preorder through
/// [`Self::enter`] and [`Self::leave`]. A root is selected when it directly
/// contains a condition, or a standalone call while the module declares an
/// assertion, outside its nested roots, has fewer than
/// [`MAX_ROOT_NODES`] syntax nodes, and uses only syntax the flow builder
/// supports.
///
/// The builder treats statements it does not model, such as `switch` or
/// `try`, as opaque regions, so they do not reject a root. A root is not
/// selected when its own syntax, outside nested roots, contains a `with`
/// statement, bogus syntax, or nesting deeper than [`MAX_DEPTH`]. A `with`
/// statement changes what the names inside it refer to, and TypeScript types
/// its contents as `any`.
///
/// Types never run, so the scanner ignores conditions and bindings inside
/// them, skips type aliases and interfaces entirely, and skips everything but
/// the name of a function overload declaration. Roots nested in types, such as
/// a function in a computed property key, are therefore never selected.
#[derive(Default)]
pub(crate) struct FlowRootScanner {
    open: Vec<OpenRoot>,
    /// Bindings declared by the open roots, each root's after those of the
    /// roots that enclose it.
    bindings: Vec<JsIdentifierBinding>,
    visited: usize,
    /// Depth of the node most recently entered and not yet left.
    depth: usize,
    /// Depth of the type alias, interface, or function overload declaration
    /// that encloses the current node, whose contents the scanner ignores.
    declaration: Option<usize>,
    /// Depth of the outermost type, including type predicates, that encloses
    /// the current node.
    type_region: Option<usize>,
    /// Depth of the next node when it is the expression of an expression
    /// statement, or of parentheses that form that expression.
    statement_expression: Option<usize>,
    selected: Vec<SelectedRoot>,
    /// Roots that need flow only if a standalone call in their own code calls
    /// an assertion.
    deferred: Vec<SelectedRoot>,
    /// The bindings of the selected and deferred roots.
    kept_bindings: Vec<JsIdentifierBinding>,
    /// Whether the module contains an `asserts` return annotation.
    has_assertion_signature: bool,
}

/// An execution root that may need flow.
pub(super) struct SelectedRoot {
    pub(super) root: AnyJsControlFlowRoot,
    /// The range of the root's bindings in [`FlowRootScanner::kept_bindings`].
    bindings: Range<usize>,
    /// Whether the root's own code, outside nested roots and types, contains
    /// a `this` expression, so paths on its `this` may narrow without any
    /// variable of its own.
    pub(super) has_this: bool,
}

impl FlowRootScanner {
    pub(crate) fn enter(&mut self, node: &JsSyntaxNode) {
        self.visited += 1;
        self.depth += 1;
        let kind = node.kind();
        // An assertion can be declared after its calls, including as a function
        // overload declaration.
        self.has_assertion_signature |= kind == JsSyntaxKind::TS_ASSERTS_RETURN_TYPE;
        if self.declaration.is_some() {
            return;
        }
        if AnyJsControlFlowRoot::can_cast(kind) {
            if let Some(parent) = self.open.last_mut() {
                parent.check_depth(self.depth);
            }
            self.open.push(OpenRoot {
                first_node: self.visited - 1,
                first_binding: self.bindings.len(),
                depth: self.depth,
                has_condition: false,
                has_call_statement: false,
                has_this: false,
                is_unsupported: false,
            });
        }
        let Some(root) = self.open.last_mut() else {
            return;
        };
        root.check_depth(self.depth);
        if matches!(
            kind,
            JsSyntaxKind::TS_TYPE_ALIAS_DECLARATION
                | JsSyntaxKind::TS_INTERFACE_DECLARATION
                | JsSyntaxKind::TS_DECLARE_FUNCTION_DECLARATION
        ) {
            // The signature never runs, but its name is a value of this root.
            if let Some(declaration) = TsDeclareFunctionDeclaration::cast_ref(node)
                && let Ok(AnyJsBinding::JsIdentifierBinding(binding)) = declaration.id()
            {
                self.bindings.push(binding);
            }
            self.declaration = Some(self.depth);
            return;
        }
        root.check_node(kind);
        if self.type_region.is_some() {
            return;
        }
        // The walk enters the expression of an expression statement right after
        // the statement, because it is the statement's first child node.
        if self.statement_expression.take() == Some(self.depth) {
            match kind {
                JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION => {
                    self.statement_expression = Some(self.depth + 1);
                }
                JsSyntaxKind::JS_CALL_EXPRESSION => root.has_call_statement = true,
                _ => {}
            }
        }
        if AnyTsReturnType::can_cast(kind) {
            self.type_region = Some(self.depth);
        } else if AnyFlowConditionSource::can_cast(kind) {
            root.has_condition = true;
        } else if kind == JsSyntaxKind::JS_THIS_EXPRESSION {
            root.has_this = true;
        } else if let Some(binding) = JsIdentifierBinding::cast_ref(node) {
            self.bindings.push(binding);
        } else if kind == JsSyntaxKind::JS_EXPRESSION_STATEMENT {
            self.statement_expression = Some(self.depth + 1);
        }
    }

    pub(crate) fn leave(&mut self, node: &JsSyntaxNode) {
        let depth = self.depth;
        self.depth -= 1;
        if let Some(declaration) = self.declaration {
            if declaration == depth {
                self.declaration = None;
            }
            return;
        }
        if self.type_region == Some(depth) {
            self.type_region = None;
        }
        if !AnyJsControlFlowRoot::can_cast(node.kind()) {
            return;
        }
        let Some(root) = self.open.pop() else {
            return;
        };
        if (root.has_condition || root.has_call_statement)
            && !root.is_unsupported
            && self.visited - root.first_node < MAX_ROOT_NODES
            && let Some(node) = AnyJsControlFlowRoot::cast_ref(node)
        {
            let start = self.kept_bindings.len();
            self.kept_bindings
                .extend(self.bindings.drain(root.first_binding..));
            let selected = SelectedRoot {
                root: node,
                bindings: start..self.kept_bindings.len(),
                has_this: root.has_this,
            };
            if root.has_condition {
                self.selected.push(selected);
            } else {
                self.deferred.push(selected);
            }
        }
        self.bindings.truncate(root.first_binding);
    }

    /// Returns the selected roots.
    ///
    /// A root with a standalone call but no condition of its own is selected
    /// only when the module contains an `asserts` return annotation. The walk
    /// can reach that annotation after the root.
    pub(super) fn selected(&self) -> impl Iterator<Item = &SelectedRoot> {
        let deferred: &[SelectedRoot] = if self.has_assertion_signature {
            &self.deferred
        } else {
            &[]
        };
        self.selected.iter().chain(deferred)
    }

    /// Returns the bindings that the own code of `root` declares, outside
    /// nested roots and types, in source order.
    pub(super) fn bindings(&self, root: &SelectedRoot) -> &[JsIdentifierBinding] {
        &self.kept_bindings[root.bindings.clone()]
    }
}

struct OpenRoot {
    first_node: usize,
    /// Index in [`FlowRootScanner::bindings`] of the first binding the root
    /// declares.
    first_binding: usize,
    depth: usize,
    has_condition: bool,
    /// Whether the root's own code has an expression statement that is a call,
    /// possibly in parentheses.
    has_call_statement: bool,
    /// Whether the root's own code reads `this`.
    has_this: bool,
    is_unsupported: bool,
}

impl OpenRoot {
    /// Marks the root unsupported when a node of its own syntax at `depth` is
    /// nested too deeply below it.
    fn check_depth(&mut self, depth: usize) {
        if depth - self.depth > MAX_DEPTH {
            self.is_unsupported = true;
        }
    }

    /// Marks the root unsupported when the flow builder does not support a
    /// node of `kind` in its own syntax, regardless of the node's descendants.
    fn check_node(&mut self, kind: JsSyntaxKind) {
        if self.is_unsupported {
            return;
        }
        self.is_unsupported = kind.is_bogus()
            || kind.is_metavariable()
            || kind == JsSyntaxKind::JS_WITH_STATEMENT
            // A property signature is only checked as a root, never as part of
            // another root's syntax, because it is itself an execution root.
            || kind == JsSyntaxKind::TS_PROPERTY_SIGNATURE_TYPE_MEMBER;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_js_parser::{JsParserOptions, parse};
    use biome_js_syntax::AnyJsRoot;
    use biome_languages::JsFileSource;
    use biome_rowan::WalkEvent;

    /// Returns the texts of the roots selected in the TypeScript `source`, in
    /// selection order.
    fn selected(source: &str) -> Vec<String> {
        let parsed = parse(source, JsFileSource::ts(), JsParserOptions::default());
        assert!(!parsed.has_errors(), "{source}");
        selected_in(&parsed.tree())
    }

    /// Returns the texts of the roots selected in `root`, in selection order.
    fn selected_in(root: &AnyJsRoot) -> Vec<String> {
        scan(root)
            .selected()
            .map(|selected| selected.root.syntax().text_trimmed().to_string())
            .collect()
    }

    fn scan(root: &AnyJsRoot) -> FlowRootScanner {
        let mut scanner = FlowRootScanner::default();
        for event in root.syntax().preorder() {
            match event {
                WalkEvent::Enter(node) => scanner.enter(&node),
                WalkEvent::Leave(node) => scanner.leave(&node),
            }
        }
        scanner
    }

    #[test]
    fn roots_need_a_condition_of_their_own() {
        let source = "function f(x) { read(x); const g = () => x && read(x); }";
        assert_eq!(selected(source), ["() => x && read(x)"]);
        assert_eq!(
            selected("function f(x) { if (x) read(x); }"),
            ["function f(x) { if (x) read(x); }"]
        );
    }

    #[test]
    fn selected_roots_record_whether_their_own_code_reads_this() {
        const SOURCE: &str = "class K { m() { if (this.x) read(this.x); const g = (a) => { if (a) read(a); }; } n(a) { if (a) read(a); const h = () => this; } k(a): this { if (a) read(a); return a; } }";
        let parsed = parse(SOURCE, JsFileSource::ts(), JsParserOptions::default());
        assert!(!parsed.has_errors());
        let has_this: Vec<(String, bool)> = scan(&parsed.tree())
            .selected()
            .map(|selected| {
                let text = selected.root.syntax().text_trimmed().to_string();
                (
                    text[..text.find('{').unwrap()].trim().to_owned(),
                    selected.has_this,
                )
            })
            .collect();
        // A `this` in a nested root or in a type does not count.
        assert_eq!(
            has_this,
            [
                ("(a) =>".to_owned(), false),
                ("m()".to_owned(), true),
                ("n(a)".to_owned(), false),
                ("k(a): this".to_owned(), false),
            ]
        );
    }

    #[test]
    fn selected_roots_keep_the_bindings_of_their_own_code() {
        const SOURCE: &str = "function f(a, b: (x: string) => void) { const c = a; const g = (d) => d && c; const h = (z) => z; if (a) { let e = c; } }";
        let parsed = parse(SOURCE, JsFileSource::ts(), JsParserOptions::default());
        assert!(!parsed.has_errors());
        let scanner = scan(&parsed.tree());
        let bindings: Vec<Vec<String>> = scanner
            .selected()
            .map(|selected| {
                scanner
                    .bindings(selected)
                    .iter()
                    .map(|binding| binding.syntax().text_trimmed().to_string())
                    .collect()
            })
            .collect();
        assert_eq!(
            bindings,
            [vec!["d"], vec!["f", "a", "b", "c", "g", "h", "e"]]
        );
    }

    #[test]
    fn standalone_calls_need_an_assertion_signature_in_the_module() {
        const CALLS: &str = "function f(x) { check(x); } function g(x) { ((check)(x)); }";
        assert!(selected(CALLS).is_empty());
        // The signature can follow the calls that use it.
        let source = format!("{CALLS} declare function check(input: unknown): asserts input;");
        assert_eq!(
            selected(&source),
            [
                "function f(x) { check(x); }",
                "function g(x) { ((check)(x)); }"
            ]
        );
        for body in ["void check(x);", "const y = check(x);", "(0, check(x));"] {
            let source = format!(
                "function f(x) {{ {body} }} declare function check(input: unknown): asserts input;"
            );
            assert!(selected(&source).is_empty(), "{body}");
        }
    }

    #[test]
    fn statements_the_builder_does_not_model_keep_their_root() {
        for body in [
            "try { read(x); } catch (error) { read(error); }",
            "try { read(x); } finally { read(x); }",
            "switch (x) { case 1: read(x); break; default: read(x); }",
            "for (const key in x) read(key);",
            "for (const item of x) read(item);",
            "class C extends Base { field = read(x); }",
            "enum E { A = 1 }",
        ] {
            let source = format!("function f(x) {{ if (x) read(x); {body} }}");
            assert_eq!(selected(&source), [source.as_str()], "{body}");
        }
        for body in [
            "declare const d: string | null;",
            "declare module \"m\" { export const v: string; }",
            "declare global { var v: string; }",
            "import y = require(\"y\");",
            "export import z = N.z;",
            "namespace N { export const z = 1; }",
            "export class C {}",
            "export enum E { A }",
            "export declare class D {}",
            "export = x;",
        ] {
            let source = format!("declare const x: string | null; if (x) read(x); {body}");
            assert_eq!(selected(&source), [source.as_str()], "{body}");
        }
    }

    #[test]
    fn with_statements_reject_only_their_own_root() {
        let selected_in_script = |source: &str| {
            let parsed = parse(
                source,
                JsFileSource::js_script(),
                JsParserOptions::default(),
            );
            assert!(!parsed.has_errors(), "{source}");
            selected_in(&parsed.tree())
        };
        assert!(
            selected_in_script("function f(x) { if (x) read(x); with (x) read(x); }").is_empty()
        );
        assert_eq!(
            selected_in_script(
                "function f(x) { if (x) read(x); const g = () => { with (x) read(x); }; }"
            ),
            ["function f(x) { if (x) read(x); const g = () => { with (x) read(x); }; }"]
        );
    }

    #[test]
    fn patterns_class_expressions_and_logical_assignments_keep_their_root() {
        for body in [
            "const { value = read(x), ...rest } = x;",
            "const [first, ...others] = x;",
            "({ value: y } = x);",
            "[y = read(x)] = x;",
            "y &&= read(x);",
            "y ||= read(x);",
            "y ??= read(x);",
            "const C = class extends Base { field = read(x); static { read(x); } m() {} };",
        ] {
            let source = format!("function f(x) {{ if (x) read(x); {body} }}");
            assert_eq!(selected(&source), [source.as_str()], "{body}");
        }
        for source in [
            "function f({ value }, [first]) { if (value) read(first); }",
            "if (x) read(x); export default class { field = read(x); }",
        ] {
            assert_eq!(selected(source), [source], "{source}");
        }
    }

    #[test]
    fn type_declarations_are_not_inspected() {
        let source = "function f(x: string | null) { interface I { eval: { [K in keyof typeof arguments]: T } } type T = typeof eval; if (x) read(x); }";
        assert_eq!(selected(source).len(), 1);
    }

    #[test]
    fn namespaces_are_roots_of_their_own() {
        const SOURCE: &str =
            "let x: string | null = null; if (x) read(x); namespace N { if (x) read(x); }";
        assert_eq!(
            selected(SOURCE),
            ["namespace N { if (x) read(x); }", SOURCE]
        );
    }

    #[test]
    fn bogus_syntax_rejects_the_root() {
        let parsed = parse(
            "function f(x) { if (x) read(x); let 5 = x; }",
            JsFileSource::ts(),
            JsParserOptions::default(),
        );
        assert!(parsed.has_errors());
        assert!(selected_in(&parsed.tree()).is_empty());
    }

    #[test]
    fn deep_syntax_rejects_the_root() {
        let nested = |depth: usize| {
            format!(
                "function f(x) {{ if (x) read(x); {}x{}; }}",
                "(".repeat(depth),
                ")".repeat(depth)
            )
        };
        assert_eq!(selected(&nested(MAX_DEPTH / 2)).len(), 1);
        assert!(selected(&nested(MAX_DEPTH)).is_empty());
    }

    #[test]
    fn nested_root_nodes_count_toward_the_depth_of_their_parent() {
        // The arrow sits four levels below `f`, plus one per parenthesis.
        let nested = |parentheses: usize| {
            format!(
                "function f(x) {{ if (x) read(x); {}() => 0{}; }}",
                "(".repeat(parentheses),
                ")".repeat(parentheses)
            )
        };
        assert_eq!(selected(&nested(MAX_DEPTH - 4)).len(), 1);
        assert!(selected(&nested(MAX_DEPTH - 3)).is_empty());
    }

    #[test]
    fn nested_roots_count_toward_the_root_node_limit() {
        // Each `other;` statement adds three syntax nodes to the nested arrow.
        for (statements, expected) in [(1, 1), (MAX_ROOT_NODES / 3, 0)] {
            let source = format!(
                "function f(x) {{ const g = () => {{ {} }}; if (x) read(x); }}",
                "other;".repeat(statements)
            );
            assert_eq!(selected(&source).len(), expected, "{statements}");
        }
    }
}
