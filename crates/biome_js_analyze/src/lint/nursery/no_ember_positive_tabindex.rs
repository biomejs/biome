use biome_analyze::{Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_html_parser::{HtmlParseOptions, parse_html};
use biome_html_syntax::{AnyHtmlElement, HtmlFileSource};
use biome_js_syntax::{JsFileSource, JsModule};
use biome_rowan::{AstNode, TextRange};
use regex::Regex;
use std::sync::LazyLock;

declare_lint_rule! {
    /// Disallow positive `tabindex` values in Glimmer templates.
    ///
    /// Positive `tabindex` values create a confusing tab order for keyboard users
    /// and are considered an accessibility anti-pattern. Screen reader users rely
    /// on a logical tab order, and positive tabindex disrupts that flow.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,ignore
    /// <template>
    ///   <button tabindex="1">Click me</button>
    /// </template>
    /// ```
    ///
    /// ```js,ignore
    /// <template>
    ///   <div tabindex="5">Content</div>
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js,ignore
    /// <template>
    ///   <button>Click me</button>
    ///   <button tabindex="0">Focusable</button>
    ///   <button tabindex="-1">Not in tab order</button>
    /// </template>
    /// ```
    ///
    pub NoEmberPositiveTabindex {
        version: "next",
        name: "noEmberPositiveTabindex",
        language: "js",
        recommended: true,
    }
}

/// Regex to match Glimmer <template> tags
static GLIMMER_TEMPLATE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"<template>[\s\S]*?</template>").expect("Invalid Glimmer template regex")
});

#[derive(Debug)]
pub struct PositiveTabindexViolation {
    range: TextRange,
    element_name: String,
    tabindex_value: String,
}

impl Rule for NoEmberPositiveTabindex {
    type Query = Ast<JsModule>;
    type State = Vec<PositiveTabindexViolation>;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let module = ctx.query();

        // Check if this is a Glimmer file (.gjs/.gts)
        let source_type = ctx.source_type::<JsFileSource>();
        if !source_type.as_embedding_kind().is_glimmer() {
            return None;
        }

        // Get the source text
        let source = module.syntax().text_with_trivia().to_string();

        // Find all violations in template blocks
        let violations = find_positive_tabindex_violations(&source);

        if violations.is_empty() {
            None
        } else {
            Some(violations)
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let violation = state.first()?;

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                violation.range,
                markup! {
                    "Avoid using positive "<Emphasis>"tabindex"</Emphasis>" values."
                },
            )
            .note(markup! {
                "Positive tabindex values create a confusing tab order for keyboard users."
            })
            .note(markup! {
                "Use "<Emphasis>"tabindex=\"0\""</Emphasis>" to make elements focusable or "<Emphasis>"tabindex=\"-1\""</Emphasis>" to remove them from tab order."
            })
            .note(markup! {
                "Consider restructuring your HTML instead of using tabindex."
            }),
        )
    }
}

/// Find all positive tabindex attribute violations in the source text
fn find_positive_tabindex_violations(source: &str) -> Vec<PositiveTabindexViolation> {
    let mut violations = Vec::new();

    // Find all <template> blocks
    for template_match in GLIMMER_TEMPLATE.find_iter(source) {
        let template_content = template_match.as_str();
        let template_start = template_match.start();

        // Parse the template with Glimmer-enabled HTML parser
        let file_source = HtmlFileSource::glimmer();
        let options = HtmlParseOptions::from(&file_source);
        let parse = parse_html(template_content, options);

        let root = parse.tree();
        let root_node = root.syntax();

        // Traverse all HTML elements
        for node in root_node.descendants() {
            if let Some(element) = AnyHtmlElement::cast(node) {
                // Check if element has tabindex attribute
                if let Some(attr) = element.find_attribute_by_name("tabindex") {
                    // Get the attribute value through initializer
                    if let Some(initializer) = attr.initializer() {
                        if let Ok(value) = initializer.value() {
                            let value_text = value.to_string();
                            // Remove quotes from the value
                            let cleaned_value =
                                value_text.trim().trim_matches('"').trim_matches('\'');

                            // Parse as integer and check if positive (> 0)
                            if let Ok(tabindex_num) = cleaned_value.parse::<i32>() {
                                if tabindex_num > 0 {
                                    // Get the element name for better diagnostics
                                    let element_name = element
                                        .name()
                                        .map(|n| n.to_string())
                                        .unwrap_or_else(|| "element".to_string());

                                    // Calculate the absolute position in the original source
                                    let attr_range = attr.range();
                                    let absolute_start: u32 = (template_start
                                        + usize::from(attr_range.start()))
                                    .try_into()
                                    .unwrap();
                                    let absolute_end: u32 = (template_start
                                        + usize::from(attr_range.end()))
                                    .try_into()
                                    .unwrap();

                                    violations.push(PositiveTabindexViolation {
                                        range: TextRange::new(
                                            absolute_start.into(),
                                            absolute_end.into(),
                                        ),
                                        element_name,
                                        tabindex_value: cleaned_value.to_string(),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    violations
}
