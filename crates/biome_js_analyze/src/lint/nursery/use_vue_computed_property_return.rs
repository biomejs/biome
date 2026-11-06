use crate::services::control_flow::{AnyJsControlFlowRoot, control_flow_graph};
use crate::services::vue::VueComponent;
use crate::utils::function_return::{InvalidReturn, invalid_returns};
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsExpression, AnyJsFunctionBody, AnyJsObjectMember, AnyJsRoot, TextRange,
};
use biome_rowan::{AstNode, AstSeparatedList};
use biome_rule_options::use_vue_computed_property_return::UseVueComputedPropertyReturnOptions;
use biome_vue_semantic::Computed;

declare_lint_rule! {
    /// Require computed properties in Vue components to return a value.
    ///
    /// The value of a computed property is whatever its getter function returns. When the
    /// getter can finish without returning a value, the computed property is `undefined`,
    /// which is almost always a mistake, such as a forgotten `return` keyword or a missing
    /// `else` branch.
    ///
    /// This rule checks the getters of properties in the `computed` option, and the getters
    /// passed to `computed()` at the top level of `<script setup>`. A `return` statement
    /// without a value is reported too, because it also makes the computed property
    /// `undefined`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   computed: {
    ///     fullName() {
    ///       this.firstName + " " + this.lastName;
    ///     },
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   computed: {
    ///     label() {
    ///       if (this.count > 0) {
    ///         return `${this.count} items`;
    ///       }
    ///     },
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup>
    /// import { computed, ref } from "vue";
    ///
    /// const count = ref(0);
    /// const double = computed(() => {
    ///   count.value * 2;
    /// });
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup>
    /// import { computed, ref } from "vue";
    ///
    /// const count = ref(0);
    /// const double = computed(() => {
    ///   if (count.value > 0) {
    ///     return;
    ///   }
    ///   return count.value * 2;
    /// });
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script>
    /// export default {
    ///   computed: {
    ///     fullName() {
    ///       return this.firstName + " " + this.lastName;
    ///     },
    ///     label: {
    ///       get() {
    ///         return this.count > 0 ? `${this.count} items` : "empty";
    ///       },
    ///       set(value) {
    ///         this.count = Number.parseInt(value);
    ///       },
    ///     },
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ```vue
    /// <script setup>
    /// import { computed, ref } from "vue";
    ///
    /// const count = ref(0);
    /// const double = computed(() => count.value * 2);
    /// const label = computed(() => {
    ///   if (count.value > 0) {
    ///     return `${count.value} items`;
    ///   }
    ///   return "empty";
    /// });
    /// </script>
    /// ```
    ///
    /// ## See Also
    ///
    /// - To require `get` accessors of classes and objects to return a value, see [`useGetterReturn`](https://biomejs.dev/linter/rules/use-getter-return/).
    ///
    pub UseVueComputedPropertyReturn {
        version: "2.6.5",
        name: "useVueComputedPropertyReturn",
        language: "js",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("return-in-computed-property").inspired()],
    }
}

