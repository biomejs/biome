use crate::{declare_migration, MigrationAction};
use biome_analyze::context::RuleContext;
use biome_analyze::{ActionCategory, Ast, Rule, RuleAction, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::{category, Applicability};
use biome_json_analyze::utils::matches_path;
use biome_json_factory::make::{ident, json_member_name};
use biome_json_syntax::JsonMemberName;
use biome_rowan::{AstNode, BatchMutationExt};

declare_migration! {
    pub(crate) IndentSize {
        version: "1.3.0",
        name: "indentSize",
    }
}

impl Rule for IndentSize {
    type Query = Ast<JsonMemberName>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if matches_path(Some(node), &["formatter", "indentSize"]) {
            Some(())
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                category!("migrate"),
                node.range(),
                markup! {
                    "The option "<Emphasis>"formatter.indentSize"</Emphasis>" is deprecated."
                }
                .to_owned(),
            )
            .deprecated(),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<MigrationAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        let new_node = json_member_name(ident("\"indentWidth\""));
        mutation.replace_node(node.clone(), new_node);

        Some(RuleAction::new(
            ActionCategory::QuickFix,
            Applicability::Always,
            markup! {
                "Use the property "<Emphasis>"formatter.indentWidth"</Emphasis>" instead."
            }
            .to_owned(),
            mutation,
        ))
    }
}
