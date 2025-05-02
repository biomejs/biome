use crate::{MigrationAction, declare_migration};
use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, Rule, RuleAction, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::{Applicability, category};
use biome_json_factory::make::{
    json_member, json_member_list, json_member_name, json_object_value, json_string_literal,
    json_string_value, token,
};
use biome_json_syntax::{AnyJsonValue, JsonMember, JsonRoot, T};
use biome_rowan::{AstNode, BatchMutationExt, TriviaPieceKind};

declare_migration! {
    pub(crate) OrganizeImports {
        version: "2.0.0",
        name: "organizeImports",
    }
}

impl Rule for OrganizeImports {
    type Query = Ast<JsonRoot>;
    type State = (JsonMember, bool);
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let list = node
            .value()
            .ok()?
            .as_json_object_value()?
            .json_member_list();

        for list_item in list.into_iter().flatten() {
            let name = list_item.name().ok()?;
            let text = name.inner_string_text().ok()?;

            if text.text() == "organizeImports" {
                if let Some(object) = list_item.value().ok()?.as_json_object_value() {
                    for member_item in object.json_member_list().into_iter().flatten() {
                        let name = member_item.name().ok()?;
                        let name = name.inner_string_text().ok()?;
                        if name.text() == "enabled" {
                            let value = member_item
                                .value()
                                .ok()?
                                .as_json_boolean_value()?
                                .value_token()
                                .ok()?;
                            if value.text() == "false" {
                                return Some((list_item, false));
                            }
                        }
                    }
                }
                return Some((list_item, true));
            }
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, (member, _): &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            category!("migrate"),
            member.range(),
            markup! {
                "The "<Emphasis>"organizeImports"</Emphasis>" configuration has been moved."
            }
            .to_owned(),
        ).note(markup!{
            "The import sorting was the first assist action, however Biome analyzer infrastructure wasn't mature enough, so it was exposed as a standalone tool. The infrastructure is now ready to welcome it as an assist action."
        }))
    }

    fn action(ctx: &RuleContext<Self>, (member, enabled): &Self::State) -> Option<MigrationAction> {
        let mut mutation = ctx.root().begin();
        let string_literal = if *enabled {
            json_string_literal("on")
        } else {
            json_string_literal("off")
        };
        let action_member = json_member(
            json_member_name(
                json_string_literal("organizeImports").with_leading_trivia(vec![
                    (TriviaPieceKind::Newline, "\n"),
                    (TriviaPieceKind::Whitespace, " ".repeat(8).as_str()),
                ]),
            ),
            token(T![:]).with_trailing_trivia(vec![(TriviaPieceKind::Whitespace, " ")]),
            AnyJsonValue::JsonStringValue(json_string_value(string_literal)),
        );
        let source_member = json_member(
            json_member_name(json_string_literal("source").with_leading_trivia(vec![
                (TriviaPieceKind::Newline, "\n"),
                (TriviaPieceKind::Whitespace, " ".repeat(6).as_str()),
            ])),
            token(T![:]).with_trailing_trivia(vec![(TriviaPieceKind::Whitespace, " ")]),
            AnyJsonValue::JsonObjectValue(json_object_value(
                token(T!['{']).with_leading_trivia(vec![(TriviaPieceKind::Whitespace, " ")]),
                json_member_list(vec![action_member], vec![]),
                token(T!['}']).with_leading_trivia(vec![
                    (TriviaPieceKind::Newline, "\n"),
                    (TriviaPieceKind::Whitespace, " ".repeat(6).as_str()),
                ]),
            )),
        );

        let actions_member = json_member(
            json_member_name(json_string_literal("actions").with_leading_trivia(vec![
                (TriviaPieceKind::Newline, "\n"),
                (TriviaPieceKind::Whitespace, " ".repeat(4).as_str()),
            ])),
            token(T![:]).with_trailing_trivia(vec![(TriviaPieceKind::Whitespace, " ")]),
            AnyJsonValue::JsonObjectValue(json_object_value(
                token(T!['{']).with_leading_trivia(vec![(TriviaPieceKind::Whitespace, " ")]),
                json_member_list(vec![source_member], vec![]),
                token(T!['}']).with_leading_trivia(vec![
                    (TriviaPieceKind::Newline, "\n"),
                    (TriviaPieceKind::Whitespace, " ".repeat(4).as_str()),
                ]),
            )),
        );
        let assist_member = json_member(
            json_member_name(json_string_literal("assist").with_leading_trivia(vec![
                (TriviaPieceKind::Newline, "\n"),
                (TriviaPieceKind::Whitespace, " ".repeat(2).as_str()),
            ])),
            token(T![:]).with_trailing_trivia(vec![(TriviaPieceKind::Whitespace, " ")]),
            AnyJsonValue::JsonObjectValue(json_object_value(
                token(T!['{']).with_leading_trivia(vec![(TriviaPieceKind::Whitespace, " ")]),
                json_member_list(vec![actions_member], vec![]),
                token(T!['}']).with_leading_trivia(vec![
                    (TriviaPieceKind::Newline, "\n"),
                    (TriviaPieceKind::Whitespace, " ".repeat(2).as_str()),
                ]),
            )),
        );

        mutation.replace_node(member.clone(), assist_member);

        Some(RuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            Applicability::Always,
            markup! {
                "Remove the old configuration, and turn off the relative assist action."
            }
            .to_owned(),
            mutation,
        ))
    }
}
