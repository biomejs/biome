use crate::configuration::{
    create_formatted_object_value, create_group_member, create_rules_member,
    replace_value_to_member, seek_rules_configuration,
};
use crate::{declare_migration, MigrationAction};
use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, Rule, RuleAction, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::{category, Applicability};
use biome_json_syntax::JsonMember;
use biome_rowan::{AstNode, BatchMutationExt, TextRange};

declare_migration! {
    pub(crate) UseWhile {
        version: "2.0.0",
        name: "useWhile",
    }
}

impl Rule for UseWhile {
    type Query = Ast<JsonMember>;
    type State = (TextRange, JsonMember);
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let name = node.name().ok()?;
        let text = name.inner_string_text().ok()?;

        if text.text() == "style" {
            if let Some(object) = node.value().ok()?.as_json_object_value() {
                for item in object.json_member_list().into_iter().flatten() {
                    let name = item.name().ok()?;
                    let text = name.inner_string_text().ok()?;
                    if text.text() == "useWhile" {
                        return Some((name.range(), item));
                    }
                }
            }
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, (range, _): &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            category!("migrate"),
            range,
            markup! {
                "The rule "<Emphasis>"useWhile"</Emphasis>" has ben moved to the "<Emphasis>"complexity"</Emphasis>" group."
            }
                .to_owned(),
        ))
    }

    fn action(
        ctx: &RuleContext<Self>,
        (_, use_while_member): &Self::State,
    ) -> Option<MigrationAction> {
        let style_member = ctx.query();
        let root = ctx.root();
        // we create a new style member without the useWhile rule
        let style_list = style_member.map_members()?;
        let style_list: Vec<_> = style_list
            .into_iter()
            .filter_map(|member| {
                let name = member.name().ok()?.inner_string_text().ok()?;
                if name.text() == "useWhile" {
                    None
                } else {
                    Some(member)
                }
            })
            .collect();

        let linter_member = style_member.find_member_by_name_upwards("linter")?;
        let style_member =
            replace_value_to_member(style_member, create_formatted_object_value(style_list, 6))?;

        let rules_member = seek_rules_configuration(&linter_member)?;

        let rules_list = rules_member.map_members()?;
        let mut new_list = vec![];
        let mut complexity_member_added = false;
        for item in rules_list {
            let name = item.name().ok()?.inner_string_text().ok()?;
            if name == "style" {
                new_list.push(style_member.clone());
            } else if name == "complexity" {
                let mut members = item.map_members()?;
                members.push(use_while_member.clone());
                new_list.push(create_group_member("complexity", members));
                complexity_member_added = true;
            } else {
                new_list.push(item);
            }
        }

        if !complexity_member_added {
            new_list.push(create_group_member(
                "complexity",
                vec![use_while_member.clone()],
            ));
        }

        let mut mutation = root.begin();
        mutation.replace_node(rules_member.clone(), create_rules_member(new_list));

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
