use super::builder::{MAX_DEPTH, has_operands, mutated_array};
use biome_js_control_flow::AnyJsControlFlowRoot;
use biome_js_syntax::{
    AnyJsBinding, AnyJsBindingPattern, AnyJsExpression, AnyJsForInOrOfInitializer,
    AnyJsOptionalChainExpression, AnyTsReturnType, AnyTsType, JsAssignmentExpression,
    JsAssignmentOperator, JsCallExpression, JsConditionalExpression, JsDoWhileStatement,
    JsForInStatement, JsForOfStatement, JsForStatement, JsIdentifierBinding, JsIfStatement,
    JsLogicalExpression, JsPostUpdateExpression, JsPreUpdateExpression, JsSwitchStatement,
    JsSyntaxKind, JsSyntaxNode, JsUnaryExpression, JsUnaryOperator, JsVariableDeclarator,
    JsWhileStatement, TsDeclareFunctionDeclaration,
};
use biome_rowan::{AstNode, AstSeparatedList, SyntaxKind, declare_node_union};
use std::ops::Range;

/// Limits the syntax nodes, including nested roots, of a root that gets flow.
pub(super) const MAX_ROOT_NODES: usize = 16_384;

declare_node_union! {
    /// Syntax that tests a condition along some path. Every `switch` and
    /// every `for...in` loop counts, whatever its discriminant or expression,
    /// because one that can't narrow is rare. A `for...in` loop tests that
    /// its expression is not nullish when its head declares a variable.
    AnyFlowConditionSource = JsIfStatement
        | JsWhileStatement
        | JsDoWhileStatement
        | JsForStatement
        | JsForInStatement
        | JsSwitchStatement
        | JsLogicalExpression
        | JsConditionalExpression
}

declare_node_union! {
    /// Syntax that writes a variable or a property at a write point of the
    /// flow graph: an assignment, `++`, and `--`. Other writes need a check
    /// beyond their syntax kind; see [`is_write`].
    AnyFlowWriteSource = JsAssignmentExpression | JsPreUpdateExpression | JsPostUpdateExpression
}

/// Selects the execution roots that may need flow while the module visitor
/// walks the syntax tree, so collection never walks the module again.
///
/// The visitor must report every node of the module in preorder through
/// [`Self::enter`] and [`Self::leave`]. A root is selected when it directly
/// contains a condition, a write that [`AnyFlowWriteSource`] describes, or a
/// statement call that may assert, outside its nested roots, has fewer than
/// [`MAX_ROOT_NODES`] syntax nodes, and uses only syntax the flow builder
/// supports. A root selected only by statement calls keeps them. Collection,
/// which has the semantic model, then builds the root only when one of their
/// callees can assert.
///
/// The builder treats statements it does not model, such as a class
/// declaration, as opaque regions, so they do not reject a root. A root is not
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
    /// The roots that may need flow, in the order in which the walk leaves
    /// them, so a nested root comes before the roots that enclose it.
    selected: Vec<SelectedRoot>,
    /// The bindings of the selected roots.
    kept_bindings: Vec<JsIdentifierBinding>,
    /// The statement calls of the open roots that may assert, each root's
    /// after those of the roots that enclose it.
    calls: Vec<JsCallExpression>,
    /// The statement calls of the roots selected only by them.
    kept_calls: Vec<JsCallExpression>,
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
    /// For a root that only statement calls select, the range of those calls
    /// in [`FlowRootScanner::kept_calls`].
    calls: Option<Range<usize>>,
}

