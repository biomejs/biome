use biome_analyze::{Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_syntax::JsImport;
use biome_rowan::{AstNode, TextRange};

declare_lint_rule! {
    /// Disallow importing files from `/mixins/` directories.
    ///
    /// Mixins are a deprecated pattern in Ember. Modern Ember applications should use:
    /// - Composition patterns
    /// - Utility functions
    /// - Native JavaScript classes with inheritance
    /// - Decorators (e.g., `@tracked`, `@action`)
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import MyMixin from 'app/mixins/my-mixin';
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import UtilityMixin from '../../mixins/utilities/validation';
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import Component from '@glimmer/component';
    ///
    /// export default class MyComponent extends Component {}
    /// ```
    ///
    /// ```js
    /// import { action } from '@ember/object';
    ///
    /// export default class MyComponent {
    ///   @action
    ///   handleClick() {}
    /// }
    /// ```
    ///
    pub NoEmberMixins {
        version: "next",
        name: "noEmberMixins",
        language: "js",
        recommended: true,
    }
}

impl Rule for NoEmberMixins {
    type Query = Ast<JsImport>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let import = ctx.query();

        // Get the import source
        let import_clause = import.import_clause().ok()?;
        let source = import_clause.source().ok()?;
        let source_text = source.inner_string_text().ok()?;
        let path = source_text.text();

        // Check if the import path contains "/mixins/"
        if path.contains("/mixins/") {
            return Some(import.range());
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state,
                markup! {
                    "Don't import from "<Emphasis>"/mixins/"</Emphasis>" directories."
                },
            )
            .note(markup! {
                "Mixins are deprecated in modern Ember. Use composition patterns, utility functions, or decorators instead."
            })
            .note(markup! {
                "See the Ember Octane migration guide: "<Hyperlink href="https://guides.emberjs.com/release/upgrading/current-edition/">"https://guides.emberjs.com/release/upgrading/current-edition/"</Hyperlink>
            })
        )
    }
}
