use crate::version_services::Version;
use crate::{declare_migration, MigrationAction};
use biome_analyze::context::RuleContext;
use biome_analyze::{Rule, RuleAction, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::{category, Applicability};
use biome_json_factory::make::{
    json_member, json_member_list, json_member_name, json_object_value, json_string_literal,
    json_string_value, token,
};
use biome_json_syntax::{
    AnyJsonValue, JsonMember, JsonMemberList, JsonObjectValue, JsonRoot, JsonSyntaxToken, T,
};
use biome_rowan::{AstNode, BatchMutationExt, TriviaPieceKind, WalkEvent};
use rustc_hash::FxHashSet;

declare_migration! {
    pub(crate) StyleRules {
        version: "2.0.0",
        name: "styleRules",
    }
}

const STYLE_RULES_THAT_WERE_ERROR: [&str; 24] = [
    "useNumberNamespace",
    "noNonnullAssertion",
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
    "noVar",
    "noUselessElse",
    "useNumericLiterals",
    "noCommaOperator",
    "useConst",
    "noArguments",
    "useSelfClosingElements",
    "useImportType",
    "useTemplate",
    "useSingleVarDeclarator",
    "useWhile",
];

impl Rule for StyleRules {
    type Query = Version<JsonRoot>;
    type State = FxHashSet<Box<str>>;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let version = ctx.version();

        if version != ctx.metadata().version {
            return None;
        }
        let mut nodes = FxHashSet::default();
        for rule in STYLE_RULES_THAT_WERE_ERROR {
            nodes.insert(Box::from(rule));
        }

        let events = node.syntax().preorder();

        for event in events {
            match event {
                WalkEvent::Enter(node) => {
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
                WalkEvent::Leave(_) => {}
            }
        }

        Some(nodes)
    }

    fn diagnostic(_ctx: &RuleContext<Self>, _node: &Self::State) -> Option<RuleDiagnostic> {
        let root = _ctx.root();
        Some(RuleDiagnostic::new(
            category!("migrate"),
            root.range(),
            markup! {
                "Biome defaults have changed."
            }
                .to_owned(),
        ).note(markup!{
            "To avoid regressions with your code base, Biome will update the configuration file to maintain the compatibility with your current setup."
        }))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<MigrationAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        let mut separators = vec![];
        let members: Vec<_> = state
            .iter()
            .enumerate()
            .map(|(index, text)| {
                if index + 1 < state.len() {
                    separators.push(token(T![,]))
                }
                create_member(
                    text.as_ref(),
                    AnyJsonValue::JsonStringValue(json_string_value(json_string_literal("error"))),
                    8,
                )
            })
            .collect();

        let member = find_object_to_change(ctx.root())?;

        match member {
            MemberFound::Style(object) => {
                let new_member =
                    create_style_member_from_existing_one(members, separators, object.clone())?;
                mutation.replace_node(object, new_member);
            }
            MemberFound::Rules(object) => {
                let new_member =
                    create_rules_member_from_existing_one(members, separators, object.clone())?;
                mutation.replace_node(object, new_member);
            }
            MemberFound::Linter(member) => {
                let new_member =
                    create_linter_member_from_existing_one(members, separators, member.clone())?;
                mutation.replace_node(member, new_member);
            }
            // TODO: implement
            MemberFound::None => {
                let list = node
                    .value()
                    .ok()?
                    .as_json_object_value()?
                    .json_member_list();
                let mut new_list = vec![];
                let mut new_separators = vec![];
                for item in list.clone() {
                    let item = item.ok()?;
                    new_list.push(item);
                    new_separators.push(token(T![,]));
                }
                let member = create_new_linter_member(members, separators, 2);
                new_list.push(member);
                mutation.replace_node(list, json_member_list(new_list, new_separators));
            }
        };

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

fn create_object(list: JsonMemberList, spaces: usize) -> JsonObjectValue {
    json_object_value(
        token(T!['{']).with_leading_trivia(vec![(TriviaPieceKind::Whitespace, " ")]),
        list,
        token(T!['}']).with_leading_trivia(vec![
            (TriviaPieceKind::Newline, "\n"),
            (TriviaPieceKind::Whitespace, " ".repeat(spaces).as_str()),
        ]),
    )
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

/// Creates
/// ```json
/// {
///     "style": {}
/// }
/// ```
fn create_style_member_from_existing_one(
    members: Vec<JsonMember>,
    separators: Vec<JsonSyntaxToken>,
    member: JsonMember,
) -> Option<JsonMember> {
    let mut new_members = vec![];
    let mut new_separators = vec![];
    let list = member
        .value()
        .ok()?
        .as_json_object_value()?
        .json_member_list();
    for item in list {
        let item = item.ok()?;
        new_members.push(item);
        new_separators.push(token(T![,]));
    }
    new_members.extend(members);
    new_separators.extend(separators);

    Some(style_member(new_members, new_separators, 8))
}

/// Creates
///
/// ```json
/// {
///     "rules": {
///         "style": {}
///     }
/// }
/// ```
fn create_rules_member_from_existing_one(
    members: Vec<JsonMember>,
    separators: Vec<JsonSyntaxToken>,
    member: JsonMember,
) -> Option<JsonMember> {
    let mut new_members = vec![];
    let mut new_separators = vec![];
    let list = member
        .value()
        .ok()?
        .as_json_object_value()?
        .json_member_list();
    for item in list {
        let item = item.ok()?;
        new_members.push(item);
        new_separators.push(token(T![,]));
    }

    new_members.push(style_member(members, separators, 6));

    Some(rules_member(new_members, new_separators, 6))
}

/// Creates
///
/// ```json
/// {
///     "linter": {
///         "rules": {
///             "style": {}
///         }
///     }
/// }
/// ```
fn create_linter_member_from_existing_one(
    members: Vec<JsonMember>,
    separators: Vec<JsonSyntaxToken>,
    member: JsonMember,
) -> Option<JsonMember> {
    let mut new_members = vec![];
    let mut new_separators = vec![];
    let list = member
        .value()
        .ok()?
        .as_json_object_value()?
        .json_member_list();
    for item in list {
        let item = item.ok()?;
        new_members.push(item);
        new_separators.push(token(T![,]));
    }

    new_members.push(rules_member(members, separators, 4));
    Some(linter_member(new_members, new_separators, 4))
}

#[derive(Debug)]
enum MemberFound {
    Style(JsonMember),
    Rules(JsonMember),
    Linter(JsonMember),
    None,
}

fn find_object_to_change(root: JsonRoot) -> Option<MemberFound> {
    let events = root.syntax().preorder();
    let mut member_found = MemberFound::None;

    for event in events {
        match event {
            WalkEvent::Enter(node) => {
                if let Some(member) = JsonMember::cast(node) {
                    let name = member.name().ok()?.inner_string_text().ok()?;

                    if name.text() == "rules" {
                        member_found = MemberFound::Rules(member);
                    } else if name.text() == "linter" {
                        member_found = MemberFound::Linter(member);
                    } else if name.text() == "style" {
                        member_found = MemberFound::Style(member);
                    }
                }
            }
            WalkEvent::Leave(_) => {}
        }
    }

    Some(member_found)
}

fn style_member(
    members: Vec<JsonMember>,
    separators: Vec<JsonSyntaxToken>,
    indentation: usize,
) -> JsonMember {
    let list = json_member_list(members, separators);
    let object = create_object(list, indentation);
    create_member("style", AnyJsonValue::JsonObjectValue(object), indentation)
}

fn rules_member(
    members: Vec<JsonMember>,
    separators: Vec<JsonSyntaxToken>,
    indentation: usize,
) -> JsonMember {
    let list = json_member_list(members, separators);
    let object = create_object(list, indentation);
    create_member("rules", AnyJsonValue::JsonObjectValue(object), indentation)
}

fn linter_member(
    members: Vec<JsonMember>,
    separators: Vec<JsonSyntaxToken>,
    indentation: usize,
) -> JsonMember {
    let list = json_member_list(members, separators);
    let object = create_object(list, indentation);
    create_member("linter", AnyJsonValue::JsonObjectValue(object), indentation)
}

fn create_new_linter_member(
    members: Vec<JsonMember>,
    separators: Vec<JsonSyntaxToken>,
    indentation: usize,
) -> JsonMember {
    let style = style_member(members, separators, indentation * 3);
    let rules = rules_member(vec![style], vec![], indentation * 2);
    let linter = linter_member(vec![rules], vec![], indentation * 1);
    linter
}
