use std::collections::HashMap;

use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_js_syntax::{AnyJsxAttribute, AnyJsxAttributeValue, JsxElement};
use biome_rowan::AstNode;

declare_rule! {
    /// Prefer using semantic element over role, since browsers now support semantic HTML element with the same meaning.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// <div role="button">...</div>
    /// <div role="img">...</div>
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// <div role="img">...</div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// <button>...</button>
    /// <img src="" alt="" />
    /// ```
    ///
    pub UseSemanticElements {
        version: "next",
        name: "useSemanticElements",
        recommended: false,
        sources: &[RuleSource::EslintJsxA11y("prefer-tag-over-role")],
    }
}

impl Rule for UseSemanticElements {
    type Query = Ast<JsxElement>;
    type State = (String, String);
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let mut semantics = HashMap::new();
        semantics.insert("button", "<button>");
        semantics.insert("article", "<article>");
        semantics.insert("cell", "<td>");
        semantics.insert("columnheader", "<th scope=\"col\">");
        semantics.insert("definition", "<dfn>");
        semantics.insert("term", "<dfn>");
        semantics.insert("table", "<table>");
        semantics.insert("img", "<img> or <picture>");
        semantics.insert("figure", "<figure>");
        semantics.insert("meter", "<meter>");
        semantics.insert("list", "either <ul> or <ol>");
        semantics.insert("heading", "<h1> through <h6>");
        semantics.insert("row", "the <tr> with <table>");
        semantics.insert("rowgroup", "<thead>, <tfoot> and <tbody>");
        semantics.insert("rowheader", "<th scope=\"row\">");

        for attribute in node.opening_element().ok()?.attributes() {
            if let AnyJsxAttribute::JsxAttribute(attr) = attribute {
                if attr.name().ok()?.text() == "role" {
                    if let AnyJsxAttributeValue::JsxString(value) =
                        attr.initializer()?.value().ok()?
                    {
                        for (role, alt) in semantics.iter() {
                            if value.text() == format!("\"{}\"", role) {
                                return Some((role.to_string(), alt.to_string()));
                            }
                        }
                    }
                }
            };
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, (role, alt): &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Role "<Emphasis>{role}</Emphasis>" should not be used as browsers now support semantic element with same meaning."
                },
            )
            .note(markup! {
                "Prefer using "<Emphasis>{alt}</Emphasis>" instead."
            }),
        )
    }
}
