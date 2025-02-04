use crate::rule_mover::AnalyzerMover;
use crate::{declare_migration, MigrationAction};
use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, Rule, RuleAction, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::{category, Applicability};
use biome_json_syntax::JsonMember;
use biome_rowan::{AstNode, TextRange};

declare_migration! {
    pub(crate) UseWhile {
        version: "2.0.0",
        name: "useWhile",
    }
}

impl Rule for UseWhile {
    type Query = Ast<JsonMember>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let name = node.name().ok()?;
        let text = name.inner_string_text().ok()?;

        if text.text() == "style" {
            if let Some(object) = node.value().ok()?.as_json_object_value() {
                for item in object.json_member_list().into_iter().flatten() {
                    let name = item.name().ok()?;
                    let text = name.inner_string_text().ok()?;
                    if text.text() == "useWhile" {
                        return Some(name.range());
                    }
                }
            }
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            category!("migrate"),
            state,
            markup! {
                "The rule "<Emphasis>"useWhile"</Emphasis>" has ben moved to the "<Emphasis>"complexity"</Emphasis>" group."
            }
                .to_owned(),
        ))
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<MigrationAction> {
        let root = ctx.root();
        let mut rule_mover = AnalyzerMover::from_root(root.clone());
        rule_mover.move_rule("useWhile", "style", "complexity");
        let mutation = rule_mover.run_queries()?;

        // mutation.replace_node(state.clone(), new_list);
        Some(RuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            Applicability::Always,
            markup! {
                "Move the rule in the correct group."
            }
            .to_owned(),
            mutation,
        ))
    }
}
