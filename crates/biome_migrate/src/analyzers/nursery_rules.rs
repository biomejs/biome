use crate::{declare_migration, MigrationAction};
use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, FixKind, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::category;
use biome_json_factory::make::{
    json_member, json_member_list, json_member_name, json_object_value, json_string_literal, token,
};
use biome_json_syntax::{AnyJsonValue, JsonMember, JsonMemberList, JsonRoot, JsonSyntaxToken, T};
use biome_rowan::{
    AstNode, AstNodeExt, AstSeparatedList, BatchMutationExt, TextRange, TriviaPieceKind, WalkEvent,
};
use rustc_hash::FxHashMap;
use std::iter::repeat;

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
    nursery_rule: JsonMember,
    /// The member of the group where the new rule should be moved
    nursery_group: JsonMember,
    /// The comma separator to be deleted
    optional_separator: Option<JsonSyntaxToken>,
    /// The name of the target rule
    target_rule_name: &'static str,
    /// The new group name
    target_group_name: &'static str,
}

impl MigrateRuleState {
    fn as_rule_name_range(&self) -> TextRange {
        self.nursery_rule.range()
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

/// - Left: name of the rule in the nursery group
/// - Right: name of the target group and name of the target rule (sometimes we change name)
#[rustfmt::skip]
const RULES_TO_MIGRATE: &[(&str, (&str, &str))] = &[
    // CSS
    ("noDuplicateAtImportRules", ("suspicious", "noDuplicateAtImportRules")),
    ("noDuplicateFontNames", ("suspicious", "noDuplicateFontNames")),
    ("noDuplicateSelectorsKeyframeBlock", ("suspicious", "noDuplicateSelectorsKeyframeBlock")),
    ("noEmptyBlock", ("suspicious", "noEmptyBlock")),
    ("noImportantInKeyframe", ("suspicious", "noImportantInKeyframe")),
    ("noInvalidDirectionInLinearGradient", ("correctness", "noInvalidDirectionInLinearGradient")),
    ("noInvalidPositionAtImportRule", ("correctness", "noInvalidPositionAtImportRule")),
    ("noShorthandPropertyOverrides", ("suspicious", "noShorthandPropertyOverrides")),
    ("noUnknownFunction", ("correctness", "noUnknownFunction")),
    ("noUnknownMediaFeatureName", ("correctness", "noUnknownMediaFeatureName")),
    ("noUnknownProperty", ("correctness", "noUnknownProperty")),
    ("noUnknownUnit", ("correctness", "noUnknownUnit")),
    ("noUnmatchableAnbSelector", ("correctness", "noUnmatchableAnbSelector")),
    ("useConsistentGridAreas", ("correctness", "noInvalidGridAreas")),
    ("useGenericFontNames", ("a11y", "useGenericFontNames")),
    // JS
    ("noBarrelFile", ("performance", "noBarrelFile")),
    ("noConsole", ("suspicious", "noConsole")),
    ("noConstantMathMinMaxClamp", ("correctness", "noConstantMathMinMaxClamp")),
    ("noDoneCallback", ("style", "noDoneCallback")),
    ("noDuplicateTestHooks", ("suspicious", "noDuplicateTestHooks")),
    ("noEvolvingTypes", ("suspicious", "noEvolvingTypes")),
    ("noExcessiveNestedTestSuites", ("complexity", "noExcessiveNestedTestSuites")),
    ("noExportsInTest", ("suspicious", "noExportsInTest")),
    ("noFlatMapIdentity", ("correctness", "noFlatMapIdentity")),
    ("noFocusedTests", ("suspicious", "noFocusedTests")),
    ("noLabelWithoutControl", ("a11y", "noLabelWithoutControl")),
    ("noMisplacedAssertion", ("suspicious", "noMisplacedAssertion")),
    ("noNamespaceImport", ("style", "noNamespaceImport")),
    ("noNodejsModules", ("correctness", "noNodejsModules")),
    ("noReactSpecificProps", ("suspicious", "noReactSpecificProps")),
    ("noReExportAll", ("performance", "noReExportAll")),
    ("noSkippedTests", ("suspicious", "noSkippedTests")),
    ("noSuspiciousSemicolonInJsx", ("suspicious", "noSuspiciousSemicolonInJsx")),
    ("noUndeclaredDependencies", ("correctness", "noUndeclaredDependencies")),
    ("noUnusedFunctionParameters", ("correctness", "noUnusedFunctionParameters")),
    ("noUselessStringConcat", ("complexity", "noUselessStringConcat")),
    ("noUselessTernary", ("complexity", "noUselessTernary")),
    ("noUselessUndefinedInitialization", ("complexity", "noUselessUndefinedInitialization")),
    ("noYodaExpression", ("style", "noYodaExpression")),
    ("useArrayLiterals", ("correctness", "useArrayLiterals")),
    ("useConsistentBuiltinInstantiation", ("style", "useConsistentBuiltinInstantiation")),
    ("useDateNow", ("complexity", "useDateNow")),
    ("useDefaultSwitchClause", ("style", "useDefaultSwitchClause")),
    ("useErrorMessage", ("suspicious", "useErrorMessage")),
    ("useExplicitLengthCheck", ("style", "useExplicitLengthCheck")),
    ("useFocusableInteractive", ("a11y", "useFocusableInteractive")),
    ("useImportExtensions", ("correctness", "useImportExtensions")),
    ("useJsxKeyInIterable", ("correctness", "useJsxKeyInIterable") ),
    ("useNodeAssertStrict", ("style", "useNodeAssertStrict")),
    ("useNumberToFixedDigitsArgument", ("suspicious", "useNumberToFixedDigitsArgument")),
    ("useSemanticElements", ("a11y", "useSemanticElements")),
    ("useThrowNewError", ("style", "useThrowNewError")),
    ("useThrowOnlyError", ("style", "useThrowOnlyError")),
    ("useTopLevelRegex", ("performance", "useTopLevelRegex")),
];

impl Rule for NurseryRules {
    type Query = Ast<JsonRoot>;
    type State = MigrateRuleState;
    type Signals = Box<[Self::State]>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let mut rules_to_migrate = Vec::new();

        if let Some(nursery_group) = find_group_by_name(node, "nursery") {
            let mut rules_should_be_migrated = FxHashMap::default();
            for (nursery_rule_name, (target_group_name, target_rule_name)) in RULES_TO_MIGRATE {
                rules_should_be_migrated
                    .insert(*nursery_rule_name, (*target_group_name, *target_rule_name));
            }
            let Some(nursery_group_object) = nursery_group
                .value()
                .ok()
                .and_then(|node| node.as_json_object_value().cloned())
            else {
                return rules_to_migrate.into_boxed_slice();
            };

            let mut separator_iterator = nursery_group_object
                .json_member_list()
                .separators()
                .flatten()
                .enumerate()
                // Repeat the first separator,
                // so when the rule to be deleted is the first rule,
                // its trailing comma is also deleted:
                // {
                //    "ruleA": "error",
                //    "ruleB": "error",
                //    "ruleC": "error"
                // }
                .flat_map(|(i, s)| repeat(s).take(if i == 0 { 2 } else { 1 }));

            for nursery_rule in nursery_group_object.json_member_list().iter().flatten() {
                let optional_separator = separator_iterator.next();

                let Ok(nursery_rule_name) = nursery_rule
                    .name()
                    .and_then(|node| node.inner_string_text())
                else {
                    continue;
                };

                if let Some((target_group_name, target_rule_name)) =
                    rules_should_be_migrated.get(nursery_rule_name.text())
                {
                    rules_to_migrate.push(MigrateRuleState {
                        nursery_rule: nursery_rule.clone(),
                        nursery_group: nursery_group.clone(),
                        optional_separator,
                        target_rule_name,
                        target_group_name,
                    })
                }
            }
        }

        rules_to_migrate.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                category!("migrate"),
                state.as_rule_name_range(),
                markup! {
                    "This rule has been promoted to "<Emphasis>{state.target_group_name}"/"{state.target_rule_name}</Emphasis>"."
                }
                .to_owned(),
            )
            .deprecated(),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<MigrationAction> {
        let node = ctx.query();
        let MigrateRuleState {
            target_group_name,
            target_rule_name,
            optional_separator,
            nursery_group,
            nursery_rule,
        } = state;
        let mut mutation = ctx.root().begin();
        let mut rule_already_exists = false;

        // If the target group exists, then we just need to delete the rule from the nursery group,
        // and update the target group by adding a new member with the name of rule we are migrating
        if let Some(target_group) = find_group_by_name(node, target_group_name) {
            let target_group_value = target_group.value().ok()?;
            let target_group_value_object = target_group_value.as_json_object_value()?;

            let current_rules = target_group_value_object.json_member_list();
            let mut current_rule_separators = target_group_value_object
                .json_member_list()
                .separators()
                .flatten();
            let current_rules_count = current_rules.len();

            let mut separators = Vec::with_capacity(current_rules_count + 1);
            let mut new_rules = Vec::with_capacity(current_rules_count + 1);

            for current_rule in current_rules.iter() {
                let current_rule = current_rule.ok()?;
                if current_rule
                    .name()
                    .and_then(|node| node.inner_string_text())
                    .is_ok_and(|text| text.text() == *target_rule_name)
                {
                    rule_already_exists = true;
                    break;
                }
                new_rules.push(current_rule.clone());
                if let Some(current_rule_separator) = current_rule_separators.next() {
                    separators.push(current_rule_separator);
                } else {
                    separators.push(token(T![,]));
                }
            }

            // We only add the rule if the rule doesn't already exist in the target group
            // to avoid duplicate rules in the target group
            if !rule_already_exists {
                let new_rule_member =
                    make_new_rule_name_member(target_rule_name, &nursery_rule.clone().detach())?;
                new_rules.push(new_rule_member);
                mutation.replace_node(current_rules, json_member_list(new_rules, separators));
            }

            // Remove the stale nursery rule and the corresponding comma separator
            mutation.remove_node(nursery_rule.clone());
            if let Some(separator) = optional_separator {
                mutation.remove_token(separator.clone());
            }
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

                    if &node != nursery_group {
                        return Some(node);
                    }

                    let object = node.value().ok()?;
                    let object = object.as_json_object_value()?;
                    let mut separators: Vec<_> =
                        object.json_member_list().separators().flatten().collect();
                    let new_nursery_group: Vec<_> = object
                        .json_member_list()
                        .iter()
                        .enumerate()
                        .filter_map(|(i, node)| {
                            let node = node.ok()?;
                            if &node == nursery_rule {
                                if i < separators.len() {
                                    separators.remove(i);
                                } else {
                                    separators.pop();
                                }
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
                            json_member_list(new_nursery_group, separators),
                            token(T!['}']),
                        )),
                    );

                    Some(new_member)
                })
                .collect();

            let new_member = json_member(
                json_member_name(
                    json_string_literal(target_group_name)
                        .with_leading_trivia([(TriviaPieceKind::Whitespace, "\n")]),
                ),
                token(T![:]),
                AnyJsonValue::JsonObjectValue(json_object_value(
                    token(T!['{']),
                    json_member_list(
                        vec![make_new_rule_name_member(target_rule_name, nursery_rule)?],
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
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            if rule_already_exists {
                markup! {
                    "Remove the stale rule from the nursery group."
                }
            } else {
                markup! {
                    "Move the rule to the new stable group."
                }
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
