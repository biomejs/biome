use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{HtmlAttributeList, VueDirective};
use biome_rowan::{AstNode, AstNodeList, TextRange};
use biome_rule_options::no_vue_v_if_with_v_for::NoVueVIfWithVForOptions;

declare_lint_rule! {
    /// Disallow using `v-if`, `v-else-if`, or `v-else` and `v-for` directives on the same element.
    ///
    /// There are two common cases where `v-if` and `v-for` on the same element can be tempting:
    /// - To filter items in a list (e.g. `v-for="user in users" v-if="user.isActive"`). In these cases, replace users with a new computed property that returns your filtered list (e.g. activeUsers).
    /// - To avoid rendering a list if it should be hidden (e.g. `v-for="user in users" v-if="shouldShowUsers"`). In these cases, move the v-if to a container element.
    ///
    /// Putting `v-else-if` or `v-else` on the same element as `v-for` works, but the condition
    /// is checked before the loop runs, which is hard to tell when reading the template.
    /// Move the `v-else-if` or `v-else` to a wrapper element, such as a `<template>`.
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
    /// ```vue,expect_diagnostic
    /// <div v-if="loading">Loading...</div>
    /// <TodoItem v-else v-for="todo in todos" :todo="todo" />
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
    /// <div v-if="loading">Loading...</div>
    /// <template v-else>
    ///     <TodoItem v-for="todo in todos" :todo="todo" />
    /// </template>
    /// ```
    ///
    pub NoVueVIfWithVFor {
        version: "2.3.6",
        name: "noVueVIfWithVFor",
        language: "html",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[
            RuleSource::EslintVueJs("no-use-v-if-with-v-for").same(),
            RuleSource::EslintVueJs("no-use-v-else-with-v-for").same(),
        ],
    }
}

pub struct State {
    v_for_range: TextRange,
    conditional_range: TextRange,
    conditional: ConditionalDirective,
}

impl Rule for NoVueVIfWithVFor {
    type Query = Ast<HtmlAttributeList>;
    type State = State;
    type Signals = Option<Self::State>;
    type Options = NoVueVIfWithVForOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();

        let mut v_for = None;
        let mut conditional = None;
        for attr in node.iter() {
            if let Some(directive) = attr
                .as_any_vue_directive()
                .and_then(|dir| dir.as_vue_directive())
            {
                if directive.is_for() {
                    v_for = Some(directive.range());
                } else if conditional.is_none()
                    && let Some(kind) = ConditionalDirective::from_directive(directive)
                {
                    conditional = Some((kind, directive.range()));
                }
            }
        }

        let v_for_range = v_for?;
        let (conditional, conditional_range) = conditional?;
        Some(State {
            v_for_range,
            conditional_range,
            conditional,
        })
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let conditional = state.conditional;
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            state.v_for_range,
            markup! {
                "Using "<Emphasis>{conditional}</Emphasis>" and "<Emphasis>"v-for"</Emphasis>" on the same element is discouraged."
            },
        );
        let diagnostic = match conditional {
            ConditionalDirective::If => diagnostic
                .note(markup! {
                    "Using "<Emphasis>"v-if"</Emphasis>" and "<Emphasis>"v-for"</Emphasis>" on the same element can lead to unexpected behavior and performance issues."
                })
                .detail(state.conditional_range, markup! {
                    "This "<Emphasis>"v-if"</Emphasis>" should be moved to the wrapper element, or you should use a computed property to filter the list instead."
                }),
            ConditionalDirective::ElseIf | ConditionalDirective::Else => diagnostic
                .note(markup! {
                    "The "<Emphasis>{conditional}</Emphasis>" is checked before the "<Emphasis>"v-for"</Emphasis>" loop runs, which is hard to tell when reading the template."
                })
                .detail(state.conditional_range, markup! {
                    "Move this "<Emphasis>{conditional}</Emphasis>" to a wrapper element, such as a "<Emphasis>"<template>"</Emphasis>"."
                }),
        };
        Some(diagnostic)
    }
}

/// A conditional rendering directive that can't share an element with `v-for`.
#[derive(Clone, Copy)]
pub enum ConditionalDirective {
    If,
    ElseIf,
    Else,
}

impl ConditionalDirective {
    fn from_directive(directive: &VueDirective) -> Option<Self> {
        if directive.is_if() {
            Some(Self::If)
        } else if directive.is_else_if() {
            Some(Self::ElseIf)
        } else if directive.is_else() {
            Some(Self::Else)
        } else {
            None
        }
    }
}

impl biome_console::fmt::Display for ConditionalDirective {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::If => "v-if",
            Self::ElseIf => "v-else-if",
            Self::Else => "v-else",
        })
    }
}
