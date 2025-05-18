use crate::{MigrationAction, declare_migration};
use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, Rule, RuleAction, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::{Applicability, category};
use biome_json_factory::make::{
    json_member, json_member_list, json_member_name, json_object_value, json_string_literal,
    json_string_value, token,
};
use biome_json_syntax::{
    AnyJsonValue, JsonMember, JsonMemberList, JsonObjectValue, JsonRoot, JsonSyntaxToken, T,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, TriviaPieceKind};
use rustc_hash::FxHashSet;

declare_migration! {
    pub(crate) StyleRules {
        version: "2.0.0",
        name: "styleRules",
    }
}

const STYLE_RULES_THAT_WERE_ERROR: [&str; 10] = [
    "useNumberNamespace",
    "useAsConstAssertion",
    "noParameterAssign",
    "noInferrableTypes",
    "useDefaultParameterLast",
    "noUnusedTemplateLiteral",
    "useEnumInitializers",
    "noUselessElse",
    "useSelfClosingElements",
    "useSingleVarDeclarator",
];

impl Rule for StyleRules {
    type Query = Ast<JsonRoot>;
    type State = Box<str>;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let mut nodes = FxHashSet::default();
        for rule in STYLE_RULES_THAT_WERE_ERROR {
            nodes.insert(Box::from(rule));
        }

        for node in node.syntax().descendants() {
            let Some(node) = JsonMember::cast(node) else {
                continue;
            };
            let node_text = node
                .name()
                .ok()
                .and_then(|node| node.inner_string_text().ok());

            let Some(node_text) = node_text else {
                continue;
            };

            if node_text == "recommended" {
                let recommended_disabled = node
                    .value()
                    .ok()
                    .and_then(|n| n.as_json_boolean_value().cloned())
                    .and_then(|n| n.value_token().ok())
                    .is_some_and(|n| n.text() == "false");
                if recommended_disabled {
                    return vec![];
                }
            }

            if node_text == "style" {
                let list = node
                    .value()
                    .ok()
                    .and_then(|n| n.as_json_object_value().cloned())
                    .map(|n| n.json_member_list());

                let Some(list) = list else { continue };
                for item in list {
                    let member = item
                        .ok()
                        .and_then(|n| n.name().ok())
                        .and_then(|n| n.inner_string_text().ok());
                    if let Some(node_text) = member {
                        nodes.remove(&Box::from(node_text.text()));
                    }
                }
            }
        }

