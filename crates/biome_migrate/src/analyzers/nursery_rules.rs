use crate::{declare_migration, MigrationAction};
use biome_analyze::context::RuleContext;
use biome_analyze::{ActionCategory, Ast, FixKind, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::category;
use biome_json_factory::make::{
    json_member, json_member_list, json_member_name, json_object_value, json_string_literal, token,
};
use biome_json_syntax::{AnyJsonValue, JsonMember, JsonMemberList, JsonRoot, T};
use biome_rowan::{
    AstNode, AstNodeExt, AstSeparatedList, BatchMutationExt, TextRange, TriviaPieceKind, WalkEvent,
};
use rustc_hash::FxHashMap;

declare_migration! {
    pub(crate) NurseryRules {
        version: "1.7.0",
        name: "nurseryRules",
        fix_kind: FixKind::Unsafe,
    }
}

#[derive(Debug)]
pub(crate) struct MigrateRuleState {
    /// The member of the new rule
    nursery_rule_member: JsonMember,
    /// The member of the group where the new rule should be moved
    nursery_group: JsonMember,
    /// The name of the new rule
    new_rule_name: &'static str,
    /// The new group name
    new_group_name: &'static str,
}

impl MigrateRuleState {
    fn as_rule_name_range(&self) -> TextRange {
        self.nursery_rule_member.range()
    }
}

/// It attempts the find the group by name:
///
/// ```json5
/// {
///     "groupName": {}
/// //  ^^^^^^^^^^^
/// }
/// ```
fn find_group_by_name(root: &JsonRoot, group_name: &str) -> Option<JsonMember> {
    let preorder = root.syntax().preorder();
    let mut group = None;
    for event in preorder {
        if let WalkEvent::Enter(node) = event {
            let Some(member) = JsonMember::cast(node) else {
                continue;
            };
            let Ok(text) = member.name().and_then(|n| n.inner_string_text()) else {
                continue;
            };
            if text.text() == group_name {
                group = Some(member);
                break;
            }
        }
    }

    group
}

// used for testing purposes
/// - Left: name of the rule in the nursery group
/// - Right: name of the new group and name of the new rule (sometimes we change name)
#[cfg(debug_assertions)]
const RULES_TO_MIGRATE: &[(&str, (&str, &str))] = &[
    (
        "noExcessiveNestedTestSuites",
        ("complexity", "noExcessiveNestedTestSuites"),
    ),
    ("noUselessTernary", ("complexity", "noUselessTernary")),
    (
        "useJsxKeyInIterable",
        ("correctness", "useJsxKeyInIterable"),
    ),
    ("oldName", ("suspicious", "noSuspiciousSemicolonInJsx")),
];

#[cfg(not(debug_assertions))]
// Used in production
const RULES_TO_MIGRATE: &[(&str, (&str, &str))] = &[
    (
        "noExcessiveNestedTestSuites",
        ("complexity", "noExcessiveNestedTestSuites"),
    ),
    ("noUselessTernary", ("complexity", "noUselessTernary")),
    (
        "useJsxKeyInIterable",
        ("correctness", "useJsxKeyInIterable"),
    ),
    ("noBarrelFile", ("performance", "noBarrelFile")),
    ("noReExportAll", ("performance", "noReExportAll")),
    ("noNamespaceImport", ("style", "noNamespaceImport")),
    ("useNodeAssertStrict", ("style", "useNodeAssertStrict")),
    (
        "noDuplicateTestHooks",
        ("suspicious", "noDuplicateTestHooks"),
    ),
    ("noExportsInTest", ("suspicious", "noExportsInTest")),
    ("noFocusedTests", ("suspicious", "noFocusedTests")),
    ("noSkippedTests", ("suspicious", "noSkippedTests")),
    (
        "noSuspiciousSemicolonInJsx",
        ("suspicious", "noSuspiciousSemicolonInJsx"),
    ),
    ("useImportRestrictions", ("style", "useImportRestrictions")),
    (
        "noConstantMathMinMaxClamp",
        ("correctness", "noConstantMathMinMaxClamp"),
    ),
    ("noFlatMapIdentity", ("correctness", "noFlatMapIdentity")),
    ("noNodejsModules", ("correctness", "noNodejsModules")),
    ("useArrayLiterals", ("correctness", "useArrayLiterals")),
];

impl Rule for NurseryRules {
    type Query = Ast<JsonRoot>;
    type State = MigrateRuleState;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let mut rules_to_migrate = vec![];

        let nursery_group = find_group_by_name(node, "nursery");

        if let Some(nursery_member) = nursery_group {
            let mut rules = FxHashMap::default();
            for (group, (new_group, new_name)) in RULES_TO_MIGRATE {
                rules.insert(*group, (*new_group, *new_name));
            }
            let object_value = nursery_member
                .value()
                .ok()
                .and_then(|node| node.as_json_object_value().cloned());

            let Some(object_value) = object_value else {
                return rules_to_migrate;
            };

            for group_member in object_value.json_member_list().iter().flatten() {
                let Ok(member_name_text) = group_member
                    .name()
                    .and_then(|node| node.inner_string_text())
                else {
                    continue;
                };
                let new_rule = rules.get(member_name_text.text()).copied();
                if let Some((new_group, new_rule)) = new_rule {
                    rules_to_migrate.push(MigrateRuleState {
                        nursery_rule_member: group_member.clone(),
                        nursery_group: nursery_member.clone(),
                        new_rule_name: new_rule,
                        new_group_name: new_group,
                    })
                }
            }
        }

        rules_to_migrate
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                category!("migrate"),
                state.as_rule_name_range(),
                markup! {
                    "This rule has been promoted to "<Emphasis>{state.new_group_name}"/"{state.new_rule_name}</Emphasis>"."
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
            nursery_rule_member: nursery_rule,
        } = state;
        let mut mutation = ctx.root().begin();

        let new_group = find_group_by_name(node, new_group_name);

        // If the group exists, then we just need to update that group by adding a new member
        // with the name of rule we are migrating
        if let Some(group) = new_group {
            let value = group.value().ok()?;
            let value = value.as_json_object_value()?;

            let mut separators = vec![];
            let mut new_list = vec![];

            let old_list_node = value.json_member_list();
            let new_rule_member =
                make_new_rule_name_member(new_rule_name, &nursery_rule.clone().detach())?;

            for member in old_list_node.iter() {
                let member = member.ok()?;
                new_list.push(member.clone());
                separators.push(token(T![,]));
            }
            new_list.push(new_rule_member);
            mutation.replace_node(old_list_node, json_member_list(new_list, separators));
            mutation.remove_node(nursery_rule.clone());
        }
        // If we don't have a group, we have to create one. To avoid possible side effects with our mutation logic
        // we recreate the "rules" object by removing the `rules.nursery.<nursery_rule_name>` member (hence we create a new list),
        // and add a new member `rules.<new_group_name>.<new_rule_name>`. This new group is added at the very end.
        else {
            let rules = nursery_group
                .syntax()
                .ancestors()
                .find_map(JsonMemberList::cast)?;
            let mut new_members: Vec<_> = rules
                .iter()
                .filter_map(|node| {
                    let node = node.ok()?;

                    if &node == nursery_group {
                        let object = node.value().ok()?;
                        let object = object.as_json_object_value()?;
                        let new_nursery_group: Vec<_> = object
                            .json_member_list()
                            .iter()
                            .filter_map(|node| {
                                let node = node.ok()?;
                                if &node == nursery_rule {
                                    None
                                } else {
                                    Some(node)
                                }
                            })
                            .collect();

                        let new_member = json_member(
                            node.name().ok()?.clone(),
                            token(T![:]),
                            AnyJsonValue::JsonObjectValue(json_object_value(
                                token(T!['{']),
                                json_member_list(new_nursery_group, vec![]),
                                token(T!['}']),
                            )),
                        );

                        return Some(new_member);
                    }

                    Some(node)
                })
                .collect();
            let new_member = json_member(
                json_member_name(
                    json_string_literal(new_group_name)
                        .with_leading_trivia([(TriviaPieceKind::Whitespace, "\n")]),
                ),
                token(T![:]),
                AnyJsonValue::JsonObjectValue(json_object_value(
                    token(T!['{']),
                    json_member_list(
                        vec![make_new_rule_name_member(new_rule_name, nursery_rule)?],
                        vec![],
                    ),
                    token(T!['}']).with_leading_trivia_pieces(
                        nursery_group
                            .syntax()
                            .last_token()?
                            .leading_trivia()
                            .pieces(),
                    ),
                )),
            )
            .with_leading_trivia_pieces(
                nursery_group
                    .syntax()
                    .first_token()?
                    .leading_trivia()
                    .pieces(),
            )?
            .with_trailing_trivia_pieces(
                nursery_group
                    .syntax()
                    .last_token()?
                    .trailing_trivia()
                    .pieces(),
            )?;
            new_members.push(new_member);
            let mut separators = vec![];
            for (index, _) in new_members.iter().enumerate() {
                if index < new_members.len() - 1 {
                    separators.push(token(T![,]))
                }
            }

            mutation.replace_node(rules, json_member_list(new_members, separators));
        };

        Some(MigrationAction::new(
            ActionCategory::QuickFix,
            ctx.metadata().applicability(),
            markup! {
                "Move the rule to the new stable group."
            }
            .to_owned(),
            mutation,
        ))
    }
}

/// It creates a new [JsonMember] by using the old nursery group node, but with the new name
fn make_new_rule_name_member(
    rule_name: &str,
    nursery_rule_member: &JsonMember,
) -> Option<JsonMember> {
    Some(json_member(
        json_member_name(json_string_literal(rule_name))
            .with_leading_trivia_pieces(
                nursery_rule_member
                    .syntax()
                    .first_token()?
                    .leading_trivia()
                    .pieces(),
            )?
            .with_trailing_trivia_pieces(
                nursery_rule_member
                    .syntax()
                    .last_token()?
                    .trailing_trivia()
                    .pieces(),
            )?,
        nursery_rule_member.colon_token().ok()?,
        nursery_rule_member.value().ok()?,
    ))
}
