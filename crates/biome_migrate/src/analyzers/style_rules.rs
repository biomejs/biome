use crate::configuration::{
    create_group_member, create_linter_member, create_member, create_rules_member,
    create_separator, get_severity_from_member, seek_linter_configuration_downwards,
    traverse_linter_from_root, TraverseResult,
};
use crate::{declare_migration, MigrationAction};
use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, Rule, RuleAction, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::{category, Applicability};
use biome_json_factory::make::{
    json_member, json_member_list, json_object_value, json_string_literal, json_string_value, token,
};
use biome_json_syntax::{AnyJsonValue, JsonMember, JsonRoot, T};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, TriviaPieceKind, WalkEvent};
use rustc_hash::FxHashMap;

declare_migration! {
    pub(crate) StyleRules {
        version: "2.0.0",
        name: "styleRules",
    }
}

const STYLE_RULES_THAT_WERE_ERROR: [&str; 22] = [
    "useNumberNamespace",
    "noNonNullAssertion",
    "useAsConstAssertion",
    "noParameterAssign",
    "noInferrableTypes",
    "useNodejsImportProtocol",
    "useExportType",
    "useDefaultParameterLast",
    "noUnusedTemplateLiteral",
    "useExponentiationOperator",
    "useEnumInitializers",
    "useShorthandFunctionType",
    "useLiteralEnumMembers",
    "noUselessElse",
    "useNumericLiterals",
    "noCommaOperator",
    "useConst",
    "noArguments",
    "useSelfClosingElements",
    "useImportType",
    "useTemplate",
    "useSingleVarDeclarator",
];

pub struct StyleRulesState {
    /// The name of the style rule to add
    rule_name: Box<str>,
    /// If the rule is already present, we change its severity to `"warn"`
    rule_member: Option<JsonMember>,
}

impl Rule for StyleRules {
    type Query = Ast<JsonRoot>;
    type State = StyleRulesState;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let mut rules_to_track = FxHashMap::default();
        for rule in STYLE_RULES_THAT_WERE_ERROR {
            rules_to_track.insert(Box::from(rule), None);
        }

        let linter = seek_linter_configuration_downwards(node);
        if let Some(linter) = linter {
            let events = linter.syntax().preorder();
            for event in events {
                match event {
                    WalkEvent::Enter(node) => {
                        if let Some(node) = JsonMember::cast(node) {
                            track_rules(node, &mut rules_to_track);
                        }
                    }
                    WalkEvent::Leave(_) => {}
                }
            }
        }

        rules_to_track
            .into_iter()
            .map(|(k, v)| StyleRulesState {
                rule_name: k,
                rule_member: v,
            })
            .collect()
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let root = ctx.root();
        Some(RuleDiagnostic::new(
            category!("migrate"),
            root.range(),
            markup! {
                "Biome style rule "<Emphasis>{state.rule_name}</Emphasis>" isn't recommended anymore."
            }
                .to_owned(),
        ).note(markup!{
            "To avoid regressions with your code base, Biome will now enable this "<Emphasis>{state.rule_name}</Emphasis>" to match your current setup."
        }))
    }

    fn action(ctx: &RuleContext<Self>, rule_to_move: &Self::State) -> Option<MigrationAction> {
        // we generate the JSON member of the lint rule
        let new_member_to_use = rule_to_move
            .rule_member
            .clone()
            .and_then(update_rule_member)
            .unwrap_or(create_member(
                rule_to_move.rule_name.as_ref(),
                AnyJsonValue::JsonStringValue(json_string_value(
                    json_string_literal("error")
                        .with_leading_trivia(vec![(TriviaPieceKind::Whitespace, " ")]),
                )),
                8,
            ));

        let root = ctx.query();
        let mut mutation = ctx.root().begin();

        let member_found = traverse_linter_from_root(root, "style");

        match member_found {
            Some(member_found) => match member_found {
                TraverseResult::Linter(linter_member) => {
                    let style_member = create_group_member("style", vec![new_member_to_use]);
                    let rules_member = create_rules_member(vec![style_member]);
                    let list = linter_member
                        .value()
                        .ok()?
                        .as_json_object_value()?
                        .json_member_list();
                    let mut new_list: Vec<_> = list.iter().flatten().collect();
                    new_list.push(rules_member);
                    let separators = create_separator(new_list.as_slice());
                    let new_member_list = json_member_list(new_list, separators);
                    mutation.replace_node(list, new_member_list);
                }
                TraverseResult::Rules(rules_member) => {
                    let style_member = create_group_member("style", vec![new_member_to_use]);
                    let list = rules_member
                        .value()
                        .ok()?
                        .as_json_object_value()?
                        .json_member_list();
                    let mut new_list: Vec<_> = list.iter().flatten().collect();
                    new_list.push(style_member);
                    let separators = create_separator(new_list.as_slice());
                    let new_member_list = json_member_list(new_list, separators);
                    mutation.replace_node(list, new_member_list);
                }
                TraverseResult::Group(style_member) => {
                    let list = style_member
                        .value()
                        .ok()?
                        .as_json_object_value()?
                        .json_member_list();
                    let mut new_list: Vec<_> = list.iter().flatten().collect();
                    new_list.push(new_member_to_use);
                    let separators = create_separator(new_list.as_slice());
                    let new_member_list = json_member_list(new_list, separators);
                    mutation.replace_node(list, new_member_list);
                }
            },
            None => {
                let style_member = create_group_member("style", vec![new_member_to_use]);
                let rules_member = create_rules_member(vec![style_member]);
                let linter_member = create_linter_member(vec![rules_member]);
                let original_list = root
                    .value()
                    .ok()?
                    .as_json_object_value()?
                    .json_member_list();
                let mut new_list: Vec<_> = original_list.iter().flatten().collect();
                new_list.push(linter_member);
                let separators = create_separator(new_list.as_slice());

                mutation.replace_node(original_list, json_member_list(new_list, separators));
            }
        }

        Some(RuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            Applicability::Always,
            markup! {
                "Update the configuration to enable these rules."
            }
            .to_owned(),
            mutation,
        ))
    }
}

