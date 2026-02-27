use crate::services::module_graph::ResolvedImports;
use biome_analyze::{Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_syntax::{AnyJsxAttributeValue, JsxAttribute, JsxString};
use biome_module_graph::{ImportTreeDisplay, ImportTreeNode};
use biome_rowan::{TextRange, TextSize, TokenText};

declare_lint_rule! {
    /// Reports CSS class names in JSX `className` or `class` attributes that are not defined
    /// in any imported CSS file.
    ///
    /// When a JSX file imports CSS files, every class name used in `className=` or `class=`
    /// attributes is checked against the available class definitions. Classes that are not
    /// defined are reported.
    ///
    /// This rule only applies to string literals. Dynamic class names (template strings,
    /// expressions, computed values) are not checked.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,ignore
    /// import "./styles.css";
    /// export default () => <div className="missing" />;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js,ignore
    /// import "./styles.css";
    /// export default () => <div className="header" />;
    /// ```
    ///
    pub NoUndeclaredStyles {
        version: "next",
        name: "noUndeclaredStyles",
        language: "js",
        recommended: false,
        issue_number: Some("9156"),
        domains: &[RuleDomain::Project],
    }
}

impl Rule for NoUndeclaredStyles {
    type Query = ResolvedImports<JsxAttribute>;
    type State = UndeclaredClass;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let attr = ctx.query();

        let Some(class_data) = extract_class_attribute(attr) else {
            return Vec::new();
        };

        let module_graph = ctx.module_graph();
        let file_path = ctx.file_path();

        // Check each class name in the attribute value
        let mut signals = Vec::new();
        let mut offset: u32 = 0;

        for class_name in class_data.inner_text.split_ascii_whitespace() {
            // Find where this class name starts within the inner text
            let class_offset = class_data.inner_text[offset as usize..]
                .find(class_name)
                .map_or(offset, |o| offset + o as u32);

            // Lazily check if this class exists in any CSS file using the minimal iterator
            let mut found_class = false;
            for step in module_graph.traverse_import_tree_for_classes(file_path) {
                if step
                    .css_classes
                    .iter()
                    .any(|token| token.text() == class_name)
                {
                    found_class = true;
                    break; // Early termination - found the class!
                }
            }

            // Only if class NOT found, build diagnostic with import tree
            if !found_class {
                let import_tree = module_graph.build_import_tree(file_path);
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
                "The CSS class "<Emphasis>{&state.name}</Emphasis>" is not defined in any imported stylesheet."
            },
        )
        .note(markup! {
            "Referencing undefined classes often indicates a typo or a missing stylesheet, and will result in elements not being styled as intended."
        });

        // Show the import tree with checked CSS files
        if let Some(import_tree) = &state.import_tree {
            let working_directory = ctx.working_directory();
            let tree_display = ImportTreeDisplay::new(import_tree, working_directory);
            diag = diag.note(markup! {
                "Checked import tree:\n\n"{tree_display}
            });
        }

        Some(diag.note(markup! {
            "Either import a CSS file that defines this class or remove this class name."
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
    inner_text: TokenText,
    inner_file_start: u32,
}

/// Extracts className or class attribute value and position if this is a valid class attribute.
fn extract_class_attribute(attr: &JsxAttribute) -> Option<ClassAttributeData> {
    let name_token = attr.name_value_token().ok()?;
    let name_text = name_token.text_trimmed();

    // Check for className (React) or class (SolidJS, etc.)
    if name_text != "className" && name_text != "class" {
        return None;
    }

    let initializer = attr.initializer()?;
    let value = initializer.value().ok()?;

    // We only check string literals (not template strings or expressions)
    let string_literal: JsxString = match value {
        AnyJsxAttributeValue::JsxString(s) => s,
        // Skip expression values for now (they could be template strings, variables, etc.)
        AnyJsxAttributeValue::JsxExpressionAttributeValue(_)
        | AnyJsxAttributeValue::AnyJsxTag(_) => {
            return None;
        }
    };

    let value_token = string_literal.value_token().ok()?;
    let inner_text = string_literal.inner_string_text().ok()?;

    // The inner content starts one byte into the token (after the opening quote).
    let inner_file_start = u32::from(value_token.text_trimmed_range().start()) + 1;

    Some(ClassAttributeData {
        inner_text,
        inner_file_start,
    })
}
