use crate::services::semantic::Semantic;
use biome_analyze::{Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsFunction, AnyJsIdentifierReference, JsIdentifierBinding, JsReferenceIdentifier,
    JsSyntaxKind, JsVariableDeclarator,
};
use biome_languages::JsFileSource;
use biome_rowan::{AstNode, TextRange, WalkEvent};
use biome_rule_options::no_astro_invalid_get_static_paths_reference::NoAstroInvalidGetStaticPathsReferenceOptions;

declare_lint_rule! {
    /// Disallow references to frontmatter-local values from Astro's `getStaticPaths`.
    ///
    /// Astro executes `getStaticPaths` separately from the component's frontmatter.
    /// Move required values inside the function or import them from another module.
    /// Imports, exported declarations, local variables, and type-only references are allowed.
    /// This rule checks exported functions and function-valued variables named `getStaticPaths`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```astro,expect_diagnostic
    /// ---
    /// const posts = ["first"];
    /// export function getStaticPaths() {
    ///   return posts.map(post => ({ params: { post } }));
    /// }
    /// ---
    /// ```
    ///
    /// ### Valid
    ///
    /// ```astro
    /// ---
    /// export function getStaticPaths() {
    ///   const posts = ["first"];
    ///   return posts.map(post => ({ params: { post } }));
    /// }
    /// ---
    /// ```
    ///
    /// ## References
    ///
    /// - [Astro getStaticPaths](https://docs.astro.build/en/reference/routing-reference/#getstaticpaths)
    pub NoAstroInvalidGetStaticPathsReference {
        version: "next",
        name: "noAstroInvalidGetStaticPathsReference",
        language: "js",
        recommended: false,
        severity: Severity::Error,
        domains: &[RuleDomain::Astro],
    }
}

impl Rule for NoAstroInvalidGetStaticPathsReference {
    type Query = Semantic<AnyJsFunction>;
    type State = (TextRange, TextRange);
    type Signals = Box<[Self::State]>;
    type Options = NoAstroInvalidGetStaticPathsReferenceOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx
            .source_type::<JsFileSource>()
            .as_embedding_kind()
            .is_astro_frontmatter()
        {
            return Box::default();
        }
        let function = ctx.query();
        let Some(id) = function_binding(function) else {
            return Box::default();
        };
        let Ok(name) = id.name_token() else {
            return Box::default();
        };
        if name.text_trimmed() != "getStaticPaths" || !ctx.model().as_binding(&id).is_exported() {
            return Box::default();
        }
        function
            .syntax()
            .preorder_of_kinds(JsReferenceIdentifier::KIND_SET)
            .filter_map(|event| {
                let WalkEvent::Enter(node) = event else {
                    return None;
                };
                let reference = JsReferenceIdentifier::cast(node)?;
                if AnyJsIdentifierReference::from(reference.clone()).is_only_type() {
                    return None;
                }
                let binding = ctx.model().binding(&reference)?;
                if function.range().contains_range(binding.range())
                    || binding.is_imported()
                    || binding.is_exported()
                {
                    return None;
                }
                Some((reference.range(), binding.range()))
            })
            .collect()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(rule_category!(), state.0, markup! {
            "This frontmatter value is unavailable inside getStaticPaths."
        }).detail(state.1, markup! { "This value is declared outside the function." })
        .note(markup! { "Astro executes getStaticPaths in a separate scope before rendering the component." })
        .note(markup! { "Move the declaration inside getStaticPaths or import the value from another module." }))
    }
}

fn function_binding(function: &AnyJsFunction) -> Option<JsIdentifierBinding> {
    if let AnyJsFunction::JsFunctionDeclaration(declaration) = function {
        return declaration.id().ok()?.as_js_identifier_binding().cloned();
    }
    for parent in function.syntax().ancestors().skip(1) {
        if let Some(declarator) = JsVariableDeclarator::cast_ref(&parent) {
            return declarator
                .id()
                .ok()?
                .as_any_js_binding()?
                .as_js_identifier_binding()
                .cloned();
        }
        if !matches!(
            parent.kind(),
            JsSyntaxKind::JS_INITIALIZER_CLAUSE
                | JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION
                | JsSyntaxKind::TS_AS_EXPRESSION
                | JsSyntaxKind::TS_SATISFIES_EXPRESSION
        ) {
            break;
        }
    }
    None
}
