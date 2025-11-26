use crate::services::semantic::Semantic;
use biome_analyze::{Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_syntax::{JsCallExpression, JsImport, global_identifier};
use biome_rowan::{AstNode, TextRange};

declare_lint_rule! {
    /// Disallow usage of classic Ember classes created with `.extend()`.
    ///
    /// Modern Ember uses native JavaScript classes instead of the classic `.extend()` pattern.
    /// Using native classes provides better IDE support, improved type checking, and aligns
    /// with standard JavaScript practices.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import EmberObject from '@ember/object';
    /// const MyClass = EmberObject.extend({
    ///   foo: 'bar'
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import Component from '@ember/component';
    /// export default Component.extend({
    ///   tagName: 'div'
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import Component from '@glimmer/component';
    /// export default class MyComponent extends Component {
    ///   foo = 'bar';
    /// }
    /// ```
    ///
    /// ```js
    /// // Custom extend methods are fine
    /// const obj = myCustomObject.extend();
    /// ```
    ///
    pub NoEmberClassicClasses {
        version: "next",
        name: "noEmberClassicClasses",
        language: "js",
        recommended: true,
    }
}

impl Rule for NoEmberClassicClasses {
    type Query = Semantic<JsCallExpression>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call_expr = ctx.query();
        let model = ctx.model();

        // Check if this is a member expression like `Something.extend()`
        let callee = call_expr.callee().ok()?;
        let member_expr = callee.as_js_static_member_expression()?;

        // Check if the method name is "extend"
        let member = member_expr.member().ok()?;
        if member.value_token().ok()?.text_trimmed() != "extend" {
            return None;
        }

        // Get the object (e.g., EmberObject, Component, Route, etc.)
        let object = member_expr.object().ok()?;

        // Use global_identifier to get the reference and name
        let (reference, _name) = global_identifier(&object)?;

        // Get the binding for this identifier
        let binding = model.binding(&reference)?;

        // Check if it's imported from an Ember package
        if binding.is_imported()
            && let Some(import_source) = get_import_source(&binding)
            && import_source.starts_with("@ember/")
        {
            return Some(call_expr.range());
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state,
                markup! {
                    "Don't use classic Ember classes with "<Emphasis>".extend()"</Emphasis>"."
                },
            )
            .note(markup! {
                "Use native JavaScript class syntax instead."
            })
            .note(markup! {
                "See: "<Hyperlink href="https://guides.emberjs.com/release/upgrading/current-edition/native-classes/">"https://guides.emberjs.com/release/upgrading/current-edition/native-classes/"</Hyperlink>
            })
        )
    }
}

/// Helper function to get the import source from a binding
fn get_import_source(binding: &biome_js_semantic::Binding) -> Option<String> {
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
