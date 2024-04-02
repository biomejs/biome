use crate::{declare_migration, MigrationAction};
use biome_analyze::context::RuleContext;
use biome_analyze::{ActionCategory, Ast, Rule, RuleAction, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::{category, Applicability};
use biome_json_factory::make::{ident, json_member_name};
use biome_json_syntax::{JsonMember, JsonMemberList, JsonMemberName, JsonObjectValue, JsonRoot};
use biome_rowan::{AstNode, BatchMutationExt, TextRange, WalkEvent};
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
    nursery_rule_range: TextRange,
    /// The member of the group where the new rule should be moved
    nursery_group: JsonMember,
    /// The name of the new rule
    rule_name: &'static str,

    group_name: &'static str,
}

impl MigrateRuleState {
    fn as_rule_name_range(&self) -> TextRange {
        self.nursery_rule_range
    }
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

        let mut nursery_group = None;
        let preorder = node.syntax().preorder();
        for event in preorder {
            match event {
                WalkEvent::Enter(node) => {
                    if let Some(member) = JsonMember::cast(node) {
                        let member_name = member.name();
                        if let Ok(member_name) = member_name {
                            if let Ok(text) = member_name.inner_string_text() {
                                if text.text() == "nursery" {
                                    nursery_group = Some(member);
                                    break;
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }

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
                                        rule_name: new_rule,
                                        nursery_rule_range: group_member.clone().range(),
                                        nursery_group: nursery_member.clone(),
                                        group_name: new_group,
                                    })
                                }
                            }
                        }
                    }
                }
            }
        }

        dbg!(&rules_to_migrate);

        rules_to_migrate
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                category!("migrate"),
                state.as_rule_name_range(),
                markup! {
                    "This rule is has been promoted to "<Emphasis>{state.group_name}"/"{state.rule_name}</Emphasis>"."
                }
                .to_owned(),
            )
            .deprecated(),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<MigrationAction> {
        let MigrateRuleState {
            nursery_group: group_member,
            rule_name,
            ..
        } = state;
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        None
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
