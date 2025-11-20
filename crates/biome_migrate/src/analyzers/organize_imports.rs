use crate::{MigrationAction, declare_migration};
use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, Rule, RuleAction, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::{Applicability, category};
use biome_json_factory::make;
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
    type State = JsonMember;
    type Signals = Box<[Self::State]>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let Ok(AnyJsonValue::JsonObjectValue(root)) = node.value() else {
            return Box::default();
        };
        let mut result = Vec::new();
        for member in root.json_member_list().into_iter().flatten() {
            let Ok(name) = member.name().and_then(|n| n.inner_string_text()) else {
                continue;
            };
            match name.text() {
                "organizeImports" => {
                    result.push(member);
                }
                "overrides" => {
                    let Ok(AnyJsonValue::JsonArrayValue(overrides)) = member.value() else {
                        continue;
                    };
                    for override_item in overrides.elements() {
                        let Ok(AnyJsonValue::JsonObjectValue(override_item)) = override_item else {
                            continue;
                        };
                        for member in override_item.json_member_list().into_iter().flatten() {
                            let Ok(name) = member.name().and_then(|n| n.inner_string_text()) else {
                                continue;
                            };
                            if name.text() == "organizeImports" {
                                result.push(member);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        result.into()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, member: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            category!("migrate"),
            member.name().ok()?.range(),
            markup! {
                "The "<Emphasis>"organizeImports"</Emphasis>" configuration has been moved."
            }
            .to_owned(),
        ).note(markup!{
            "The import sorting was the first assist action, however Biome analyzer infrastructure wasn't mature enough, so it was exposed as a standalone tool. The infrastructure is now ready to welcome it as an assist action."
        }))
    }

    fn action(ctx: &RuleContext<Self>, member: &Self::State) -> Option<MigrationAction> {
        let is_enabled = is_organize_imports_enabled(member);
        let on_or_off = if is_enabled { "on" } else { "off" };
        let indent = member.syntax().first_token()?.indentation_trivia_pieces();

        let action_member = make::json_member(
            make::json_member_name(make::json_string_literal("organizeImports")),
            make::token(T![:]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
            make::json_string_value(
                make::json_string_literal(on_or_off)
                    .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
            )
            .into(),
        );
        let source_member = make::json_member(
            make::json_member_name(make::json_string_literal("source")),
            make::token(T![:]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
            make::json_object_value(
                make::token(T!['{']).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
                make::json_member_list([action_member], []),
                make::token(T!['}']).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
            )
            .into(),
        );
        let actions_member = make::json_member(
            make::json_member_name(make::json_string_literal("actions")),
            make::token(T![:]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
            make::json_object_value(
                make::token(T!['{']).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
                make::json_member_list([source_member], []),
                make::token(T!['}']).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
            )
            .into(),
        );
        let assist_member = make::json_member(
            make::json_member_name(
                make::json_string_literal("assist").prepend_trivia_pieces(indent.clone()),
            ),
            make::token(T![:]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
            AnyJsonValue::JsonObjectValue(make::json_object_value(
                make::token(T!['{']).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
                make::json_member_list([actions_member], []),
                make::token(T!['}']),
            )),
        );

        let mut mutation = ctx.root().begin();
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

fn is_organize_imports_enabled(organize_imports_member: &JsonMember) -> bool {
    if let Ok(AnyJsonValue::JsonObjectValue(object)) = organize_imports_member.value() {
        for member_val in object.json_member_list().into_iter().flatten() {
            if member_val
                .name()
                .and_then(|val| val.inner_string_text())
                .is_ok_and(|val| val.text() == "enabled")
                && let Ok(AnyJsonValue::JsonBooleanValue(enabled)) = member_val.value()
                && let Ok(enabled) = enabled.value_token()
            {
                return enabled.text_trimmed() != "false";
            }
        }
    }
    true
}
