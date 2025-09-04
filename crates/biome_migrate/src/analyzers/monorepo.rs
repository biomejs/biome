use crate::declare_migration;
use crate::services::IsRoot;
use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, Rule, RuleAction, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::{Applicability, category};
use biome_json_factory::make::{
    json_boolean_value, json_member, json_member_list, json_member_name, json_string_literal, token,
};
use biome_json_syntax::{AnyJsonValue, JsonLanguage, JsonRoot, T};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, TriviaPieceKind};
use std::collections::VecDeque;

declare_migration! {
    pub(crate) Monorepo {
        version: "2.0.0",
        name: "monorepo",
    }
}

impl Rule for Monorepo {
    type Query = Ast<JsonRoot>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let is_root_service = ctx
            .get_service::<IsRoot>()
            .expect("IsRoot service not found.");

        let root = ctx.root();
        let root = root.value().ok()?;
        let root = root.as_json_object_value()?;
        let value = root
            .find_member("root")
            .and_then(|member| member.value().ok())
            .and_then(|value| value.as_json_boolean_value().cloned())
            .and_then(|value| value.value_token().ok());

        if is_root_service.is_root() || value.is_some_and(|value| value.text_trimmed() == "false") {
            None
        } else {
            Some(())
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            category!("migrate"),
            ctx.query().range(),
            markup! {
                "Found a configuration file that isn't in the root of the project. Add "<Emphasis>"root: \"false\""</Emphasis>" to the configuration file."
            }
            .to_owned(),
        ).note("Since the introduction of monorepo support, nested configuration files that need to have a special field disabled so they can function correctly."))
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleAction<JsonLanguage>> {
        let root = ctx.root();
        let mut mutation = root.clone().begin();

        let root = root.value().ok()?;
        let root = root.as_json_object_value()?;
        let member_list = root.json_member_list();
        let mut separators: Vec<_> = member_list.separators().flatten().collect();
        let mut list: VecDeque<_> = member_list.iter().flatten().collect();

        list.push_front(json_member(
            json_member_name(json_string_literal("root")),
            token(T![:]).with_trailing_trivia(vec![(TriviaPieceKind::Whitespace, " ")]),
            AnyJsonValue::JsonBooleanValue(json_boolean_value(token(T![false]))),
        ));
        separators.push(token(T![,]));

        let new_list = json_member_list(list, separators);

        mutation.replace_node(member_list, new_list);

        Some(RuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            Applicability::Always,
            markup! { "Add the new filed to this configuration file." }.to_owned(),
            mutation,
        ))
    }
}
