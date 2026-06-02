use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_semantic::Binding;
use biome_js_syntax::{
    AnyJsExpression, JsFileSource, JsLabeledStatement, JsReferenceIdentifier, JsSyntaxKind,
    JsVariableDeclaration, JsVariableDeclarator,
};
use biome_rowan::AstNode;
use biome_rule_options::no_svelte_immutable_reactive_statements::NoSvelteImmutableReactiveStatementsOptions;

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow reactive statements that reference only immutable values.
    ///
    /// In Svelte 3 and 4, a reactive statement (`$:`) re-runs whenever one of the reactive
    /// values it references changes. When every referenced value is immutable, the statement
    /// runs only once, just like a regular statement. Marking it reactive is then misleading:
    /// it suggests the statement reacts to changes when it cannot.
    ///
    /// Because Biome analyzes the `<script>` block in isolation, it cannot see reassignments
    /// that happen in the template (for example `on:click={() => count++}`). To avoid false
    /// positives, only `const` bindings initialized with a literal or a function and `import`ed
    /// bindings are considered immutable. A `let`/`var`, a prop (`export let`), a store accessed
    /// with the `$` prefix, or a `const` holding an object whose members may be mutated are all
    /// treated as potentially reactive.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```svelte,expect_diagnostic
    /// <script>
    /// const base = 1;
    /// $: doubled = base * 2;
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```svelte
    /// <script>
    /// let count = 0;
    /// $: doubled = count * 2;
    /// </script>
    /// ```
    ///
    /// ```svelte
    /// <script>
    /// export let base;
    /// $: doubled = base * 2;
    /// </script>
    /// ```
    ///
    pub NoSvelteImmutableReactiveStatements {
        version: "next",
        name: "noSvelteImmutableReactiveStatements",
        language: "js",
        domains: &[RuleDomain::Svelte],
        sources: &[RuleSource::EslintSvelte("no-immutable-reactive-statements").same()],
        recommended: false,
    }
}

impl Rule for NoSvelteImmutableReactiveStatements {
    type Query = Semantic<JsLabeledStatement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoSvelteImmutableReactiveStatementsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let labeled_statement = ctx.query();

        // Reactive statements are labeled with `$` and only exist in Svelte components.
        if labeled_statement.label_token().ok()?.text_trimmed() != "$" {
            return None;
        }
        if !ctx
            .source_type::<JsFileSource>()
            .as_embedding_kind()
            .is_svelte()
        {
            return None;
        }

        let model = ctx.model();
        let body = labeled_statement.body().ok()?;
        let statement_range = labeled_statement.range();

        let mut has_immutable_dependency = false;
        for node in body.syntax().descendants() {
            match node.kind() {
                // Mutating a member (`obj.prop = …`) or using `++`/`--` is a side effect that
                // the author may genuinely want to re-run. Stay on the safe side and bail out.
                JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT
                | JsSyntaxKind::JS_COMPUTED_MEMBER_ASSIGNMENT
                | JsSyntaxKind::JS_PRE_UPDATE_EXPRESSION
                | JsSyntaxKind::JS_POST_UPDATE_EXPRESSION => {
                    return None;
                }
                JsSyntaxKind::JS_REFERENCE_IDENTIFIER => {
                    let reference = JsReferenceIdentifier::unwrap_cast(node);
                    let name = reference.value_token().ok()?;
                    // `$`-prefixed identifiers are reactive: stores (`$store`), built-ins
                    // (`$$props`), or runes (`$state`).
                    if name.text_trimmed().starts_with('$') {
                        return None;
                    }

                    let Some(binding) = model.binding(&reference) else {
                        // Unresolved or global references could be reactive, so don't report.
                        return None;
                    };

                    // Ignore bindings declared inside the reactive statement itself; they are
                    // locals, not reactive dependencies.
                    if statement_range.contains(binding.syntax().text_trimmed_range().start()) {
                        continue;
                    }

                    if !is_immutable_binding(&binding) {
                        return None;
                    }

                    has_immutable_dependency = true;
                }
                _ => {}
            }
        }

        has_immutable_dependency.then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "This reactive statement only references immutable values, so it never re-runs."
                },
            )
            .note(markup! {
                "A reactive statement ("<Emphasis>"$:"</Emphasis>") re-runs when the reactive values it references change. "
                "Since every referenced value is immutable, this runs only once, like a regular statement."
            })
            .note(markup! {
                "Reference a reactive value (a "<Emphasis>"let"</Emphasis>" that is reassigned, a store, or a prop), or convert it to a regular statement."
            }),
        )
    }
}

/// Returns whether the given binding is guaranteed to be immutable, even taking into account
/// reassignments that could happen in the Svelte template (which Biome cannot see).
///
/// Only `import`ed bindings and `const` bindings initialized with a literal or a function qualify.
fn is_immutable_binding(binding: &Binding) -> bool {
    if binding.is_imported() {
        return true;
    }

    let Some(declarator) = binding
        .syntax()
        .ancestors()
        .find_map(JsVariableDeclarator::cast)
    else {
        return false;
    };
    let is_const = declarator
        .syntax()
        .ancestors()
        .find_map(JsVariableDeclaration::cast)
        .is_some_and(|declaration| declaration.is_const());
    if !is_const {
        return false;
    }

    // A `const` holding an object or array can still have its members mutated elsewhere, so only
    // literals and functions are treated as immutable.
    matches!(
        declarator.initializer().and_then(|it| it.expression().ok()),
        Some(
            AnyJsExpression::AnyJsLiteralExpression(_)
                | AnyJsExpression::JsFunctionExpression(_)
                | AnyJsExpression::JsArrowFunctionExpression(_)
        )
    )
}