fn update_rule_member(member: JsonMember) -> Option<JsonMember> {
    let severity = get_severity_from_member(&member);
    if severity.as_deref() == Some("off") {
        return None;
    }
    let value = member.value().ok()?;
    let new_value = match value {
        // This matches
        AnyJsonValue::JsonObjectValue(object) => {
            let new_list: Vec<_> = object
                .json_member_list()
                .iter()
                .flatten()
                .filter_map(|member: JsonMember| {
                    let name = member.name().ok()?.inner_string_text().ok()?;
                    if name.text() == "severity" {
                        create_member_severity_member(&member)
                    } else {
                        Some(member)
                    }
                })
                .collect();
            let mut separators = vec![];
            for _ in 0..new_list.len().saturating_sub(1) {
                separators.push(token(T![,]));
            }

            let new_list = json_member_list(new_list, separators)
                .with_leading_trivia_pieces(object.syntax().first_leading_trivia()?.pieces())?
                .with_trailing_trivia_pieces(object.syntax().last_trailing_trivia()?.pieces())?;
            AnyJsonValue::JsonObjectValue(json_object_value(
                object.l_curly_token().ok()?,
                new_list,
                object.r_curly_token().ok()?,
            ))
        }
        AnyJsonValue::JsonStringValue(string_literal) => AnyJsonValue::JsonStringValue(
            json_string_value(json_string_literal("error"))
                .with_leading_trivia_pieces(
                    string_literal.syntax().first_leading_trivia()?.pieces(),
                )?
                .with_trailing_trivia_pieces(
                    string_literal.syntax().last_trailing_trivia()?.pieces(),
                )?,
        ),
        _ => return None,
    };

    Some(json_member(
        member.name().ok()?,
        member.colon_token().ok()?,
        new_value
            .with_leading_trivia_pieces(member.syntax().first_leading_trivia()?.pieces())?
            .with_trailing_trivia_pieces(member.syntax().last_trailing_trivia()?.pieces())?,
    ))
}

fn track_rules(node: JsonMember, rules_to_track: &mut FxHashMap<Box<str>, Option<JsonMember>>) {
    let node_text = node
        .name()
        .ok()
        .and_then(|node| node.inner_string_text().ok());

    if matches!(node_text.as_deref(), Some("style")) {
        let list = node.map_members();

        let Some(list) = list else {
            return;
        };
        for item in list {
            let member = item.name().ok().and_then(|n| n.inner_string_text().ok());
            if let Some(node_text) = member {
                let severity = get_severity_from_member(&item);
                if matches!(severity.as_deref(), Some("off" | "error")) {
                    rules_to_track.remove(node_text.text());
                }
                if let Some(state) = rules_to_track.get_mut(node_text.text()) {
                    *state = Some(item.clone());
                }
            }
        }
    }
}

fn create_member_severity_member(current_member: &JsonMember) -> Option<JsonMember> {
    Some(json_member(
        current_member.name().ok()?,
        current_member.colon_token().ok()?,
        AnyJsonValue::JsonStringValue(json_string_value(
            json_string_literal("on").with_leading_trivia(vec![(TriviaPieceKind::Whitespace, " ")]),
        )),
    ))
}
