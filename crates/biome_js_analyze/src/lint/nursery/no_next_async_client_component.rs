use crate::react::components::{
    AnyPotentialReactComponentDeclaration, ReactComponentInfo, ReactComponentKind,
};
use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsExpression, AnyJsRoot};
use biome_rowan::{AstNode, TokenText};
use biome_rule_options::no_next_async_client_component::NoNextAsyncClientComponentOptions;

declare_lint_rule! {
    /// Prevent client components from being async functions.
    ///
    /// This rule prevents the use of async functions for client components in Next.js applications.
    /// Client components marked with "use client" directive should not be async as this can cause
    /// hydration mismatches, break component rendering lifecycle, and lead to unexpected behavior
    /// with React's concurrent features.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// "use client";
    ///
    /// export default async function MyComponent() {
    ///   return <div>Hello</div>;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// "use client";
    ///
    /// export default function MyComponent() {
    ///   return <div>Hello</div>;
    /// }
    /// ```
    ///
    /// ```jsx
    /// // No "use client" directive - server component can be async
    /// export default async function ServerComponent() {
    ///   const data = await fetch('/api/data');
    ///   return <div>{data}</div>;
    /// }
    /// ```
    ///
    pub NoNextAsyncClientComponent {
        version: "2.2.0",
        name: "noNextAsyncClientComponent",
        language: "js",
        sources: &[RuleSource::EslintNext("no-async-client-component").same()],
        recommended: false,
        severity: Severity::Warning,
        domains: &[RuleDomain::Next],
    }
}

impl Rule for NoNextAsyncClientComponent {
    type Query = Ast<AnyPotentialReactComponentDeclaration>;
    type State = Option<TokenText>;
    type Signals = Option<Self::State>;
    type Options = NoNextAsyncClientComponentOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let declaration = ctx.query();
        let component = ReactComponentInfo::from_declaration(declaration.syntax())?;

        // Only check function components
        let ReactComponentKind::Function(_) = component.kind else {
            return None;
        };

        // Check if we're in a module with "use client" directive
        let root = ctx.root();
        let has_use_client = match root {
            AnyJsRoot::JsModule(module) => has_use_client_directive(module.directives()),
            AnyJsRoot::JsScript(script) => has_use_client_directive(script.directives()),
            _ => false,
        };

        if !has_use_client {
            return None;
        }

        // Check if the component function is async
        let is_async = match declaration {
            AnyPotentialReactComponentDeclaration::JsFunctionDeclaration(func) => {
                func.async_token().is_some()
            }
            AnyPotentialReactComponentDeclaration::JsFunctionExportDefaultDeclaration(func) => {
                func.async_token().is_some()
            }
            AnyPotentialReactComponentDeclaration::JsVariableDeclarator(declarator) => declarator
                .initializer()
                .and_then(|init| init.expression().ok())
                .and_then(|expr| match expr {
                    AnyJsExpression::JsArrowFunctionExpression(arrow) => arrow.async_token(),
                    AnyJsExpression::JsFunctionExpression(func) => func.async_token(),
                    _ => None,
                })
                .is_some(),
            AnyPotentialReactComponentDeclaration::JsAssignmentExpression(assignment) => assignment
                .right()
                .ok()
                .and_then(|expr| match expr {
                    AnyJsExpression::JsArrowFunctionExpression(arrow) => arrow.async_token(),
                    AnyJsExpression::JsFunctionExpression(func) => func.async_token(),
                    _ => None,
                })
                .is_some(),
            AnyPotentialReactComponentDeclaration::JsExportDefaultExpressionClause(export) => {
                export
                    .expression()
                    .ok()
                    .and_then(|expr| match expr {
                        AnyJsExpression::JsArrowFunctionExpression(arrow) => arrow.async_token(),
                        AnyJsExpression::JsFunctionExpression(func) => func.async_token(),
                        _ => None,
                    })
                    .is_some()
            }
            AnyPotentialReactComponentDeclaration::JsMethodObjectMember(method) => {
                method.async_token().is_some()
            }
            AnyPotentialReactComponentDeclaration::JsPropertyObjectMember(prop) => prop
                .value()
                .ok()
                .and_then(|expr| match expr {
                    AnyJsExpression::JsArrowFunctionExpression(arrow) => arrow.async_token(),
                    AnyJsExpression::JsFunctionExpression(func) => func.async_token(),
                    _ => None,
                })
                .is_some(),
            AnyPotentialReactComponentDeclaration::JsMethodClassMember(method) => {
                method.async_token().is_some()
            }
            _ => false,
        };

        if !is_async {
            return None;
        }

        let component_name = component
            .name
            .or(component.name_hint)
            .map(|token| token.token_text_trimmed());

        Some(component_name)
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let declaration = ctx.query();

        Some(match state {
            Some(component_name) => {
                let name_text = component_name.text();
                RuleDiagnostic::new(
                    rule_category!(),
                    declaration.range(),
                    markup! {
                        "The component "<Emphasis>{name_text}</Emphasis>" is an async client component, which is not allowed."
                    },
                )
            }
            None => {
                RuleDiagnostic::new(
                    rule_category!(),
                    declaration.range(),
                    markup! {
                        "Async client component are not allowed."
                    },
                )
            }
        }
        .note(markup! {
            "Client components with \"use client\" directive should not be async functions as this can cause hydration mismatches and break React's rendering lifecycle."
        })
        .note(markup! {
            "Consider using useEffect for async operations inside the component, or remove the \"use client\" directive if this should be a server component."
        }))
    }
}

fn has_use_client_directive(
    directives: impl IntoIterator<Item = biome_js_syntax::JsDirective>,
) -> bool {
    directives.into_iter().any(|directive| {
        directive
            .inner_string_text()
            .is_ok_and(|text| text.text() == "use client")
    })
}
