use crate::{declare_migration, MigrationAction};
use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, Rule, RuleAction, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::{category, Applicability};
use biome_json_factory::make::{json_member_name, json_string_literal};
use biome_json_syntax::{JsonMember, JsonMemberName};
use biome_rowan::{AstNode, BatchMutationExt};

declare_migration! {
    pub(crate) TrailingComma {
        version: "2.0.0",
        name: "trailingComma",
    }
}

impl Rule for TrailingComma {
    type Query = Ast<JsonMember>;
    type State = JsonMemberName;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let name = node.name().ok()?;
        let node_text = name.inner_string_text().ok()?;
        if node_text.text() == "trailingComma" {
            return Some(name);
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            category!("migrate"),
            state.range(),
            markup! {
                "The option trailingComma is removed. "
            }
            .to_owned(),
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<MigrationAction> {
        let mut mutation = ctx.root().begin();
        let new_name = json_member_name(json_string_literal("trailingCommas"));
        mutation.replace_node(state.clone(), new_name);

        Some(RuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            Applicability::Always,
            markup! {
                "Use the option trailingCommas instead."
            }
            .to_owned(),
            mutation,
        ))
    }
}
