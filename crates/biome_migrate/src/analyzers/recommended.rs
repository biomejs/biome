use crate::{MigrationAction, declare_migration};
use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, Rule, RuleAction, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::{Applicability, category};
use biome_json_factory::make::{
    json_member, json_member_name, json_string_literal, json_string_value, token,
};
use biome_json_syntax::{AnyJsonValue, JsonMember, T};
use biome_rowan::{AstNode, BatchMutationExt};

declare_migration! {
    pub(crate) Recommended {
        version: "2.5.0",
        name: "recommended",
    }
}

impl Rule for Recommended {
    type Query = Ast<JsonMember>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let node_text = node.name().ok()?.inner_string_text()?;
        if node_text.text() == "recommended" {
            return Some(());
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            category!("migrate"),
            ctx.query().range(),
            markup! {
                "The recommended option is deprecated."
            }
            .to_owned(),
        ))
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<MigrationAction> {
        let node = ctx.query();
        let member_value = node.value().ok()?;
        let bool_value = member_value.as_json_boolean_value()?;
        let mut mutation = ctx.root().begin();
        let bool_value_str = bool_value.value_token().ok()?;

        let new_value = if bool_value_str.text() == "true" {
            "recommended"
        } else {
            "none"
        };

        let name = node.name().ok()?;
        let name_token = name.as_json_member_name()?.value_token().ok()?;
        let colon_token = node.colon_token().ok()?;
        let value = node
            .value()
            .ok()?
            .as_json_boolean_value()?
            .value_token()
            .ok()?;

        let new_value = json_string_value(
            json_string_literal(new_value)
                .with_leading_trivia_pieces(value.leading_trivia().pieces())
                .with_trailing_trivia_pieces(value.trailing_trivia().pieces()),
        );
        // Preserve the original key's surrounding whitespace.
        let new_key = json_string_literal("preset")
            .with_leading_trivia_pieces(name_token.leading_trivia().pieces())
            .with_trailing_trivia_pieces(name_token.trailing_trivia().pieces());
        // Preserve both sides of the colon separately — original code overwrote
        // leading with trailing, which broke inline formatting.
        let new_colon = token(T![:])
            .with_leading_trivia_pieces(colon_token.leading_trivia().pieces())
            .with_trailing_trivia_pieces(colon_token.trailing_trivia().pieces());
        let member = json_member(
            json_member_name(new_key).into(),
            new_colon,
            AnyJsonValue::JsonStringValue(new_value.clone()),
        );

        mutation.replace_node(node.clone(), member);

        Some(RuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            Applicability::Always,
            markup! {
                "Use the new "<Emphasis>"preset"</Emphasis>" field instead."
            }
            .to_owned(),
            mutation,
        ))
    }
}
