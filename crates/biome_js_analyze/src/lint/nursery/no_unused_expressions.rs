use std::borrow::Cow;

use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsExpression, AnyJsFunction, AnyJsLiteralExpression, AnyJsRoot, JsBlockStatement,
    JsExpressionStatement, JsSyntaxNode, JsUnaryOperator, TsModuleBlock,
};
use biome_rowan::{AstNode, SyntaxResult};
use biome_rule_options::no_unused_expressions::NoUnusedExpressionsOptions;

declare_lint_rule! {
    /// Disallow expression statements that are neither a function call nor an
    /// assignment.
    ///
    /// When an expression is used as a statement, it should be explicitly clear
    /// what the intention behind the expression is. This is clear for function
    /// calls and assignments, because the call or the assignment itself is the
    /// primary intention behind the statement. For other expression kinds, the
    /// intention is much more ambiguous; it could be the expression contains
    /// side-effects that are not very explicit, but it could also be that it is
    /// an error where the author forgot to use the result of the expression,
    /// such as a forgotten `return` keyword, or it could point to a function
    /// that the author forgot to call.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// 0
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if(0) 0
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// {0}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// f(0), {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// a && b()
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// a, b()
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// c = a, b
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// a() && function namedFunctionInExpressionContext () {f();}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// (function anIncompleteIIFE () {});
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// injectGlobal`body{ color: red; }`
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// Set<number>
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// 1 as number
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// window!
    /// ```
    ///
    /// JSX expressions are considered invalid when used as a statement too:
    ///
    /// ```jsx,expect_diagnostic
    /// <MyComponent />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <></>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// {} // In this context, this is a block statement, not an object literal
    ///
    /// { myLabel: foo() } // In this context, this is a block statement with a label and expression, not an object literal
    ///
    /// function namedFunctionDeclaration () {}
    ///
    /// (function aGenuineIIFE () {}());
    ///
    /// f()
    ///
    /// a = 0
    ///
    /// new C
    ///
    /// delete a.b
    ///
    /// void a
    /// ```
    ///
    /// ### Handling of Directives
    ///
    /// Any stand-alone string at the start of a script, module, or function is
    /// considered a directive and is therefore allowed.
    ///
    /// ```js
    /// "use strict";
    /// "use asm"
    /// "use stricter";
    /// "use babel"
    /// "any other strings like this in the directive prologue";
    /// "this is still the directive prologue";
    ///
    /// function foo() {
    ///     "bar";
    /// }
    ///
    /// class Foo {
    ///     someMethod() {
    ///         "use strict";
    ///     }
    /// }
    /// ```
    ///
    /// The following are **not** considered valid directives:
    ///
    /// ```js,expect_diagnostic
    /// doSomething();
    /// "use strict"; // this isn't in a directive prologue, because there is a non-directive statement before it
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function foo() {
    ///     "bar" + 1;
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// class Foo {
    ///     static {
    ///         "use strict"; // class static blocks do not have directive prologues
    ///     }
    /// }
    /// ```
    pub NoUnusedExpressions {
        version: "2.2.5",
        name: "noUnusedExpressions",
        language: "js",
        sources: &[RuleSource::Eslint("no-unused-expressions").same()],
        recommended: false,
    }
}

impl Rule for NoUnusedExpressions {
    type Query = Ast<JsExpressionStatement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoUnusedExpressionsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if is_directive(node) {
            return None;
        }

        is_disallowed(&node.expression().ok()?).ok()?.then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Expected an assignment or function call but found an expression instead."
                },
            )
            .note(markup! {
                "This expression may be unintentionally unused or it might be a function that you forgot to call."
            }),
        )
    }
}

/// Determines if a node should be treated as a directive.
///
/// Only string literals are treated as directives, and only if they are not
/// preceeded by any non-directive nodes.
fn is_directive(node: &JsExpressionStatement) -> bool {
    if node
        .syntax()
        .parent()
        .is_some_and(|parent| !is_at_top_level(&parent))
    {
        return false;
    }

    let mut node = Cow::Borrowed(node);
    loop {
        if !looks_like_directive(&node) {
            break false;
        }

        let Some(prev_node) = node.syntax().prev_sibling() else {
            break true;
        };
        match JsExpressionStatement::cast(prev_node) {
            Some(prev_expr) => node = Cow::Owned(prev_expr),
            None => break false,
        }
    }
}

