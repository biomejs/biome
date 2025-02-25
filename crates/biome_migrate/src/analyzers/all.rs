use crate::{declare_migration, MigrationAction};
use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, Rule, RuleAction, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::{category, Applicability};
use biome_json_syntax::{JsonMember, TextRange};
use biome_rowan::{AstNode, BatchMutationExt};

declare_migration! {
    pub(crate) RulesAll {
        version: "2.0.0",
        name: "all",
    }
}

impl Rule for RulesAll {
    type Query = Ast<JsonMember>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let name = node.name().ok()?;
        let node_text = name.inner_string_text().ok()?;
        if node_text.text() == "all" {
            return Some(name.range());
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            category!("migrate"),
            state,
            markup! {
                "The property "<Emphasis>"all"</Emphasis>" has been removed."
            }
                .to_owned(),
        ).note(markup!{
            "Due to the increasing number of rules that span in scope and use-case, certain rules can conflict with each other. The option has become more harmful than useful."
        }))
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<MigrationAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();
        mutation.remove_node(node.clone());

        Some(RuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            Applicability::Always,
            markup! {
                "Remove the property."
            }
            .to_owned(),
            mutation,
        ))
    }
}
