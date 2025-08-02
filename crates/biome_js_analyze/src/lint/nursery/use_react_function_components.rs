use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_rowan::{AstNode, AstNodeList};
use biome_rule_options::use_react_function_components::UseReactFunctionComponentsOptions;

use crate::react::components::{
    AnyPotentialReactComponentDeclaration, ReactComponentInfo, ReactComponentKind,
};

declare_lint_rule! {
    /// Enforce that components are defined as functions and never as classes.
    ///
    /// React in particular allows users to create components using functions or classes.
    /// However, using functions is generally preferred. This rule enforces the use of function components.
    ///
    /// This rule makes an exception for class components that implement `componentDidCatch` because there is
    /// currently no hook alternative for React. This function is typically used for defining error boundaries.
    /// It's recommended to define your error boundary once and then reuse it across your application.
    ///
    /// If you are using Preact, it has a [`useErrorBoundary`](https://preactjs.com/guide/v10/hooks/#useerrorboundary) hook.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// class Foo extends React.Component {
    ///   render() {
    ///     return (
    ///       <div>This is a class component.</div>
    ///     );
    ///   }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// function Foo() {
    ///   return <div>This is a function component.</div>;
    /// }
    /// ```
    ///
    pub UseReactFunctionComponents {
        version: "2.1.3",
        name: "useReactFunctionComponents",
        language: "jsx",
        recommended: false,
        domains: &[RuleDomain::React],
        sources: &[RuleSource::ReactPreferFunctionComponent("react-prefer-function-component").same()],
    }
}

impl Rule for UseReactFunctionComponents {
    type Query = Ast<AnyPotentialReactComponentDeclaration>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseReactFunctionComponentsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let info = ReactComponentInfo::from_declaration(node.syntax())?;

        match info.kind {
            ReactComponentKind::Class(_) if has_component_did_catch(node) => None,
            ReactComponentKind::Class(_) => Some(()),
            ReactComponentKind::Function(_) => None,
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Class components are not allowed. Function components are the preferred way to write components."
                },
            )
            .note(markup! {
                "Refactor this into a function component."
            }),
        )
    }
}

/// Checks if a class component has a `componentDidCatch` method.
fn has_component_did_catch(node: &AnyPotentialReactComponentDeclaration) -> bool {
    use AnyPotentialReactComponentDeclaration::*;
    match node {
        JsClassExportDefaultDeclaration(js_class_export_default_declaration) => {
            js_class_export_default_declaration
                .members()
                .iter()
                .any(|member| {
                    member
                        .name()
                        .ok()
                        .flatten()
                        .and_then(|name| name.name())
                        .is_some_and(|name| name.text() == "componentDidCatch")
                })
        }
        JsClassDeclaration(js_class_declaration) => {
            js_class_declaration.members().iter().any(|member| {
                member
                    .name()
                    .ok()
                    .flatten()
                    .and_then(|name| name.name())
                    .is_some_and(|name| name.text() == "componentDidCatch")
            })
        }
        _ => false,
    }
}
