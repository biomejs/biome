use crate::rule_mover::RuleMover;
use crate::version_services::Version;
use crate::{declare_migration, MigrationAction};
use biome_analyze::context::RuleContext;
use biome_analyze::{Rule, RuleAction, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::{category, Applicability};
use biome_json_factory::make::{
    ident, json_member, json_member_list, json_member_name, json_string_literal, json_string_value,
    token,
};
use biome_json_syntax::{AnyJsonValue, JsonMember, JsonMemberList, T};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, TriviaPieceKind};

declare_migration! {
    pub(crate) NoVar {
        version: "2.0.0",
        name: "noVar",
    }
}

impl Rule for NoVar {
    type Query = Version<JsonMember>;
    type State = JsonMemberList;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let version = ctx.version();

        if version != "2.0.0" {
            return None;
        }

        let name = node.name().ok()?;
        let text = name.inner_string_text().ok()?;

        if text.text() == "noVar" {
            return node.syntax().parent().and_then(JsonMemberList::cast);
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            category!("migrate"),
            ctx.query().range(),
            markup! {
                "The rule "<Emphasis>"style/noVar"</Emphasis>" has ben moved to the "<Emphasis>"suspicious"</Emphasis>" group."
            }
                .to_owned(),
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<MigrationAction> {
        let node = ctx.query();
        let root = ctx.root();
        let linter_member = RuleMover::from_root(root.clone(), "suspicious");
        let mut mutation = root.begin();

        let members = vec![json_member(
            json_member_name(json_string_literal("noVar").with_leading_trivia(vec![
                (TriviaPieceKind::Newline, "\n"),
                (TriviaPieceKind::Whitespace, " ".repeat(8).as_str()),
            ])),
            token(T![:]),
            AnyJsonValue::JsonStringValue(json_string_value(json_string_literal("on"))),
        )];

        linter_member.replace(&mut mutation, members, vec![]);

        let mut separators = vec![];
        let mut members = vec![];
        let len = state.len();
        let mut iter = state.iter().filter_map(|n| n.ok()).enumerate().peekable();

        while let Some((index, item)) = iter.peek() {
            let is_last = index + 1 == len;
            let is_no_var = item.name().ok()?.inner_string_text().ok()?.text() == "noVar";
            if is_no_var && is_last {
                iter.next();
                continue;
            }
            if is_no_var {
                members.push(item.clone());
                if !is_last {
                    separators.push(token(T![,]))
                }
            }

            iter.next();
        }
        let new_list = json_member_list(members, separators);
        dbg!(&new_list);
        // mutation.replace_node(state.clone(), new_list);
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
