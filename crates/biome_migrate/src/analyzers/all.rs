use crate::{MigrationAction, declare_migration};
use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, Rule, RuleAction, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::{Applicability, category};
use biome_json_syntax::{JsonMember, T};
use biome_rowan::{AstNode, BatchMutationExt, SyntaxElement};

declare_migration! {
    pub(crate) RulesAll {
        version: "2.0.0",
        name: "all",
    }
}

impl Rule for RulesAll {
    type Query = Ast<JsonMember>;
    type State = JsonMember;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let name = node.name().ok()?;
        let node_text = name.inner_string_text().ok()?;
        if node_text.text() == "all" {
            return Some(node.clone());
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            category!("migrate"),
            state.name().ok()?.range(),
            markup! {
                "The property "<Emphasis>"all"</Emphasis>" has been removed."
            }
                .to_owned(),
        ).note(markup!{
            "Due to the increasing number of rules that span in scope and use-case, certain rules can conflict with each other. The option has become more harmful than useful."
        }))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<MigrationAction> {
        let node = state;
        let mut mutation = ctx.root().begin();

        mutation.remove_node(node.clone());

        // If the next sibling token is a comma, remove it to keep as a valid JSON.
        if let Some(SyntaxElement::Token(next_token)) = node.syntax().next_sibling_or_token() {
            if next_token.kind() == T![,] {
                mutation.remove_token(next_token);
            }
        } else {
            // Otherwise, the current node is the last member of the list.
            // Find a previous sibling token and remove it if found to keep as a valid JSON.
            if let Some(SyntaxElement::Token(prev_token)) = node.syntax().prev_sibling_or_token()
                && prev_token.kind() == T![,]
            {
                mutation.remove_token(prev_token);
            }
        }

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
