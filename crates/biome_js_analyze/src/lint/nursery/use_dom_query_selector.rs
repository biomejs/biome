use crate::{JsRuleAction, services::semantic::Semantic};
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
    options::PreferredQuote,
};
use biome_console::markup;
use biome_js_factory::make::{
    self, js_string_literal_expression, js_string_literal_single_quotes, js_template_chunk,
    js_template_chunk_element,
};
use biome_js_syntax::{
    AnyJsExpression, AnyJsMemberExpression, AnyJsTemplateElement, JsCallExpression,
    JsTemplateExpression, T, static_value::StaticValue,
};
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt, TextRange};
use biome_rule_options::use_dom_query_selector::UseDomQuerySelectorOptions;

declare_lint_rule! {
    /// Prefer `querySelector()` and `querySelectorAll()` over older DOM query APIs.
    ///
    /// This rule prefers `querySelector()` over `getElementById()`, and `querySelectorAll()` over
    /// `getElementsByClassName()`, `getElementsByTagName()`, and `getElementsByName()`.
    ///
    /// Using the more modern DOM query APIs can often make the intent of a DOM lookup clearer and
    /// more concise than the older APIs, especially for complex selectors or if filtering by multiple attributes.
    /// Additionally, these newer APIs are more flexible and can be easily refined later with more
    /// specific selectors without needing to change the method being called.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// document.getElementById("foo");
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// document.getElementsByClassName("foo bar");
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// document.getElementsByTagName("main");
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// document.querySelector("#foo");
    /// ```
    ///
    /// ```js
    /// document.querySelectorAll(".foo.bar");
    /// ```
    ///
    pub UseDomQuerySelector {
        version: "2.4.13",
        name: "useDomQuerySelector",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("prefer-query-selector").inspired()],
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

pub struct RuleState {
    /// The legacy DOM query method that triggered the rule.
    method: QueryMethod,
    /// The source range of the method name used for diagnostics.
    range: TextRange,
    /// Whether the rule can rewrite the current call.
    fixable: bool,
}

impl Rule for UseDomQuerySelector {
    type Query = Semantic<JsCallExpression>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = UseDomQuerySelectorOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();

        if call.is_optional() || call.is_optional_chain() {
            return None;
        }

        let callee = call.callee().ok()?.omit_parentheses();
        let member = AnyJsMemberExpression::cast(callee.into_syntax())?;
        if member.is_optional_chain()
            || matches!(member, AnyJsMemberExpression::JsComputedMemberExpression(_))
        {
            return None;
        }

        let method_name = member.member_name()?;
        let method = QueryMethod::from_name(method_name.text())?;

        let argument = first_and_only_argument(call)?;
        if is_definitely_not_dom_node(&member.object().ok()?.omit_parentheses()) {
            return None;
        }

        Some(RuleState {
            method,
            range: method_name.range(),
            fixable: can_fix_argument(&argument, method),
        })
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let mut diag = RuleDiagnostic::new(
            rule_category!(),
            state.range,
            markup! {
                "This DOM query uses an older DOM query API."
            },
        )
        .note(markup! {
            "Using the more modern DOM query APIs consistently makes DOM lookups easier to read and easier to refine with more specific selectors."
        });
        if !state.fixable {
            diag = diag.note(markup! {
                "Use "<Emphasis>"querySelector()"</Emphasis>" or "<Emphasis>"querySelectorAll()"</Emphasis>" instead."
            });
        }
        Some(diag)
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        if !state.fixable {
            return None;
        }

        let call = ctx.query();
        let callee = call.callee().ok()?.omit_parentheses();
        let member = AnyJsMemberExpression::cast(callee.into_syntax())?;
        let argument = first_and_only_argument(call)?;

        let mut mutation = ctx.root().begin();
        replace_method_name(
            &mut mutation,
            &member,
            state.method.preferred_name(),
            ctx.preferred_quote(),
        )?;

        if let Some(replacement_argument) =
            build_replacement_argument(&argument, state.method, ctx.preferred_quote())
        {
            mutation.replace_node(argument, replacement_argument);
        }

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            match state.method {
                QueryMethod::ElementById => {
                    markup! { "Use "<Emphasis>".querySelector()"</Emphasis>" instead." }
                }
                QueryMethod::ElementsByClassName
                | QueryMethod::ElementsByTagName
                | QueryMethod::ElementsByName => {
                    markup! { "Use "<Emphasis>".querySelectorAll()"</Emphasis>" instead." }
                }
            },
            mutation,
        ))
    }
}

