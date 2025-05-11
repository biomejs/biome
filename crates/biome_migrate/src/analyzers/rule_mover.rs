use std::str::FromStr;

use biome_analyze::{Ast, FixKind, QueryMatch, Rule, RuleDiagnostic, context::RuleContext};
use biome_configuration::analyzer::{RuleGroup, RuleName};
use biome_console::markup;
use biome_diagnostics::category;
use biome_json_factory::make;
use biome_json_syntax::{AnyJsonValue, JsonMember, JsonMemberList, JsonObjectValue, T};
use biome_rowan::{
    AstNode, AstSeparatedList, BatchMutationExt, Language, SyntaxToken, TriviaPieceKind,
    chain_trivia_pieces,
};

use crate::{MigrationAction, declare_migration};

declare_migration! {
    pub(crate) RuleMover {
        version: "1.7.0",
        name: "ruleMover",
        fix_kind: FixKind::Safe,
    }
}

/// Rules that have been renamed.
/// The first element of every pair is the old name of the rule.
#[rustfmt::skip]
const RULE_RENAMINGS: &[(&str, RuleName)] = &[
    ("noConsoleLog", RuleName::NoConsole),
    ("noInvalidNewBuiltin", RuleName::NoInvalidBuiltinInstantiation),
    ("noMultipleSpacesInRegularExpressionLiterals", RuleName::NoAdjacentSpacesInRegex),
    ("noNewSymbol", RuleName::NoInvalidBuiltinInstantiation),
    ("noUnnecessaryContinue", RuleName::NoUselessContinue),
    ("useShorthandArrayType", RuleName::UseConsistentArrayType),
    ("useSingleCaseStatement", RuleName::NoSwitchDeclarations),
];

impl Rule for RuleMover {
    type Query = Ast<JsonMember>;
    type State = State;
    type Signals = Box<[Self::State]>;
    type Options = ();

