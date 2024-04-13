use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use biome_js_syntax::{AnyJsFunction, JsReturnStatement};

declare_rule! {
    /// Disallow early returns in components.
    ///
    /// Solid components only run once, and so conditionals should be inside JSX.
    ///
    /// https://github.com/solidjs-community/eslint-plugin-solid/blob/main/docs/components-return-once.md
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function Component() {
    ///   if (condition) {
    ///     return <div />;
    ///   }
    ///   return <span />;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// function Component() {
    ///   return <div />;
    /// }
    /// ```
    ///
    pub ComponentsReturnOnce {
        version: "next",
        name: "componentsReturnOnce",
        // TODO: eslint plugin solid source
        recommended: false,
    }
}

/// Returns `true` if the function is (probably) a component (because it is PascalCase)
fn is_component_name(name: &str) -> bool {
    name.chars()
        .next()
        .map(|x| x.is_uppercase())
        .unwrap_or_default()
}

pub enum ReturnType {
    Conditional,
    EarlyReturn,
}

impl ReturnType {
    pub fn get_message(&self) -> &str {
        match self {
            Self::Conditional => "Solid components run once, so a conditional return breaks reactivity. Move the condition inside a JSX element, such as a fragment or <Show />.",
            Self::EarlyReturn =>  "Solid components run once, so an early return breaks reactivity. Move the condition inside a JSX element, such as a fragment or <Show />."
        }
    }
}

impl Rule for ComponentsReturnOnce {
    type Query = Ast<AnyJsFunction>;
    type State = Vec<(ReturnType, JsReturnStatement)>;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let func = ctx.query();

        if let Some(binding) = func.binding() {
            if let Some(ident) = binding.as_js_identifier_binding() {
                if let Ok(name) = ident.name_token() {
                    if !is_component_name(name.text()) {
                        // NOTE: Only consider components, not normal functions
                        return None;
                    }
                }
            }
        }

        let body = func.body().ok()?;
        let body = body.as_js_function_body()?;

        let mut returns = vec![];

        for statement in body.statements() {
            if let Some(ret) = statement.as_js_return_statement() {
                if let Some(arg) = ret.argument() {
                    if arg.as_js_conditional_expression().is_some() {
                        returns.push((ReturnType::Conditional, ret.to_owned()));
                    }
                }
            }
        }

        Some(returns)
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let mut issues = state.iter();

        if let Some((return_type, statement)) = issues.next() {
            let span = statement.return_token().ok()?.text_range();

            let mut diagnostic =
                RuleDiagnostic::new(rule_category!(), span, return_type.get_message());

            for (return_type, statement) in issues {
                let span = statement.return_token().unwrap().text_range();
                diagnostic = diagnostic.detail(span, return_type.get_message());
            }

            Some(diagnostic)
        } else {
            None
        }
    }
}
