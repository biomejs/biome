use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_syntax::{AnyJsExpression, JsCallExpression, global_identifier};
use biome_rowan::{AstNode, TextRange};

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow usage of the deprecated `getWithDefault()` method.
    ///
    /// `getWithDefault()` was deprecated in Ember 3.x and removed in Ember 4.0.
    /// Use optional chaining (`?.`) or nullish coalescing (`??`) instead.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import { getWithDefault } from '@ember/object';
    ///
    /// const value = getWithDefault(obj, 'key', 'default');
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import Component from '@glimmer/component';
    ///
    /// class MyComponent extends Component {
    ///   someMethod() {
    ///     return this.getWithDefault('property', 'fallback');
    ///   }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import { get } from '@ember/object';
    ///
    /// const value = get(obj, 'key') ?? 'default';
    /// ```
    ///
    /// ```js
    /// class MyComponent {
    ///   someMethod() {
    ///     return this.property ?? 'fallback';
    ///   }
    /// }
    /// ```
    ///
    pub NoEmberGetWithDefault {
        version: "next",
        name: "noEmberGetWithDefault",
        language: "js",
        sources: &[RuleSource::Eslint("ember/no-get-with-default").same()],
        recommended: true,
    }
}

impl Rule for NoEmberGetWithDefault {
    type Query = Semantic<JsCallExpression>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call_expression = ctx.query();
        let callee = call_expression.callee().ok()?;
        let model = ctx.model();

        // Check for direct call: getWithDefault()
        if let Some((reference, name)) = global_identifier(&callee) {
            if name.text() == "getWithDefault" {
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

        // Check for member expression calls: this.getWithDefault(), obj.getWithDefault()
        if let AnyJsExpression::JsStaticMemberExpression(member_expr) = callee {
            let member = member_expr.member().ok()?;
            let member_name = member.as_js_name()?.value_token().ok()?;

            if member_name.text_trimmed() == "getWithDefault" {
                return Some(call_expression.range());
            }
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state,
                markup! {
                    "Don't use the deprecated "<Emphasis>"getWithDefault()"</Emphasis>" method."
                },
            )
            .note(markup! {
                "The "<Emphasis>"getWithDefault()"</Emphasis>" method was deprecated in Ember 3.x and removed in Ember 4.0."
            })
            .note(markup! {
                "Use optional chaining ("<Emphasis>"?."</Emphasis>") or nullish coalescing ("<Emphasis>"??"</Emphasis>") instead."
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
