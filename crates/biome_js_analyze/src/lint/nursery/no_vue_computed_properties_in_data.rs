use crate::services::vue::VueComponent;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyFunctionLike, AnyJsMemberExpression, JsComputedMemberAssignment,
    JsGetterClassMember, JsGetterObjectMember, JsLanguage, JsSetterClassMember,
    JsSetterObjectMember, JsStaticMemberAssignment, JsSyntaxNode, TextRange,
};
use biome_rowan::{AstNode, SyntaxKindSet};
use biome_rule_options::no_vue_computed_properties_in_data::NoVueComputedPropertiesInDataOptions;
use biome_vue_semantic::{Computed, RefBase};

declare_lint_rule! {
    /// Disallow using computed properties inside the `data` function of a Vue component.
    ///
    /// Vue runs the `data` function before it sets up the component's computed properties.
    /// Reading a computed property there through `this` gives `undefined` instead of the
    /// computed value.
    ///
    /// Only code written directly in `data` is checked. Code inside a function created in
    /// `data`, such as an arrow function stored as a data property, is not reported.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   data() {
    ///     return {
    ///       total: this.doubled,
    ///     };
    ///   },
    ///   computed: {
    ///     doubled() {
    ///       return 2;
    ///     },
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script>
    /// export default {
    ///   props: ["initial"],
    ///   data() {
    ///     return {
    ///       count: this.initial,
    ///       label: this.formatLabel(),
    ///       read: () => this.doubled,
    ///     };
    ///   },
    ///   computed: {
    ///     doubled() {
    ///       return this.count * 2;
    ///     },
    ///   },
    ///   methods: {
    ///     formatLabel() {
    ///       return "Count";
    ///     },
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    pub NoVueComputedPropertiesInData {
        version: "2.6.4",
        name: "noVueComputedPropertiesInData",
        language: "js",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-computed-properties-in-data").same()],
    }
}

impl Rule for NoVueComputedPropertiesInData {
    type Query = VueComponent;
    type State = RuleState;
    type Signals = Box<[Self::State]>;
    type Options = NoVueComputedPropertiesInDataOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let component = ctx.query();
        let Some(data) = component
            .data_function_node()
            .and_then(|data| ctx.vue_model().node(&data))
            .and_then(AnyFunctionLike::cast)
        else {
            return Box::default();
        };
        let mut signals = Vec::new();
        for property in component.computed() {
            for reference in property.uses() {
                if reference.base() != RefBase::This {
                    continue;
                }
                // A use written in another tree than the component has no
                // node here.
                let Some(name) = ctx.vue_model().node(&reference) else {
                    continue;
                };
                if runs_with(&data, &name) {
                    signals.push(RuleState {
                        computed: property.clone(),
                        range: member_access_range(&name),
                    });
                }
            }
        }
        signals.into_boxed_slice()
    }

    fn text_range(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<TextRange> {
        Some(state.range)
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "The computed property "<Emphasis>{state.computed.name()}</Emphasis>" is used inside the "<Emphasis>"data"</Emphasis>" function."
                },
            )
            .detail(
                state.computed.range(),
                markup! { "The computed property is declared here." },
            )
            .note(markup! {
                "Vue runs the "<Emphasis>"data"</Emphasis>" function before it sets up computed properties, so this value is "<Emphasis>"undefined"</Emphasis>"."
            })
            .note(markup! {
                "Compute the value from props or other data instead, or turn this data property into a computed property."
            }),
        )
    }
}

pub struct RuleState {
    /// The computed property that is used.
    computed: Computed,
    /// The range of the member access, such as `this.name`.
    range: TextRange,
}

/// Nodes that start code that runs at a different time than the function
/// that contains them.
const FUNCTION_KINDS: SyntaxKindSet<JsLanguage> = AnyFunctionLike::KIND_SET
    .union(JsGetterObjectMember::KIND_SET)
    .union(JsSetterObjectMember::KIND_SET)
    .union(JsGetterClassMember::KIND_SET)
    .union(JsSetterClassMember::KIND_SET);

/// Returns `true` when `node` is in `function` and runs as part of `function`
/// itself, and not inside a function nested in it.
fn runs_with(function: &AnyFunctionLike, node: &JsSyntaxNode) -> bool {
    for ancestor in node.ancestors() {
        if ancestor == *function.syntax() {
            return true;
        }
        if FUNCTION_KINDS.matches(ancestor.kind()) {
            return false;
        }
    }
    false
}

/// Returns the range of the member access whose member name is `name`, such
/// as `this.name` for `name`.
fn member_access_range(name: &JsSyntaxNode) -> TextRange {
    name.ancestors()
        .find(|node| MEMBER_KINDS.matches(node.kind()))
        .map_or(name.text_trimmed_range(), |member| {
            member.text_trimmed_range()
        })
}

/// Nodes that read or assign a member of an object.
const MEMBER_KINDS: SyntaxKindSet<JsLanguage> = AnyJsMemberExpression::KIND_SET
    .union(JsStaticMemberAssignment::KIND_SET)
    .union(JsComputedMemberAssignment::KIND_SET);