impl FlowRootScanner {
    pub(crate) fn enter(&mut self, node: &JsSyntaxNode) {
        self.visited += 1;
        self.depth += 1;
        let kind = node.kind();
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
                first_call: self.calls.len(),
                depth: self.depth,
                has_condition: false,
                has_write: false,
                has_array_mutation: false,
                has_empty_array: false,
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
        if let Some(call) = JsCallExpression::cast_ref(node) {
            root.has_array_mutation |= mutated_array(&call).is_some();
            if is_statement_call(&call) && may_assert(&call) {
                self.calls.push(call);
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
        } else if !root.has_condition && is_evaluation_condition(node) {
            root.has_condition = true;
        } else if !root.has_write && is_write(node) {
            root.has_write = true;
        } else if let Some(declarator) = JsVariableDeclarator::cast_ref(node) {
            root.has_empty_array |= declarator.variable_annotation().is_none()
                && declarator
                    .initializer()
                    .and_then(|initializer| initializer.expression().ok())
                    .is_some_and(|expression| {
                        matches!(expression, AnyJsExpression::JsArrayExpression(array) if array.elements().is_empty())
                    });
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
        let has_write = root.has_write || (root.has_array_mutation && root.has_empty_array);
        let has_calls = self.calls.len() > root.first_call;
        if (root.has_condition || has_write || has_calls)
            && !root.is_unsupported
            && self.visited - root.first_node < MAX_ROOT_NODES
            && let Some(node) = AnyJsControlFlowRoot::cast_ref(node)
        {
            let start = self.kept_bindings.len();
            self.kept_bindings
                .extend(self.bindings.drain(root.first_binding..));
            let calls = (!root.has_condition && !has_write).then(|| {
                let start = self.kept_calls.len();
                self.kept_calls.extend(self.calls.drain(root.first_call..));
                start..self.kept_calls.len()
            });
            self.selected.push(SelectedRoot {
                root: node,
                bindings: start..self.kept_bindings.len(),
                has_this: root.has_this,
                calls,
            });
        }
        self.bindings.truncate(root.first_binding);
        self.calls.truncate(root.first_call);
    }

    /// Returns the selected roots, each nested root before the roots that
    /// enclose it.
    pub(super) fn selected(&self) -> impl Iterator<Item = &SelectedRoot> {
        self.selected.iter()
    }

    /// Returns the statement calls of `root` that may assert, when only they
    /// select it, or `None` when a condition or a write of its own code does.
    pub(super) fn statement_calls(&self, root: &SelectedRoot) -> Option<&[JsCallExpression]> {
        root.calls.clone().map(|calls| &self.kept_calls[calls])
    }

    /// Returns the bindings that the own code of `root` declares, outside
    /// nested roots and types, in source order.
    pub(super) fn bindings(&self, root: &SelectedRoot) -> &[JsIdentifierBinding] {
        &self.kept_bindings[root.bindings.clone()]
    }
}

/// Returns whether `node` evaluates part of an expression only on some paths
/// without being an [`AnyFlowConditionSource`]. A logical assignment runs its
/// right operand depending on the value of its target. A `?.` that a key or
/// arguments follow in its chain runs them only when its object is not
/// nullish. TypeScript treats both as conditions.
fn is_evaluation_condition(node: &JsSyntaxNode) -> bool {
    if let Some(assignment) = JsAssignmentExpression::cast_ref(node) {
        return assignment.operator().is_ok_and(|operator| {
            matches!(
                operator,
                JsAssignmentOperator::LogicalAndAssign
                    | JsAssignmentOperator::LogicalOrAssign
                    | JsAssignmentOperator::NullishCoalescingAssign
            )
        });
    }
    let Some(mut link) = AnyJsOptionalChainExpression::cast_ref(node) else {
        return false;
    };
    if !link.is_optional() {
        return false;
    }
    // The links that follow `node` in its chain enclose it as their object.
    loop {
        if has_operands(&link) {
            return true;
        }
        let Some(outer) = link
            .syntax()
            .parent()
            .and_then(AnyJsOptionalChainExpression::cast)
        else {
            return false;
        };
        if !outer
            .object()
            .is_ok_and(|object| object.syntax() == link.syntax())
        {
            return false;
        }
        link = outer;
    }
}

/// Returns whether `call` is a statement call, one that TypeScript adds to
/// the flow: the expression of an expression statement, or an operand of the
/// comma operator there, through any parentheses around the comma
/// expression. A call in parentheses is none, as in `(assert(x));`.
fn is_statement_call(call: &JsCallExpression) -> bool {
    let mut parent = call.syntax().parent();
    let mut in_sequence = false;
    while let Some(node) = parent {
        match node.kind() {
            JsSyntaxKind::JS_EXPRESSION_STATEMENT => return true,
            JsSyntaxKind::JS_SEQUENCE_EXPRESSION => in_sequence = true,
            JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION if in_sequence => {}
            _ => return false,
        }
        parent = node.parent();
    }
    false
}

/// Returns whether a statement call `call` may call an assertion, as far as
/// its syntax tells: its callee, in parentheses or not, is an identifier,
/// `this`, or a member access whose object is one of these or another such
/// member access, and it passes an argument or reads its callee from an
/// object, which `asserts this` narrows.
fn may_assert(call: &JsCallExpression) -> bool {
    let Ok(callee) = call.callee().map(AnyJsExpression::omit_parentheses) else {
        return false;
    };
    let is_member = matches!(callee, AnyJsExpression::JsStaticMemberExpression(_));
    let mut expression = callee;
    for _ in 0..MAX_DEPTH {
        expression = match expression {
            AnyJsExpression::JsIdentifierExpression(_) | AnyJsExpression::JsThisExpression(_) => {
                return is_member
                    || call
                        .arguments()
                        .is_ok_and(|arguments| !arguments.args().is_empty());
            }
            AnyJsExpression::JsParenthesizedExpression(parenthesized) => {
                match parenthesized.expression() {
                    Ok(expression) => expression,
                    Err(_) => return false,
                }
            }
            AnyJsExpression::JsStaticMemberExpression(member) => match member.object() {
                Ok(object) => object,
                Err(_) => return false,
            },
            _ => return false,
        };
    }
    false
}

/// Returns whether `node` may write a variable or a property at a write
/// point of the flow graph. That is the case for an [`AnyFlowWriteSource`],
/// `delete`, the head of a `for...of` loop that assigns a target, and a
/// declarator with a type annotation and an initializer, when the annotation
/// is a union or the declarator destructures a pattern.
fn is_write(node: &JsSyntaxNode) -> bool {
    if AnyFlowWriteSource::can_cast(node.kind()) {
        return true;
    }
    if let Some(unary) = JsUnaryExpression::cast_ref(node) {
        return unary.operator() == Ok(JsUnaryOperator::Delete);
    }
    if let Some(declarator) = JsVariableDeclarator::cast_ref(node) {
        let Some(annotation) = declarator
            .variable_annotation()
            .and_then(|annotation| annotation.type_annotation().ok().flatten())
        else {
            return false;
        };
        return declarator.initializer().is_some()
            && (annotation
                .ty()
                .is_ok_and(|ty| matches!(ty, AnyTsType::TsUnionType(_)))
                || declarator
                    .id()
                    .is_ok_and(|id| !matches!(id, AnyJsBindingPattern::AnyJsBinding(_))));
    }
    JsForOfStatement::cast_ref(node).is_some_and(|statement| {
        matches!(
            statement.initializer(),
            Ok(AnyJsForInOrOfInitializer::AnyJsAssignmentPattern(_))
        )
    })
}

struct OpenRoot {
    first_node: usize,
    /// Index in [`FlowRootScanner::bindings`] of the first binding the root
    /// declares.
    first_binding: usize,
    /// Index in [`FlowRootScanner::calls`] of the first statement call of
    /// the root's own code.
    first_call: usize,
    depth: usize,
    has_condition: bool,
    /// Whether the root's own code writes a variable or a property, as
    /// [`is_write`] decides it.
    has_write: bool,
    /// Whether the root's own code adds values to an array that a variable
    /// holds, as `a.push(v)` does, which writes the variable when its type
    /// evolves with its writes.
    has_array_mutation: bool,
    /// Whether the root's own code declares a variable without a type
    /// annotation that an empty array literal initializes, whose type may
    /// evolve with its writes.
    has_empty_array: bool,
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

    /// Returns the texts of the roots that a condition or a write selects in
    /// the TypeScript `source`, in selection order.
    fn selected(source: &str) -> Vec<String> {
        let parsed = parse(source, JsFileSource::ts(), JsParserOptions::default());
        assert!(!parsed.has_errors(), "{source}");
        selected_in(&parsed.tree())
    }

    /// Returns the texts of the roots that a condition or a write selects in
    /// `root`, in selection order.
    fn selected_in(root: &AnyJsRoot) -> Vec<String> {
        let scanner = scan(root);
        scanner
            .selected()
            .filter(|selected| scanner.statement_calls(selected).is_none())
            .map(|selected| selected.root.syntax().text_trimmed().to_string())
            .collect()
    }

    /// Returns the texts of the roots that only statement calls select in the
    /// TypeScript `source`, each with the texts of those calls.
    fn selected_by_calls(source: &str) -> Vec<(String, Vec<String>)> {
        let parsed = parse(source, JsFileSource::ts(), JsParserOptions::default());
        assert!(!parsed.has_errors(), "{source}");
        let scanner = scan(&parsed.tree());
        scanner
            .selected()
            .filter_map(|selected| {
                let calls = scanner.statement_calls(selected)?;
                Some((
                    selected.root.syntax().text_trimmed().to_string(),
                    calls
                        .iter()
                        .map(|call| call.syntax().text_trimmed().to_string())
                        .collect(),
                ))
            })
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
    fn switch_statements_and_for_in_loops_are_conditions() {
        for source in [
            "function f(x) { switch (x) { case 1: read(x); } }",
            "function f(x) { switch (g()) { default: read(x); } }",
            "function f(x) { for (const key in x) read(x); }",
            "function f(x) { for (key in g()) read(x); }",
        ] {
            assert_eq!(selected(source), [source], "{source}");
        }
        // A `for...of` loop tests nothing.
        assert!(selected("function f(x) { for (const item of x) read(item); }").is_empty());
    }

    #[test]
    fn logical_assignments_and_optional_links_before_operands_are_conditions() {
        for source in [
            "function f(x) { x ??= read(x); }",
            "function f(x) { x.v ||= read(x); }",
            "function f(x) { x &&= read(x); }",
            "function f(x) { x?.read(x); }",
            "function f(x) { x?.[read(x)]; }",
            "function f(x) { x?.(x); }",
            "function f(x) { x?.a.b.read(x); }",
            "function f(x) { x.a?.b[read(x)].c; }",
        ] {
            assert_eq!(selected(source), [source], "{source}");
        }
        // A `?.` that no key or arguments follow evaluates nothing more, and
        // parentheses end a chain.
        for source in [
            "function f(x) { x?.a.b; }",
            "function f(x) { x?.read(); }",
            "function f(x) { (x?.a).read(x); }",
            "function f(x) { read(x?.a, x); }",
        ] {
            assert!(selected(source).is_empty(), "{source}");
        }
    }

    #[test]
    fn writes_select_their_root() {
        for source in [
            "function f(x) { x = read(); }",
            "function f(x) { x += read(x); }",
            "function f(o) { o.v++; }",
            "function f(o) { --o.v; }",
            "function f(o) { delete o.v; }",
            "function f(x) { [x] = read(); }",
            "function f(x) { for (x of read()) {} }",
            "function f() { const x: string | number = read(); }",
            "function f() { const { x }: { x: string | number } = read(); }",
        ] {
            assert_eq!(selected(source), [source], "{source}");
        }
        // A declarator without a union annotation or without an initializer,
        // a declared loop variable, and a write in a nested root don't.
        for source in [
            "function f() { const x: string = read(); }",
            "function f() { let x: string | number; }",
            "function f() { for (const x of read()) {} }",
            "function f() { const x = read(); }",
            "function f(x) { return () => 0; }",
        ] {
            assert!(selected(source).is_empty(), "{source}");
        }
        assert_eq!(
            selected("function f(x) { const g = () => { x = 1; }; }"),
            ["() => { x = 1; }"]
        );
    }

    #[test]
    fn pushes_select_a_root_that_declares_an_empty_array() {
        for source in [
            "function f(v) { const a = []; a.push(v); }",
            "function f(v) { let a = []; a.unshift(v); }",
            "function f(v) { var a = []; (a)!.push(v); }",
        ] {
            assert_eq!(selected(source), [source], "{source}");
        }
        // The empty array and the call must both be in the root's own code,
        // and the callee must name `push` or `unshift` on a variable.
        for source in [
            "function f(a, v) { a.push(v); }",
            "function f(v) { const a: number[] = []; a.push(v); }",
            "function f(v) { const a = [v]; a.push(v); }",
            "function f(v) { const a = []; a.pop(v); }",
            "function f(v) { const a = []; a[\"push\"](v); }",
            "function f(v) { const a = []; return () => a.push(v); }",
        ] {
            assert!(selected(source).is_empty(), "{source}");
        }
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
    fn statement_calls_that_may_assert_select_their_root() {
        for (body, calls) in [
            ("check(x);", &["check(x)"][..]),
            ("(check)(x);", &["(check)(x)"]),
            ("a.b.check(x);", &["a.b.check(x)"]),
            ("this.check();", &["this.check()"]),
            ("(0, check(x));", &["check(x)"]),
            ("check(x), other(x);", &["check(x)", "other(x)"]),
            ("((check(x), 0));", &["check(x)"]),
        ] {
            let source = format!("function f(x) {{ {body} }}");
            assert_eq!(
                selected_by_calls(&source),
                [(
                    source.clone(),
                    calls.iter().map(|call| (*call).to_owned()).collect()
                )],
                "{body}"
            );
        }
        // A call in parentheses, a call that is no statement, a callee that
        // is no dotted name, and a call of a name without arguments can't
        // assert anything.
        for body in [
            "(check(x));",
            "((check)(x));",
            "void check(x);",
            "const y = check(x);",
            "(0, (check(x)));",
            "a[0](x);",
            "get()(x);",
            "check();",
        ] {
            let source = format!("function f(x) {{ {body} }}");
            assert!(selected_by_calls(&source).is_empty(), "{body}");
        }
        // A root that a condition selects keeps no calls of its own.
        assert!(selected_by_calls("function f(x) { check(x); if (x) {} }").is_empty());
    }

    #[test]
    fn statements_other_than_with_keep_their_root() {
        for body in [
            "try { read(x); } catch (error) { read(error); }",
            "try { read(x); } finally { read(x); }",
            "switch (x) { case 1: read(x); break; default: read(x); }",
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
