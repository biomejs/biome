use biome_analyze::{Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_syntax::JsPropertyObjectMember;
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Disallow the classic `actions` hash pattern in Ember classes.
    ///
    /// Modern Ember uses `@action` decorators instead of the `actions` hash pattern.
    /// The `actions` hash was used in classic Ember classes (EmberObject.extend())
    /// but has been replaced by decorators in modern Ember (Octane and later).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import Component from '@ember/component';
    ///
    /// export default Component.extend({
    ///   actions: {
    ///     handleClick() {
    ///       console.log('clicked');
    ///     }
    ///   }
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import Component from '@glimmer/component';
    /// import { action } from '@ember/object';
    ///
    /// export default class MyComponent extends Component {
    ///   @action
    ///   handleClick() {
    ///     console.log('clicked');
    ///   }
    /// }
    /// ```
    ///
    /// ```js
    /// // Non-Ember context - actions property is fine
    /// const config = {
    ///   actions: ['read', 'write', 'delete']
    /// };
    /// ```
    ///
    pub NoEmberActionsHash {
        version: "next",
        name: "noEmberActionsHash",
        language: "js",
        recommended: false,
    }
}

impl Rule for NoEmberActionsHash {
    type Query = Ast<JsPropertyObjectMember>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let property = ctx.query();

        // Get the property name
        let name = property.name().ok()?;
        let name_token = name.as_js_literal_member_name()?;
        let name_text = name_token.name().ok()?;

        // Check if the property is named "actions"
        if name_text.text() != "actions" {
            return None;
        }

        // Check if the value is an object expression
        let value = property.value().ok()?;
        value.as_js_object_expression()?;

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let property = ctx.query();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                property.range(),
                markup! {
                    "Don't use the "<Emphasis>"actions"</Emphasis>" hash pattern."
                },
            )
            .note(markup! {
                "Use "<Emphasis>"@action"</Emphasis>" decorators on individual methods instead."
            })
            .note(markup! {
                "See: https://guides.emberjs.com/release/upgrading/current-edition/action-syntax/"
            }),
        )
    }
}