/// Legacy DOM query methods recognized by this rule.
#[derive(Clone, Copy, Debug)]
enum QueryMethod {
    /// `getElementById()`.
    ElementById,
    /// `getElementsByClassName()`.
    ElementsByClassName,
    /// `getElementsByTagName()`.
    ElementsByTagName,
    /// `getElementsByName()`.
    ElementsByName,
}

impl QueryMethod {
    /// Maps the legacy DOM query method name to the corresponding enum variant.
    fn from_name(name: &str) -> Option<Self> {
        Some(match name {
            "getElementById" => Self::ElementById,
            "getElementsByClassName" => Self::ElementsByClassName,
            "getElementsByTagName" => Self::ElementsByTagName,
            "getElementsByName" => Self::ElementsByName,
            _ => return None,
        })
    }

    /// Returns the preferred `querySelector*` replacement for this legacy method.
    fn preferred_name(self) -> &'static str {
        match self {
            Self::ElementById => "querySelector",
            Self::ElementsByClassName | Self::ElementsByTagName | Self::ElementsByName => {
                "querySelectorAll"
            }
        }
    }
}

/// Returns the only argument passed to `call`.
///
/// Calls with zero arguments or more than one argument are ignored because this
/// rule only rewrites the single-selector form.
///
/// The source rule ignores spread arguments, so they are also ignored here.
fn first_and_only_argument(call: &JsCallExpression) -> Option<AnyJsExpression> {
    let mut args = call.arguments().ok()?.args().into_iter();
    let argument = args.next()?.ok()?.as_any_js_expression()?.clone();

    if args.next().is_none() {
        Some(argument)
    } else {
        None
    }
}

/// Returns `true` when the object of a legacy DOM query call is syntactically
/// incapable of being a DOM node. It's a mechanism to avoid obvious false positives.
///
/// This rule is supposed to report nearly any call shape like `value.getElementById(...)`
/// and only excludes receivers whose syntax guarantees they are not DOM nodes,
/// such as:
///
/// - literals like `"text"`, `null`, and `1`
/// - array and object literals like `[]` and `{}`
/// - function and class expressions
/// - template literals like `` `text` ``
/// - the global `undefined` value
///
/// Everything else is treated as potentially DOM-like, including identifiers,
/// member expressions, and other unknown expressions.
fn is_definitely_not_dom_node(expr: &AnyJsExpression) -> bool {
    let expr = expr.clone().omit_parentheses();

    matches!(
        expr,
        AnyJsExpression::AnyJsLiteralExpression(_)
            | AnyJsExpression::JsArrayExpression(_)
            | AnyJsExpression::JsArrowFunctionExpression(_)
            | AnyJsExpression::JsClassExpression(_)
            | AnyJsExpression::JsFunctionExpression(_)
            | AnyJsExpression::JsObjectExpression(_)
            | AnyJsExpression::JsTemplateExpression(_)
    ) || expr
        .as_static_value()
        .is_some_and(|value| matches!(value, StaticValue::Undefined(_)))
}

/// Returns `true` when the query argument can be safely converted into a CSS
/// selector for `method`.
///
/// This accepts non-empty string-like values and `null`, which keeps cases like
/// `document.getElementById(null)` aligned with `document.querySelector(null)`.
fn can_fix_argument(argument: &AnyJsExpression, method: QueryMethod) -> bool {
    if matches!(method, QueryMethod::ElementsByTagName) {
        return true;
    }

    let argument = argument.clone().omit_parentheses();
    match argument {
        AnyJsExpression::AnyJsLiteralExpression(literal) => {
            if literal.as_js_null_literal_expression().is_some() {
                return true;
            }

            literal
                .as_js_string_literal_expression()
                .and_then(|string| string.inner_string_text().ok())
                .is_some_and(|text| !text.text().trim().is_empty())
        }
        AnyJsExpression::JsTemplateExpression(template) => template_is_fixable(&template),
        _ => false,
    }
}

/// Returns `true` for untagged template literals whose full value is known and
/// non-empty.
///
/// For example, `` `main` `` is fixable, while `` tag`main` `` and
/// `` `${name}` `` are not.
fn template_is_fixable(template: &JsTemplateExpression) -> bool {
    template.tag().is_none()
        && template
            .elements()
            .iter()
            .all(|element| element.as_js_template_chunk_element().is_some())
        && AnyJsExpression::JsTemplateExpression(template.clone())
            .as_static_value()
            .is_some_and(|value| {
                value
                    .as_string_constant()
                    .is_some_and(|text| !text.trim().is_empty())
            })
}