    fn run(ctx: &biome_analyze::context::RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if node
            .name()
            .and_then(|name| name.inner_string_text())
            .is_ok_and(|name| name.text() != "rules")
        {
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
                    if new_rule.group() != current_group {
                        result.push(State {
                            rule_node,
                            new_rule,
                            old_group: Some(current_group),
                            old_rule_name: None,
                        });
                    }
                } else if let Some((old_rule_name, new_rule)) = RULE_RENAMINGS
                    .iter()
                    .find(|(old_name, _)| old_name == &rule_name)
                    .copied()
                {
                    result.push(State {
                        rule_node,
                        new_rule,
                        old_group: (new_rule.group() != current_group).then_some(current_group),
                        old_rule_name: Some(old_rule_name),
                    });
                }
            }
        }
        result.into()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let rule_name = state.new_rule.as_str();
        let new_group = state.new_rule.group();
        let new_group_name = new_group.as_str();
        let msg = if let Some(old_group) = state.old_group {
            let action = if old_group == RuleGroup::Nursery {
                "promoted"
            } else if new_group == RuleGroup::Nursery {
                "unpromoted"
            } else {
                "moved"
            };
            markup! { "This rule has been "{action}" to "<Emphasis>{new_group_name}"/"{rule_name}</Emphasis>"." }
                .to_owned()
        } else {
            markup! { "This rule has been renamed to "<Emphasis>{rule_name}</Emphasis>"." }
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
        let rule_node = if let Some(old_rule_name) = state.old_rule_name {
            let new_name =
                make::json_member_name(make::json_string_literal(state.new_rule.as_str()));
            let rule_node = rule_node.with_name(new_name);
            if let Some(value) = rule_node
                .value()
                .ok()
                .and_then(|value| transform_value(value, old_rule_name))
            {
                rule_node.with_value(value)
            } else {
                rule_node
            }
        } else {
            rule_node
        };

        // If the rule is not moved and just renamed, we return early.
        if state.old_group.is_none() {
            mutation.replace_node(state.rule_node.clone(), rule_node);
            return Some(MigrationAction::new(
                ctx.metadata().action_category(ctx.category(), ctx.group()),
                ctx.metadata().applicability(),
                markup! { "Rename the rule." }.to_owned(),
                mutation,
            ));
        };

        // Update or create the group where the rule is moved to
        // If the group exists, then `new_group_range` will be the range of the group
        let new_group_name = state.new_rule.group().as_str();
        let (new_group_range, new_group_node) = if let Some(new_group) =
            rules.find_member(new_group_name)
        {
            // The group exists, so we update it.
            let AnyJsonValue::JsonObjectValue(group_obj) = new_group.value().ok()? else {
                return None;
            };
            let old_list = group_obj.json_member_list();
            let mut new_elements = Vec::with_capacity(old_list.len() + 1);
            for elt in old_list.elements() {
                new_elements.push((elt.node.ok()?, elt.trailing_separator.ok()?));
            }
            let last_has_separator = new_elements.last().is_some_and(|(_, sep)| sep.is_some());
            // Add the new rule
            new_elements.push((rule_node, None));
            handle_trvia(
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
            if let Some((last_node, _)) = new_elements.last() {
                if let Some(first_token) = last_node.syntax().first_token() {
                    indent.extend(first_token.indentation_trivia_pieces());
                }
            }
            // Add the new group and rule
            let mut indent = Vec::new();
            if let Some(Ok(last_group_node)) = old_rules_list.last() {
                if let Some(first_token) = last_group_node.syntax().first_token() {
                    indent.extend(first_token.indentation_trivia_pieces());
                }
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
                            [rule_node],
                            [make::token(T![,])
                                .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")])],
                        )
                    } else {
                        make::json_member_list([rule_node], [])
                    },
                    make::token(T!['}']).with_leading_trivia([(TriviaPieceKind::Whitespace, " ")]),
                )
                .into(),
            );
            (None, new_group_node)
        };

        // Update `rules` by updating or adding `new_group_node`.
        // It is also in charge of removing the old rule.
        let mut new_rules_elements = Vec::with_capacity(old_rules_list.len() + 1);
        let rule_group_node = state.rule_node.syntax().grand_parent()?.parent()?;
        for elt in old_rules_list.elements() {
            let node = elt.node.ok()?;
            let node = if node.range() == rule_group_node.text_range() {
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
                handle_trvia(
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
            } else if new_group_range.is_some_and(|range| range == node.range()) {
                // Update the group
                new_group_node.clone()
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
        handle_trvia(
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

        mutation.replace_node(old_rules_list, new_rules_list);
        Some(MigrationAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Move the rule." }.to_owned(),
            mutation,
        ))
    }
}

pub struct State {
    rule_node: JsonMember,
    /// Set if the rule is moved to a new group.
    old_group: Option<RuleGroup>,
    /// Set if the rule is renamed.
    old_rule_name: Option<&'static str>,
    new_rule: RuleName,
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

// TODO: copied from `biome_js_analyze/src/assist/source/organize_imports/specifiers_attributes.rs`
// could be worth to share it.
fn handle_trvia<'a, L: Language + 'a, N: AstNode<Language = L> + 'a>(
    // Mutable iterator of a list of nodes and their optional separators
    iter: impl std::iter::ExactSizeIterator<Item = (&'a mut N, &'a mut Option<SyntaxToken<L>>)>,
    needs_last_separator: bool,
    make_separator: fn() -> SyntaxToken<L>,
) {
    let last_index = iter.len().saturating_sub(1);
    for (i, (node, optional_separator)) in iter.enumerate() {
        if let Some(separator) = optional_separator {
            // Remove the last separator at the separator has no attached comments
            if i == last_index
                && !(needs_last_separator
                    || separator.has_leading_comments()
                    || separator.has_trailing_comments())
            {
                // Transfer the separator trivia
                if let Some(new_node) = node.clone().append_trivia_pieces(chain_trivia_pieces(
                    separator.leading_trivia().pieces(),
                    separator.trailing_trivia().pieces(),
                )) {
                    *node = new_node;
                }
                *optional_separator = None;
            }
        } else if i != last_index || needs_last_separator {
            // The last node is moved and has no trailing separator.
            // Thus we build a new separator and remove its trailing whitespaces.
            if let Some(new_node) = node.clone().trim_trailing_trivia() {
                *node = new_node;
            }
            *optional_separator = Some(make_separator());
        }
    }
}