/// Determines if a node exists at the top-level based on its `parent` node.
fn is_at_top_level(parent: &JsSyntaxNode) -> bool {
    AnyJsRoot::can_cast(parent.kind())
        || TsModuleBlock::can_cast(parent.kind())
        || JsBlockStatement::cast_ref(parent).is_some_and(|block| {
            block
                .syntax()
                .parent()
                .is_some_and(|parent: JsSyntaxNode| AnyJsFunction::can_cast(parent.kind()))
        })
}

fn looks_like_directive(node: &JsExpressionStatement) -> bool {
    matches!(
        node.expression(),
        Ok(AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsStringLiteralExpression(_)
        ))
    )
}

fn is_disallowed(expr: &AnyJsExpression) -> SyntaxResult<bool> {
    let is_disallowed = match expr {
        AnyJsExpression::AnyJsLiteralExpression(_)
        | AnyJsExpression::JsArrayExpression(_)
        | AnyJsExpression::JsArrowFunctionExpression(_)
        | AnyJsExpression::JsBinaryExpression(_)
        | AnyJsExpression::JsClassExpression(_)
        | AnyJsExpression::JsComputedMemberExpression(_)
        | AnyJsExpression::JsConditionalExpression(_)
        | AnyJsExpression::JsFunctionExpression(_)
        | AnyJsExpression::JsIdentifierExpression(_)
        | AnyJsExpression::JsImportMetaExpression(_)
        | AnyJsExpression::JsInExpression(_)
        | AnyJsExpression::JsInstanceofExpression(_)
        | AnyJsExpression::JsLogicalExpression(_)
        | AnyJsExpression::JsNewTargetExpression(_)
        | AnyJsExpression::JsObjectExpression(_)
        | AnyJsExpression::JsSequenceExpression(_)
        | AnyJsExpression::JsStaticMemberExpression(_)
        | AnyJsExpression::JsSuperExpression(_)
        | AnyJsExpression::JsTemplateExpression(_)
        | AnyJsExpression::JsThisExpression(_)
        | AnyJsExpression::JsxTagExpression(_) => true,
        AnyJsExpression::JsAwaitExpression(_)
        | AnyJsExpression::JsAssignmentExpression(_)
        | AnyJsExpression::JsCallExpression(_)
        | AnyJsExpression::JsImportCallExpression(_)
        | AnyJsExpression::JsNewExpression(_)
        | AnyJsExpression::JsYieldExpression(_)
        | AnyJsExpression::JsPostUpdateExpression(_)
        | AnyJsExpression::JsPreUpdateExpression(_)
        | AnyJsExpression::JsBogusExpression(_)
        | AnyJsExpression::JsMetavariable(_) => false,
        AnyJsExpression::JsParenthesizedExpression(expr) => is_disallowed(&expr.expression()?)?,
        AnyJsExpression::JsUnaryExpression(expr) => match expr.operator()? {
            JsUnaryOperator::BitwiseNot
            | JsUnaryOperator::LogicalNot
            | JsUnaryOperator::Minus
            | JsUnaryOperator::Plus
            | JsUnaryOperator::Typeof => true,
            JsUnaryOperator::Delete | JsUnaryOperator::Void => false,
        },
        AnyJsExpression::TsAsExpression(expr) => is_disallowed(&expr.expression()?)?,
        AnyJsExpression::TsInstantiationExpression(expr) => is_disallowed(&expr.expression()?)?,
        AnyJsExpression::TsNonNullAssertionExpression(expr) => is_disallowed(&expr.expression()?)?,
        AnyJsExpression::TsSatisfiesExpression(expr) => is_disallowed(&expr.expression()?)?,
        AnyJsExpression::TsTypeAssertionExpression(expr) => is_disallowed(&expr.expression()?)?,
    };
    Ok(is_disallowed)
}
