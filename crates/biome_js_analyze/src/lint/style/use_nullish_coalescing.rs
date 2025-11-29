use crate::JsRuleAction;
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, JsCallArgumentList, JsCallExpression,
    JsConditionalExpression, JsForStatement, JsIfStatement, JsLogicalExpression, JsLogicalOperator,
    JsSyntaxKind, JsUnaryExpression, JsUnaryOperator, JsWhileStatement, T,
};
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt};
use biome_rule_options::use_nullish_coalescing::UseNullishCoalescingOptions;

declare_lint_rule! {
    /// Enforce using nullish coalescing operator (`??`) instead of logical or (`||`)
    /// when providing default values with safe fallbacks.
    ///
    /// The `??` operator only checks for `null` and `undefined`, while `||` checks
    /// for any falsy value including `0`, `''`, and `false`. Using `??` prevents bugs
    /// where legitimate falsy values are incorrectly treated as missing.
    ///
    /// This rule only suggests replacement when the fallback value is "obviously safe":
    /// values that are never falsy themselves (like objects, arrays, and non-empty strings).
    /// This avoids false positives where `||` behavior with other falsy values might be intentional.
    ///
    /// ## Safe fallbacks (will be reported):
    /// - Object literals: `x || {}`
    /// - Array literals: `x || []`
    /// - Non-empty string literals: `x || 'default'`
    /// - Function/class expressions: `x || function() {}`
    /// - `new` expressions: `x || new Map()`
    /// - Member access: `x || defaults.value`
    ///
    /// ## Unsafe fallbacks (will NOT be reported):
    /// - Numeric literals: `x || 0` (0 is falsy)
    /// - Empty strings: `x || ''` (empty string is falsy)
    /// - Boolean literals: `x || false` (false is falsy)
    /// - Identifiers: `x || fallback` (unknown type)
    /// - Function calls: `x || getData()` (unknown return type)
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const config = options || {};
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const name = input || 'Anonymous';
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// // Already using ??
    /// const config = options ?? {};
    /// ```
    ///
    /// ```js
    /// // Numeric fallback - could be intentional for 0
    /// const count = value || 0;
    /// ```
    ///
    /// ```js
    /// // Boolean fallback - could be intentional for false
    /// const enabled = setting || false;
    /// ```
    ///
    /// ```js
    /// // Unknown fallback type - too ambiguous
    /// const data = primary || secondary;
    /// ```
    ///
    /// ## Options
    ///
    /// The rule provides the option described below.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "ignoreConditionalTests": true
    ///     }
    /// }
    /// ```
    ///
    /// ### ignoreConditionalTests
    ///
    /// When set to `true` (default), the rule ignores `||` in test positions
    /// (if, while, for, ternary conditions) where falsy-check behavior is often intentional.
    ///
    pub UseNullishCoalescing {
        version: "next",
        name: "useNullishCoalescing",
        language: "js",
        sources: &[RuleSource::EslintTypeScript("prefer-nullish-coalescing").same()],
        recommended: false,
        severity: Severity::Information,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseNullishCoalescing {
    type Query = biome_analyze::Ast<JsLogicalExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseNullishCoalescingOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let logical = ctx.query();

        // Only check || operators
        let operator = logical.operator().ok()?;
        if operator != JsLogicalOperator::LogicalOr {
            return None;
        }

        // Check options for conditional test position
        let options = ctx.options();
        if options.ignore_conditional_tests && is_in_test_position(logical) {
            return None;
        }

        // Skip if in boolean coercion context (Boolean(), !!, !)
        if is_in_boolean_context(logical) {
            return None;
        }

        // Skip if part of mixed logical chain (a || b && c)
        if is_in_mixed_logical_chain(logical) {
            return None;
        }

        // Check if the right-hand side is a "safe" default value
        let right = logical.right().ok()?;
        if !is_safe_default_value(&right) {
            return None;
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.operator_token().ok()?.text_trimmed_range(),
                markup! {
                    "Prefer "<Emphasis>"??"</Emphasis>" over "<Emphasis>"||"</Emphasis>" for default values."
                },
            )
            .note(markup! {
                "Using ?? ensures that only null and undefined trigger the fallback, "
                "preserving intentional falsy values like 0 or empty strings."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        // Replace || token with ??
        let old_token = node.operator_token().ok()?;
        let new_token = make::token(T![??])
            .with_leading_trivia_pieces(old_token.leading_trivia().pieces())
            .with_trailing_trivia_pieces(old_token.trailing_trivia().pieces());

        mutation.replace_token(old_token, new_token);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use "<Emphasis>"??"</Emphasis>" instead." }.to_owned(),
            mutation,
        ))
    }
}

/// Checks if a logical expression is in a test position (if/while/for/ternary condition).
///
/// Returns `true` if the expression is used as the condition in a control flow statement,
/// where the truthiness check is the primary purpose rather than value coalescing.
///
/// ## Examples
/// ```js
/// if (foo || bar) { }           // Returns true (test position)
/// while (foo || bar) { }         // Returns true (test position)
/// for (; foo || bar; ) { }       // Returns true (test position)
/// x ? foo || bar : y             // Returns true (test position)
/// const val = foo || bar         // Returns false (not a test position)
/// ```
fn is_in_test_position(logical: &JsLogicalExpression) -> bool {
    let logical_range = logical.syntax().text_trimmed_range();

    for ancestor in logical.syntax().ancestors() {
        // Check if this logical expression is in the test position of any control flow
        let test_range = match ancestor.kind() {
            JsSyntaxKind::JS_IF_STATEMENT => JsIfStatement::cast_ref(&ancestor)
                .and_then(|stmt| stmt.test().ok())
                .map(|test| test.syntax().text_trimmed_range()),
            JsSyntaxKind::JS_WHILE_STATEMENT => JsWhileStatement::cast_ref(&ancestor)
                .and_then(|stmt| stmt.test().ok())
                .map(|test| test.syntax().text_trimmed_range()),
            JsSyntaxKind::JS_FOR_STATEMENT => JsForStatement::cast_ref(&ancestor)
                .and_then(|stmt| stmt.test())
                .map(|test| test.syntax().text_trimmed_range()),
            JsSyntaxKind::JS_CONDITIONAL_EXPRESSION => JsConditionalExpression::cast_ref(&ancestor)
                .and_then(|expr| expr.test().ok())
                .map(|test| test.syntax().text_trimmed_range()),
            _ => None,
        };

        // If we found a test expression and it contains our logical expression, we're in test position
        if let Some(range) = test_range
            && range.contains_range(logical_range)
        {
            return true;
        }
    }

    false
}

/// Checks if an expression is a "safe" default value for nullish coalescing.
///
/// A "safe" default is one where replacing `||` with `??` won't change behavior
/// in problematic ways. These are values that:
/// 1. Are never falsy themselves (objects, arrays, non-empty strings, functions)
/// 2. Would be obviously wrong as a "falsy replacement" (e.g., `count || {}`)
///
/// ## Safe defaults (returns true):
/// - Object literals: `{}`
/// - Array literals: `[]`
/// - Non-empty string literals: `'default'`
/// - Template literals with content
/// - Function/class expressions
/// - `new` expressions
/// - Member access: `obj.prop`
///
/// ## Unsafe defaults (returns false):
/// - Numeric literals: `0`, `1`
/// - Empty string: `''`
/// - Boolean literals: `true`, `false`
/// - `null`, `undefined`
/// - Identifiers: `fallback` (could be any type)
fn is_safe_default_value(expr: &AnyJsExpression) -> bool {
    match expr {
        // Object literals are never falsy
        AnyJsExpression::JsObjectExpression(_) => true,

        // Array literals are never falsy
        AnyJsExpression::JsArrayExpression(_) => true,

        // Function expressions are never falsy
        AnyJsExpression::JsFunctionExpression(_) => true,
        AnyJsExpression::JsArrowFunctionExpression(_) => true,

        // Class expressions are never falsy
        AnyJsExpression::JsClassExpression(_) => true,

        // new expressions return objects (never falsy)
        AnyJsExpression::JsNewExpression(_) => true,

        // String literals - only non-empty strings are safe
        // Numbers, booleans, null are NOT safe (they could be falsy or intentional)
        AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsStringLiteralExpression(string_lit),
        ) => string_lit
            .inner_string_text()
            .ok()
            .is_some_and(|text| !text.is_empty()),

        // Other literal expressions (numbers, booleans, null) are not safe
        AnyJsExpression::AnyJsLiteralExpression(_) => false,

        // Template literals - only those with content are safe
        AnyJsExpression::JsTemplateExpression(template) => {
            // If it has elements (expressions or non-empty chunks), it's likely non-empty
            template.elements().iter().next().is_some()
        }

        // Member access suggests non-null value expected
        // e.g., `config || defaults.config` - accessing property implies object
        AnyJsExpression::JsStaticMemberExpression(_) => true,
        AnyJsExpression::JsComputedMemberExpression(_) => true,

        // Parenthesized - check inner expression
        AnyJsExpression::JsParenthesizedExpression(paren) => paren
            .expression()
            .ok()
            .is_some_and(|inner| is_safe_default_value(&inner)),

        // Everything else is unsafe or ambiguous
        _ => false,
    }
}

