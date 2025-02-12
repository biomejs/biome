use crate::{declare_migration, MigrationAction};
use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, Rule, RuleAction, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::{category, Applicability};
use biome_json_factory::make::{
    json_array_element_list, json_array_value, json_member, json_member_list, json_member_name,
    json_object_value, json_string_literal, json_string_value, token,
};
use biome_json_syntax::{AnyJsonValue, JsonMember, JsonRoot, JsonStringValue, T};
use biome_rowan::{AstNode, BatchMutationExt, TriviaPieceKind, WalkEvent};
use std::collections::HashMap;
use std::sync::LazyLock;

declare_migration! {
    pub(crate) DeletedRules {
        version: "2.0.0",
        name: "deletedRules",
    }
}

static REMOVED_RULES: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    HashMap::from([
        ("noConsoleLog", "noConsole"),
        ("useSingleCaseStatement", "noSwitchDeclarations"),
        ("useShorthandArrayType", "useConsistentArrayType"),
        ("noNewSymbol", "noInvalidBuiltinInstantiation"),
    ])
});

pub(crate) struct RuleState {
    rule_member: JsonMember,
    rule_name: Box<str>,
}

impl Rule for DeletedRules {
    type Query = Ast<JsonRoot>;
    type State = RuleState;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let mut rules = vec![];
        let events = node.syntax().preorder();

        for event in events {
            match event {
                WalkEvent::Enter(node) => {
                    let member = JsonMember::cast(node);
                    if let Some(member) = member {
                        let Ok(name) = member.name() else { continue };

                        let Ok(node_text) = name.inner_string_text() else {
                            continue;
                        };

                        if REMOVED_RULES.contains_key(node_text.text()) {
                            rules.push(RuleState {
                                rule_member: member,
                                rule_name: Box::from(node_text.text()),
                            });
                        }
                    }
                }
                WalkEvent::Leave(_) => {}
            }
        }

        rules
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let name = state.rule_member.name().ok()?;
        Some(RuleDiagnostic::new(
            category!("migrate"),
            name.range(),
            markup! {
                "The rule "<Emphasis>"noConsoleLog"</Emphasis>" has been removed."
            }
            .to_owned(),
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<MigrationAction> {
        let mut mutation = ctx.root().begin();

        let value = state.rule_member.value().ok()?;
        let value = match value {
            AnyJsonValue::JsonStringValue(json_string_value) => Some(json_string_value),
            AnyJsonValue::JsonObjectValue(json_object_value) => {
                let list = json_object_value.json_member_list();
                let mut value = None;
                for item in list.into_iter().flatten() {
                    let text = item.name().ok().and_then(|n| n.inner_string_text().ok());
                    if let Some(text) = text {
                        if text.text() == "level" {
                            value = item.value().ok()?.as_json_string_value().cloned();
                        }
                    }
                }

                value
            }
            _ => None,
        };
        match state.rule_name.as_ref() {
            "noConsoleLog" => {
                mutation.replace_node(state.rule_member.clone(), create_console_log_member(value))
            }
            "useSingleCaseStatement" => mutation.replace_node(
                state.rule_member.clone(),
                create_no_switch_declarations(value),
            ),
            "useShorthandArrayType" => mutation.replace_node(
                state.rule_member.clone(),
                create_consistent_array_type(value),
            ),
            "noNewSymbol" | "noInvalidNewBuiltin" => {
                mutation.replace_node(state.rule_member.clone(), create_no_invalid_builtin(value))
            }
            _ => return None,
        }

        Some(RuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            Applicability::Always,
            markup! {
                "Use the rule "<Emphasis>{state.rule_name}</Emphasis>" instead."
            }
            .to_owned(),
            mutation,
        ))
    }
}

/// Creates the member for `noConsoleLog`
fn create_console_log_member(value: Option<JsonStringValue>) -> JsonMember {
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

    json_member(
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
    )
}

/// Creates the member for `noSwitchDeclarations`
fn create_no_switch_declarations(value: Option<JsonStringValue>) -> JsonMember {
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

    json_member(
        json_member_name(
            json_string_literal("noSwitchDeclarations").with_leading_trivia(vec![
                (TriviaPieceKind::Newline, "\n"),
                (TriviaPieceKind::Whitespace, " ".repeat(8).as_str()),
            ]),
        ),
        token(T![:]).with_trailing_trivia(vec![(TriviaPieceKind::Whitespace, " ")]),
        AnyJsonValue::JsonObjectValue(json_object_value(
            token(T!['{']),
            json_member_list(vec![level_option], vec![]),
            token(T!['}']).with_leading_trivia(vec![
                (TriviaPieceKind::Newline, "\n"),
                (TriviaPieceKind::Whitespace, " ".repeat(8).as_str()),
            ]),
        )),
    )
}

/// Creates the member for `useConsistentArrayType`
fn create_consistent_array_type(value: Option<JsonStringValue>) -> JsonMember {
    let allow_option = json_member(
        json_member_name(json_string_literal("syntax").with_leading_trivia(vec![
            (TriviaPieceKind::Newline, "\n"),
            (TriviaPieceKind::Whitespace, " ".repeat(12).as_str()),
        ])),
        token(T![:]).with_trailing_trivia(vec![(TriviaPieceKind::Whitespace, " ")]),
        AnyJsonValue::JsonStringValue(json_string_value(json_string_literal("shorthand"))),
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
            value.unwrap_or(json_string_value(json_string_literal("warn"))),
        ),
    );

    json_member(
        json_member_name(
            json_string_literal("useConsistentArrayType").with_leading_trivia(vec![
                (TriviaPieceKind::Newline, "\n"),
                (TriviaPieceKind::Whitespace, " ".repeat(8).as_str()),
            ]),
        ),
        token(T![:]).with_trailing_trivia(vec![(TriviaPieceKind::Whitespace, " ")]),
        AnyJsonValue::JsonObjectValue(json_object_value(
            token(T!['{']),
            json_member_list(vec![level_option, rule_options], vec![token(T![,])]),
            token(T!['}']).with_leading_trivia(vec![
                (TriviaPieceKind::Newline, "\n"),
                (TriviaPieceKind::Whitespace, " ".repeat(8).as_str()),
            ]),
        )),
    )
}

/// Creates the member for `noInvalidBuiltinInstantiation`
fn create_no_invalid_builtin(value: Option<JsonStringValue>) -> JsonMember {
    let level_option = json_member(
        json_member_name(json_string_literal("level").with_leading_trivia(vec![
            (TriviaPieceKind::Newline, "\n"),
            (TriviaPieceKind::Whitespace, " ".repeat(10).as_str()),
        ])),
        token(T![:]).with_trailing_trivia(vec![(TriviaPieceKind::Whitespace, " ")]),
        AnyJsonValue::JsonStringValue(
            value.unwrap_or(json_string_value(json_string_literal("warn"))),
        ),
    );

    json_member(
        json_member_name(
            json_string_literal("noInvalidBuiltinInstantiation").with_leading_trivia(vec![
                (TriviaPieceKind::Newline, "\n"),
                (TriviaPieceKind::Whitespace, " ".repeat(8).as_str()),
            ]),
        ),
        token(T![:]).with_trailing_trivia(vec![(TriviaPieceKind::Whitespace, " ")]),
        AnyJsonValue::JsonObjectValue(json_object_value(
            token(T!['{']),
            json_member_list(vec![level_option], vec![]),
            token(T!['}']).with_leading_trivia(vec![
                (TriviaPieceKind::Newline, "\n"),
                (TriviaPieceKind::Whitespace, " ".repeat(8).as_str()),
            ]),
        )),
    )
}
