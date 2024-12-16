use biome_json_factory::make::{
    json_member, json_member_list, json_member_name, json_object_value, json_string_literal, token,
};
use biome_json_syntax::{
    AnyJsonValue, JsonLanguage, JsonMember, JsonMemberList, JsonObjectValue, JsonRoot,
    JsonSyntaxToken, T,
};
use biome_rowan::{AstNode, BatchMutation, TriviaPieceKind, WalkEvent};
use rustc_hash::FxHashMap;
use std::str::FromStr;

#[derive(Debug, Eq, Hash, PartialEq)]
pub(crate) enum Group {
    Style,
    Suspicious,
    Nursery,
}

impl Group {
    pub(crate) fn as_str<'a>(&self) -> &'a str {
        match self {
            Group::Style => "style",
            Group::Suspicious => "suspicious",
            Group::Nursery => "nursery",
        }
    }
}

impl FromStr for Group {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "style" => Group::Style,
            "suspicious" => Group::Suspicious,
            "nursery" => Group::Nursery,
            _ => return Err(()),
        })
    }
}

#[derive(Debug)]
pub(crate) enum MemberKind {
    Groups {
        groups: FxHashMap<Group, JsonMember>,
    },
    Rules(JsonMember),
    Linter(JsonMember),
    None,
}

impl MemberKind {
    pub(crate) fn new_groups() -> Self {
        Self::Groups {
            groups: FxHashMap::default(),
        }
    }

    pub(crate) fn push_group(&mut self, group: Group, member: JsonMember) {
        match self {
            MemberKind::Groups { groups } => {
                groups.insert(group, member);
            }
            MemberKind::Rules(_) | MemberKind::Linter(_) | MemberKind::None => {}
        }
    }
}

pub(crate) struct RuleMover {
    pub(crate) member_found: MemberKind,
    root: JsonRoot,
    queries: Vec<MoverQuery>,
}

pub(crate) struct MoverQuery {
    rule_name: String,
    from_group: Group,
    to_group: Group,
    rule_member: Option<JsonMember>,
}

impl RuleMover {
    /// Attempts to find  `linter`, `linter.rules` or `linter.rules.<group>`
    pub(crate) fn from_root(root: JsonRoot) -> Self {
        let events = root.syntax().preorder();
        let mut member_found = MemberKind::None;

        for event in events {
            match event {
                WalkEvent::Enter(node) => {
                    if let Some(member) = JsonMember::cast(node) {
                        let Some(name) =
                            member.name().ok().and_then(|n| n.inner_string_text().ok())
                        else {
                            continue;
                        };

                        if name.text() == "rules" {
                            member_found = MemberKind::Rules(member);
                        } else if name.text() == "linter" {
                            member_found = MemberKind::Linter(member);
                        } else if let Ok(group) = Group::from_str(name.text()) {
                            if matches!(member_found, MemberKind::Groups { .. }) {
                                member_found.push_group(group, member);
                            } else {
                                member_found = MemberKind::new_groups();
                                member_found.push_group(group, member);
                            }
                        }
                    }
                }
                WalkEvent::Leave(_) => {}
            }
        }

        Self {
            root,
            member_found,
            queries: vec![],
        }
    }

    pub(crate) fn register_move(&mut self, rule_name: impl ToString, from: &str, to: &str) {
        let from_group = Group::from_str(from).expect("to be a valid group");
        let to_group = Group::from_str(to).expect("to be a valid group");

        let mut rule_member = None;
        if let MemberKind::Groups { groups } = &self.member_found {
            for (group, member) in groups.iter() {
                if group == &from_group {
                    let list = member
                        .value()
                        .ok()
                        .and_then(|n| n.as_json_object_value().cloned())
                        .map(|n| n.json_member_list());
                    if let Some(list) = list {
                        for item in list {
                            if let Some(member) = item.ok() {
                                let text =
                                    member.name().ok().and_then(|n| n.inner_string_text().ok());

                                if matches!(text, Some(rule_name)) {
                                    rule_member = Some(member)
                                }
                            }
                        }
                    }
                }
            }
        }

        self.queries.push(MoverQuery {
            rule_name: rule_name.to_string(),
            from_group,
            to_group,
            rule_member,
        })
    }

    pub(crate) fn replace(
        self,
        mutation: &mut BatchMutation<JsonLanguage>,
        members: Vec<JsonMember>,
        separators: Vec<JsonSyntaxToken>,
    ) -> Option<()> {
        match self.member_found {
            MemberKind::Group(object) => {
                let new_member = create_group_from_existing_one(
                    members,
                    separators,
                    object.clone(),
                    self.group,
                )?;
                mutation.replace_node(object, new_member);
            }
            MemberKind::Rules(object) => {
                let new_member = create_rules_member_from_existing_one(
                    members,
                    separators,
                    object.clone(),
                    self.group,
                )?;
                mutation.replace_node(object, new_member);
            }
            MemberKind::Linter(member) => {
                let new_member =
                    create_linter_member_from_existing_one(members, separators, member.clone())?;
                mutation.replace_node(member, new_member);
            }
            MemberKind::None => {
                let list = self
                    .root
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
                let member = create_new_linter_member(members, separators, 2, self.group);
                new_list.push(member);
                mutation.replace_node(list, json_member_list(new_list, new_separators));
            }
        };

        Some(())
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
fn create_group_from_existing_one(
    members: Vec<JsonMember>,
    separators: Vec<JsonSyntaxToken>,
    member: JsonMember,
    new_group: &str,
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

    Some(group_member(new_members, new_separators, 8, new_group))
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
    new_group: &str,
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

    new_members.push(group_member(members, separators, 6, new_group));

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

fn group_member(
    members: Vec<JsonMember>,
    separators: Vec<JsonSyntaxToken>,
    indentation: usize,
    new_group: &str,
) -> JsonMember {
    let list = json_member_list(members, separators);
    let object = create_object(list, indentation);
    create_member(
        new_group,
        AnyJsonValue::JsonObjectValue(object),
        indentation,
    )
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
    new_group: &str,
) -> JsonMember {
    let style = group_member(members, separators, indentation * 3, new_group);
    let rules = rules_member(vec![style], vec![], indentation * 2);
    let linter = linter_member(vec![rules], vec![], indentation * 1);
    linter
}
