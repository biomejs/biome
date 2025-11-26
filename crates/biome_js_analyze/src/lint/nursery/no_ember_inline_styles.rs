use biome_analyze::{Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_html_parser::{HtmlParseOptions, parse_html};
use biome_html_syntax::{AnyHtmlElement, HtmlFileSource};
use biome_js_syntax::{JsFileSource, JsModule};
use biome_rowan::{AstNode, TextRange};
use crate::services::semantic::OriginalSourceText;
use regex::Regex;
use std::sync::LazyLock;

declare_lint_rule! {
    /// Disallow inline `style` attributes in Glimmer templates.
    ///
    /// Inline styles should be avoided for maintainability and Content Security Policy compliance.
    /// Using CSS classes or a CSS-in-JS solution provides better separation of concerns and makes
    /// styling easier to maintain across your application.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,ignore
    /// <template>
    ///   <div style="color: red;">Text</div>
    /// </template>
    /// ```
    ///
    /// ```js,ignore
    /// <template>
    ///   <button style="background: blue;">Click</button>
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js,ignore
    /// <template>
    ///   <div class="my-styled-div">Text</div>
    /// </template>
    /// ```
    ///
    pub NoEmberInlineStyles {
        version: "next",
        name: "noEmberInlineStyles",
        language: "js",
        recommended: true,
    }
}

/// Regex to match Glimmer <template> tags
static GLIMMER_TEMPLATE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"<template>[\s\S]*?</template>").expect("Invalid Glimmer template regex")
});

#[derive(Debug)]
pub struct InlineStyleViolation {
    range: TextRange,
    element_name: String,
}

impl Rule for NoEmberInlineStyles {
    type Query = Ast<JsModule>;
    type State = InlineStyleViolation;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let _module = ctx.query();

        // Check if this is a Glimmer file (.gjs/.gts)
        let source_type = ctx.source_type::<JsFileSource>();
        if !source_type.as_embedding_kind().is_glimmer() {
            return vec![];
        }

        // Get the ORIGINAL source text (before template extraction)
        // This is crucial because the parsed module has templates replaced with
        // placeholders like __BIOME_GLIMMER_TEMPLATE_0__
        let Some(original_source) = ctx.get_service::<OriginalSourceText>() else {
            return vec![];
        };
        let source = original_source.text();

        // Find all violations in template blocks
        find_inline_style_violations(source)
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let violation = state;

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                violation.range,
                markup! {
                    "Avoid using inline "<Emphasis>"style"</Emphasis>" attributes."
                },
            )
            .note(markup! {
                "Inline styles make it harder to maintain consistent styling across your application."
            })
            .note(markup! {
                "Consider using CSS classes or a CSS-in-JS solution instead."
            })
            .note(markup! {
                "Inline styles may also violate Content Security Policy (CSP) rules."
            }),
        )
    }
}

/// Find all inline style attribute violations in the source text
fn find_inline_style_violations(source: &str) -> Vec<InlineStyleViolation> {
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
                // Check if element has style attribute
                if let Some(attr) = element.find_attribute_by_name("style") {
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

                    violations.push(InlineStyleViolation {
                        range: TextRange::new(absolute_start.into(), absolute_end.into()),
                        element_name,
                    });
                }
            }
        }
    }

    violations
}
