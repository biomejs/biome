use crate::services::module_graph::HtmlModuleGraph;
use biome_analyze::{Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_html_syntax::{
    AnyHtmlAttributeInitializer, AnyHtmlTagName, HtmlAttribute, HtmlOpeningElement,
    HtmlSelfClosingElement,
};
use biome_module_graph::CssTraversalStep;
use biome_rowan::{AstNode, TextRange, TextSize};
use biome_string_case::StrOnlyExtension;

declare_lint_rule! {
    /// Reports CSS class names in HTML `class` attributes that are not defined
    /// in any `<style>` block or linked stylesheet in the same file.
    ///
    /// When an HTML file has `<style>` blocks or `<link rel="stylesheet">` elements,
    /// every class name used in `class="..."` attributes is checked against the
    /// available class definitions. Classes that are not defined are reported.
    ///
    /// Components (custom elements) are excluded from this check, as they may receive
    /// class names as props or use scoped styling. A component is identified by:
    /// - Tag names starting with an uppercase letter (e.g., `MyComponent`)
    /// - Tag names containing a hyphen (e.g., `my-component`)
    /// - Member expressions (e.g., `Component.Item`)
    ///
    /// If the file has no style information (no `<style>` blocks and no linked
    /// stylesheets), this rule does not emit diagnostics to avoid false positives.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,ignore
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
    /// ```html,ignore
    /// <style>.card { border: 1px solid; }</style>
    /// <MyComponent class="any-class">Components are not checked</MyComponent>
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

impl Rule for NoUndeclaredStyles {
    type Query = HtmlModuleGraph<HtmlAttribute>;
    type State = UndeclaredClass;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let attr = ctx.query();

        let Some(class_data) = Self::extract_class_attribute(attr) else {
            return Vec::new();
        };

        // Skip components (custom elements) in framework files.
        if Self::is_component_element(attr) {
            return Vec::new();
        }

        let module_graph = ctx.module_graph();
        let file_path = ctx.file_path();

        // Collect all available CSS classes by traversing the import tree
        let (available_classes, traversal_path) =
            module_graph.collect_available_classes_from_import_tree(file_path);

        // If there's no style information available, skip this file entirely
        // to avoid all-false-positives on unstyled HTML files.
        if available_classes.is_empty() {
            return Vec::new();
        }

        // Check each class name in the attribute value.
        let mut signals = Vec::new();
        let mut offset: u32 = 0;

        for class_name in class_data.inner_text.split_ascii_whitespace() {
            // Find where this class name starts within the inner text.
            let class_offset = class_data.inner_text[offset as usize..]
                .find(class_name)
                .map_or(offset, |o| offset + o as u32);

            if !available_classes.contains(class_name) {
                let start = TextSize::from(class_data.inner_file_start + class_offset);
                let end = start + TextSize::from(class_name.len() as u32);
                signals.push(UndeclaredClass {
                    range: TextRange::new(start, end),
                    name: class_name.to_string(),
                    traversal_path: traversal_path.clone(),
                });
            }

            offset = class_offset + class_name.len() as u32;
        }

        signals
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let mut diag = RuleDiagnostic::new(
            rule_category!(),
            state.range,
            markup! {
                "The CSS class "<Emphasis>{&state.name}</Emphasis>" is not defined in any available stylesheet."
            },
        )
        .note(markup! {
            "Referencing undefined classes often indicates a typo or a missing stylesheet, and will result in elements not being styled as intended."
        });

        // Show the traversal path if we checked any CSS files
        if !state.traversal_path.is_empty() {
            // Build a formatted string showing all checked files
            let mut checked_files = String::from("Checked the following CSS files:\n");
            for step in &state.traversal_path {
                let path_display = step.css_path.as_str();
                if step.is_direct {
                    checked_files.push_str(&format!(
                        "  - {} (linked via <link rel=\"stylesheet\">)\n",
                        path_display
                    ));
                } else {
                    let parent_display = step.importer_path.as_str();
                    checked_files.push_str(&format!(
                        "  - {} (imported by {})\n",
                        path_display, parent_display
                    ));
                }
            }

            diag = diag.note(markup! {
                {checked_files}
            });
        }

        Some(
            diag.note(markup! {
                "Either define this class in a `<style>` block, import a CSS file that contains it, or remove this class name."
            }),
        )
    }
}

/// Stores the text range and name of an undeclared class.
pub struct UndeclaredClass {
    /// Range of this class name token within the source file.
    pub range: TextRange,
    /// The class name that was not found.
    pub name: String,
    /// The import tree traversal path showing which CSS files were checked.
    pub traversal_path: Vec<CssTraversalStep>,
}

/// Helper to extract class attribute data needed for analysis.
struct ClassAttributeData {
    inner_text: String,
    inner_file_start: u32,
}

impl NoUndeclaredStyles {
    /// Checks if the attribute belongs to a component (custom element).
    /// Components are identified by:
    /// - Tag names with hyphens (e.g., my-component)
    /// - Tag names starting with uppercase (e.g., MyComponent)
    /// - HtmlComponentName nodes
    fn is_component_element(attr: &HtmlAttribute) -> bool {
        let Some(parent) = attr.syntax().parent() else {
            return false;
        };

        // Try casting to opening or self-closing element
        let tag_name = if let Some(opening) = HtmlOpeningElement::cast_ref(&parent) {
            opening.name().ok()
        } else if let Some(self_closing) = HtmlSelfClosingElement::cast_ref(&parent) {
            self_closing.name().ok()
        } else {
            None
        };

        let Some(tag_name) = tag_name else {
            return false;
        };

        match tag_name {
            // Explicit component node type
            AnyHtmlTagName::HtmlComponentName(_) => true,
            // Member expressions like Component.Item
            AnyHtmlTagName::HtmlMemberName(_) => true,
            // Regular tag names - check for component patterns
            AnyHtmlTagName::HtmlTagName(name) => {
                let Ok(token) = name.value_token() else {
                    return false;
                };
                let text = token.text_trimmed();

                // Component if starts with uppercase or contains hyphen
                text.chars().next().is_some_and(|c| c.is_uppercase()) || text.contains('-')
            }
        }
    }

    /// Extracts class attribute value and position if this is a valid class attribute.
    fn extract_class_attribute(attr: &HtmlAttribute) -> Option<ClassAttributeData> {
        let name_node = attr.name().ok()?;
        let name_token = name_node.value_token().ok()?;

        if name_token.text_trimmed().to_lowercase_cow() != "class" {
            return None;
        }

        let initializer = attr.initializer()?;
        let value = initializer.value().ok()?;
        let html_string = match value {
            AnyHtmlAttributeInitializer::HtmlString(s) => s,
            _ => return None,
        };

        let value_token = html_string.value_token().ok()?;
        let inner_text = html_string.inner_string_text().ok()?;

        // The inner content starts one byte into the token (after the opening quote).
        let inner_file_start = u32::from(value_token.text_trimmed_range().start()) + 1;

        Some(ClassAttributeData {
            inner_text: inner_text.text().to_string(),
            inner_file_start,
        })
    }
}
