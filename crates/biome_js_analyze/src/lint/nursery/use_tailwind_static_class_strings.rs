use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsExpression, AnyJsTemplateElement, JsBinaryOperator, JsLogicalOperator};
use biome_rowan::{AstNode, AstSeparatedList, TextRange};
use biome_rule_options::use_tailwind_static_class_strings::UseTailwindStaticClassStringsOptions;
use biome_tailwind_logic::class_context::{AnyJsClassContext, TailwindClassContext};

declare_lint_rule! {
    /// Require complete, statically written class names.
    ///
    /// Tailwind detects class names in source text. Dynamic string construction
    /// can hide complete class names from its source scanner.
    /// Choose between complete class names instead. Template interpolation and
    /// concatenation are reported even when separated by whitespace or composed
    /// entirely of literals. Variables and conditional choices are allowed.
    /// This rule does not follow variable assignments or inspect tagged templates.
    ///
    /// The `class` and `className` JSX attributes and Tailwind
    /// utility functions such as `clsx`, `cn`, and `twMerge` are checked.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div className={`bg-${color}`} />;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div className={"text-" + color} />;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// clsx(`bg-${color}`);
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div className={`p-4 ${classes}`} />;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <div className={active ? "bg-red-500" : "bg-blue-500"} />;
    /// <div className="p-4 text-white" />;
    /// ```
    pub UseTailwindStaticClassStrings {
        version: "next",
        name: "useTailwindStaticClassStrings",
        language: "jsx",
        domains: &[RuleDomain::Tailwind],
        recommended: false,
        sources: &[RuleSource::EslintShadcn("require-static-classes").inspired()],
    }
}

impl Rule for UseTailwindStaticClassStrings {
    type Query = TailwindClassContext<AnyJsClassContext>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = UseTailwindStaticClassStringsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        match ctx.query() {
            AnyJsClassContext::JsxAttribute(attribute) => attribute
                .initializer()
                .and_then(|initializer| initializer.value().ok())
                .and_then(|value| value.as_jsx_expression_attribute_value().cloned())
                .and_then(|value| value.expression().ok())
                .and_then(|expression| dynamic_class_range(&expression)),
            AnyJsClassContext::JsCallExpression(call) => {
                let arguments = call.arguments().ok()?;
                arguments
                    .args()
                    .iter()
                    .flatten()
                    .find_map(|argument| dynamic_class_range(argument.as_any_js_expression()?))
            }
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(), range,
            markup! { "This class string is constructed dynamically." },
        )
        .note(markup! { "Tailwind scans source text for complete class names." })
        .note(markup! { "Use complete class names, choosing between them with a conditional expression or a lookup." }))
    }
}

/// Finds the first dynamic class expression, excluding conditions and object values.
fn dynamic_class_range(expression: &AnyJsExpression) -> Option<TextRange> {
    match expression.inner_expression()? {
        AnyJsExpression::JsTemplateExpression(template) => (template.tag().is_none()
            && template
                .elements()
                .into_iter()
                .any(|element| matches!(element, AnyJsTemplateElement::JsTemplateElement(_))))
        .then(|| template.range()),
        AnyJsExpression::JsBinaryExpression(binary)
            if binary.operator().ok() == Some(JsBinaryOperator::Plus) =>
        {
            Some(binary.range())
        }
        AnyJsExpression::JsConditionalExpression(conditional) => conditional
            .consequent()
            .ok()
            .and_then(|expression| dynamic_class_range(&expression))
            .or_else(|| {
                conditional
                    .alternate()
                    .ok()
                    .and_then(|expression| dynamic_class_range(&expression))
            }),
        AnyJsExpression::JsLogicalExpression(logical) => {
            if logical.operator().ok() != Some(JsLogicalOperator::LogicalAnd)
                && let Some(range) = logical
                    .left()
                    .ok()
                    .and_then(|expression| dynamic_class_range(&expression))
            {
                return Some(range);
            }
            logical
                .right()
                .ok()
                .and_then(|expression| dynamic_class_range(&expression))
        }
        AnyJsExpression::JsArrayExpression(array) => array
            .elements()
            .iter()
            .flatten()
            .find_map(|element| dynamic_class_range(element.as_any_js_expression()?)),
        AnyJsExpression::JsObjectExpression(object) => {
            object.members().iter().flatten().find_map(|member| {
                let property = member.as_js_property_object_member()?;
                let name = property.name().ok()?;
                let computed = name.as_js_computed_member_name()?;
                dynamic_class_range(&computed.expression().ok()?)
            })
        }
        _ => None,
    }
}
