use std::str::FromStr;

use biome_analyze::{
    Ast, FixKind, QueryMatch, Rule, RuleDiagnostic, context::RuleContext, utils::fix_separators,
};
use biome_configuration::analyzer::{
    RuleGroup, RuleName,
    assist::{self, ActionName},
};
use biome_console::markup;
use biome_diagnostics::category;
use biome_json_factory::make;
use biome_json_syntax::{AnyJsonValue, JsonMember, JsonMemberList, JsonObjectValue, T};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, TriviaPieceKind};

use crate::{MigrationAction, declare_migration};

declare_migration! {
    pub(crate) RuleMover {
        version: "1.7.0",
        name: "ruleMover",
        fix_kind: FixKind::Safe,
    }
}

/// Linter rules that have been renamed.
/// The first element of every pair is the old name of the rule.
#[rustfmt::skip]
const RULE_RENAMING: &[(&str, RuleName)] = &[
    ("noConsoleLog", RuleName::NoConsole),
    ("noInvalidNewBuiltin", RuleName::NoInvalidBuiltinInstantiation),
    ("noMultipleSpacesInRegularExpressionLiterals", RuleName::NoAdjacentSpacesInRegex),
    ("noNewSymbol", RuleName::NoInvalidBuiltinInstantiation),
    ("noUnnecessaryContinue", RuleName::NoUselessContinue),
    ("useNamedOperation", RuleName::UseGraphqlNamedOperations),
    ("useShorthandArrayType", RuleName::UseConsistentArrayType),
    ("useSingleCaseStatement", RuleName::NoSwitchDeclarations),
    ("noReactPropAssign", RuleName::NoReactPropAssignments),
    ("noGlobalDirnameFilename", RuleName::NoGlobalDirnameFilename),
    ("noConstantBinaryExpression", RuleName::NoConstantBinaryExpressions),
    ("noDestructuredProps", RuleName::NoSolidDestructuredProps),
    ("noImplicitCoercion", RuleName::NoImplicitCoercions),
    ("noUnknownAtRule", RuleName::NoUnknownAtRules),
    ("useAdjacentGetterSetter", RuleName::UseGroupedAccessorPairs),
    ("useConsistentObjectDefinition", RuleName::UseConsistentObjectDefinitions),
    ("useConsistentResponse", RuleName::UseStaticResponseMethods),
    ("useForComponent", RuleName::UseSolidForComponent),
    ("useJsonImportAttribute", RuleName::UseJsonImportAttributes),
    ("useUnifiedTypeSignature", RuleName::UseUnifiedTypeSignatures),
];

/// Assist actions that have been renamed.
/// The first element of every pair is the old name of the action.
#[rustfmt::skip]
const ACTION_RENAMING: &[(&str, ActionName)] = &[];