        nodes.into_iter().collect()
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let root = ctx.root();
        Some(RuleDiagnostic::new(
            category!("migrate"),
            root.range(),
            markup! {
                "Biome style rule "{state}" isn't recommended anymore."
            }
                .to_owned(),
        ).note(markup!{
            "To avoid regressions with your code base, Biome will update the configuration file to maintain the compatibility with your current setup."
        }))
    }

    fn action(ctx: &RuleContext<Self>, rule_to_move: &Self::State) -> Option<MigrationAction> {
        let root = ctx.root();
        let new_rule_member = json_member(
            json_member_name(
                json_string_literal(rule_to_move.as_ref()).with_leading_trivia([
                    (TriviaPieceKind::Newline, "\n"),
                    (TriviaPieceKind::Whitespace, " ".repeat(8).as_str()),
                ]),
            ),
            token(T![:]),
            AnyJsonValue::JsonStringValue(json_string_value(
                json_string_literal("error")
                    .with_leading_trivia([(TriviaPieceKind::Whitespace, " ")]),
            )),
        );
        let linter_member = get_linter_field(ctx.root());
        let rules_member = linter_member
            .as_ref()
            .and_then(|linter_member| find_member_by_name(linter_member, "rules"));
        let styles_member = rules_member
            .as_ref()
            .and_then(|rules_member| find_member_by_name(rules_member, "style"));

        let new_styles_member = match styles_member {
            None => create_style_member(vec![new_rule_member], vec![], 6),
            Some(styles_member) => {
                let (list, separators) =
                    add_or_replace_member(styles_member, new_rule_member.clone())?;
                create_style_member(list, separators, 6)
            }
        };

        let new_rules_member = match rules_member {
            None => create_rules_member(vec![new_styles_member], vec![], 4),
            Some(rules_member) => {
                let (list, separators) =
                    add_or_replace_member(rules_member, new_styles_member.clone())?;
                create_rules_member(list, separators, 4)
            }
        };

        let new_linter_member = match linter_member {
            None => create_linter_member(vec![new_rules_member], vec![], 2),
            Some(linter_member) => {
                let (list, separators) =
                    add_or_replace_member(linter_member, new_rules_member.clone())?;

                create_linter_member(list, separators, 2)
            }
        };

        let mut mutation = ctx.root().begin();

        let root_list = root
            .value()
            .ok()?
            .as_json_object_value()?
            .json_member_list();
        let mut linter_replaced = false;
        for member in root_list.iter().flatten() {
            let member_name = member.name().ok()?.inner_string_text().ok()?;
            if member_name.text() == "linter" {
                mutation.replace_node(member, new_linter_member.clone());
                linter_replaced = true;
            }
        }

        if !linter_replaced {
            let mut new_root_list = root_list.iter().flatten().collect::<Vec<_>>();
            new_root_list.push(new_linter_member);
            let mut separators = vec![];
            for _ in 0..new_root_list.len().saturating_sub(1) {
                separators.push(token(T![,]))
            }
            debug_assert_eq!(new_root_list.len(), separators.len() + 1);
            mutation.replace_node(root_list, json_member_list(new_root_list, separators));
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

fn find_member_by_name(member: &JsonMember, field_name: &str) -> Option<JsonMember> {
    member
        .value()
        .ok()?
        .as_json_object_value()?
        .find_member(field_name)
}

fn get_linter_field(root: JsonRoot) -> Option<JsonMember> {
    root.value()
        .ok()?
        .as_json_object_value()?
        .find_member("linter")
}

fn create_member(text: &str, value: AnyJsonValue, level: usize) -> JsonMember {
    json_member(
        json_member_name(json_string_literal(text).with_leading_trivia(vec![
            (TriviaPieceKind::Newline, "\n"),
            (TriviaPieceKind::Whitespace, " ".repeat(level).as_str()),
        ])),
        token(T![:]),
        value,
    )
}

fn create_object(list: JsonMemberList, spaces: usize) -> JsonObjectValue {
    json_object_value(
        token(T!['{']).with_leading_trivia([(TriviaPieceKind::Whitespace, " ")]),
        list,
        token(T!['}']).with_leading_trivia([
            (TriviaPieceKind::Newline, "\n"),
            (TriviaPieceKind::Whitespace, " ".repeat(spaces).as_str()),
        ]),
    )
}

fn create_rules_member(
    members: Vec<JsonMember>,
    separators: Vec<JsonSyntaxToken>,
    white_space: usize,
) -> JsonMember {
    let list = json_member_list(members, separators);
    let object = create_object(list, white_space);
    create_member("rules", AnyJsonValue::JsonObjectValue(object), white_space)
}

fn create_style_member(
    members: Vec<JsonMember>,
    separators: Vec<JsonSyntaxToken>,
    white_space: usize,
) -> JsonMember {
    let list = json_member_list(members, separators);
    let object = create_object(list, white_space);
    create_member("style", AnyJsonValue::JsonObjectValue(object), white_space)
}

fn create_linter_member(
    members: Vec<JsonMember>,
    separators: Vec<JsonSyntaxToken>,
    white_space: usize,
) -> JsonMember {
    let list = json_member_list(members, separators);
    let object = create_object(list, white_space);
    create_member("linter", AnyJsonValue::JsonObjectValue(object), white_space)
}

fn add_or_replace_member(
    parent_member: JsonMember,
    member_to_replace_or_add: JsonMember,
) -> Option<(Vec<JsonMember>, Vec<JsonSyntaxToken>)> {
    let (list, mut separators) = parent_member.unzip_elements()?;
    let mut new_list = vec![];
    let mut member_replaced = false;

    let new_member_name = member_to_replace_or_add
        .name()
        .ok()?
        .inner_string_text()
        .ok()?;
    for member in &list {
        let member_name = member.name().ok()?.inner_string_text().ok()?;
        if member_name.text() == new_member_name.text() {
            new_list.push(member_to_replace_or_add.clone());
            member_replaced = true
        } else {
            new_list.push(member.clone());
        }
    }
    if !member_replaced {
        new_list.push(member_to_replace_or_add);
        if new_list.len() > 1 {
            separators.push(token(T![,]));
        }
    }
    debug_assert_eq!(new_list.len() - 1, separators.len());

    Some((new_list, separators))
}
