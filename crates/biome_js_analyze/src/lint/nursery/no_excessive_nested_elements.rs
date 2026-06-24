use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, RuleSuppressions, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{JsLanguage, JsxElement};
use biome_rowan::AstNode;
use biome_rule_options::no_excessive_nested_elements::NoExcessiveNestedElementsOptions;

declare_lint_rule! {
    /// Disallow deeply nested JSX elements.
    ///
    /// Deeply nested JSX elements reduce code readability and make it harder to
    /// understand the component structure at a glance. This rule enforces a
    /// maximum nesting depth for JSX elements.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// function Component() {
    ///     return (
    ///         <div>
    ///             <div>
    ///                 <div>
    ///                     <div>
    ///                         <div>
    ///                             <div>
    ///                                 <div>
    ///                                     <div>
    ///                                         <div>
    ///                                             <div>
    ///                                                 <div>
    ///                                                     <span>Too deep!</span>
    ///                                                 </div>
    ///                                             </div>
    ///                                         </div>
    ///                                     </div>
    ///                                 </div>
    ///                             </div>
    ///                         </div>
    ///                     </div>
    ///                 </div>
    ///             </div>
    ///         </div>
    ///     );
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// function Component() {
    ///     return (
    ///         <div>
    ///             <div>
    ///                 <span>Reasonable depth</span>
    ///             </div>
    ///         </div>
    ///     );
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// ### maxDepth
    ///
    /// The maximum allowed nesting depth for JSX elements (default: 10).
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "maxDepth": 3
    ///     }
    /// }
    /// ```
    ///
    pub NoExcessiveNestedElements {
        version: "next",
        name: "noExcessiveNestedElements",
        language: "js",
        sources: &[RuleSource::EslintReact("jsx-max-depth").inspired()],
        recommended: false,
        severity: Severity::Warning,
    }
}

#[derive(Debug)]
pub struct ElementDepth(usize);

impl Rule for NoExcessiveNestedElements {
    type Query = Ast<JsxElement>;
    type State = ElementDepth;
    type Signals = Option<Self::State>;
    type Options = NoExcessiveNestedElementsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();

        let depth = element
            .syntax()
            .ancestors()
            .filter_map(JsxElement::cast)
            .count();

        if depth > ctx.options().max_depth() as usize {
            Some(ElementDepth(depth))
        } else {
            None
        }
    }

    fn suppressed_nodes(
        ctx: &RuleContext<Self>,
        _state: &Self::State,
        suppressions: &mut RuleSuppressions<JsLanguage>,
    ) {
        let element = ctx.query();

        suppressions.suppress_node(element.syntax().clone());

        for nested in element.syntax().descendants().filter_map(JsxElement::cast) {
            suppressions.suppress_node(nested.syntax().clone());
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let element = ctx.query();
        let max = ctx.options().max_depth();
        let depth = state.0;

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                element.range(),
                markup! {
                    "This JSX element is nested too deeply."
                },
            )
            .note(markup! {
                "Elements nested "{depth}" levels deep are harder to read and maintain. The configured maximum is "{max}"."
            })
            .note(markup! {
                "Extract deeply nested JSX into separate components to reduce nesting."
            }),
        )
    }
}
