use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::HtmlAttributeList;
use biome_rowan::{AstNode, AstNodeList, TextRange};
use biome_rule_options::no_vue_v_if_with_v_for::NoVueVIfWithVForOptions;

declare_lint_rule! {
    /// Disallow using `v-if` and `v-for` directives on the same element.
    ///
    /// There are two common cases where this can be tempting:
    /// - To filter items in a list (e.g. `v-for="user in users" v-if="user.isActive"`). In these cases, replace users with a new computed property that returns your filtered list (e.g. activeUsers).
    /// - To avoid rendering a list if it should be hidden (e.g. `v-for="user in users" v-if="shouldShowUsers"`). In these cases, move the v-if to a container element.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <TodoItem
    ///     v-if="complete"
    ///     v-for="todo in todos"
    ///     :todo="todo"
    /// />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <ul v-if="complete">
    ///     <TodoItem
    ///         v-for="todo in todos"
    ///         :todo="todo"
    ///     />
    /// </ul>
    /// ```
    ///
    pub NoVueVIfWithVFor {
        version: "2.3.6",
        name: "noVueVIfWithVFor",
        language: "html",
        recommended: true,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-use-v-if-with-v-for").same()],
    }
}

pub struct State {
    v_for_range: TextRange,
    v_if_range: TextRange,
}

impl Rule for NoVueVIfWithVFor {
    type Query = Ast<HtmlAttributeList>;
    type State = State;
    type Signals = Option<Self::State>;
    type Options = NoVueVIfWithVForOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();

        let mut v_for = None;
        let mut v_if = None;
        for attr in node.iter() {
            if let Some(directive) = attr
                .as_any_vue_directive()
                .and_then(|dir| dir.as_vue_directive())
            {
                if directive
                    .name_token()
                    .is_ok_and(|t| t.text_trimmed() == "v-if")
                {
                    v_if = Some(directive.range());
                } else if directive
                    .name_token()
                    .is_ok_and(|t| t.text_trimmed() == "v-for")
                {
                    v_for = Some(directive.range());
                }
            }
        }

        if let (Some(v_if_range), Some(v_for_range)) = (v_if, v_for) {
            return Some(State {
                v_for_range,
                v_if_range,
            });
        }
        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.v_for_range,
                markup! {
                    "Using "<Emphasis>"v-if"</Emphasis>" and "<Emphasis>"v-for"</Emphasis>" on the same element is discouraged."
                },
            )
            .note(markup! {
                "Using "<Emphasis>"v-if"</Emphasis>" and "<Emphasis>"v-for"</Emphasis>" on the same element can lead to unexpected behavior and performance issues."
            })
            .detail(state.v_if_range, markup! {
                    "This "<Emphasis>"v-if"</Emphasis>" should be moved to the wrapper element, or you should use a computed property to filter the list instead."
            }),
        )
    }
}
