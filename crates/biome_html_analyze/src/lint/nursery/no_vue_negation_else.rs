use crate::HtmlRuleAction;
use crate::services::vue::VueSemantic;
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::{
    AnyHtmlAttribute, AnyVueDirective, HtmlRoot, HtmlSyntaxToken, VueDirective,
};
use biome_rowan::{AstNodeExt, BatchMutationExt};
use biome_rule_options::no_vue_negation_else::NoVueNegationElseOptions;
use biome_vue_semantic::{BranchKind, DirectiveKind, Element, RootShape, Snippet};

declare_lint_rule! {
    /// Disallow negated conditions in `v-if` and `v-else-if` when a `v-else` branch follows.
    ///
    /// A condition that starts with `!`, or that compares with `!=` or `!==`, makes the
    /// reader reverse the logic. When a `v-else` branch follows, the reader has to reverse
    /// it a second time to understand when the `v-else` content shows. Removing the
    /// negation and swapping the two branches shows the same content in the same cases,
    /// and is easier to read.
    ///
    /// A negated condition without a `v-else` branch is allowed, because there is no
    /// branch to swap with. A double negation such as `!!value` is also allowed, because it
    /// converts the value to `true` or `false` instead of reversing it.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <div v-if="!isLoading">Content</div>
    ///   <div v-else>Loading...</div>
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <div v-if="status !== 'done'">Working</div>
    ///   <div v-else>Done</div>
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <div v-if="isAdmin">Admin</div>
    ///   <div v-else-if="!isMember">Guest</div>
    ///   <div v-else>Member</div>
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <template>
    ///   <div v-if="isLoading">Loading...</div>
    ///   <div v-else>Content</div>
    ///
    ///   <div v-if="!isLoading">Content</div>
    ///
    ///   <div v-if="!isAdmin">Guest</div>
    ///   <div v-else-if="isMember">Member</div>
    ///
    ///   <div v-if="!!user">Signed in</div>
    ///   <div v-else>Signed out</div>
    /// </template>
    /// ```
    ///
    /// ## See Also
    ///
    /// - To disallow negated conditions in JavaScript `if` statements and ternaries, see [`noNegationElse`](https://biomejs.dev/linter/rules/no-negation-else/).
    pub NoVueNegationElse {
        version: "next",
        name: "noVueNegationElse",
        language: "html",
        recommended: false,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-negated-v-if-condition").inspired()],
        fix_kind: FixKind::Safe,
    }
}

/// A negated condition on the branch just before a `v-else` branch.
pub struct NegatedBranch {
    /// The negated condition of the `v-if` or `v-else-if` directive.
    condition: Snippet,
    /// The element with the negated condition.
    consequent: Element,
    /// The element with the `v-else` directive.
    alternate: Element,
    /// Whether the fix is offered. It isn't when the branches sit inside an element that
    /// the fix of another reported branch replaces, or when either branch has `v-once`.
    has_fix: bool,
}

impl Rule for NoVueNegationElse {
    type Query = VueSemantic<HtmlRoot>;
    type State = NegatedBranch;
    type Signals = Box<[Self::State]>;
    type Options = NoVueNegationElseOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let mut signals: Vec<_> = ctx
            .model()
            .chains()
            .filter_map(|chain| {
                // `v-else` always ends a chain, so it can only be the last branch.
                let mut previous = None;
                let mut last = None;
                for branch in chain.branches() {
                    previous = last.replace(branch);
                }
                let (previous, last) = (previous?, last?);
                if last.kind != BranchKind::Else {
                    return None;
                }
                let condition = previous.condition?;
                matches!(
                    condition.root(),
                    RootShape::Not { .. } | RootShape::NotEqual { .. }
                )
                .then(|| {
                    // `v-once` on the first branch caches the whole conditional, and on any
                    // other branch it doesn't. Swapping the elements moves `v-once` to the
                    // other branch, which changes what Vue caches.
                    let has_once = [&previous.element, &last.element]
                        .into_iter()
                        .any(|element| element.directive(DirectiveKind::Once).is_some());
                    NegatedBranch {
                        condition,
                        consequent: previous.element,
                        alternate: last.element,
                        has_fix: !has_once,
                    }
                })
            })
            .collect();

        // A fix replaces both branch elements in full. When all fixes are applied together,
        // a fix inside one of those elements edits the same part of the document and one of
        // the two edits is lost, so only the outer fix is offered. An outer branch whose fix
        // is withheld for `v-once` replaces nothing, so it doesn't block the inner fix.
        let replaced: Vec<_> = signals
            .iter()
            .filter(|signal| signal.has_fix)
            .flat_map(|signal| [signal.consequent.range(), signal.alternate.range()])
            .collect();
        for signal in &mut signals {
            let range = signal.consequent.range();
            signal.has_fix &= !replaced
                .iter()
                .any(|outer| *outer != range && outer.contains_range(range));
        }
        signals.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.condition.host_range(),
                markup! {
                    "This condition is negated, but a "<Emphasis>"v-else"</Emphasis>" branch follows it."
                },
            )
            .note(markup! {
                "The reader has to reverse the condition twice to know when the "<Emphasis>"v-else"</Emphasis>" content shows."
            })
            .note(markup! {
                "Remove the negation and swap the content of the two branches."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<HtmlRuleAction> {
        if !state.has_fix {
            return None;
        }
        // The `!` is the first character of the operator in every case. Removing it turns
        // `!a` into `a`, and replacing it with `=` turns `a != b` into `a == b` and
        // `a !== b` into `a === b`.
        let (operator, replacement) = match state.condition.root() {
            RootShape::Not { operator } => (*operator, ""),
            RootShape::NotEqual { operator } => (*operator, "="),
            _ => return None,
        };
        let model = ctx.model();
        let condition_directive = vue_directive(model.node(&state.condition.attr()?)?)?;
        let else_directive =
            vue_directive(model.node(&state.alternate.directive(DirectiveKind::Else)?)?)?;
        let condition_element = model.node(&state.consequent)?;
        let else_element = model.node(&state.alternate)?;

        let value_token = condition_directive
            .initializer()?
            .value()
            .ok()?
            .as_html_string()?
            .value_token()
            .ok()?;
        if !value_token.text_trimmed_range().contains_range(operator) {
            return None;
        }
        let negation_start =
            usize::from(operator.start() - value_token.text_trimmed_range().start());
        let value_text = value_token.text_trimmed();
        let new_value_text = format!(
            "{}{replacement}{}",
            &value_text[..negation_start],
            &value_text[negation_start + 1..]
        );
        let new_value_token =
            HtmlSyntaxToken::new_detached(value_token.kind(), &new_value_text, [], []);

        let new_condition_directive = condition_directive
            .clone()
            .replace_token(value_token, new_value_token)?;
        let new_condition_element = else_element
            .clone()
            .replace_node(else_directive.clone(), new_condition_directive)?;
        let new_else_element = condition_element
            .clone()
            .replace_node(condition_directive, else_directive)?;

        let mut mutation = ctx.root().begin();
        mutation.replace_node(condition_element, new_condition_element);
        mutation.replace_node(else_element, new_else_element);

        Some(HtmlRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the negation and swap the branches." }.to_owned(),
            mutation,
        ))
    }
}

/// Returns the attribute as a directive written with its full name, such as `v-if`.
fn vue_directive(attribute: AnyHtmlAttribute) -> Option<VueDirective> {
    match attribute {
        AnyHtmlAttribute::AnyVueDirective(AnyVueDirective::VueDirective(directive)) => {
            Some(directive)
        }
        _ => None,
    }
}
