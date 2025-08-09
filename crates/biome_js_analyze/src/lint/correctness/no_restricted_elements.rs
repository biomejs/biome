use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, Rule, RuleDiagnostic, RuleSource, declare_lint_rule};
use biome_console::markup;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_rowan::AstNode;
use biome_rule_options::no_restricted_elements::NoRestrictedElementsOptions;

declare_lint_rule! {
    /// Disallow the use of configured elements.
    ///
    /// This rule disallows the use of configured elements. Without elements configured,
    /// this rule doesn't do anything.
    ///
    /// This rule is useful in situations where you want to enforce the use of specific components
    /// instead of certain HTML or custom elements. For example, in a React project,
    /// you might want to ensure that developers use a custom `TextField` component
    /// instead of the native `<input>` element to maintain consistency and apply
    /// custom styling or behavior.
    ///
    /// Here are some scenarios where this rule can be beneficial:
    ///
    /// * Consistency: Ensuring that all input fields use a custom component instead of the native
    ///   element to maintain a consistent look and feel across the application.
    /// * Accessibility: Enforcing the use of custom components that have built-in
    ///   accessibility features, ensuring that the application is accessible to all users.
    /// * Custom Behavior: Requiring the use of components that encapsulate specific business logic
    ///   or validation, reducing the risk of errors and improving code maintainability.
    /// * Styling: Ensuring that all elements adhere to the design system by using
    ///   custom components that apply consistent styling.
    ///
    /// By disallowing certain elements and enforcing the use of custom components,
    /// this rule helps maintain code quality and consistency across the codebase.
    ///
    /// ## Options
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "elements": {
    ///             "input": "input is not allowed, use TextField component instead",
    ///             "CustomComponent": "deprecated"
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// Restricting the use of HTML elements:
    ///
    /// ```jsx,expect_diagnostic,use_options
    /// <input />
    /// ```
    ///
    /// Restricting the use of custom components:
    ///
    /// ```jsx,expect_diagnostic,use_options
    /// <CustomComponent />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <TextField />
    /// ```
    pub NoRestrictedElements {
        version: "2.0.0",
        name: "noRestrictedElements",
        language: "jsx",
        sources: &[
            RuleSource::EslintReact("forbid-elements").same(),
        ],
        recommended: false,
    }
}

impl Rule for NoRestrictedElements {
    type Query = Ast<AnyJsxElement>;
    type State = Box<str>;
    type Signals = Option<Self::State>;
    type Options = NoRestrictedElementsOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let options = ctx.options();
        let node = ctx.query();
        let element_name = node.name().ok()?.syntax().text_trimmed().into_text();
        options.elements.get(element_name.text()).cloned()
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            markup! { {state} }.to_owned(),
        ))
    }
}
