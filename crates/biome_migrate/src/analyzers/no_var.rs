use crate::{declare_migration, MigrationAction};
use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, Rule, RuleAction, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::{category, Applicability};
use biome_json_factory::make::{
    json_member, json_member_list, json_member_name, json_object_value, json_string_literal, token,
};
use biome_json_syntax::{AnyJsonValue, JsonMember, T};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, TriviaPieceKind};

declare_migration! {
    pub(crate) NoVar {
        version: "2.0.0",
        name: "noVar",
    }
}

impl Rule for NoVar {
    type Query = Ast<JsonMember>;
    type State = JsonMember;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let name = node.name().ok()?;
        let text = name.inner_string_text().ok()?;

        let mut no_var_member = None;
        if text.text() == "style" {
            let object = node.value().ok()?;
            let object = object.as_json_object_value()?;
            for item in object.json_member_list().into_iter().flatten() {
                let name = item.name().ok()?;
                let text = name.inner_string_text().ok()?;
                if text.text() == "noVar" {
                    no_var_member = Some(item);
                }
            }
        }

        no_var_member
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            category!("migrate"),
            state.range(),
            markup! {
                "The rule "<Emphasis>"style/noVar"</Emphasis>" has ben moved to the "<Emphasis>"suspicious"</Emphasis>" group."
            }
                .to_owned(),
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<MigrationAction> {
        let mut mutation = ctx.root().begin();
        let no_var_member = state;
        let style_member = ctx.query();

        let style_member_list = style_member
            .value()
            .ok()?
            .as_json_object_value()?
            .json_member_list()
            .iter()
            .flatten()
            .filter_map(|member| {
                let name = member.name().ok()?;
                let text = name.inner_string_text().ok()?;
                if text.text() == "noVar" {
                    None
                } else {
                    Some(member)
                }
            })
            .collect::<Vec<_>>();

        let mut separators = vec![];
        for _ in 0..style_member_list.len().saturating_sub(1) {
            separators.push(token(T![,]))
        }

        let rules_member = style_member.syntax().ancestors().find_map(|node| {
            let member = JsonMember::cast(node)?;
            let name = member.name().ok()?;
            let text = name.inner_string_text().ok()?;
            if text.text() == "rules" {
                Some(member)
            } else {
                None
            }
        })?;

        let style_member = json_member(
            style_member.name().ok()?,
            style_member.colon_token().ok()?,
            AnyJsonValue::JsonObjectValue(json_object_value(
                token(T!['{']).with_leading_trivia(vec![(TriviaPieceKind::Whitespace, " ")]),
                json_member_list(style_member_list, separators),
                token(T!['}']).with_leading_trivia(vec![
                    (TriviaPieceKind::Newline, "\n"),
                    (TriviaPieceKind::Whitespace, " ".repeat(6).as_str()),
                ]),
            )),
        );

        let linter_member_list = rules_member
            .value()
            .ok()?
            .as_json_object_value()?
            .json_member_list();

        let suspicious_member = linter_member_list
            .iter()
            .flatten()
            .find_map(|member| {
                let name = member.name().ok()?.inner_string_text().ok()?;
                if name.text() == "suspicious" {
                    Some(member)
                } else {
                    None
                }
            })
            .and_then(|suspicious_member| {
                // we inject the new member here
                let (list, mut separators) = suspicious_member.unzip_elements()?;
                for _ in 0..list.len().saturating_sub(1) {
                    separators.push(token(T![,]))
                }

                Some(json_member(
                    suspicious_member.name().ok()?,
                    suspicious_member.colon_token().ok()?,
                    AnyJsonValue::JsonObjectValue(json_object_value(
                        suspicious_member
                            .value()
                            .ok()?
                            .as_json_object_value()?
                            .l_curly_token()
                            .ok()?,
                        json_member_list(list, separators),
                        suspicious_member
                            .value()
                            .ok()?
                            .as_json_object_value()?
                            .r_curly_token()
                            .ok()?,
                    )),
                ))
            })
            .unwrap_or(json_member(
                json_member_name(json_string_literal("suspicious").with_leading_trivia(vec![
                    (TriviaPieceKind::Newline, "\n"),
                    (TriviaPieceKind::Whitespace, " ".repeat(4).as_str()),
                ])),
                token(T![:]).with_trailing_trivia(vec![(TriviaPieceKind::Whitespace, " ")]),
                AnyJsonValue::JsonObjectValue(json_object_value(
                    token(T!['{']).with_leading_trivia(vec![(TriviaPieceKind::Whitespace, " ")]),
                    json_member_list(vec![no_var_member.clone()], vec![]),
                    token(T!['}']).with_leading_trivia(vec![
                        (TriviaPieceKind::Newline, "\n"),
                        (TriviaPieceKind::Whitespace, " ".repeat(4).as_str()),
                    ]),
                )),
            ));

        let mut separators = vec![];
        let mut new_members = vec![];
        for item in linter_member_list.iter().flatten() {
            let name = item.name().ok()?.inner_string_text().ok()?;
            if name.text() != "suspicious" {
                new_members.push(suspicious_member.clone());
            } else if name.text() == "style" {
                new_members.push(style_member.clone());
            } else {
                new_members.push(item);
            }
        }
        new_members.push(suspicious_member);
        for _ in 0..new_members.len().saturating_sub(1) {
            separators.push(token(T![,]))
        }

        mutation.replace_node(
            linter_member_list,
            json_member_list(new_members, separators),
        );

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
