use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, Rule, RuleAction, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::{Applicability, category};
use biome_json_factory::make;
use biome_json_syntax::{
    AnyJsonValue, JsonArrayValue, JsonLanguage, JsonMember, JsonMemberList, T,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, TriviaPieceKind};

use crate::declare_migration;

declare_migration! {
    pub(crate) NoRestrictedGlobals {
        version: "2.0.0",
        name: "noRestrictedGlobals",
    }
}

impl Rule for NoRestrictedGlobals {
    type Query = Ast<JsonMember>;
    type State = JsonArrayValue;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if node.name().ok()?.inner_string_text().ok()?.text() != "noRestrictedGlobals" {
            return None;
        }

        let options = find_json_member_by_name(
            node.value()
                .ok()?
                .as_json_object_value()?
                .json_member_list(),
            "options",
        )?;

        let denied_globals = find_json_member_by_name(
            options
                .value()
                .ok()?
                .as_json_object_value()?
                .json_member_list(),
            "deniedGlobals",
        )?;

        denied_globals.value().ok()?.as_json_array_value().cloned()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            category!("migrate"),
            state.parent::<JsonMember>()?.syntax().text_trimmed_range(),
            markup! {
                "The "<Emphasis>"deniedGlobals"</Emphasis>" option of the "<Emphasis>"noRestrictedGlobals"</Emphasis>" rule has been changed from an array to a record."
            }
                .to_owned(),
        ).note(markup! { "You can now customize the message for each restricted global name."}))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleAction<JsonLanguage>> {
        let mut mutation = ctx.root().begin();

        let members = state
            .elements()
            .into_iter()
            .flatten()
            .filter_map(|value| value.as_json_string_value().cloned())
            .filter_map(|value| {
                let trailing_trivia = value.syntax().last_trailing_trivia()?;
                let value = value.with_trailing_trivia_pieces([])?;

                Some(make::json_member(
                    make::json_member_name(value.value_token().ok()?),
                    make::token(T![:]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
                    make::json_string_value(make::json_string_literal(
                        "TODO: Add a custom message here.",
                    ))
                    .with_trailing_trivia_pieces(trailing_trivia.pieces())?
                    .into(),
                ))
            })
            .collect::<Vec<_>>();

        let separators = state.elements().separators().flatten().collect::<Vec<_>>();

        let object = make::json_object_value(
            make::token(T!['{'])
                .with_leading_trivia_pieces(state.l_brack_token().ok()?.leading_trivia().pieces())
                .with_trailing_trivia_pieces(
                    state.l_brack_token().ok()?.trailing_trivia().pieces(),
                ),
            make::json_member_list(members, separators),
            make::token(T!['}'])
                .with_leading_trivia_pieces(state.r_brack_token().ok()?.leading_trivia().pieces())
                .with_trailing_trivia_pieces(
                    state.r_brack_token().ok()?.trailing_trivia().pieces(),
                ),
        );

        mutation.replace_node(AnyJsonValue::from(state.clone()), object.into());

        Some(RuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            Applicability::Always,
            markup! { "Turn into a record." }.to_owned(),
            mutation,
        ))
    }
}

fn find_json_member_by_name(members: JsonMemberList, name: &str) -> Option<JsonMember> {
    members.into_iter().flatten().find(|member| {
        member
            .name()
            .ok()
            .and_then(|name| name.inner_string_text().ok())
            .is_some_and(|text| text.text() == name)
    })
}