/// Checks if a logical expression is in a boolean coercion context.
///
/// These contexts explicitly convert to boolean, so `||` vs `??` distinction
/// doesn't matter for the final result type.
///
/// ## Boolean contexts:
/// - `Boolean(x || y)` - explicit Boolean() call
/// - `!!(x || y)` - double negation
/// - `!(x || y)` - negation (result is boolean)
fn is_in_boolean_context(logical: &JsLogicalExpression) -> bool {
    let Some(parent) = logical.syntax().parent() else {
        return false;
    };

    // Check for !(x || y) or !!(x || y)
    if let Some(unary) = JsUnaryExpression::cast(parent.clone())
        && matches!(unary.operator(), Ok(JsUnaryOperator::LogicalNot))
    {
        return true;
    }

    // Check for Boolean(x || y)
    if let Some(arg_list) = JsCallArgumentList::cast(parent.clone())
        && let Some(call) = arg_list.syntax().parent().and_then(JsCallExpression::cast)
        && let Ok(callee) = call.callee()
        && let Some(ident) = callee.as_js_identifier_expression()
        && let Ok(name) = ident.name()
        && name.to_trimmed_text() == "Boolean"
    {
        return true;
    }

    false
}

/// Checks if a logical expression is part of a mixed logical operation chain.
///
/// JavaScript requires parentheses when mixing `??` with `&&` or `||`, so we
/// should not suggest `??` when the expression is part of such a chain.
///
/// ## Examples:
/// ```js
/// a || b && c  // Mixed with &&, don't suggest ??
/// a || b || c  // Pure || chain, can suggest ??
/// (a || b) ?? c  // Already has ?? sibling, don't suggest
/// ```
fn is_in_mixed_logical_chain(logical: &JsLogicalExpression) -> bool {
    // Check parent for different logical operator
    if let Some(parent) = logical.syntax().parent()
        && let Some(parent_logical) = JsLogicalExpression::cast(parent)
        && let Ok(parent_op) = parent_logical.operator()
        && parent_op != JsLogicalOperator::LogicalOr
    {
        // Parent is && or ??, this is a mixed chain
        return true;
    }

    // Check children for different logical operator
    let left = logical.left().ok();
    let right = logical.right().ok();

    for child in [left, right].into_iter().flatten() {
        if let Some(child_logical) = child.as_js_logical_expression()
            && let Ok(child_op) = child_logical.operator()
            && child_op != JsLogicalOperator::LogicalOr
        {
            // Child is && or ??, this is a mixed chain
            return true;
        }
    }

    false
}
