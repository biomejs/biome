use crate::{declare_migration, MigrationAction};
use biome_analyze::context::RuleContext;
use biome_analyze::{ActionCategory, Ast, Rule, RuleAction, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::{category, Applicability};
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

        let node_text = node.inner_string_text().ok()?;
        if node_text.text() == "indentSize" {
            return Some(());
        }
        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                category!("migrate"),
                node.range(),
                markup! {
                    "The option "<Emphasis>"indentSize"</Emphasis>" is deprecated."
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

        Some(RuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::Always,
            message: markup! {
                "Use the property "<Emphasis>"indentWidth"</Emphasis>" instead."
            }
            .to_owned(),
            mutation,
        })
    }
}
