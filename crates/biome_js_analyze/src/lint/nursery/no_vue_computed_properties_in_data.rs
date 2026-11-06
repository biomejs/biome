use crate::services::vue::VueComponent;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyFunctionLike, AnyJsMemberExpression, AnyJsRoot, JsComputedMemberAssignment,
    JsGetterClassMember, JsGetterObjectMember, JsLanguage, JsSetterClassMember,
    JsSetterObjectMember, JsStaticMemberAssignment, TextRange,
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
            .data_function_range()
            .and_then(|range| find_function(&ctx.root(), range))
        else {
            return Box::default();
        };
        let data_range = data.syntax().text_trimmed_range();
        let snippet = component.snippet().map(|snippet| snippet.id());

        let mut signals = Vec::new();
        for property in component.computed() {
            for reference in property.uses() {
                let range = reference.range();
                if reference.base() == RefBase::This
                    && reference.snippet().map(|snippet| snippet.id()) == snippet
                    && data_range.contains_range(range)
                    && runs_with(&data, range)
                {
                    signals.push(RuleState {
                        computed: property.clone(),
                        range: member_access_range(&data, range),
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

/// Returns the function in `root` whose range is exactly `range`.
fn find_function(root: &AnyJsRoot, range: TextRange) -> Option<AnyFunctionLike> {
    root.syntax()
        .covering_element(range)
        .ancestors()
        .find_map(AnyFunctionLike::cast)
        .filter(|function| function.syntax().text_trimmed_range() == range)
}

/// Returns `true` when the code at `range` runs as part of `function` itself,
/// and not inside a function nested in it.
fn runs_with(function: &AnyFunctionLike, range: TextRange) -> bool {
    function
        .syntax()
        .covering_element(range)
        .ancestors()
        .take_while(|node| node != function.syntax())
        .all(|node| !FUNCTION_KINDS.matches(node.kind()))
}

/// Returns the range of the member access whose member name is at `name`,
/// such as `this.name` for `name`.
fn member_access_range(function: &AnyFunctionLike, name: TextRange) -> TextRange {
    function
        .syntax()
        .covering_element(name)
        .ancestors()
        .find(|node| MEMBER_KINDS.matches(node.kind()))
        .map_or(name, |member| member.text_trimmed_range())
}

/// Nodes that read or assign a member of an object.
const MEMBER_KINDS: SyntaxKindSet<JsLanguage> = AnyJsMemberExpression::KIND_SET
    .union(JsStaticMemberAssignment::KIND_SET)
    .union(JsComputedMemberAssignment::KIND_SET);
