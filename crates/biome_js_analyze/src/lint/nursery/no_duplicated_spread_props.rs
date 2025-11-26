use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::AnyJsxAttribute;
use biome_js_syntax::{JsxAttributeList, JsxOpeningElement, JsxSelfClosingElement};
use biome_rowan::{AstNode, declare_node_union};
use biome_rule_options::no_duplicated_spread_props::NoDuplicatedSpreadPropsOptions;
use std::collections::HashSet;

declare_lint_rule! {
    /// Disallow JSX prop spreading the same identifier multiple times.
    ///
    /// Enforces that any unique expression is only spread once.
    /// Generally spreading the same expression twice is an indicator of a mistake since any attribute between the spreads may be overridden when the intent was not to.
    /// Even when that is not the case this will lead to unnecessary computations being performed.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div {...props} something="else" {...props} />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <div something="else" {...props} />
    /// ```
    ///
    pub NoDuplicatedSpreadProps {
        version: "2.3.8",
        name: "noDuplicatedSpreadProps",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintReact("jsx-props-no-spread-multi").same()],
        domains: &[RuleDomain::React, RuleDomain::Solid],
    }
}

impl Rule for NoDuplicatedSpreadProps {
    type Query = Ast<NoDuplicatedSpreadPropsQuery>;
    type State = String;
    type Signals = Option<Self::State>;
    type Options = NoDuplicatedSpreadPropsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let binding = ctx.query();

        match binding {
            NoDuplicatedSpreadPropsQuery::JsxOpeningElement(node) => {
                let attributes = node.attributes();
                validate_attributes(&attributes)
            }
            NoDuplicatedSpreadPropsQuery::JsxSelfClosingElement(node) => {
                let attributes = node.attributes();
                validate_attributes(&attributes)
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "The expression "<Emphasis>{state}</Emphasis>" has spread more than once."
                },
            )
            .note(markup! {
                "Spreading an expression more than once will lead to unnecessary computations being performed. Reduce spreads of this expression down to 1."
            }),
        )
    }
}

declare_node_union! {
    pub NoDuplicatedSpreadPropsQuery =
        JsxOpeningElement
        | JsxSelfClosingElement
}

fn validate_attributes(list: &JsxAttributeList) -> Option<String> {
    let mut seen_spreads = HashSet::new();

    for attribute in list {
        if let AnyJsxAttribute::JsxSpreadAttribute(spread) = attribute
            && let Some(argument) = spread.argument().ok()
            && let Some(express) = argument.as_js_identifier_expression()
            && let Some(name) = express.name().ok()
            && let Some(value_token) = name.value_token().ok()
        {
            let text = value_token.text_trimmed().to_string();
            if !seen_spreads.insert(text.clone()) {
                return Some(text);
            }
        }
    }

    None
}
