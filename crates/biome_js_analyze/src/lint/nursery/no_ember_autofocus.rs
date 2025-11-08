use biome_analyze::{Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_html_parser::{HtmlParseOptions, parse_html};
use biome_html_syntax::{AnyHtmlElement, HtmlFileSource};
use biome_js_syntax::{JsFileSource, JsModule};
use biome_rowan::{AstNode, TextRange};
use regex::Regex;
use std::sync::LazyLock;

declare_lint_rule! {
    /// Disallow the `autofocus` attribute on HTML elements and components.
    ///
    /// The `autofocus` attribute automatically focuses an element when the page loads.
    /// However, it should be avoided because:
    /// - It can cause accessibility issues for screen reader users
    /// - It can create unexpected behavior when pages load
    /// - It can interfere with user navigation
    /// - It may cause issues with single-page applications
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,ignore
    /// <template>
    ///   <input autofocus />
    /// </template>
    /// ```
    ///
    /// ```js,ignore
    /// <template>
    ///   <button autofocus>Click me</button>
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js,ignore
    /// <template>
    ///   <input />
    ///   <button>Click me</button>
    /// </template>
    /// ```
    ///
    pub NoEmberAutofocus {
        version: "next",
        name: "noEmberAutofocus",
        language: "js",
        recommended: true,
    }
}

/// Regex to match Glimmer <template> tags
static GLIMMER_TEMPLATE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"<template>[\s\S]*?</template>").expect("Invalid Glimmer template regex")
});

#[derive(Debug)]
pub struct AutofocusViolation {
    range: TextRange,
    element_name: String,
}

impl Rule for NoEmberAutofocus {
    type Query = Ast<JsModule>;
    type State = Vec<AutofocusViolation>;
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
        let violations = find_autofocus_violations(&source);

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
                    "Avoid using the "<Emphasis>"autofocus"</Emphasis>" attribute."
                },
            )
            .note(markup! {
                "Autofocusing elements can cause accessibility and usability issues."
            })
            .note(markup! {
                "Consider using JavaScript to manage focus programmatically when needed."
            }),
        )
    }
}

/// Find all autofocus attribute violations in the source text
fn find_autofocus_violations(source: &str) -> Vec<AutofocusViolation> {
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
                // Check if element has autofocus attribute
                if let Some(attr) = element.find_attribute_by_name("autofocus") {
                    // Get the element name for better diagnostics
                    let element_name = element
                        .name()
                        .map(|n| n.to_string())
                        .unwrap_or_else(|| "element".to_string());

                    // Calculate the absolute position in the original source
                    let attr_range = attr.range();
                    let absolute_start: u32 = (template_start + usize::from(attr_range.start()))
                        .try_into()
                        .unwrap();
                    let absolute_end: u32 = (template_start + usize::from(attr_range.end()))
                        .try_into()
                        .unwrap();

                    violations.push(AutofocusViolation {
                        range: TextRange::new(absolute_start.into(), absolute_end.into()),
                        element_name,
                    });
                }
            }
        }
    }

    violations
}
