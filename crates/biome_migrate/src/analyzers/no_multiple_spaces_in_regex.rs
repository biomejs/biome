use crate::{MigrationAction, declare_migration};
use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, Rule, RuleAction, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::{Applicability, category};
use biome_json_factory::make::{json_member_name, json_string_literal};
use biome_json_syntax::JsonMemberName;
use biome_rowan::{AstNode, BatchMutationExt};

declare_migration! {
    pub(crate) UseMultipleSpacesInRegex {
        version: "2.0.0",
        name: "useMultipleSpacesInRegex",
    }
}

impl Rule for UseMultipleSpacesInRegex {
    type Query = Ast<JsonMemberName>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let name = ctx.query();
        let text = name.inner_string_text().ok()?;

        if text.text() == "noMultipleSpacesInRegularExpressionLiterals" {
            return Some(());
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            category!("migrate"),
            ctx.query().syntax().text_trimmed_range(),
            markup! {
                "The rule "<Emphasis>"noMultipleSpacesInRegularExpressionLiterals"</Emphasis>" has been renamed "<Emphasis>"noAdjacentSpacesInRegex"</Emphasis>"."
            }
            .to_owned(),
        ))
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<MigrationAction> {
        let mut mutation = ctx.root().begin();
        let new_member_name = json_member_name(json_string_literal("noAdjacentSpacesInRegex"));

        mutation.replace_node(ctx.query().clone(), new_member_name);

        // mutation.replace_node(state.clone(), new_list);
        Some(RuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            Applicability::Always,
            markup! {
                "Rename the rule."
            }
            .to_owned(),
            mutation,
        ))
    }
}
