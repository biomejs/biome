use crate::{declare_migration, MigrationAction};
use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, Rule, RuleAction, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::{category, Applicability};
use biome_json_factory::make::{ident, json_string_value};
use biome_json_syntax::{JsonMember, TextRange};
use biome_rowan::{AstNode, BatchMutationExt};
use std::env;

declare_migration! {
    pub(crate) Schema {
        version: "1.5.0",
        name: "schema",
    }
}

impl Rule for Schema {
    type Query = Ast<JsonMember>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let node_text = node.name().ok()?.inner_string_text().ok()?;
        let member_value = node.value().ok()?;
        if node_text.text() == "$schema" {
            let string_value = member_value.as_json_string_value()?;
            let value = string_value.inner_string_text().ok()?;
            let value = value
                .text()
                .strip_prefix("https://biomejs.dev/schemas/")?
                .strip_suffix("/schema.json");

            if let Ok(version) = env::var("BIOME_VERSION") {
                if let Some(current_version) = value {
                    if current_version != version {
                        return Some(string_value.range());
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
                "The version of the schema is outdated."
            }
            .to_owned(),
        ).note(markup!{
            "Having an old version of the schema won't allow you to see new options or deprecated ones."
        }))
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<MigrationAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();
        let version = env::var("BIOME_VERSION").ok()?;
        let schema = format!("\"https://biomejs.dev/schemas/{version}/schema.json\"");

        let new_node = json_string_value(ident(&schema));
        let member_value = node.value().ok()?;
        let member_value = member_value.as_json_string_value()?;
        mutation.replace_node(member_value.clone(), new_node);

        Some(RuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            Applicability::Always,
            markup! {
                "Update the URL."
            }
            .to_owned(),
            mutation,
        ))
    }
}
