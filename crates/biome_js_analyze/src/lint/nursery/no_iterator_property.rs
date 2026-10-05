use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsExpression, AnyJsMemberExpression, AnyJsName, assign_ext::AnyJsMemberAssignment,
};
use biome_rowan::{AstNode, TextRange, declare_node_union};
use biome_rule_options::no_iterator_property::NoIteratorPropertyOptions;

declare_lint_rule! {
    /// Disallow the use of the `__iterator__` property.
    ///
    /// `__iterator__` was a non-standard property that only Firefox supported. Assigning a function
    /// to it changed which values a `for...in` loop produced for an object. It was never part of
    /// the JavaScript standard, and Firefox has since removed it, so no current browser or
    /// JavaScript runtime uses it. Code that relies on it silently stops working.
    ///
    /// To make an object iterable, define a [`Symbol.iterator`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/iterator)
    /// method instead, and loop over the object with `for...of`. The
    /// [iteration protocols](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Iteration_protocols)
    /// guide explains how this works.
    ///
    /// The rule reports both reading and assigning `__iterator__`, whether the code uses a dot
    /// (`foo.__iterator__`) or brackets with a string (`foo["__iterator__"]`).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// Foo.prototype.__iterator__ = function () {
    ///     return new FooIterator(this);
    /// };
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// foo.__iterator__ = function () {};
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// foo["__iterator__"] = function () {};
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// // A variable named `__iterator__` isn't a property, so it's allowed.
    /// const __iterator__ = foo;
    ///
    /// // Brackets with a variable look up the variable's value, not a property named `__iterator__`.
    /// foo[__iterator__];
    ///
    /// // The standard way to make objects iterable.
    /// Foo.prototype[Symbol.iterator] = function* () {
    ///     yield 1;
    /// };
    /// ```
    ///
    pub NoIteratorProperty {
        version: "next",
        name: "noIteratorProperty",
        language: "js",
        sources: &[RuleSource::Eslint("no-iterator").same()],
        recommended: true,
        severity: Severity::Warning,
    }
}

declare_node_union! {
    pub AnyIteratorPropertyQuery = AnyJsMemberExpression | AnyJsMemberAssignment
}

impl Rule for NoIteratorProperty {
    type Query = Ast<AnyIteratorPropertyQuery>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoIteratorPropertyOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        Member::from_query(ctx.query())?
            .is_iterator_property()
            .then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let member = Member::from_query(ctx.query())?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                member.range(),
                markup! {
                    "Unexpected use of the obsolete "<Emphasis>"__iterator__"</Emphasis>" property."
                },
            )
            .note(markup! {
                <Emphasis>"__iterator__"</Emphasis>" is a non-standard extension that modern JavaScript engines no longer support."
            })
            .note(markup! {
                "Use "<Emphasis>"Symbol.iterator"</Emphasis>" to make an object iterable instead."
            }),
        )
    }
}

const ITERATOR_PROPERTY: &str = "__iterator__";

/// The property part of a member expression or member assignment.
enum Member {
    /// The name after the dot, e.g. `__iterator__` in `foo.__iterator__`.
    Static(AnyJsName),
    /// The expression between the brackets, without parentheses, e.g. `"__iterator__"` in
    /// `foo["__iterator__"]`.
    Computed(AnyJsExpression),
}

impl Member {
    /// Returns `None` when a syntax error left the member missing.
    fn from_query(query: &AnyIteratorPropertyQuery) -> Option<Self> {
        let member = match query {
            AnyIteratorPropertyQuery::AnyJsMemberExpression(
                AnyJsMemberExpression::JsStaticMemberExpression(expression),
            ) => Self::Static(expression.member().ok()?),
            AnyIteratorPropertyQuery::AnyJsMemberExpression(
                AnyJsMemberExpression::JsComputedMemberExpression(expression),
            ) => Self::Computed(expression.member().ok()?.omit_parentheses()),
            AnyIteratorPropertyQuery::AnyJsMemberAssignment(
                AnyJsMemberAssignment::JsStaticMemberAssignment(assignment),
            ) => Self::Static(assignment.member().ok()?),
            AnyIteratorPropertyQuery::AnyJsMemberAssignment(
                AnyJsMemberAssignment::JsComputedMemberAssignment(assignment),
            ) => Self::Computed(assignment.member().ok()?.omit_parentheses()),
        };
        Some(member)
    }

    /// Returns `true` if the member is `__iterator__`, either as a name or as a string literal or
    /// an untagged template literal without substitutions.
    fn is_iterator_property(&self) -> bool {
        match self {
            Self::Static(name) => name
                .as_js_name()
                .and_then(|name| name.value_token().ok())
                .is_some_and(|token| token.text_trimmed() == ITERATOR_PROPERTY),
            Self::Computed(expression) => {
                // A tagged template is a function call, so its result isn't known statically.
                if expression
                    .as_js_template_expression()
                    .is_some_and(|template| template.tag().is_some())
                {
                    return false;
                }
                expression
                    .as_static_value()
                    .is_some_and(|value| value.as_string_constant() == Some(ITERATOR_PROPERTY))
            }
        }
    }

    fn range(&self) -> TextRange {
        match self {
            Self::Static(name) => name.range(),
            Self::Computed(expression) => expression.range(),
        }
    }
}
