use crate::services::vue::VueSemantic;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsExpression, JsArrowFunctionExpression, JsAssignmentExpression, JsBinaryExpression,
    JsBinaryOperator, JsConditionalExpression, JsExpressionTemplateRoot, JsLanguage,
    JsLogicalExpression, JsSequenceExpression,
};
use biome_rowan::{AstNode, SyntaxKindSet, WalkEvent};
use biome_rule_options::no_vue_deprecated_filter::NoVueDeprecatedFilterOptions;
use biome_vue_semantic::{DirectiveKind, Snippet, SnippetHost};

declare_lint_rule! {
    /// Disallow the filter syntax that Vue 3 removed.
    ///
    /// Vue 2 let you pass a value through a function with a `|` character, as in
    /// `{{ price | currency }}`. This is called a filter. Vue 3 removed filters, so it reads
    /// the `|` as the bitwise OR operator instead. The template still compiles, but the
    /// function is never called, and the page shows a different value.
    ///
    /// Call a function instead, for example `{{ currency(price) }}`, or use a
    /// [computed property](https://vuejs.org/guide/essentials/computed.html).
    ///
    /// This rule reports a `|` in `{{ }}` and in `v-bind` values, unless it is inside
    /// parentheses, brackets, or braces.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <p>{{ price | currency }}</p>
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <a :title="name | capitalize | truncate(20)"></a>
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <template>
    ///   <p>{{ currency(price) }}</p>
    ///   <a :title="truncate(capitalize(name), 20)"></a>
    ///   <p>{{ name || "Anonymous" }}</p>
    ///   <p>{{ (flags | mask) }}</p>
    /// </template>
    /// ```
    ///
    /// ## References
    ///
    /// - [Vue 3 migration guide: Filters](https://v3-migration.vuejs.org/breaking-changes/filters.html)
    pub NoVueDeprecatedFilter {
        version: "next",
        name: "noVueDeprecatedFilter",
        language: "js",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-deprecated-filter").same()],
    }
}

impl Rule for NoVueDeprecatedFilter {
    type Query = VueSemantic<JsExpressionTemplateRoot>;
    /// The expression written with filters.
    type State = AnyJsExpression;
    type Signals = Option<Self::State>;
    type Options = NoVueDeprecatedFilterOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let snippet = ctx.snippet()?;
        if !accepts_filters(&snippet) {
            return None;
        }
        let expression = ctx.query().expression()?;
        has_filter_pipe(&expression).then_some(expression)
    }

    fn diagnostic(_ctx: &RuleContext<Self>, expression: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                expression.range(),
                markup! {
                    "This expression uses a filter, which Vue 3 no longer supports."
                },
            )
            .note(markup! {
                "Vue 3 reads the "<Emphasis>"|"</Emphasis>" as the bitwise OR operator, so the expression doesn't call the filter."
            })
            .note(markup! {
                "Call the filter as a function instead, or move the expression into a computed property."
            }),
        )
    }
}

/// Returns whether Vue 2 read filters in `snippet`: a `{{ }}` interpolation or
/// a `v-bind` value outside of `v-pre`. Vue doesn't compile an element with
/// `v-pre` or its descendants, so their attributes and content stay plain text.
fn accepts_filters(snippet: &Snippet) -> bool {
    let is_host = match snippet.host() {
        SnippetHost::Interpolation(_) => true,
        SnippetHost::DirectiveValue(_) => snippet
            .attr()
            .is_some_and(|attr| attr.directive_kind() == Some(DirectiveKind::Bind)),
        _ => false,
    };
    is_host
        && !std::iter::successors(snippet.element(), |element| element.parent())
            .any(|element| element.directive(DirectiveKind::Pre).is_some())
}

/// Returns whether `expression` has a `|` operator outside of parentheses,
/// brackets, and braces.
///
/// Every other binary operator binds more tightly than `|`, so a `|` below one
/// of them must be in parentheses. Only the operators that bind more loosely
/// can hold a `|` without them, so the search only descends through those. A
/// `|` in the target of an assignment, or in the parameters or block body of an
/// arrow function, has another node as its parent.
fn has_filter_pipe(expression: &AnyJsExpression) -> bool {
    let root = expression.syntax();
    let mut preorder = root.preorder_of_kinds(JsBinaryExpression::KIND_SET.union(LOOSE_KINDS));
    while let Some(event) = preorder.next() {
        let WalkEvent::Enter(node) = event else {
            continue;
        };
        // The subtree of a loose expression that isn't at the top is skipped,
        // so a loose parent is itself at the top.
        let is_top = &node == root
            || node
                .parent()
                .is_some_and(|parent| LOOSE_KINDS.matches(parent.kind()));
        if !is_top {
            preorder.skip_subtree();
        } else if let Some(binary) = JsBinaryExpression::cast_ref(&node) {
            if binary.operator() == Ok(JsBinaryOperator::BitwiseOr) {
                return true;
            }
            preorder.skip_subtree();
        }
    }
    false
}

/// The expressions whose operators bind more loosely than `|`.
const LOOSE_KINDS: SyntaxKindSet<JsLanguage> = JsLogicalExpression::KIND_SET
    .union(JsConditionalExpression::KIND_SET)
    .union(JsSequenceExpression::KIND_SET)
    .union(JsAssignmentExpression::KIND_SET)
    .union(JsArrowFunctionExpression::KIND_SET);
