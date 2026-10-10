use super::builder::MAX_DEPTH;
use biome_js_control_flow::AnyJsControlFlowRoot;
use biome_js_semantic::{JsDeclarationKind, SemanticModel};
use biome_js_syntax::{
    AnyJsStatement, AnyTsReturnType, JsAssignmentExpression, JsAssignmentOperator,
    JsConditionalExpression, JsDoWhileStatement, JsForStatement, JsIfStatement,
    JsLogicalExpression, JsReferenceIdentifier, JsSyntaxKind, JsSyntaxNode, JsWhileStatement,
    unescape_js_identifier,
};
use biome_rowan::{AstNode, SyntaxKind, TextRange, declare_node_union};

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
/// contains a condition, outside its nested roots, has fewer than
/// [`MAX_ROOT_NODES`] syntax nodes, uses only syntax the flow builder supports,
/// and neither it nor a nested root may read the global `eval` or the implicit
/// `arguments` object: direct `eval` and mapped `arguments` can write any
/// enclosing variable.
///
/// The builder supports truthiness tests in `if`, `while`, `do` and ordinary
/// `for` statements, and expression-level `&&`, `||`, `??` and `?:`. A root is
/// not selected when its own syntax, outside nested roots, contains exception
/// handlers, `switch`, `for-in`/`for-of`, `with`, classes, destructuring,
/// logical assignments, bogus syntax, or nesting deeper than [`MAX_DEPTH`].
///
/// Types never run, so the scanner ignores conditions and references inside
/// them and skips type aliases, interfaces, and function overload declarations
/// entirely. Roots nested in types, such as a function in a computed property
/// key, are therefore never selected.
#[derive(Default)]
pub(crate) struct FlowRootScanner {
    open: Vec<OpenRoot>,
    visited: usize,
    /// Depth of the node most recently entered and not yet left.
    depth: usize,
    /// Depth of the type alias, interface, or function overload declaration
    /// that encloses the current node, whose contents the scanner ignores.
    declaration: Option<usize>,
    /// Depth of the outermost type, including type predicates, that encloses
    /// the current node.
    type_region: Option<usize>,
    selected: Vec<AnyJsControlFlowRoot>,
}

