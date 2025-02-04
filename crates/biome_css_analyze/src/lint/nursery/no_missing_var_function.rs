use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_css_syntax::{AnyCssProperty, CssDashedIdentifier, CssDeclaration, CssSyntaxKind};
use biome_diagnostics::Severity;
use biome_rowan::AstNode;

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow missing var function for css variables.
    ///
    /// This rule has the following limitations:
    /// - It only reports custom properties that are defined and accesible within the same source.
    /// - It does not check properties that can contain author-defined identifiers.
    /// - It ignores the following properties:
    ///   - `animation`
    ///   - `animation-name`
    ///   - `counter-increment`
    ///   - `counter-reset`
    ///   - `counter-set`
    ///   - `grid-column`
    ///   - `grid-column-end`
    ///   - `grid-column-start`
    ///   - `grid-row`
    ///   - `grid-row-end`
    ///   - `grid-row-start`
    ///   - `list-style`
    ///   - `list-style-type`
    ///   - `transition`
    ///   - `transition-property`
    ///   - `view-transition-name`
    ///   - `will-change`
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// a {
    ///   --foo: red;
    ///   color: --foo;
    /// }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// .parent {
    ///   --foo: red;
    ///   .child {
    ///     color: --foo;
    ///   }
    /// }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// @property --bar {}
    ///
    /// a {
    ///   color: --bar;
    /// }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// :root {
    ///   --baz: 0;
    /// }
    ///
    /// a {
    ///   --foo: --baz;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// p {
    ///   color: var(--foo);
    /// }
    /// ```
    ///
    /// ```css
    /// p {
    ///   --foo: red;
    ///   color: var(--foo);
    /// }
    /// ```
    ///
    /// ```css
    /// p {
    ///   color: --foo;
    /// }
    /// ```
    ///
    /// ```css
    /// *:root {
    /// --global: red;
    /// }
    ///
    /// a {
    ///     color: var(--global);
    /// }
    /// ```
    ///
    /// ```css
    /// @property --global-value {}
    /// a {
    ///   color: var(--global-value);
    /// }
    /// ```
    ///
    /// ```css
    /// a {
    ///   view-transition-name: --bbb;
    /// }
    /// ```
    ///
    pub NoMissingVarFunction {
        version: "1.9.2",
        name: "noMissingVarFunction",
        language: "css",
        recommended: true,
        severity: Severity::Error,
        sources: &[RuleSource::Stylelint("custom-property-no-missing-var-function")],
    }
}

pub const IGNORED_PROPERTIES: [&str; 18] = [
    "animation",
    "animation-name",
    "container-name",
    "counter-increment",
    "counter-reset",
    "counter-set",
    "grid-column",
    "grid-column-end",
    "grid-column-start",
    "grid-row",
    "grid-row-end",
    "grid-row-start",
    "list-style",
    "list-style-type",
    "transition",
    "transition-property",
    "view-transition-name",
    "will-change",
];

impl Rule for NoMissingVarFunction {
    type Query = Semantic<CssDashedIdentifier>;
    type State = CssDashedIdentifier;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        if is_wrapped_in_var(node) {
            return None;
        }

        let property_name = get_property_name(node)?;
        let custom_variable_name = node.to_trimmed_string();

        if IGNORED_PROPERTIES.contains(&property_name.as_str()) {
            return None;
        }

        let model = ctx.model();
        let rule = model.get_rule_by_range(node.range())?;

        if rule
            .declarations
            .iter()
            .any(|decl| decl.property.name == custom_variable_name)
        {
            return Some(node.clone());
        }

        let mut parent_id = rule.parent_id;
        while let Some(id) = parent_id {
            let parent_rule = model.get_rule_by_id(id)?;
            if parent_rule
                .declarations
                .iter()
                .any(|decl| decl.property.name == custom_variable_name)
            {
                return Some(node.clone());
            }
            parent_id = parent_rule.parent_id;
        }

        if model
            .global_custom_variables()
            .contains_key(&custom_variable_name)
        {
            return Some(node.clone());
        }

        None
    }

    fn diagnostic(_: &RuleContext<Self>, node: &Self::State) -> Option<RuleDiagnostic> {
        let span = node.range();
        let custom_variable_name = node.to_trimmed_string();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                span,
                markup! {
                    "CSS variables '"<Emphasis>{custom_variable_name}</Emphasis>"' is used without the 'var()' function"
                },
            )
            .note(markup! {
                    "CSS variables should be used with the 'var()' function to ensure proper fallback behavior and browser compatibility."
            }),
        )
    }
}

fn is_wrapped_in_var(node: &CssDashedIdentifier) -> bool {
    let mut current_node = node.syntax().parent();
    while let Some(parent) = current_node {
        match parent.kind() {
            // Ignore declarations of custom properties
            // e.g. `--custom-property: {}`
            CssSyntaxKind::CSS_GENERIC_PROPERTY => return true,
            // e.g `color: --custom-property;`
            //             ^^^^^^^^^^^^^^^^ CSS_GENERIC_COMPONENT_VALUE_LIST
            CssSyntaxKind::CSS_GENERIC_COMPONENT_VALUE_LIST => return false,
            CssSyntaxKind::CSS_FUNCTION => return parent.text_trimmed().starts_with("var"),
            _ => {}
        }
        current_node = parent.parent();
    }
    false
}

fn get_property_name(node: &CssDashedIdentifier) -> Option<String> {
    let mut current_node = node.syntax().parent();
    while let Some(parent) = current_node {
        if let Some(node) = CssDeclaration::cast(parent.clone()) {
            let prop = node.property().ok()?;
            return match prop {
                AnyCssProperty::CssBogusProperty(_) => None,
                AnyCssProperty::CssComposesProperty(prop) => {
                    Some(prop.name().ok()?.to_trimmed_string())
                }
                AnyCssProperty::CssGenericProperty(prop) => {
                    Some(prop.name().ok()?.to_trimmed_string())
                }
            };
        }
        current_node = parent.parent();
    }
    None
}