impl Rule for UseVueComputedPropertyReturn {
    type Query = VueComponent;
    type State = InvalidComputedReturn;
    type Signals = Box<[Self::State]>;
    type Options = UseVueComputedPropertyReturnOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let root = ctx.root();
        let mut signals = Vec::new();
        for computed in ctx.query().computed() {
            let Some(getter) = computed
                .getter_range()
                .and_then(|range| getter_function(&root, range))
            else {
                continue;
            };
            let Some(cfg) = control_flow_graph(ctx, &getter) else {
                continue;
            };
            let function = getter.range();
            signals.extend(invalid_returns(&cfg).into_iter().map(|problem| {
                InvalidComputedReturn {
                    computed: computed.clone(),
                    function,
                    problem,
                }
            }));
        }
        signals.into_boxed_slice()
    }

    fn text_range(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<TextRange> {
        Some(state.range())
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let name = state.computed.name();
        let diagnostic = match state.problem {
            InvalidReturn::MissingReturn => RuleDiagnostic::new(
                rule_category!(),
                state.range(),
                markup! {
                    "The computed property "<Emphasis>{name}</Emphasis>" doesn't always return a value."
                },
            )
            .note(markup! {
                "Vue uses the value the getter returns as the value of the computed property. When the getter ends without a "<Emphasis>"return"</Emphasis>" statement, the computed property is "<Emphasis>"undefined"</Emphasis>"."
            })
            .note(markup! {
                "Make sure that every path through the getter ends with a "<Emphasis>"return"</Emphasis>" statement that returns a value."
            }),
            InvalidReturn::EmptyReturn(_) => RuleDiagnostic::new(
                rule_category!(),
                state.range(),
                markup! {
                    "This "<Emphasis>"return"</Emphasis>" statement in the computed property "<Emphasis>{name}</Emphasis>" doesn't return a value."
                },
            )
            .note(markup! {
                "A "<Emphasis>"return"</Emphasis>" statement without a value makes the computed property "<Emphasis>"undefined"</Emphasis>"."
            })
            .note(markup! {
                "Return the value of the computed property from this statement."
            }),
        };
        Some(diagnostic)
    }
}

pub struct InvalidComputedReturn {
    computed: Computed,
    /// The range of the getter function.
    function: TextRange,
    problem: InvalidReturn,
}

impl InvalidComputedReturn {
    fn range(&self) -> TextRange {
        match self.problem {
            InvalidReturn::MissingReturn => self.function,
            InvalidReturn::EmptyReturn(range) => range,
        }
    }
}

/// Returns the getter function of a computed property from the range the Vue
/// semantic model records for it.
///
/// The range covers one of `fullName() {}`, `fullName: function () {}`, a
/// `get` member of an object, or the first argument of `computed()`. Returns
/// `None` when the getter isn't a function written in place, and for an arrow
/// function with an expression body, which always returns a value.
fn getter_function(root: &AnyJsRoot, range: TextRange) -> Option<AnyJsControlFlowRoot> {
    if !root.syntax().text_range_with_trivia().contains_range(range) {
        return None;
    }
    let node = root
        .syntax()
        .covering_element(range)
        .ancestors()
        .find(|node| node.text_trimmed_range() == range)?;
    match AnyJsObjectMember::cast_ref(&node) {
        Some(member) => getter_function_from_member(member),
        None => getter_function_from_expression(&AnyJsExpression::cast(node)?),
    }
}

fn getter_function_from_member(member: AnyJsObjectMember) -> Option<AnyJsControlFlowRoot> {
    match member {
        AnyJsObjectMember::JsMethodObjectMember(method) => {
            Some(AnyJsControlFlowRoot::JsMethodObjectMember(method))
        }
        AnyJsObjectMember::JsPropertyObjectMember(property) => {
            getter_function_from_expression(&property.value().ok()?)
        }
        _ => None,
    }
}

fn getter_function_from_expression(expression: &AnyJsExpression) -> Option<AnyJsControlFlowRoot> {
    match expression.inner_expression()? {
        AnyJsExpression::JsFunctionExpression(function) => {
            Some(AnyJsControlFlowRoot::AnyJsFunction(function.into()))
        }
        AnyJsExpression::JsArrowFunctionExpression(function) => {
            if matches!(function.body().ok()?, AnyJsFunctionBody::AnyJsExpression(_)) {
                return None;
            }
            Some(AnyJsControlFlowRoot::AnyJsFunction(function.into()))
        }
        // computed({ get() {}, set(value) {} })
        AnyJsExpression::JsObjectExpression(object) => {
            let get = object
                .members()
                .iter()
                .flatten()
                .find(|member| member.name().is_some_and(|name| name.text() == "get"))?;
            getter_function_from_member(get)
        }
        _ => None,
    }
}
