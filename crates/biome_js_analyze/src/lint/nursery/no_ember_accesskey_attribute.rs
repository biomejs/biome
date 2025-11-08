use biome_analyze::{context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_html_parser::{parse_html, HtmlParseOptions};
use biome_html_syntax::{AnyHtmlElement, HtmlFileSource};
use biome_js_syntax::{JsFileSource, JsModule};
use biome_rowan::{AstNode, TextRange};
use regex::Regex;
use std::sync::LazyLock;

declare_lint_rule! {
    /// Disallow the `accesskey` attribute on HTML elements and components.
    ///
    /// The `accesskey` attribute defines keyboard shortcuts to activate or focus elements.
    /// However, it should be avoided because:
    /// - Keyboard shortcuts vary across browsers and operating systems
    /// - Screen readers may have conflicts with the shortcuts
    /// - Shortcuts may conflict with assistive technology shortcuts
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```gjs,expect_diagnostic
    /// <template>
    ///   <button accesskey="s">Save</button>
    /// </template>
    /// ```
    ///
    /// ```gjs,expect_diagnostic
    /// <template>
    ///   <Button accesskey="s" />
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```gjs
    /// <template>
    ///   <button>Save</button>
    ///   <button aria-label="Save">S</button>
    /// </template>
    /// ```
    ///
    pub NoEmberAccesskeyAttribute {
        version: "next",
        name: "noEmberAccesskeyAttribute",
        language: "js",
        recommended: true,
    }
}

/// Regex to match Glimmer <template> tags
static GLIMMER_TEMPLATE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"<template>[\s\S]*?</template>").expect("Invalid Glimmer template regex")
});

#[derive(Debug)]
pub struct AccesskeyViolation {
    range: TextRange,
    element_name: String,
}

impl Rule for NoEmberAccesskeyAttribute {
    type Query = Ast<JsModule>;
    type State = Vec<AccesskeyViolation>;
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
        let violations = find_accesskey_violations(&source);

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
                    "Avoid using the "<Emphasis>"accesskey"</Emphasis>" attribute."
                },
            )
            .note(markup! {
                "Keyboard shortcuts using the "<Emphasis>"accesskey"</Emphasis>" attribute are inconsistent across browsers and operating systems."
            })
            .note(markup! {
                "Consider using a more accessible approach like visible buttons with clear labels or keyboard event handlers."
            }),
        )
    }
}

/// Find all accesskey attribute violations in the source text
fn find_accesskey_violations(source: &str) -> Vec<AccesskeyViolation> {
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
                // Check if element has accesskey attribute
                if let Some(attr) = element.find_attribute_by_name("accesskey") {
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

                    violations.push(AccesskeyViolation {
                        range: TextRange::new(absolute_start.into(), absolute_end.into()),
                        element_name,
                    });
                }
            }
        }
    }

    violations
}
