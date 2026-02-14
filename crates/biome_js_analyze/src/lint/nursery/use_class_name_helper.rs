use biome_analyze::{
    Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsExpression, JsTemplateExpression, JsxAttribute};
use biome_rowan::{AstNode, TextRange, TokenText};
use biome_rule_options::use_class_name_helper::UseClassNameHelperOptions;

declare_lint_rule! {
    /// Disallow template literals with interpolations in class attributes.
    ///
    /// Template literals with `${...}` interpolations inside `className` / `class` attributes
    /// can break class extraction and other Tailwind tooling workflows.
    /// This rule encourages using class helpers (`cn`, `clsx`, `cva`, etc.)
    /// with discrete string arguments so extraction, merging, and editor tooling stay reliable.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div className={`px-4 ${isActive ? "bg-blue-500" : "bg-gray-500"}`} />;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div class={`px-4 ${isActive ? "bg-blue-500" : "bg-gray-500"}`} />;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <div className={cn("px-4", isActive && "bg-blue-500", !isActive && "bg-gray-500")} />;
    /// ```
    ///
    /// ## Options
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "attributes": ["className", "class"],
    ///     "helperFunctions": ["cn", "clsx", "cva", "cx", "classNames"]
    ///   }
    /// }
    /// ```
    ///
    /// ### attributes
    ///
    /// Attribute names checked by this rule.
    ///
    /// ### helperFunctions
    ///
    /// Preferred helper names suggested in diagnostics.
    ///
    pub UseClassNameHelper {
        version: "next",
        name: "useClassNameHelper",
        language: "jsx",
        recommended: false,
    }
}

impl Rule for UseClassNameHelper {
    type Query = Ast<JsxAttribute>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = UseClassNameHelperOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let attribute = ctx.query();
        let attribute_name = get_attribute_name(attribute)?;
        if !ctx.options().has_attribute(attribute_name.text()) {
            return None;
        }

        let expression = attribute
            .initializer()?
            .value()
            .ok()?
            .as_jsx_expression_attribute_value()?
            .expression()
            .ok()?
            .inner_expression()?;

        let template = find_interpolated_template(&expression)?;

        Some(template.template_range().unwrap_or(template.range()))
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let helper_functions = ctx.options().helper_functions_for_diagnostic();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                *state,
                markup! {
                    "Avoid using template literals with interpolations in this class attribute."
                },
            )
            .note(markup! {
                "Prefer helper functions like "<Emphasis>{helper_functions.as_str()}</Emphasis>" with discrete string arguments to improve extraction and tooling reliability."
            })
            .note(markup! {
                "Template interpolations can prevent class-name tooling from detecting and autocompleting classes."
            }),
        )
    }
}

fn get_attribute_name(attribute: &JsxAttribute) -> Option<TokenText> {
    Some(
        attribute
            .name()
            .ok()?
            .as_jsx_name()?
            .value_token()
            .ok()?
            .token_text_trimmed(),
    )
}

fn find_interpolated_template(expression: &AnyJsExpression) -> Option<JsTemplateExpression> {
    if let AnyJsExpression::JsTemplateExpression(template) = expression
        && !template.is_constant()
    {
        return Some(template.clone());
    }

    expression
        .syntax()
        .descendants()
        .filter_map(JsTemplateExpression::cast)
        .find(|template| !template.is_constant())
}
