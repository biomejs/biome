use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, QueryMatch, Rule, RuleDiagnostic, RuleSource,
    RuleSourceKind,
};
use biome_console::markup;
use biome_js_syntax::{
    JsAwaitExpression, JsDoWhileStatement, JsForInStatement, JsForOfStatement, JsForStatement,
    JsForVariableDeclaration, JsSyntaxKind, JsSyntaxNode, JsVariableDeclaration, JsWhileStatement,
    JsWithStatement,
};
use biome_rowan::{declare_node_union, AstNode, WalkEvent};

declare_lint_rule! {
    /// Disallow `await` inside loops.
    ///
    /// Using `await` in a loop makes your asynchronous operations run one after another instead of all at once. This can slow things down and might cause unhandled errors. Instead, create all the promises together and then wait for them simultaneously using methods like `Promise.all()`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// async function invalid() {
    ///     for (const thing of things) {
    ///         const result = await asyncWork();
    ///     }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// async function valid() {
    ///     await Promise.all(things.map((thing) => asyncWork(thing)))
    /// }
    /// ```
    ///
    pub NoAwaitInLoop {
        version: "next",
        name: "noAwaitInLoop",
        language: "js",
        sources: &[RuleSource::Eslint("no-await-in-loop")],
        source_kind: RuleSourceKind::SameLogic,
        recommended: false,
    }
}

declare_node_union! {
    pub AnyLoopNode = JsForStatement | JsForInStatement | JsForOfStatement | JsWhileStatement | JsDoWhileStatement | JsWithStatement
}

impl Rule for NoAwaitInLoop {
    type Query = Ast<AnyLoopNode>;
    type State = JsSyntaxNode;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let loop_node = ctx.query();

        // skip "for await ... of"
        if let AnyLoopNode::JsForOfStatement(for_of) = loop_node {
            if for_of.await_token().is_some() {
                return None;
            }
        }

        let mut preorder = loop_node.syntax().preorder();
        while let Some(event) = preorder.next() {
            match event {
                WalkEvent::Enter(node) => {
                    if is_boundary(&node) {
                        preorder.skip_subtree();
                    }

                    // skip valid case: for await
                    // e.g. `while (baz) { for await (x of xs)}`
                    if let Some(for_of) = JsForOfStatement::cast(node.clone()) {
                        if for_of.await_token().is_some() {
                            return Some(node);
                        }
                    }

                    // skip valid case: binding in `for`
                    // e.g. `async function foo() { for (var i = await bar; i < n; i++) {  } }`
                    if JsVariableDeclaration::can_cast(node.kind()) {
                        if let Some(parent) = node.parent() {
                            if JsForStatement::can_cast(parent.kind()) {
                                preorder.skip_subtree();
                            }
                        }
                    }

                    // skip valid case: bidning in `for in`
                    // `async function foo() { for (var bar = await baz in qux) {} }`
                    if JsForVariableDeclaration::can_cast(node.kind()) {
                        preorder.skip_subtree();
                    }

                    if JsAwaitExpression::can_cast(node.kind()) {
                        // skip valid cases: expression in `for in` and `for of`
                        // - `async function foo() { for (var bar in await baz) { } }`
                        // - `async function foo() { for (var bar of await baz) { } }`
                        if let Some(parent) = node.parent() {
                            if JsForOfStatement::can_cast(parent.kind())
                                || JsForInStatement::can_cast(parent.kind())
                            {
                                continue;
                            }
                        }
                        return Some(node);
                    }
                }
                WalkEvent::Leave(_) => {}
            }
        }
        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.text_range(),
                markup! { "Avoid using "<Emphasis>"await"</Emphasis>" inside loops." },
            )
            .note(markup! {
                "Using "<Emphasis>"await"</Emphasis>" inside loops might cause performance issues or unintended sequential execution, consider use "<Emphasis>"Promise.all()"</Emphasis>" instead."
            })
        )
    }
}

/// check whether it should stop traversing ancestors at the given node
fn is_boundary(node: &JsSyntaxNode) -> bool {
    let kind = node.kind();
    matches!(
        kind,
        JsSyntaxKind::JS_FUNCTION_DECLARATION
            | JsSyntaxKind::JS_FUNCTION_EXPRESSION
            | JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION
            | JsSyntaxKind::JS_METHOD_CLASS_MEMBER
    )
}