/// Builds the selector argument that should replace the original DOM query
/// argument.
///
/// Examples:
/// - `"foo"` becomes `"#foo"` for `getElementById()`.
/// - `"foo bar"` becomes `".foo.bar"` for `getElementsByClassName()`.
/// - `"username"` becomes `"[name='username']"` or `"[name=\"username\"]"`
///   for `getElementsByName()`, depending on the preferred outer quote style.
fn build_replacement_argument(
    argument: &AnyJsExpression,
    method: QueryMethod,
    preferred_quote: PreferredQuote,
) -> Option<AnyJsExpression> {
    let argument = argument.clone().omit_parentheses();

    if matches!(method, QueryMethod::ElementsByTagName) {
        return None;
    }

    match argument {
        AnyJsExpression::AnyJsLiteralExpression(literal) => {
            if literal.as_js_null_literal_expression().is_some() {
                return None;
            }

            let string = literal.as_js_string_literal_expression()?;
            let value = string.inner_string_text().ok()?;
            let replacement = match method {
                QueryMethod::ElementById => format!("#{}", value.text()),
                QueryMethod::ElementsByClassName => format_class_selector(value.text())?,
                QueryMethod::ElementsByName => format_name_selector(value.text(), preferred_quote)?,
                QueryMethod::ElementsByTagName => return None,
            };

            Some(make_string_literal_expression(
                &replacement,
                preferred_quote,
            ))
        }
        AnyJsExpression::JsTemplateExpression(template) => {
            let static_value =
                AnyJsExpression::JsTemplateExpression(template.clone()).as_static_value()?;
            let value = static_value.as_string_constant()?;
            let replacement = match method {
                QueryMethod::ElementById => format!("#{value}"),
                QueryMethod::ElementsByClassName => format_class_selector(value)?,
                QueryMethod::ElementsByName => format_name_selector(value, preferred_quote)?,
                QueryMethod::ElementsByTagName => return None,
            };

            Some(AnyJsExpression::JsTemplateExpression(
                make::js_template_expression(
                    make::token(T!['`']),
                    make::js_template_element_list([AnyJsTemplateElement::from(
                        js_template_chunk_element(js_template_chunk(&replacement)),
                    )]),
                    make::token(T!['`']),
                )
                .build(),
            ))
        }
        _ => None,
    }
}

/// Replaces the accessed member name in either `obj.method` or `obj["method"]`
/// form while preserving the original member-expression shape.
fn replace_method_name(
    mutation: &mut biome_rowan::BatchMutation<biome_js_syntax::JsLanguage>,
    member: &AnyJsMemberExpression,
    replacement: &str,
    preferred_quote: PreferredQuote,
) -> Option<()> {
    match member {
        AnyJsMemberExpression::JsStaticMemberExpression(static_member) => {
            mutation.replace_element(
                static_member.member().ok()?.into(),
                make::js_name(make::ident(replacement)).into(),
            );
        }
        AnyJsMemberExpression::JsComputedMemberExpression(computed_member) => {
            let current_member = computed_member.member().ok()?;
            let replacement = AnyJsExpression::AnyJsLiteralExpression(
                js_string_literal_expression(if preferred_quote.is_double() {
                    make::js_string_literal(replacement)
                } else {
                    js_string_literal_single_quotes(replacement)
                })
                .into(),
            );
            mutation.replace_node(current_member, replacement);
        }
    }

    Some(())
}

/// Converts a whitespace-separated class list into a CSS class selector.
///
/// For example, `"foo bar"` becomes `".foo.bar"`.
fn format_class_selector(value: &str) -> Option<String> {
    let mut selector = String::new();
    for class_name in value.split_whitespace() {
        selector.push('.');
        selector.push_str(class_name);
    }

    if selector.is_empty() {
        None
    } else {
        Some(selector)
    }
}

/// Wraps a `name` attribute value in an attribute selector using the opposite
/// quote style from the surrounding string literal.
///
/// For example, with double-quoted string literals, `"username"` becomes
/// `[name='username']`.
fn format_name_selector(value: &str, preferred_quote: PreferredQuote) -> Option<String> {
    if value.is_empty() {
        return None;
    }

    // use the opposite quote style for the inner quotes to conflicting
    // with the outer quotes of the string literal
    let inner_quote = if preferred_quote.is_double() {
        '\''
    } else {
        '"'
    };
    Some(format!("[name={inner_quote}{value}{inner_quote}]"))
}

/// Creates a string literal expression that matches the configured preferred
/// quote style.
fn make_string_literal_expression(value: &str, preferred_quote: PreferredQuote) -> AnyJsExpression {
    AnyJsExpression::AnyJsLiteralExpression(
        js_string_literal_expression(if preferred_quote.is_double() {
            make::js_string_literal(value)
        } else {
            js_string_literal_single_quotes(value)
        })
        .into(),
    )
}
