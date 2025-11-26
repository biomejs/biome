use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_syntax::{JsCallExpression, global_identifier};
use biome_rowan::{AstNode, TextRange};

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow usage of `get()` to access properties.
    ///
    /// Modern Ember uses native property access instead of the `get()` function.
    /// The `get()` function from `@ember/object` is deprecated and should be replaced
    /// with native property access.
    ///
    /// This rule currently flags usage of the imported `get()` function from `@ember/object`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import { get } from '@ember/object';
    ///
    /// const value = get(this, 'myProperty');
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { get } from '@ember/object';
    ///
    /// export default class MyComponent {
    ///   someMethod() {
    ///     return get(this, 'anotherProperty');
    ///   }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// export default class MyComponent {
    ///   someMethod() {
    ///     const value = this.myProperty;
    ///     return this.anotherProperty;
    ///   }
    /// }
    /// ```
    ///
    /// ```js
    /// // Custom get() function, not from @ember/object
    /// const value = customObject.get('key');
    /// ```
    ///
    pub NoEmberGet {
        version: "next",
        name: "noEmberGet",
        language: "js",
        sources: &[RuleSource::Eslint("ember/no-get").same()],
        recommended: true,
    }
}

impl Rule for NoEmberGet {
    type Query = Semantic<JsCallExpression>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call_expression = ctx.query();
        let callee = call_expression.callee().ok()?;
        let model = ctx.model();

        // Check for direct call: get()
        if let Some((reference, name)) = global_identifier(&callee) {
            if name.text() == "get" {
                // Check if it's imported (has a binding)
                if let Some(binding) = model.binding(&reference) {
                    // Check if it's imported from @ember/object
                    if binding.is_imported() {
                        if let Some(import_source) = get_import_source(&binding) {
                            if import_source == "@ember/object" {
                                return Some(call_expression.range());
                            }
                        }
                    }
                }
            }
            return None;
        }

        // Check for member expression calls: this.get(), obj.get()
        // Only flag if we're in an Ember context (conservative approach)
        // For now, we only flag the imported get() function to avoid false positives
        // A future enhancement could check if the object is an Ember object

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state,
                markup! {
                    "Don't use "<Emphasis>"get()"</Emphasis>" to access properties."
                },
            )
            .note(markup! {
                "Use native property access instead (e.g., "<Emphasis>"this.property"</Emphasis>" or "<Emphasis>"obj.property"</Emphasis>")."
            }),
        )
    }
}

/// Helper function to get the import source from a binding
fn get_import_source(binding: &biome_js_semantic::Binding) -> Option<String> {
    use biome_js_syntax::JsImport;

    let syntax = binding.syntax();

    // Navigate up the tree to find the import statement
    let mut current = Some(syntax.clone());
    while let Some(node) = current {
        if JsImport::can_cast(node.kind()) {
            let import = JsImport::unwrap_cast(node);
            let import_clause = import.import_clause().ok()?;
            let source = import_clause.source().ok()?;
            let source_text = source.inner_string_text().ok()?;
            return Some(source_text.to_string());
        }

        current = node.parent();
    }

    None
}
