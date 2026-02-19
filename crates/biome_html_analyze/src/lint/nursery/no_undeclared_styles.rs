use crate::services::module_graph::HtmlModuleGraph;
use biome_analyze::{Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_html_syntax::{AnyHtmlAttributeInitializer, HtmlAttribute};
use biome_rowan::{TextRange, TextSize};
use camino::Utf8Path;
use std::collections::HashSet;

declare_lint_rule! {
    /// Reports CSS class names in HTML `class` attributes that are not defined
    /// in any `<style>` block or linked stylesheet in the same file.
    ///
    /// When an HTML file has `<style>` blocks or `<link rel="stylesheet">` elements,
    /// every class name used in `class="..."` attributes is checked against the
    /// available class definitions. Classes that are not defined are reported.
    ///
    /// If the file has no style information (no `<style>` blocks and no linked
    /// stylesheets), this rule does not emit diagnostics to avoid false positives.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic,ignore
    /// <style>.card { border: 1px solid; }</style>
    /// <div class="header">Content</div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html,ignore
    /// <style>.card { border: 1px solid; }</style>
    /// <div class="card">Content</div>
    /// ```
    ///
    pub NoUndeclaredStyles {
        version: "next",
        name: "noUndeclaredStyles",
        language: "html",
        recommended: false,
        issue_number: Some("9156"),
        domains: &[RuleDomain::Project],
    }
}

/// Stores the text range and name of an undeclared class.
pub struct UndeclaredClass {
    /// Range of this class name token within the source file.
    pub range: TextRange,
    /// The class name that was not found.
    pub name: String,
}

impl Rule for NoUndeclaredStyles {
    type Query = HtmlModuleGraph<HtmlAttribute>;
    type State = UndeclaredClass;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let attr = ctx.query();
        let mut signals = Vec::new();

        // Only care about `class="..."` attributes.
        let Ok(name_node) = attr.name() else {
            return signals;
        };
        let Ok(name_token) = name_node.value_token() else {
            return signals;
        };
        if name_token.text_trimmed() != "class" {
            return signals;
        }

        // Get the attribute value string (without quotes).
        let Some(initializer) = attr.initializer() else {
            return signals;
        };
        let Ok(value) = initializer.value() else {
            return signals;
        };
        let html_string = match value {
            AnyHtmlAttributeInitializer::HtmlString(s) => s,
            _ => return signals,
        };
        // `value_token` is needed only to obtain the token's absolute file
        // offset so that diagnostic ranges point to the right location.
        let Ok(value_token) = html_string.value_token() else {
            return signals;
        };
        // `inner_string_text` strips the surrounding quotes and returns a
        // `TokenText` backed by the same green token — no allocation.
        let Ok(inner_text) = html_string.inner_string_text() else {
            return signals;
        };

        let module_graph = ctx.module_graph();
        let file_path = ctx.file_path();

        let Some(html_info) = module_graph.html_module_info_for_path(file_path) else {
            return signals;
        };

        // If there's no style information available, skip this file entirely
        // to avoid all-false-positives on unstyled HTML files.
        if html_info.style_classes.is_empty() && html_info.imported_stylesheets.is_empty() {
            return signals;
        }

        // Collect CssModuleInfo handles for linked stylesheets. These are
        // Arc-backed so cloning is cheap; we keep them alive so we can borrow
        // &str from their class name Text values below.
        let linked_css: Vec<_> = html_info
            .imported_stylesheets
            .iter()
            .filter_map(|p| {
                module_graph
                    .css_module_info_for_path(p.as_path().unwrap_or_else(|| Utf8Path::new("")))
            })
            .collect();

        // Collect all available class names as &str — no allocations needed.
        let mut available: HashSet<&str> = HashSet::new();
        for class in html_info.style_classes.iter() {
            available.insert(class.text());
        }
        for css_info in &linked_css {
            for class in css_info.classes.iter() {
                available.insert(class.text());
            }
        }

        // The inner content starts one byte into the token (after the opening
        // quote). This gives the absolute file offset of the first character
        // of the class list, which anchors the diagnostic ranges.
        let inner_file_start: u32 = u32::from(value_token.text_trimmed_range().start()) + 1;

        // Check each class name in the attribute value.
        let inner = inner_text.text();
        let mut offset: u32 = 0;
        for class_name in inner.split_ascii_whitespace() {
            // Find where this class name starts within `inner` (searching
            // forward from the previous position).
            let class_offset = inner[offset as usize..]
                .find(class_name)
                .map_or(offset, |o| offset + o as u32);

            if !available.contains(class_name) {
                let start = TextSize::from(inner_file_start + class_offset);
                let end = start + TextSize::from(class_name.len() as u32);
                signals.push(UndeclaredClass {
                    range: TextRange::new(start, end),
                    // One allocation per actual diagnostic emitted (not per class checked).
                    name: class_name.to_string(),
                });
            }

            offset = class_offset + class_name.len() as u32;
        }

        signals
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "The CSS class "<Emphasis>{&state.name}</Emphasis>" is not defined in any `<style>` block or linked stylesheet."
                },
            )
            .note(markup! {
                "Referencing undefined classes often indicates a typo or a missing stylesheet, and will result in elements not being styled as intended."
            })
            .note(markup! {
                "Either define this class in a `<style>` block, link a stylesheet that contains it, or remove this class name."
            }),
        )
    }
}
