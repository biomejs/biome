use crate::declare_migration;
use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, Rule, RuleAction, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::{Applicability, category};
use biome_json_factory::make::{
    json_array_element_list, json_array_value, json_member, json_member_list, json_member_name,
    json_string_literal, json_string_value, token,
};
use biome_json_syntax::{AnyJsonValue, JsonLanguage, JsonMember, T};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, TriviaPieceKind};
use std::collections::VecDeque;

declare_migration! {
    pub(crate) IgnoreScanner {
        version: "2.0.0",
        name: "ignoreScanner",
    }
}

impl Rule for IgnoreScanner {
    type Query = Ast<JsonMember>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let member = ctx.query();
        let name = member.name().ok()?;

        if name.inner_string_text().ok()?.text() != "experimentalScannerIgnores" {
            return None;
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            category!("migrate"),
            ctx.query().range(),
            markup! {
                "The field "<Emphasis>"experimentalScannerIgnores"</Emphasis>" has been removed. Its usage is now stable and moved inside a new ignore syntax."
            }
                .to_owned(),
        ))
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleAction<JsonLanguage>> {
        let member = ctx.query();

        let mut experimental_ignore_list = member
            .value()
            .ok()?
            .as_json_array_value()?
            .elements()
            .iter()
            .flatten()
            .filter_map(|element| {
                let value = element.as_json_string_value()?;
                let value = value.inner_string_text().ok()?;
                let new_inner_string = format!("!!**/{}", value.text());
                Some(AnyJsonValue::JsonStringValue(json_string_value(
                    json_string_literal(new_inner_string.as_str()),
                )))
            })
            .collect::<VecDeque<_>>();
        let mut mutation = ctx.root().begin();
        let files_member = member
            .syntax()
            .ancestors()
            .skip(1)
            .find_map(JsonMember::cast)?;

        // List without `experimentalScannerIgnores`
        // This is the node we will update via mutation
        let node_files_list = files_member
            .value()
            .ok()?
            .as_json_object_value()?
            .json_member_list();

        let mut includes_found = false;
        let mut files_list: Vec<_> = node_files_list
            .iter()
            .flatten()
            .filter_map(|member| {
                let text = member.name().ok()?.inner_string_text().ok()?;
                if text.text() == "experimentalScannerIgnores" {
                    None
                } else if text.text() == "includes" {
                    let array_value = member.value().ok()?.as_json_array_value()?.clone();
                    let mut new_list: Vec<_> =
                        array_value.clone().elements().iter().flatten().collect();
                    let mut separators = vec![];
                    new_list.extend(experimental_ignore_list.clone());

                    if new_list.len() > 1 {
                        for _ in 0..new_list.len() - 1 {
                            separators.push(
                                token(T![,])
                                    .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
                            );
                        }
                    }
                    let new_member = json_member(
                        member.name().ok()?,
                        member.colon_token().ok()?,
                        AnyJsonValue::JsonArrayValue(json_array_value(
                            array_value.l_brack_token().ok()?,
                            json_array_element_list(new_list, separators),
                            array_value.r_brack_token().ok()?,
                        )),
                    );
                    includes_found = true;
                    Some(new_member)
                } else {
                    Some(member)
                }
            })
            .collect::<Vec<_>>();

        // If includes doesn't exist, we create a new one from scratch.
        if !includes_found {
            let mut separators = vec![];
            // We're creating a new includes, and it must contain `**` as first item
            experimental_ignore_list.push_front(AnyJsonValue::JsonStringValue(json_string_value(
                json_string_literal("**"),
            )));
            if experimental_ignore_list.len() > 1 {
                for _ in 0..experimental_ignore_list.len() - 1 {
                    separators.push(
                        token(T![,]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
                    );
                }
            }
            let value = json_array_value(
                token(T!['[']),
                json_array_element_list(experimental_ignore_list, separators),
                token(T![']']),
            );
            files_list.push(json_member(
                json_member_name(json_string_literal("includes")),
                token(T![:]),
                AnyJsonValue::JsonArrayValue(value),
            ))
        }

        let mut separators = vec![];
        if files_list.len() > 1 {
            for _ in 0..files_list.len() - 1 {
                separators
                    .push(token(T![,]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]));
            }
        }

        mutation.replace_node(node_files_list, json_member_list(files_list, separators));

        Some(RuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            Applicability::Always,
            markup! { "Update the "<Emphasis>"files.includes"</Emphasis>" field." }.to_owned(),
            mutation,
        ))
    }
}