impl FlowRootScanner {
    pub(crate) fn enter(&mut self, node: &JsSyntaxNode, model: &SemanticModel) {
        self.visited += 1;
        self.depth += 1;
        if self.declaration.is_some() {
            return;
        }
        let kind = node.kind();
        if AnyJsControlFlowRoot::can_cast(kind) {
            if let Some(parent) = self.open.last_mut() {
                parent.check_depth(self.depth);
            }
            self.open.push(OpenRoot {
                first_node: self.visited - 1,
                depth: self.depth,
                range: node.text_range_with_trivia(),
                // Arrow functions read the `arguments` object of the function
                // that encloses them.
                has_arguments: !matches!(
                    kind,
                    JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION
                        | JsSyntaxKind::JS_MODULE
                        | JsSyntaxKind::JS_SCRIPT
                ),
                has_condition: false,
                has_dynamic_scope: false,
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
            self.declaration = Some(self.depth);
            return;
        }
        root.check_node(node, kind);
        if self.type_region.is_some() {
            return;
        }
        if AnyTsReturnType::can_cast(kind) {
            self.type_region = Some(self.depth);
        } else if AnyFlowConditionSource::can_cast(kind) {
            root.has_condition = true;
        } else if !root.has_dynamic_scope
            && let Some(reference) = JsReferenceIdentifier::cast_ref(node)
            && self.is_dynamic_scope_reference(&reference, model)
            && let Some(root) = self.open.last_mut()
        {
            root.has_dynamic_scope = true;
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
        if root.has_dynamic_scope {
            if let Some(parent) = self.open.last_mut() {
                parent.has_dynamic_scope = true;
            }
        } else if root.has_condition
            && !root.is_unsupported
            && self.visited - root.first_node < MAX_ROOT_NODES
            && let Some(root) = AnyJsControlFlowRoot::cast_ref(node)
        {
            self.selected.push(root);
        }
    }

    /// Returns the selected roots, each after its nested roots.
    pub(super) fn selected(&self) -> &[AnyJsControlFlowRoot] {
        &self.selected
    }

    /// Returns whether `reference` may read the global `eval` or the implicit
    /// `arguments` object, including escaped spellings.
    ///
    /// Every reference to `eval` counts: calling a variable named `eval` that
    /// holds the global function is still a direct `eval`. A reference to
    /// `arguments` is an ordinary read only when the semantic model resolves it
    /// to a parameter, catch parameter, `let`, or `const` declared inside the
    /// innermost root that has its own `arguments` object, or anywhere when no
    /// enclosing root has one, such as at the top level of a script. A `var`
    /// starts out as that object, and the object shadows the function's own
    /// name and every declaration outside that root.
    fn is_dynamic_scope_reference(
        &self,
        reference: &JsReferenceIdentifier,
        model: &SemanticModel,
    ) -> bool {
        let Ok(token) = reference.value_token() else {
            return false;
        };
        match unescape_js_identifier(token.text_trimmed()).as_ref() {
            "eval" => true,
            "arguments" => !model.binding(reference).is_some_and(|binding| {
                binding.declaration_kind() == JsDeclarationKind::Value
                    && self
                        .open
                        .iter()
                        .rev()
                        .find(|root| root.has_arguments)
                        .is_none_or(|owner| owner.range.contains_range(binding.range()))
            }),
            _ => false,
        }
    }
}

struct OpenRoot {
    first_node: usize,
    depth: usize,
    range: TextRange,
    /// Whether the root has its own `arguments` object when it runs.
    has_arguments: bool,
    has_condition: bool,
    has_dynamic_scope: bool,
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

    /// Marks the root unsupported when the flow builder does not support
    /// `node`, a node of its own syntax, regardless of its descendants.
    fn check_node(&mut self, node: &JsSyntaxNode, kind: JsSyntaxKind) {
        if self.is_unsupported {
            return;
        }
        self.is_unsupported = kind.is_bogus()
            || kind.is_metavariable()
            || (AnyJsStatement::can_cast(kind)
                && !matches!(
                    kind,
                    JsSyntaxKind::JS_BLOCK_STATEMENT
                        | JsSyntaxKind::JS_BREAK_STATEMENT
                        | JsSyntaxKind::JS_CONTINUE_STATEMENT
                        | JsSyntaxKind::JS_DEBUGGER_STATEMENT
                        | JsSyntaxKind::JS_DO_WHILE_STATEMENT
                        | JsSyntaxKind::JS_EMPTY_STATEMENT
                        | JsSyntaxKind::JS_EXPRESSION_STATEMENT
                        | JsSyntaxKind::JS_FOR_STATEMENT
                        | JsSyntaxKind::JS_FUNCTION_DECLARATION
                        | JsSyntaxKind::JS_IF_STATEMENT
                        | JsSyntaxKind::JS_LABELED_STATEMENT
                        | JsSyntaxKind::JS_RETURN_STATEMENT
                        | JsSyntaxKind::JS_THROW_STATEMENT
                        | JsSyntaxKind::JS_VARIABLE_STATEMENT
                        | JsSyntaxKind::JS_WHILE_STATEMENT
                ))
            // A property signature is only checked as a root, never as part of
            // another root's syntax, because it is itself an execution root.
            || matches!(
                kind,
                JsSyntaxKind::JS_CLASS_EXPRESSION
                    | JsSyntaxKind::JS_CLASS_EXPORT_DEFAULT_DECLARATION
                    | JsSyntaxKind::JS_ARRAY_BINDING_PATTERN
                    | JsSyntaxKind::JS_OBJECT_BINDING_PATTERN
                    | JsSyntaxKind::JS_ARRAY_ASSIGNMENT_PATTERN
                    | JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN
                    | JsSyntaxKind::TS_PROPERTY_SIGNATURE_TYPE_MEMBER
            )
            || JsAssignmentExpression::cast_ref(node).is_some_and(|assignment| {
                matches!(
                    assignment.operator(),
                    Ok(JsAssignmentOperator::LogicalAndAssign
                        | JsAssignmentOperator::LogicalOrAssign
                        | JsAssignmentOperator::NullishCoalescingAssign)
                )
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_js_parser::{JsParserOptions, parse};
    use biome_js_semantic::{SemanticModelOptions, semantic_model};
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
        let model = semantic_model(root, SemanticModelOptions::default());
        let mut scanner = FlowRootScanner::default();
        for event in root.syntax().preorder() {
            match event {
                WalkEvent::Enter(node) => scanner.enter(&node, &model),
                WalkEvent::Leave(node) => scanner.leave(&node),
            }
        }
        scanner
            .selected()
            .iter()
            .map(|root| root.syntax().text_trimmed().to_string())
            .collect()
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
    fn unsupported_syntax_rejects_only_its_own_root() {
        for body in [
            "try { read(x); } catch {}",
            "try { read(x); } finally {}",
            "switch (x) { case 1: read(x); }",
            "for (const key in x) read(key);",
            "for (const item of x) read(item);",
            "x &&= read(x);",
            "x ||= read(x);",
            "x ??= read(x);",
            "const { value = read(x) } = x;",
            "class C { field = read(x); }",
        ] {
            let source = format!("function f(x) {{ if (x) read(x); {body} }}");
            assert!(selected(&source).is_empty(), "{body}");
            let source =
                format!("function f(x) {{ if (x) read(x); const g = () => {{ {body} }}; }}");
            assert_eq!(selected(&source).len(), 1, "{body}");
        }
    }

    #[test]
    fn type_declarations_are_not_inspected() {
        let source = "function f(x: string | null) { interface I { eval: { [K in keyof typeof arguments]: T } } type T = typeof eval; if (x) read(x); }";
        assert_eq!(selected(source).len(), 1);
    }

    #[test]
    fn references_in_types_are_not_dynamic_scope() {
        for source in [
            "function f(x: typeof eval, y: arguments) { if (x) read(y); }",
            "function f(x): asserts arguments { if (x) read(x); }",
        ] {
            assert_eq!(selected(source).len(), 1, "{source}");
        }
        let source = "function f(x) { if (x) read(arguments); }";
        assert!(selected(source).is_empty());
    }

    /// Returns how many roots are selected in the sloppy-mode script `source`,
    /// the only kind of source that can declare `eval` or `arguments`.
    fn selected_in_script(source: &str) -> usize {
        let parsed = parse(
            source,
            JsFileSource::js_script(),
            JsParserOptions::default(),
        );
        assert!(!parsed.has_errors(), "{source}");
        selected_in(&parsed.tree()).len()
    }

    #[test]
    fn every_eval_reference_disables_flow() {
        for source in [
            "function f(x) { if (x) eval(x); }",
            "function f(x) { var eval = read; if (x) eval(x); }",
            "var eval; function f(x) { if (x) eval(x); }",
        ] {
            assert_eq!(selected_in_script(source), 0, "{source}");
        }
    }

    #[test]
    fn arguments_declared_in_its_own_function_is_an_ordinary_read() {
        for (source, expected) in [
            ("function f(x, arguments) { if (x) read(arguments); }", 1),
            (
                "function f(x) { let arguments = x; if (x) read(arguments); }",
                1,
            ),
            (
                "function f(x, arguments) { const g = () => arguments; if (x) read(x); }",
                1,
            ),
            (
                "function f(x) { const arguments = x; if (x) read(arguments); }",
                1,
            ),
            (
                "function f(x) { try {} catch (arguments) { const g = () => { if (x) read(arguments); }; } }",
                1,
            ),
            ("let arguments = 0; if (x) read(arguments);", 1),
            (
                "function f(x) { var arguments; if (x) read(arguments); }",
                0,
            ),
            ("function arguments(x) { if (x) read(arguments); }", 0),
            ("(function arguments(x) { if (x) read(arguments); });", 0),
            (
                "function f(x) { function arguments() {} if (x) read(arguments); }",
                0,
            ),
            (
                "function outer(arguments) { function f(x) { if (x) read(arguments); } }",
                0,
            ),
            ("function f(x) { if (x) \\u0061rguments; }", 0),
        ] {
            assert_eq!(selected_in_script(source), expected, "{source}");
        }
    }

    #[test]
    fn namespaces_are_roots_without_flow() {
        const SOURCE: &str =
            "let x: string | null = null; if (x) read(x); namespace N { if (x) read(x); }";
        assert_eq!(selected(SOURCE), [SOURCE]);
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
