use crate::services::module_graph::HtmlModuleGraph;
use biome_analyze::{Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_html_syntax::{
    AnyHtmlAttributeInitializer, AnyHtmlTagName, HtmlAttribute, HtmlOpeningElement,
    HtmlSelfClosingElement,
};
use biome_module_graph::{ImportTreeDisplay, ImportTreeNode};
use biome_rowan::{AstNode, TextRange, TextSize};
use biome_rule_options::no_undeclared_classes::NoUndeclaredClassesOptions;
use biome_string_case::StrOnlyExtension;

declare_lint_rule! {
    /// Reports CSS class names in HTML `class` attributes that are not defined
    /// in any `<style>` block or linked stylesheet available to the file.
    ///
    /// When an HTML file has `<style>` blocks or `<link rel="stylesheet">` elements,
    /// every class name used in `class="..."` attributes is checked against the
    /// available class definitions. Classes that are not defined are reported.
    ///
    /// ## Framework style scoping
    ///
    /// Different frameworks scope their embedded styles differently. For the
    /// **same file**, both locally and globally scoped classes are considered
    /// valid — a scoped `<style>` block defines classes that are available to
    /// that component's own template. When traversing **parent files** (via
    /// upward import traversal), only globally scoped classes are visible:
    ///
    /// - **HTML** `<style>`: always global.
    /// - **Vue** `<style>` (no attribute): global.
    /// - **Vue** `<style scoped>`: local — visible within the same component,
    ///   not to child components.
    /// - **Astro** `<style>` (default): local — visible within the same component,
    ///   not to child components.
    /// - **Astro** `<style is:global>`: global.
    /// - **Svelte** `<style>` (default): local — visible within the same component,
    ///   not to child components. Individual selectors inside `:global(...)` within
    ///   a scoped block are still treated as global.
    ///
    /// ## Components
    ///
    /// Components (custom elements) are excluded from this check, as they may receive
    /// class names as props or use scoped styling. A component is identified by:
    /// - Tag names starting with an uppercase letter (e.g., `MyComponent`)
    /// - Tag names containing a hyphen (e.g., `my-component`)
    /// - Member expressions (e.g., `Component.Item`)
    ///
    /// ## No false positives on unstyled files
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
    pub NoUndeclaredClasses {
        version: "next",
        name: "noUndeclaredClasses",
        language: "html",
        recommended: false,
        issue_number: Some("9156"),
        domains: &[RuleDomain::Project],
    }
}

impl Rule for NoUndeclaredClasses {
    type Query = HtmlModuleGraph<HtmlAttribute>;
    type State = UndeclaredClass;
    type Signals = Vec<Self::State>;
    type Options = NoUndeclaredClassesOptions;

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

        // Collect all CSS steps reachable from this file (inline styles, linked
        // stylesheets, and CSS imported by parent files via upward traversal).
        // If no CSS is reachable at all, skip to avoid false positives on
        // completely unstyled files.
        let css_steps: Vec<_> = module_graph
            .traverse_import_tree_for_html_classes(file_path)
            .collect();

        if css_steps.is_empty() {
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

            // Check if this class exists in any of the collected CSS steps.
            let mut found_class = false;
            for step in &css_steps {
                if step
                    .css_classes
                    .iter()
                    .any(|token| token.text() == class_name)
                {
                    found_class = true;
                    break;
                }
            }

            // Only build the import tree for diagnostic display when the class is missing.
            if !found_class {
                let import_tree = module_graph.build_import_tree_for_html(file_path);
                let start = TextSize::from(class_data.inner_file_start + class_offset);
                let end = start + TextSize::from(class_name.len() as u32);
                signals.push(UndeclaredClass {
                    range: TextRange::new(start, end),
                    name: class_name.to_string(),
                    import_tree,
                });
            }

            offset = class_offset + class_name.len() as u32;
        }

        signals
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
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

        // Show the import tree if we checked any CSS files.
        if let Some(import_tree) = &state.import_tree {
            let working_directory = ctx.working_directory();
            let tree_display = ImportTreeDisplay::new(import_tree, working_directory);
            diag = diag.note(markup! {
                "Checked import tree:\n\n"{tree_display}
            });
        }

        Some(diag.note(markup! {
            "Either define this class in a `<style>` block, import a CSS file that contains it, or remove this class name."
        }))
    }
}

/// Stores the text range and name of an undeclared class.
pub struct UndeclaredClass {
    /// Range of this class name token within the source file.
    pub range: TextRange,
    /// The class name that was not found.
    pub name: String,
    /// The import tree structure for displaying which files/CSS were checked.
    pub import_tree: Option<ImportTreeNode>,
}

/// Helper to extract class attribute data needed for analysis.
struct ClassAttributeData {
    inner_text: String,
    inner_file_start: u32,
}

impl NoUndeclaredClasses {
    /// Checks if the attribute belongs to a component (custom element).
    /// Components are identified by:
    /// - Tag names with hyphens (e.g., my-component)
    /// - Tag names starting with uppercase (e.g., MyComponent)
    /// - HtmlComponentName nodes
    fn is_component_element(attr: &HtmlAttribute) -> bool {
        // HtmlAttribute → HtmlAttributeList → HtmlOpeningElement / HtmlSelfClosingElement
        // Walk up to find the enclosing element node.
        let element_node = attr
            .syntax()
            .parent() // HtmlAttributeList
            .and_then(|n| n.parent()); // HtmlOpeningElement or HtmlSelfClosingElement

        let Some(element_node) = element_node else {
            return false;
        };

        // Try casting to opening or self-closing element
        let tag_name = if let Some(opening) = HtmlOpeningElement::cast_ref(&element_node) {
            opening.name().ok()
        } else if let Some(self_closing) = HtmlSelfClosingElement::cast_ref(&element_node) {
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
