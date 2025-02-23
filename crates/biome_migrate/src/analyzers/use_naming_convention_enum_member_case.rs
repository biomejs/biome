use crate::{declare_migration, MigrationAction};
use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, Rule, RuleAction, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::{category, Applicability};
use biome_json_factory::make;
use biome_json_syntax::{AnyJsonValue, JsonMember, JsonMemberList, T};
use biome_rowan::{AstNode, AstSeparatedList, AstSeparatedListExt, BatchMutationExt, TokenText};

declare_migration! {
    pub(crate) UseNamingConventionEnumMemberCase {
        version: "2.0.0",
        name: "useNamingConventionEnumMemberCase",
    }
}

impl Rule for UseNamingConventionEnumMemberCase {
    type Query = Ast<JsonMember>;
    type State = TokenText;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let name = node.name().ok()?;
        let text = name.inner_string_text().ok()?;
        if text.text() == "enumMemberCase" {
            let value = node.value().ok()?;
            let value = value.as_json_string_value()?.inner_string_text().ok()?;
            if matches!(
                value.text(),
                "camelCase" | "CONSTANT_CASE" | "PascalCase" | "snake_case"
            ) {
                return Some(value);
            }
        }
        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let name = node.name().ok()?;
        Some(RuleDiagnostic::new(
            category!("migrate"),
            name.range(),
            markup! {
                "The option "<Emphasis>"enumMemberCase"</Emphasis>" has ben removed."
            }
            .to_owned(),
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<MigrationAction> {
        let selector_kind = make::json_member(
            make::json_member_name(make::json_string_literal("kind")),
            make::token(T![:]),
            make::json_string_value(make::json_string_literal("enumMember")).into(),
        );
        let selector = make::json_member(
            make::json_member_name(make::json_string_literal("selector")),
            make::token(T![:]),
            make::json_object_value(
                make::token(T!['{']),
                make::json_member_list([selector_kind], []),
                make::token(T!['}']),
            )
            .into(),
        );
        let formats = make::json_member(
            make::json_member_name(make::json_string_literal("formats")),
            make::token(T![:]),
            make::json_array_value(
                make::token(T!['[']),
                make::json_array_element_list(
                    [make::json_string_value(make::json_string_literal(state.text())).into()],
                    [],
                ),
                make::token(T![']']),
            )
            .into(),
        );
        let enum_member_convention = make::json_object_value(
            make::token(T!['{']),
            make::json_member_list([selector, formats], [make::token(T![,])]),
            make::token(T!['}']),
        );

        let node = ctx.query();
        let parent = node.parent::<JsonMemberList>()?;
        let conventions = parent.into_iter().find_map(|member| {
            let member = member.ok()?;
            let member_name = member.name().ok()?.inner_string_text().ok()?;
            if member_name.text() == "conventions" {
                if let Ok(AnyJsonValue::JsonArrayValue(conventions)) = member.value() {
                    return Some(conventions);
                }
            }
            None
        });
        let mut mutation = ctx.root().begin();
        if let Some(conventions) = conventions {
            let conventions = conventions.elements();
            let new_conventions = if conventions.is_empty() {
                make::json_array_element_list([enum_member_convention.into()], [])
            } else {
                conventions.clone().splice(
                    0..0,
                    [(enum_member_convention.into(), Some(make::token(T![,])))],
                )
            };
            mutation.replace_node(conventions, new_conventions);
            mutation.remove_node(node.clone());
            if let Some(trailing_comma) = node
                .syntax()
                .last_token()
                .and_then(|ignore_last_token| ignore_last_token.next_token())
                .filter(|next_token| next_token.kind() == T![,])
            {
                mutation.remove_token(trailing_comma);
            } else if let Some(leading_comma) = node
                .syntax()
                .first_token()
                .and_then(|ignore_last_token| ignore_last_token.prev_token())
                .filter(|prev_token| prev_token.kind() == T![,])
            {
                mutation.remove_token(leading_comma);
            }
        } else {
            let conventions_array = make::json_array_value(
                make::token(T!['[']),
                make::json_array_element_list([enum_member_convention.into()], []),
                make::token(T![']']),
            );
            let conventions = make::json_member(
                make::json_member_name(make::json_string_literal("conventions")),
                make::token(T![:]),
                conventions_array.into(),
            );
            mutation.replace_node(node.clone(), conventions);
        }

        Some(RuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            Applicability::Always,
            markup! {
                "Use "<Emphasis>"conventions"</Emphasis>" instead of "<Emphasis>"enumMemberCase"</Emphasis>"."
            }
            .to_owned(),
            mutation,
        ))
    }
}
