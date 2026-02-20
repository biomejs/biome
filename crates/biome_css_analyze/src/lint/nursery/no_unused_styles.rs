use crate::services::module_graph::CssModuleGraph;
use biome_analyze::{Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_css_syntax::{CssClassSelector, CssPseudoClassFunctionSelector};
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Reports CSS class selectors that are never referenced in any JSX or HTML file.
    ///
    /// This rule checks all CSS class selectors (`.foo`) in a CSS file and verifies
    /// that each class name is referenced somewhere in a `class=` or `className=`
    /// attribute in an HTML or JSX file that imports (directly or transitively) the
    /// CSS file.
    ///
    /// Classes inside `:global(.foo)` are excluded from this check, as they are
    /// intended to be used by external consumers without explicit imports.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,file=styles.css
    /// .unused { color: red; }  /* Class "unused" is never referenced */
    /// .used { color: blue; }
    /// ```
    ///
    /// ```jsx,file=App.jsx
    /// import "./styles.css";
    /// export default () => <div className="used" />;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css,file=styles.css
    /// .button { color: blue; }
    /// .container { padding: 1rem; }
    /// ```
    ///
    /// ```jsx,file=App.jsx
    /// import "./styles.css";
    /// export default () => (
    ///     <div className="container">
    ///         <button className="button">Click</button>
    ///     </div>
    /// );
    /// ```
    ///
    pub NoUnusedStyles {
        version: "next",
        name: "noUnusedStyles",
        language: "css",
        recommended: false,
        issue_number: Some("9156"),
        domains: &[RuleDomain::Project],
    }
}

impl Rule for NoUnusedStyles {
    type Query = CssModuleGraph<CssClassSelector>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        // Skip classes inside :global(.foo) — they are globally scoped
        if is_inside_global_pseudo(node) {
            return None;
        }

        let class_name = node.name().ok()?.value_token().ok()?;
        let class_name_text = class_name.text_trimmed();

        let module_graph = ctx.module_graph();
        let file_path = ctx.file_path();

        if module_graph.is_class_referenced_by_importers(file_path, class_name_text) {
            return None;
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let class_name = node.name().ok()?.value_token().ok()?;
        let class_name_text = class_name.text_trimmed();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "The CSS class "<Emphasis>{class_name_text}</Emphasis>" is not referenced in any HTML or JSX file."
                },
            )
            .note(markup! {
                "Unused CSS classes add dead weight to your stylesheet, increase bundle size, and make the codebase harder to maintain."
            })
            .note(markup! {
                "Either use this class somewhere or remove the selector."
            }),
        )
    }
}

/// Returns `true` if the given `CssClassSelector` is a descendant of a
/// `:global(...)` pseudo-class function selector.
fn is_inside_global_pseudo(node: &CssClassSelector) -> bool {
    node.syntax().ancestors().any(|ancestor| {
        if let Some(func) = CssPseudoClassFunctionSelector::cast(ancestor) {
            func.name()
                .ok()
                .and_then(|n| n.value_token().ok())
                .is_some_and(|t| t.text_trimmed() == "global")
        } else {
            false
        }
    })
}
