use crate::react::components::{
    AnyPotentialReactComponentDeclaration, ReactComponentInfo, ReactComponentKind,
};
use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::AnyJsRoot;
use biome_rowan::{AstNode, AstNodeList};
use biome_rule_options::no_async_client_component::NoAsyncClientComponentOptions;

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
    pub NoAsyncClientComponent {
        version: "next",
        name: "noAsyncClientComponent",
        language: "js",
        sources: &[RuleSource::EslintNext("no-async-client-component").same()],
        recommended: false,
        severity: Severity::Warning,
        domains: &[RuleDomain::Next],
    }
}

impl Rule for NoAsyncClientComponent {
    type Query = Ast<AnyPotentialReactComponentDeclaration>;
    type State = String;
    type Signals = Option<Self::State>;
    type Options = NoAsyncClientComponentOptions;

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
            AnyJsRoot::JsModule(module) => module.directives().iter().any(|directive| {
                directive
                    .inner_string_text()
                    .is_ok_and(|text| text.text() == "use client")
            }),
            AnyJsRoot::JsScript(script) => script.directives().iter().any(|directive| {
                directive
                    .inner_string_text()
                    .is_ok_and(|text| text.text() == "use client")
            }),
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
            _ => false,
        };

        if !is_async {
            return None;
        }

        let component_name = component.name.or(component.name_hint).map_or_else(
            || "Component".to_string(),
            |token| token.text_trimmed().to_string(),
        );

        Some(component_name)
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let declaration = ctx.query();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                declaration.range(),
                markup! {
                    "Async client component "<Emphasis>{state}</Emphasis>" is not allowed."
                },
            )
            .note(markup! {
                "Client components with \"use client\" directive should not be async functions as this can cause hydration mismatches and break React's rendering lifecycle."
            })
            .note(markup! {
                "Consider using useEffect for async operations inside the component, or remove the \"use client\" directive if this should be a server component."
            }),
        )
    }
}
