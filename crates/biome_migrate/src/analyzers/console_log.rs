use crate::rule_mover::RuleMover;
use crate::version_services::Version;
use crate::{declare_migration, MigrationAction};
use biome_analyze::context::RuleContext;
use biome_analyze::{Rule, RuleAction, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::{category, Applicability};
use biome_json_factory::make::{
    json_array_element_list, json_array_value, json_member, json_member_list, json_member_name,
    json_object_value, json_string_literal, json_string_value, token,
};
use biome_json_syntax::{AnyJsonValue, JsonMember, TextRange, T};
use biome_rowan::{AstNode, TriviaPieceKind};

declare_migration! {
    pub(crate) ConsoleLog {
        version: "2.0.0",
        name: "consoleLog",
    }
}

impl Rule for ConsoleLog {
    type Query = Version<JsonMember>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let version = ctx.version();

        if version != "2.0.0" {
            return None;
        }

        let name = node.name().ok()?;
        let node_text = name.inner_string_text().ok()?;
        if node_text.text() == "noConsoleLog" {
            return Some(name.range());
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            category!("migrate"),
            state,
            markup! {
                "The rule "<Emphasis>"noConsoleLog"</Emphasis>" has been removed."
            }
            .to_owned(),
        ))
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<MigrationAction> {
        let mut rule_mover = RuleMover::from_root(ctx.root());
        let member = ctx.query();

        let value = member.value().ok()?;
        let value = match value {
            AnyJsonValue::JsonStringValue(json_string_value) => Some(json_string_value),
            AnyJsonValue::JsonObjectValue(json_object_value) => {
                let list = json_object_value.json_member_list();
                let mut value = None;
                for item in list {
                    if let Ok(item) = item {
                        let text = item.name().ok().and_then(|n| n.inner_string_text().ok());
                        if let Some(text) = text {
                            if text.text() == "level" {
                                value = item.value().ok()?.as_json_string_value().cloned();
                            }
                        }
                    }
                }

                value
            }
            _ => None,
        };

        let allow_option = json_member(
            json_member_name(json_string_literal("allow").with_leading_trivia(vec![
                (TriviaPieceKind::Newline, "\n"),
                (TriviaPieceKind::Whitespace, " ".repeat(12).as_str()),
            ])),
            token(T![:]).with_trailing_trivia(vec![(TriviaPieceKind::Whitespace, " ")]),
            AnyJsonValue::JsonArrayValue(json_array_value(
                token(T!['[']),
                json_array_element_list(
                    vec![AnyJsonValue::JsonStringValue(json_string_value(
                        json_string_literal("log"),
                    ))],
                    vec![],
                ),
                token(T![']']),
            )),
        );
        let rule_options = json_member(
            json_member_name(json_string_literal("options").with_leading_trivia(vec![
                (TriviaPieceKind::Newline, "\n"),
                (TriviaPieceKind::Whitespace, " ".repeat(10).as_str()),
            ])),
            token(T![:]).with_trailing_trivia(vec![(TriviaPieceKind::Whitespace, " ")]),
            AnyJsonValue::JsonObjectValue(json_object_value(
                token(T!['{']),
                json_member_list(vec![allow_option], vec![]),
                token(T!['}']).with_leading_trivia(vec![
                    (TriviaPieceKind::Newline, "\n"),
                    (TriviaPieceKind::Whitespace, " ".repeat(10).as_str()),
                ]),
            )),
        );

        let level_option = json_member(
            json_member_name(json_string_literal("level").with_leading_trivia(vec![
                (TriviaPieceKind::Newline, "\n"),
                (TriviaPieceKind::Whitespace, " ".repeat(10).as_str()),
            ])),
            token(T![:]).with_trailing_trivia(vec![(TriviaPieceKind::Whitespace, " ")]),
            AnyJsonValue::JsonStringValue(
                value.unwrap_or(json_string_value(json_string_literal("error"))),
            ),
        );

        let rule_member = json_member(
            json_member_name(json_string_literal("noConsole").with_leading_trivia(vec![
                (TriviaPieceKind::Newline, "\n"),
                (TriviaPieceKind::Whitespace, " ".repeat(8).as_str()),
            ])),
            token(T![:]).with_trailing_trivia(vec![(TriviaPieceKind::Whitespace, " ")]),
            AnyJsonValue::JsonObjectValue(json_object_value(
                token(T!['{']),
                json_member_list(vec![level_option, rule_options], vec![token(T![,])]),
                token(T!['}']).with_leading_trivia(vec![
                    (TriviaPieceKind::Newline, "\n"),
                    (TriviaPieceKind::Whitespace, " ".repeat(8).as_str()),
                ]),
            )),
        );
        rule_mover.replace_rule("noConsoleLog", rule_member, "suspicious");

        let mutation = rule_mover.run_queries()?;

        Some(RuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            Applicability::Always,
            markup! {
                "Use the rule "<Emphasis>"useConsole"</Emphasis>" instead."
            }
            .to_owned(),
            mutation,
        ))
    }
}
