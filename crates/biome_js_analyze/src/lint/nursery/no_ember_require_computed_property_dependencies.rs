use biome_analyze::{Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_syntax::{AnyJsCallArgument, JsCallExpression, global_identifier};
use biome_rowan::{AstNode, AstSeparatedList, TextRange};

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Require dependencies for computed properties.
    ///
    /// Computed properties without dependencies may not update correctly.
    /// When a computed property doesn't declare its dependencies, it won't
    /// re-compute when those properties change.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import { computed } from '@ember/object';
    ///
    /// export default Component.extend({
    ///   // No dependencies - will never update!
    ///   fullName: computed(function() {
    ///     return `${this.firstName} ${this.lastName}`;
    ///   })
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import { computed } from '@ember/object';
    ///
    /// export default Component.extend({
    ///   // Has dependencies
    ///   fullName: computed('firstName', 'lastName', function() {
    ///     return `${this.firstName} ${this.lastName}`;
    ///   })
    /// });
    /// ```
    ///
    /// ```js
    /// // Native class with @tracked is fine
    /// export default class MyComponent extends Component {
    ///   @tracked firstName;
    ///   @tracked lastName;
    ///
    ///   get fullName() {
    ///     return `${this.firstName} ${this.lastName}`;
    ///   }
    /// }
    /// ```
    ///
    pub NoEmberRequireComputedPropertyDependencies {
        version: "next",
        name: "noEmberRequireComputedPropertyDependencies",
        language: "js",
        recommended: true,
    }
}

impl Rule for NoEmberRequireComputedPropertyDependencies {
    type Query = Semantic<JsCallExpression>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call_expression = ctx.query();
        let callee = call_expression.callee().ok()?;
        let model = ctx.model();

        // Check if this is a call to computed()
        let (reference, name) = global_identifier(&callee)?;

        if name.text() != "computed" {
            return None;
        }

        // Check if it's imported from @ember/object
        if let Some(binding) = model.binding(&reference) {
            if binding.is_imported() {
                if let Some(import_source) = get_import_source(&binding) {
                    if import_source != "@ember/object" {
                        return None;
                    }
                } else {
                    return None;
                }
            } else {
                return None;
            }
        } else {
            return None;
        }

        // Now check if the computed() call has dependency keys
        let arguments = call_expression.arguments().ok()?;
        let args = arguments.args();

        // If there are no arguments at all, that's invalid
        if args.is_empty() {
            return Some(call_expression.range());
        }

        // Check if all arguments are functions (no string dependencies)
        let has_dependencies = args.iter().any(|arg| {
            matches!(
                arg,
                Ok(AnyJsCallArgument::AnyJsExpression(
                    biome_js_syntax::AnyJsExpression::AnyJsLiteralExpression(_)
                ))
            )
        });

        // If there are no string dependencies, it's invalid
        if !has_dependencies {
            return Some(call_expression.range());
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state,
                markup! {
                    "Computed properties require dependency keys."
                },
            )
            .note(markup! {
                "Add the property keys this computed depends on, or use a native getter with "<Emphasis>"@tracked"</Emphasis>" properties."
            })
            .note(markup! {
                "Example: "<Emphasis>"computed('firstName', 'lastName', function() { ... })"</Emphasis>
            })
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
