use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_html_factory::make;
use biome_html_syntax::{AnyVueVForOperator, T, VueVForValue};
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::use_vue_consistent_v_for_delimiter::{
    UseVueConsistentVForDelimiterOptions, VueVForDelimiter,
};

use crate::HtmlRuleAction;

declare_lint_rule! {
    /// Enforce a consistent delimiter between the item and the list in `v-for` directives.
    ///
    /// In a `v-for` directive, Vue accepts both `in` and `of` between the item and the
    /// list, and treats them the same way: `item in items` and `item of items` render
    /// the same list. Using the same word everywhere makes templates easier to read
    /// and search.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <div v-for="item of items"></div>
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <template>
    ///   <div v-for="item in items"></div>
    ///   <div v-for="(item, index) in items"></div>
    /// </template>
    /// ```
    ///
    /// ## Options
    ///
    /// ### `delimiter`
    ///
    /// The word to use between the item and the list: `"in"` or `"of"`.
    ///
    /// Default: `"in"`
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "delimiter": "of"
    ///   }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```vue,expect_diagnostic,use_options
    /// <template>
    ///   <div v-for="item in items"></div>
    /// </template>
    /// ```
    ///
    /// #### Valid
    ///
    /// ```vue,use_options
    /// <template>
    ///   <div v-for="item of items"></div>
    /// </template>
    /// ```
    ///
    pub UseVueConsistentVForDelimiter {
        version: "next",
        name: "useVueConsistentVForDelimiter",
        language: "html",
        recommended: false,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("v-for-delimiter-style").same()],
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseVueConsistentVForDelimiter {
    type Query = Ast<VueVForValue>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseVueConsistentVForDelimiterOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let value = ctx.query();
        // When the item or the list is missing, the parser may have taken a word
        // from them as the delimiter. In `of in items`, the item named `of` is
        // read as the delimiter, so replacing it would break the template.
        value.binding().ok()?;
        value.expression().ok()?;
        let operator = value.operator().ok()?;
        (delimiter_of(&operator) != ctx.options().delimiter()).then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let operator = ctx.query().operator().ok()?;
        let used = delimiter_of(&operator);
        let preferred = ctx.options().delimiter();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                operator.range(),
                markup! {
                    "This "<Emphasis>"v-for"</Emphasis>" uses "<Emphasis>{used}</Emphasis>" instead of "<Emphasis>{preferred}</Emphasis>"."
                },
            )
            .note(markup! {
                "Vue treats "<Emphasis>"in"</Emphasis>" and "<Emphasis>"of"</Emphasis>" the same way in "<Emphasis>"v-for"</Emphasis>", so mixing them makes templates harder to read and search."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<HtmlRuleAction> {
        let operator = ctx.query().operator().ok()?;
        let preferred = ctx.options().delimiter();
        let new_operator = match preferred {
            VueVForDelimiter::In => {
                AnyVueVForOperator::from(make::vue_v_for_in_operator(make::token(T![in])))
            }
            VueVForDelimiter::Of => {
                AnyVueVForOperator::from(make::vue_v_for_of_operator(make::token(T![of])))
            }
        };
        let mut mutation = ctx.root().begin();
        mutation.replace_node_transfer_trivia(operator, new_operator)?;
        Some(HtmlRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use "<Emphasis>{preferred}</Emphasis>" instead." }.to_owned(),
            mutation,
        ))
    }
}

fn delimiter_of(operator: &AnyVueVForOperator) -> VueVForDelimiter {
    match operator {
        AnyVueVForOperator::VueVForInOperator(_) => VueVForDelimiter::In,
        AnyVueVForOperator::VueVForOfOperator(_) => VueVForDelimiter::Of,
    }
}
