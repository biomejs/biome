use crate::services::vue::VueComputedReference;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::JsCallExpression;
use biome_rowan::AstNode;
use biome_rule_options::no_vue_computed_property_call::NoVueComputedPropertyCallOptions;
use biome_vue_semantic::{Access, Computed, Tri};

declare_lint_rule! {
    /// Disallow calling a Vue computed property whose value is not a function.
    ///
    /// A computed property is read like a regular property: `this.total` in the script, or `total`
    /// in the template. Vue runs its getter and gives you the value the getter returns. Adding
    /// parentheses, as in `this.total()`, calls that value. When the value is a string, a number,
    /// an object, or anything else that is not a function, the call throws a `TypeError` when the
    /// code runs.
    ///
    /// This rule reports a call to a computed property when every value its getter can return is
    /// something other than a function. The rule follows what the getter returns through data,
    /// props, methods and other computed properties of the component. It doesn't report the call
    /// when it can't tell what the value is, for example when the getter returns the result of an
    /// imported function.
    ///
    /// :::note
    /// In `.vue` files, this rule needs to read the template, which requires
    /// [`html.experimentalFullSupportEnabled`](https://biomejs.dev/reference/configuration/#htmlexperimentalfullsupportenabled)
    /// to be `true`. Without that setting, the rule only checks calls written in the script.
    /// :::
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import { defineComponent } from "vue";
    ///
    /// export default defineComponent({
    ///   data() {
    ///     return { price: 10, quantity: 2 };
    ///   },
    ///   computed: {
    ///     total() {
    ///       return this.price * this.quantity;
    ///     },
    ///   },
    ///   methods: {
    ///     logTotal() {
    ///       console.log(this.total());
    ///     },
    ///   },
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import { defineComponent } from "vue";
    ///
    /// export default defineComponent({
    ///   data() {
    ///     return { price: 10, quantity: 2 };
    ///   },
    ///   computed: {
    ///     total() {
    ///       return this.price * this.quantity;
    ///     },
    ///     formatter() {
    ///       return (value) => `$${value}`;
    ///     },
    ///   },
    ///   methods: {
    ///     logTotal() {
    ///       console.log(this.total);
    ///       console.log(this.formatter(this.total));
    ///     },
    ///   },
    /// });
    /// ```
    ///
    pub NoVueComputedPropertyCall {
        version: "next",
        name: "noVueComputedPropertyCall",
        language: "js",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-use-computed-property-like-method").same()],
    }
}

impl Rule for NoVueComputedPropertyCall {
    type Query = VueComputedReference;
    /// The call of the computed property.
    type State = JsCallExpression;
    type Signals = Option<Self::State>;
    type Options = NoVueComputedPropertyCallOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let reference = ctx.query();
        // `this.items.map()` calls a member of the value, not the value.
        if reference.access() != Access::Call || reference.path().next().is_some() {
            return None;
        }
        let computed = Computed::from_symbol(reference.symbol()?)?;
        if computed.value_is_function() != Tri::No {
            return None;
        }
        ctx.root()
            .syntax()
            .covering_element(reference.range())
            .ancestors()
            .find_map(JsCallExpression::cast)
    }

    fn diagnostic(ctx: &RuleContext<Self>, call: &Self::State) -> Option<RuleDiagnostic> {
        let name = ctx.query().name();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                call.range(),
                markup! {
                    "The computed property "<Emphasis>{name}</Emphasis>" is called like a method, but its value is not a function."
                },
            )
            .note(markup! {
                "Vue gives you the value that the getter of a computed property returns. Calling a value that is not a function throws an error when the code runs."
            })
            .note(markup! {
                "Remove the parentheses and the arguments to read the value of the computed property."
            }),
        )
    }
}
