use biome_analyze::{Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsDeclarationClause, AnyJsExportClause, AnyJsExportNamedSpecifier, JsExport,
    JsVariableDeclaration, TextRange, binding_ext::AnyJsBindingDeclaration,
};
use biome_languages::JsFileSource;
use biome_rowan::{AstNode, AstSeparatedList};
use biome_rule_options::no_svelte_export_let::NoSvelteExportLetOptions;

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow declaring Svelte component props with `export let`.
    ///
    /// Svelte 5 declares component props with the `$props()` rune. Exporting a `let` or `var`
    /// binding from the instance `<script>` is the legacy way of declaring props: it opts the
    /// component into legacy mode, and it is a compile error in runes mode.
    ///
    /// Exports from a `<script module>` block are regular module exports, not props, so they
    /// are ignored.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```svelte,expect_diagnostic
    /// <script>
    /// export let name;
    /// </script>
    /// ```
    ///
    /// ```svelte,expect_diagnostic
    /// <script>
    /// let className;
    /// export { className as class };
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```svelte
    /// <script>
    /// let { name, class: className } = $props();
    /// </script>
    /// ```
    ///
    /// ```svelte
    /// <script module>
    /// export let count = 0;
    /// </script>
    /// ```
    ///
    /// ### References
    ///
    /// - [Svelte `$props`](https://svelte.dev/docs/svelte/$props)
    /// - [Svelte 5 migration guide](https://svelte.dev/docs/svelte/v5-migration-guide#Let-exports-become-$props)
    pub NoSvelteExportLet {
        version: "next",
        name: "noSvelteExportLet",
        language: "js",
        domains: &[RuleDomain::Svelte],
        recommended: true,
    }
}

pub enum RuleState {
    /// `export let name;`
    Declaration(JsVariableDeclaration),
    /// `export { name };` where `name` is a `let` or `var` binding.
    Specifier(AnyJsExportNamedSpecifier),
}

impl Rule for NoSvelteExportLet {
    type Query = Semantic<JsExport>;
    type State = RuleState;
    type Signals = Box<[Self::State]>;
    type Options = NoSvelteExportLetOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx
            .source_type::<JsFileSource>()
            .as_embedding_kind()
            .is_svelte_instance_script()
        {
            return Box::default();
        }

        match ctx.query().export_clause() {
            Ok(AnyJsExportClause::AnyJsDeclarationClause(
                AnyJsDeclarationClause::JsVariableDeclarationClause(clause),
            )) => clause
                .declaration()
                .ok()
                .filter(|declaration| declaration.is_let() || declaration.is_var())
                .map(RuleState::Declaration)
                .into_iter()
                .collect(),
            Ok(AnyJsExportClause::JsExportNamedClause(clause)) => clause
                .specifiers()
                .iter()
                .flatten()
                .filter(|specifier| is_let_or_var_specifier(ctx.model(), specifier))
                .map(RuleState::Specifier)
                .collect(),
            _ => Box::default(),
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = match state {
            RuleState::Declaration(declaration) => {
                let export_token = ctx.query().export_token().ok()?;
                let kind_token = declaration.kind().ok()?;
                RuleDiagnostic::new(
                    rule_category!(),
                    TextRange::new(
                        export_token.text_trimmed_range().start(),
                        kind_token.text_trimmed_range().end(),
                    ),
                    markup! {
                        "Declaring component props with "<Emphasis>"export "{kind_token.text_trimmed()}</Emphasis>" is legacy Svelte syntax."
                    },
                )
            }
            RuleState::Specifier(specifier) => {
                let local_name = specifier.local_name().ok()?.value_token().ok()?;
                RuleDiagnostic::new(
                    rule_category!(),
                    specifier.range(),
                    markup! {
                        "Exporting "<Emphasis>{local_name.text_trimmed()}</Emphasis>" declares a component prop with legacy Svelte syntax."
                    },
                )
            }
        };
        Some(
            diagnostic
                .note(markup! {
                    "Svelte 5 declares props with the "<Emphasis>"$props()"</Emphasis>" rune. Declaring props with "<Emphasis>"export"</Emphasis>" makes the component run in legacy mode, and is a compile error in runes mode."
                })
                .note(markup! {
                    "Destructure the props from "<Emphasis>"$props()"</Emphasis>" instead, for example: "<Emphasis>"let { name } = $props();"</Emphasis>". See the "<Hyperlink href="https://svelte.dev/docs/svelte/v5-migration-guide#Let-exports-become-$props">"Svelte 5 migration guide"</Hyperlink>"."
                }),
        )
    }
}

/// Returns `true` if the specifier exports a binding declared with `let` or `var`.
fn is_let_or_var_specifier(model: &SemanticModel, specifier: &AnyJsExportNamedSpecifier) -> bool {
    if specifier.exports_only_types() {
        return false;
    }
    let Some(binding) = specifier
        .local_name()
        .ok()
        .and_then(|reference| model.binding(&reference))
    else {
        return false;
    };
    let Some(declaration) = binding.tree().declaration() else {
        return false;
    };
    let declaration = declaration
        .parent_binding_pattern_declaration()
        .unwrap_or(declaration);
    let AnyJsBindingDeclaration::JsVariableDeclarator(declarator) = declaration else {
        return false;
    };
    declarator
        .declaration()
        .is_some_and(|declaration| declaration.is_let() || declaration.is_var())
}