impl Rule for RuleMover {
    type Query = Ast<JsonMember>;
    type State = State;
    type Signals = Box<[Self::State]>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let member_name = node.name().and_then(|name| name.inner_string_text());
        let is_linter_rules = member_name
            .as_ref()
            .is_ok_and(|name| name.text() == "rules");
        let is_assist_actions = member_name.is_ok_and(|name| name.text() == "actions");
        if !is_linter_rules && !is_assist_actions {
            return Box::default();
        }
        let Ok(AnyJsonValue::JsonObjectValue(rules)) = node.value() else {
            return Box::default();
        };
        let mut result: Vec<Self::State> = Vec::new();
        for group_elt in rules.json_member_list().into_iter().flatten() {
            let Ok(AnyJsonValue::JsonObjectValue(group_rules)) = group_elt.value() else {
                continue;
            };
            let Ok(group_name) = group_elt.name().and_then(|name| name.inner_string_text()) else {
                continue;
            };
            if is_linter_rules {
                let Ok(current_group) = RuleGroup::from_str(group_name.text()) else {
                    continue;
                };
                for rule_node in group_rules.json_member_list().into_iter().flatten() {
                    let Ok(rule_name) = rule_node.name().and_then(|name| name.inner_string_text())
                    else {
                        continue;
                    };
                    let rule_name = rule_name.text();
                    if let Ok(new_rule) = RuleName::from_str(rule_name) {
                        // TODO: remove the `useNamingConvention` exception,
                        // once we have promoted the GraphQL `useNamingConvention` rule
                        //
                        // See https://github.com/biomejs/biome/issues/6018
                        if new_rule.group() != current_group && rule_name != "useNamingConvention" {
                            result.push(State {
                                rule_node,
                                new_rule_name: AnyRule::Lint(new_rule),
                                old_group_name: Some(AnyGroup::Lint(current_group)),
                                old_rule_name: None,
                            });
                        }
                    } else if let Some((old_rule_name, new_rule)) = RULE_RENAMING
                        .iter()
                        .find(|(old_name, _)| old_name == &rule_name)
                        .copied()
                    {
                        result.push(State {
                            rule_node,
                            new_rule_name: AnyRule::Lint(new_rule),
                            old_group_name: (new_rule.group() != current_group)
                                .then_some(AnyGroup::Lint(current_group)),
                            old_rule_name: Some(old_rule_name),
                        });
                    }
                }
            } else {
                let Ok(current_group) = assist::RuleGroup::from_str(group_name.text()) else {
                    continue;
                };
                for rule_node in group_rules.json_member_list().into_iter().flatten() {
                    let Ok(rule_name) = rule_node.name().and_then(|name| name.inner_string_text())
                    else {
                        continue;
                    };
                    let rule_name = rule_name.text();
                    if let Ok(new_rule) = ActionName::from_str(rule_name) {
                        if new_rule.group() != current_group {
                            result.push(State {
                                rule_node,
                                new_rule_name: AnyRule::Assist(new_rule),
                                old_group_name: Some(AnyGroup::Assist(current_group)),
                                old_rule_name: None,
                            });
                        }
                    } else if let Some((old_rule_name, new_rule)) = ACTION_RENAMING
                        .iter()
                        .find(|(old_name, _)| old_name == &rule_name)
                        .copied()
                    {
                        result.push(State {
                            rule_node,
                            new_rule_name: AnyRule::Assist(new_rule),
                            old_group_name: (new_rule.group() != current_group)
                                .then_some(AnyGroup::Assist(current_group)),
                            old_rule_name: Some(old_rule_name),
                        });
                    }
                }
            }
        }
        result.into()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let rule_kind = if matches!(state.new_rule_name, AnyRule::Lint(_)) {
            "lint rule"
        } else {
            "assist action"
        };
        let new_rule_name = state.new_rule_name.as_str();
        let msg = if let Some(old_group) = state.old_group_name {
            let action = if old_group.as_str() == RuleGroup::Nursery.as_str() {
                "promoted"
            } else if state.new_rule_name.group().as_str() == RuleGroup::Nursery.as_str() {
                "unpromoted"
            } else {
                "moved"
            };
            let new_group_name = state.new_rule_name.group().as_str();
            markup! { "This "{rule_kind}" has been "{action}" to "<Emphasis>{new_group_name}"/"{new_rule_name}</Emphasis>"." }
                .to_owned()
        } else {
            markup! { "This "{rule_kind}" has been renamed to "<Emphasis>{new_rule_name}</Emphasis>"." }
                .to_owned()
        };
        Some(
            RuleDiagnostic::new(
                category!("migrate"),
                state.rule_node.name().ok()?.range(),
                msg,
            )
            .deprecated(),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<MigrationAction> {
        let node = ctx.query();
        let rules = node.value().ok()?;
        let rules = rules.as_json_object_value()?;
        let old_rules_list = rules.json_member_list();
        let mut mutation = ctx.root().begin();

        // If the rule is moved or renamed
        let rule_node = state.rule_node.clone();
        // Rename the rule
        let new_rule_name = state.new_rule_name.as_str();
        let new_rule_node = if let Some(old_rule_name) = state.old_rule_name {
            let new_name = make::json_member_name(make::json_string_literal(new_rule_name));
            let new_rule_node = rule_node.with_name(new_name);
            if let Some(value) = new_rule_node
                .value()
                .ok()
                .and_then(|value| transform_value(value, old_rule_name))
            {
                new_rule_node.with_value(value)
            } else {
                new_rule_node
            }
        } else {
            rule_node
        };

        // Update or create the group where the rule is moved to
        // If the group exists, then `new_group_range` will be the range of the group
        let new_group_name = state.new_rule_name.group().as_str();
        let (new_group_range, new_group_node) = if let Some(new_group) =
            rules.find_member(new_group_name)
        {
            // The group exists, so we update it.
            let AnyJsonValue::JsonObjectValue(group_obj) = new_group.value().ok()? else {
                return None;
            };
            let old_list = group_obj.json_member_list();
            let mut new_elements = Vec::with_capacity(old_list.len() + 1);
            let mut last_has_separator = false;
            let mut is_rule_migrated = false;
            for elt in old_list.elements() {
                let node = elt.node.ok()?;
                if let Ok(name) = node.name().and_then(|name| name.inner_string_text()) {
                    if name.text() == new_rule_name {
                        // This happens when:
                        // - the rule has already been manually migrated, but the old one was not removed
                        // - several rules are renamed and merged into a same rule.
                        is_rule_migrated = true;
                    } else if state.old_rule_name == Some(name.text()) {
                        // Remove the old rule.
                        // This happens when the new and old rule are located in the same group.
                        continue;
                    }
                }
                let trailing_separator = elt.trailing_separator.ok()?;
                last_has_separator = trailing_separator.is_some();
                new_elements.push((node, trailing_separator));
            }
            if !is_rule_migrated {
                let new_rule_node = new_rule_node.with_leading_trivia_pieces(
                    state.rule_node.syntax().first_leading_trivia()?.pieces(),
                )?;
                // Add the new rule
                new_elements.push((new_rule_node, None));
            }
            fix_separators(
                new_elements.iter_mut().map(|(a, b)| (a, b)),
                last_has_separator,
                || make::token(T![,]),
            );
            // Reconstruct the list
            let separators: Vec<_> = new_elements
                .iter_mut()
                .filter_map(|(_, sep)| sep.take())
                .collect();
            let nodes = new_elements.into_iter().map(|(node, _)| node);
            let new_list = make::json_member_list(nodes, separators);
            (
                Some(new_group.range()),
                new_group.with_value(group_obj.with_json_member_list(new_list).into()),
            )
        } else {
            // The group does not exist, so we create it.
            let old_list = rules.json_member_list();
            let mut new_elements = Vec::with_capacity(old_list.len() + 1);
            for elt in old_list.elements() {
                new_elements.push((elt.node.ok()?, elt.trailing_separator.ok()?));
            }
            let last_has_separator = new_elements.last().is_some_and(|(_, sep)| sep.is_some());
            let mut indent = Vec::new();
            if let Some((last_node, _)) = new_elements.last()
                && let Some(first_token) = last_node.syntax().first_token()
            {
                indent.extend(first_token.indentation_trivia_pieces());
            }
            // Add the new group and rule
            let mut indent = Vec::new();
            if let Some(Ok(last_group_node)) = old_rules_list.last()
                && let Some(first_token) = last_group_node.syntax().first_token()
            {
                indent.extend(first_token.indentation_trivia_pieces());
            }
            let new_group_node = make::json_member(
                make::json_member_name(
                    make::json_string_literal(new_group_name).prepend_trivia_pieces(indent.clone()),
                ),
                make::token(T![:]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
                make::json_object_value(
                    make::token(T!['{']).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
                    if last_has_separator {
                        make::json_member_list(
                            [new_rule_node],
                            [make::token(T![,])
                                .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")])],
                        )
                    } else {
                        make::json_member_list([new_rule_node], [])
                    },
                    make::token(T!['}']).with_leading_trivia([(TriviaPieceKind::Whitespace, " ")]),
                )
                .into(),
            );
            (None, new_group_node)
        };

        // Update `rules` by updating or adding `new_group_node`.
        // It is also in charge of removing the old rule if it is not in the same group as the new one.
        let mut new_rules_elements = Vec::with_capacity(old_rules_list.len() + 1);
        let rule_group_node = state.rule_node.syntax().grand_parent()?.parent()?;
        for elt in old_rules_list.elements() {
            let node = elt.node.ok()?;
            let node = if new_group_range.is_some_and(|range| range == node.range()) {
                // Update the group
                new_group_node.clone()
            } else if node.range() == rule_group_node.text_range() {
                // Remove the old rule
                let old_list = state.rule_node.parent::<JsonMemberList>()?;
                let mut new_elements = Vec::with_capacity(old_list.len() - 1);
                let mut last_has_separator = false;
                for elt in old_list.elements() {
                    let node = elt.node.ok()?;
                    let separator = elt.trailing_separator.ok()?;
                    last_has_separator = separator.is_some();
                    if node.range() != state.rule_node.range() {
                        new_elements.push((node, separator));
                    }
                }
                fix_separators(
                    new_elements.iter_mut().map(|(a, b)| (a, b)),
                    last_has_separator,
                    || make::token(T![,]),
                );
                // Reconstruct the list
                let separators: Vec<_> = new_elements
                    .iter_mut()
                    .filter_map(|(_, sep)| sep.take())
                    .collect();
                let nodes = new_elements.into_iter().map(|(node, _)| node);
                let new_list = make::json_member_list(nodes, separators);
                let obj = old_list.parent::<JsonObjectValue>()?;
                let obj = obj.with_json_member_list(new_list);
                node.with_value(obj.into())
            } else {
                node
            };
            new_rules_elements.push((node, elt.trailing_separator.ok()?));
        }
        let last_has_separator = new_rules_elements
            .last()
            .is_some_and(|(_, sep)| sep.is_some());
        // If `new_group_node` does not exist then we add it.
        if new_group_range.is_none() {
            new_rules_elements.push((new_group_node, None));
        }
        fix_separators(
            new_rules_elements.iter_mut().map(|(a, b)| (a, b)),
            last_has_separator,
            || make::token(T![,]),
        );
        // Reconstruct the `rules` list
        let new_rules_separators: Vec<_> = new_rules_elements
            .iter_mut()
            .filter_map(|(_, sep)| sep.take())
            .collect();
        let new_rules_nodes = new_rules_elements.into_iter().map(|(node, _)| node);
        let new_rules_list = make::json_member_list(new_rules_nodes, new_rules_separators);

        let rule_kind = if matches!(state.new_rule_name, AnyRule::Lint(_)) {
            "lint rule"
        } else {
            "assist action"
        };
        let action_to_perform = if state.old_group_name.is_none() {
            "Rename"
        } else if state.old_rule_name.is_none() {
            "Move"
        } else {
            "Move and rename"
        };
        mutation.replace_node(old_rules_list, new_rules_list);
        Some(MigrationAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
                {action_to_perform}" the "{rule_kind}"."
            },
            mutation,
        ))
    }
}

pub(crate) struct State {
    rule_node: JsonMember,
    /// Set if the rule is moved to a new group.
    old_group_name: Option<AnyGroup>,
    /// Set if the rule is renamed.
    old_rule_name: Option<&'static str>,
    new_rule_name: AnyRule,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum AnyGroup {
    Lint(RuleGroup),
    Assist(assist::RuleGroup),
}
impl AnyGroup {
    fn as_str(self) -> &'static str {
        match self {
            Self::Lint(group) => group.as_str(),
            Self::Assist(group) => group.as_str(),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum AnyRule {
    Lint(RuleName),
    Assist(ActionName),
}
impl AnyRule {
    fn as_str(self) -> &'static str {
        match self {
            Self::Lint(rule) => rule.as_str(),
            Self::Assist(rule) => rule.as_str(),
        }
    }

    fn group(self) -> AnyGroup {
        match self {
            Self::Lint(rule) => AnyGroup::Lint(rule.group()),
            Self::Assist(rule) => AnyGroup::Assist(rule.group()),
        }
    }
}

fn transform_value(value: AnyJsonValue, old_rule_name: &'static str) -> Option<AnyJsonValue> {
    let options = match old_rule_name {
        "noConsoleLog" => create_console_log_options(),
        "useShorthandArrayType" => create_shorthand_array_type_options(),
        _ => {
            return Some(value);
        }
    };
    let colon = make::token(T![:]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]);
    Some(
        make::json_object_value(
            make::token(T!['{']).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
            make::json_member_list(
                [
                    make::json_member(
                        make::json_member_name(make::json_string_literal("level")),
                        colon.clone(),
                        get_rule_level(value),
                    ),
                    make::json_member(
                        make::json_member_name(make::json_string_literal("options")),
                        colon,
                        options,
                    ),
                ],
                [make::token(T![,]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")])],
            ),
            make::token(T!['}']),
        )
        .into(),
    )
}

fn get_rule_level(value: AnyJsonValue) -> AnyJsonValue {
    if let AnyJsonValue::JsonObjectValue(obj) = &value {
        for item in obj.json_member_list().into_iter().flatten() {
            let text = item.name().and_then(|n| n.inner_string_text());
            if text.is_ok_and(|name| name.text() == "level") {
                return item.value().unwrap_or(value);
            }
        }
    }
    value
}

/// Creates the member for `noConsoleLog`
fn create_console_log_options() -> AnyJsonValue {
    let allow_option = make::json_member(
        make::json_member_name(make::json_string_literal("allow")),
        make::token(T![:]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
        make::json_array_value(
            make::token(T!['[']),
            make::json_array_element_list(
                vec![AnyJsonValue::JsonStringValue(make::json_string_value(
                    make::json_string_literal("log"),
                ))],
                [],
            ),
            make::token(T![']']).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
        )
        .into(),
    );
    make::json_object_value(
        make::token(T!['{']).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
        make::json_member_list([allow_option], []),
        make::token(T!['}']).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
    )
    .into()
}

fn create_shorthand_array_type_options() -> AnyJsonValue {
    let syntax_option = make::json_member(
        make::json_member_name(make::json_string_literal("syntax")),
        make::token(T![:]).with_trailing_trivia(vec![(TriviaPieceKind::Whitespace, " ")]),
        make::json_string_value(
            make::json_string_literal("shorthand")
                .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
        )
        .into(),
    );
    make::json_object_value(
        make::token(T!['{']).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
        make::json_member_list([syntax_option], []),
        make::token(T!['}']).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
    )
    .into()
}
