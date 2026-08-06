use crate::MarkdownRuleAction;
use biome_analyze::{
    Ast, Rule, RuleAction, RuleDiagnostic, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_markdown_factory::make;
use biome_markdown_factory::make::token;
use biome_markdown_syntax::{MdHeader, T};
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt, Direction};
use biome_rule_options::use_consistent_header_level::UseConsistentHeaderLevelOptions;

declare_lint_rule! {
    /// Enforce that all headers level are consistent.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```md,expect_diagnostic
    /// \# Header 1
    ///
    /// ### Header 3
    ///
    /// ```
    ///
    /// ### Valid
    ///
    /// ```md
    /// \# Header 1
    ///
    /// ## Header 2
    /// ```
    ///
    pub UseConsistentHeaderLevel {
        version: "next",
        name: "useConsistentHeaderLevel",
        language: "md",
        recommended: true,
    }
}

impl Rule for UseConsistentHeaderLevel {
    type Query = Ast<MdHeader>;
    type State = MdHeader;
    type Signals = Option<Self::State>;
    type Options = UseConsistentHeaderLevelOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let this_header = ctx.query();

        let next_header = this_header
            .syntax()
            .siblings(Direction::Next)
            .skip(1)
            .find_map(MdHeader::cast)?;

        if next_header.level() > this_header.level() + 1 {
            Some(next_header)
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range(),
                markup! {
                    "This header skipped a level."
                },
            )
            .detail(node.range(), markup! {
                "The previous header has level"<Emphasis>{node.level()}</Emphasis>"."
            })
            .note(markup! {
                "Headers should follow a consistent level order. Failing to do so could cause issues when the file is transformed into HTML."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, header: &Self::State) -> Option<MarkdownRuleAction> {
        let mut mutation = ctx.root().begin();
        let old_list = header.before();
        // We start the new list from the original header
        let mut new_list: Vec<_> = ctx.query().before().iter().collect();
        new_list.push(make::md_hash(token(T![#])));
        let new_list = make::md_hash_list(new_list);

        mutation.replace_node(old_list, new_list);

        Some(RuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            Applicability::Always,
            markup! { "Change the level of the header." }.to_owned(),
            mutation,
        ))
    }
}
