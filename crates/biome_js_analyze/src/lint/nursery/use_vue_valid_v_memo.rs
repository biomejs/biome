use crate::services::vue::VueSnippet;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, JsArrowFunctionExpression, JsBinaryExpression,
    JsClassExpression, JsFunctionExpression, JsInExpression, JsInstanceofExpression, JsLanguage,
    JsObjectExpression, JsPostUpdateExpression, JsPreUpdateExpression, JsTemplateExpression,
    JsUnaryExpression,
};
use biome_rowan::{AstNode, SyntaxKindSet, TextRange};
use biome_rule_options::use_vue_valid_v_memo::UseVueValidVMemoOptions;
use biome_vue_semantic::DirectiveKind;

declare_lint_rule! {
    /// Enforce valid `v-memo` Vue directives.
    ///
    /// `v-memo` takes an array of values. Vue skips updating the element and its
    /// children until one of those values changes.
    ///
    /// This rule reports a `v-memo` value that can never be an array, such as an
    /// object, a string, a number, or a function. When the value is a condition,
    /// like `a ? b : c`, each possible result is checked.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic,ignore
    /// <template>
    ///     <div v-memo="{ x }"></div>
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic,ignore
    /// <template>
    ///     <div v-memo="selected ? [item.id] : item.id + 1"></div>
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue,ignore
    /// <template>
    ///     <div v-memo="[valueA, valueB]"></div>
    ///     <div v-memo="dependencies"></div>
    /// </template>
    /// ```
    ///
    pub UseVueValidVMemo {
        version: "next",
        name: "useVueValidVMemo",
        language: "js",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("valid-v-memo").same()],
    }
}

impl Rule for UseVueValidVMemo {
    type Query = VueSnippet;
    /// An expression that decides the value of `v-memo` and can never be an
    /// array.
    type State = AnyJsExpression;
    type Signals = Box<[Self::State]>;
    type Options = UseVueValidVMemoOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let is_memo = ctx
            .query()
            .attr()
            .is_some_and(|attr| attr.directive_kind() == Some(DirectiveKind::Memo));
        let Some(expression) = ctx
            .root()
            .as_js_expression_template_root()
            .and_then(|root| root.expression())
            .filter(|_| is_memo)
        else {
            return Box::default();
        };
        let mut values = Vec::new();
        collect_non_array_values(&expression, &mut values);
        values.into_boxed_slice()
    }

    fn text_range(_ctx: &RuleContext<Self>, expression: &Self::State) -> Option<TextRange> {
        Some(expression.range())
    }

    fn diagnostic(_ctx: &RuleContext<Self>, expression: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                expression.range(),
                markup! {
                    "The value of "<Emphasis>"v-memo"</Emphasis>" must be an array, but this expression never produces one."
                },
            )
            .note(markup! {
                "Vue skips updating this element until one of the values in the "<Emphasis>"v-memo"</Emphasis>" array changes, so it needs an array to compare."
            })
            .note(markup! {
                "Put the values in square brackets, for example "<Emphasis>"v-memo=\"[item.id, selected]\""</Emphasis>"."
            }),
        )
    }
}

/// Expressions whose value is never an array.
///
/// A tagged template has the same kind as a template literal, so it must be
/// ruled out before checking this set: its tag can return anything.
const NON_ARRAY_EXPRESSIONS: SyntaxKindSet<JsLanguage> = AnyJsLiteralExpression::KIND_SET
    .union(JsTemplateExpression::KIND_SET)
    .union(JsObjectExpression::KIND_SET)
    .union(JsClassExpression::KIND_SET)
    .union(JsArrowFunctionExpression::KIND_SET)
    .union(JsFunctionExpression::KIND_SET)
    .union(JsUnaryExpression::KIND_SET)
    .union(JsBinaryExpression::KIND_SET)
    .union(JsInExpression::KIND_SET)
    .union(JsInstanceofExpression::KIND_SET)
    .union(JsPreUpdateExpression::KIND_SET)
    .union(JsPostUpdateExpression::KIND_SET);

/// Pushes each expression that decides the value of `expression` and can
/// never be an array.
///
/// Both branches of a conditional decide the value, as do the right side of
/// an assignment and the last expression of a sequence.
fn collect_non_array_values(expression: &AnyJsExpression, values: &mut Vec<AnyJsExpression>) {
    let Some(expression) = expression.inner_expression() else {
        return;
    };
    match &expression {
        AnyJsExpression::JsAssignmentExpression(assignment) => {
            if let Ok(right) = assignment.right() {
                collect_non_array_values(&right, values);
            }
        }
        AnyJsExpression::JsConditionalExpression(conditional) => {
            if let Ok(consequent) = conditional.consequent() {
                collect_non_array_values(&consequent, values);
            }
            if let Ok(alternate) = conditional.alternate() {
                collect_non_array_values(&alternate, values);
            }
        }
        AnyJsExpression::JsTemplateExpression(template) if template.tag().is_some() => {}
        _ if NON_ARRAY_EXPRESSIONS.matches(expression.syntax().kind()) => {
            values.push(expression);
        }
        _ => {}
    }
}
