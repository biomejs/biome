use crate::{declare_migration, MigrationAction};
use biome_analyze::context::RuleContext;
use biome_analyze::{ActionCategory, Ast, Rule, RuleAction, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::{category, Applicability};
use biome_json_factory::make::{
    ident, json_member, json_member_list, json_member_name, json_object_value, json_string_literal,
    token,
};
use biome_json_syntax::{
    AnyJsonValue, JsonMember, JsonMemberList, JsonMemberName, JsonObjectValue, JsonRoot, T,
};
use biome_rowan::{
    AstNode, AstSeparatedList, BatchMutationExt, TextRange, TriviaPiece, TriviaPieceKind, WalkEvent,
};
use std::collections::HashMap;

declare_migration! {
    pub(crate) NurseryRules {
        version: "1.7.0",
        name: "nurseryRules",
    }
}

#[derive(Debug)]
pub(crate) struct MigrateRuleState {
    /// The member of the new rule
    nursery_rule: JsonMember,
    /// The member of the group where the new rule should be moved
    nursery_group: JsonMember,
    /// The name of the new rule
    new_rule_name: &'static str,

    new_group_name: &'static str,
}

impl MigrateRuleState {
    fn as_rule_name_range(&self) -> TextRange {
        self.nursery_rule.range()
    }
}

fn find_rule_by_group(group: &JsonMember, rule: &str) -> Option<JsonMember> {
    let object_value = group
        .value()
        .ok()
        .and_then(|node| node.as_json_object_value().cloned())?;

    let mut rule_member = None;
    for member in object_value.json_member_list() {
        let member = member.ok()?;
        let member_name = member.name().ok()?;
        let member_name_text = member_name.inner_string_text().ok()?;
        if member_name_text.text() == rule {
            rule_member = Some(member);
            break;
        }
    }

    rule_member
}

fn find_member_by_name(root: &JsonRoot, group_name: &str) -> Option<JsonMember> {
    let preorder = root.syntax().preorder();
    let mut group = None;
    for event in preorder {
        match event {
            WalkEvent::Enter(node) => {
                if let Some(member) = JsonMember::cast(node) {
                    let member_name = member.name();
                    if let Ok(member_name) = member_name {
                        if let Ok(text) = member_name.inner_string_text() {
                            if text.text() == group_name {
                                group = Some(member);
                                break;
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    group
}

const RULES_TO_MIGRATE: [(&'static str, (&'static str, &'static str)); 1] =
    [("test", ("security", "test"))];

impl Rule for NurseryRules {
    type Query = Ast<JsonRoot>;
    type State = MigrateRuleState;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let mut rules_to_migrate = vec![];

        let nursery_group = find_member_by_name(node, "nursery");

        if let Some(nursery_member) = nursery_group {
            let rules = HashMap::from(RULES_TO_MIGRATE);
            let object_value = nursery_member
                .value()
                .ok()
                .and_then(|node| node.as_json_object_value().cloned());

            if let Some(object_value) = object_value {
                for member in object_value.json_member_list() {
                    if let Ok(group_member) = member {
                        if let Ok(member_name) = group_member.name() {
                            let member_name_text = member_name.inner_string_text();
                            if let Ok(text) = member_name_text {
                                let new_rule = rules.get(text.text()).copied();
                                if let Some((new_group, new_rule)) = new_rule {
                                    rules_to_migrate.push(MigrateRuleState {
                                        new_rule_name: new_rule,
                                        nursery_rule: group_member.clone(),
                                        nursery_group: nursery_member.clone(),
                                        new_group_name: new_group,
                                    })
                                }
                            }
                        }
                    }
                }
            }
        }

        rules_to_migrate
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                category!("migrate"),
                state.as_rule_name_range(),
                markup! {
                    "This rule is has been promoted to "<Emphasis>{state.new_group_name}"/"{state.new_rule_name}</Emphasis>"."
                }
                .to_owned(),
            )
            .deprecated(),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<MigrationAction> {
        let node = ctx.query();
        let MigrateRuleState {
            new_group_name,
            new_rule_name,
            nursery_group,
            nursery_rule,
        } = state;
        let mut mutation = ctx.root().begin();

        let new_group = find_member_by_name(node, new_group_name);
        let rules = nursery_group
            .syntax()
            .ancestors()
            .find_map(JsonMemberList::cast)?;

        if let Some(group) = new_group {
        } else {
            let member = json_member(
                json_member_name(
                    json_string_literal(new_group_name)
                        .with_leading_trivia([(TriviaPieceKind::Whitespace, "\n")]),
                ),
                token(T![:]),
                AnyJsonValue::JsonObjectValue(json_object_value(
                    token(T!['{']),
                    json_member_list(vec![nursery_rule.clone()], vec![]),
                    token(T!['}']),
                )),
            );

            let mut new_members: Vec<_> = rules.iter().filter_map(|node| node.ok()).collect();

            new_members.push(member);
            let mut separators = vec![];
            for (index, _) in new_members.iter().enumerate() {
                if index < new_members.len() - 1 {
                    separators.push(token(T![,]))
                }
            }

            dbg!(&rules);
            dbg!(&new_members);

            mutation.replace_node(rules, json_member_list(new_members, separators));
        };

        Some(MigrationAction {
            mutation,
            message: markup! {
                "Move the rule to the new stable group."
            }
            .to_owned(),
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
        })
        //
        // let new_node = json_member_name(ident("\"indentWidth\""));
        // mutation.replace_node(node.clone(), new_node);
        //
        // Some(RuleAction {
        //     category: ActionCategory::QuickFix,
        //     applicability: Applicability::Always,
        //     message: markup! {
        //         "Use the property "<Emphasis>"indentWidth"</Emphasis>" instead."
        //     }
        //     .to_owned(),
        //     mutation,
        // })
    }
}
