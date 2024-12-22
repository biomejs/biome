use biome_json_factory::make::{
    json_member, json_member_list, json_member_name, json_object_value, json_string_literal,
    json_string_value, token,
};
use biome_json_syntax::{
    AnyJsonValue, JsonLanguage, JsonMember, JsonMemberList, JsonObjectValue, JsonRoot,
    JsonSyntaxToken, T,
};
use biome_rowan::{
    AstNode, AstSeparatedList, BatchMutation, BatchMutationExt, TriviaPieceKind, WalkEvent,
};
use rustc_hash::FxHashMap;
use std::str::FromStr;

const ALL_GROUPS: &[&str] = &[
    "nursery",
    "suspicious",
    "a11y",
    "security",
    "complexity",
    "style",
    "correctness",
    "performance",
];

#[derive(Debug, Eq, Hash, PartialEq)]
pub(crate) enum Group {
    Style,
    Suspicious,
    Nursery,
    A11y,
    Security,
    Complexity,
    Correctness,
    Performance,
}

impl Group {
    pub(crate) fn as_str<'a>(&self) -> &'a str {
        match self {
            Group::Style => "style",
            Group::Suspicious => "suspicious",
            Group::Nursery => "nursery",
            Group::A11y => "a11y",
            Group::Security => "security",
            Group::Complexity => "complexity",
            Group::Correctness => "correctness",
            Group::Performance => "performance",
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
            "a11y" => Group::A11y,
            "security" => Group::Security,
            "complexity" => Group::Complexity,
            "correctness" => Group::Correctness,
            "performance" => Group::Performance,
            _ => return Err(()),
        })
    }
}

pub(crate) struct RuleMover {
    groups: FxHashMap<Group, JsonMember>,
    root: JsonRoot,
    queries: Vec<Query>,
}

pub(crate) struct Query {
    rule_name: String,
    kind: QueryKind,
    rule_member: Option<JsonMember>,
}

enum QueryKind {
    Move(Group, Group),
    Replace(Group),
    Remove(Group),
    Insert(Group),
}

impl RuleMover {
    /// Attempts to find  `linter`, `linter.rules` or `linter.rules.<group>`
    pub(crate) fn from_root(root: JsonRoot) -> Self {
        let events = root.syntax().preorder();
        let mut groups = FxHashMap::default();

        for event in events {
            match event {
                WalkEvent::Enter(node) => {
                    if let Some(member) = JsonMember::cast(node) {
                        let Some(name) =
                            member.name().ok().and_then(|n| n.inner_string_text().ok())
                        else {
                            continue;
                        };

                        if let Ok(group) = Group::from_str(name.text()) {
                            groups.insert(group, member);
                        }
                    }
                }
                WalkEvent::Leave(_) => {}
            }
        }

        Self {
            root,
            groups,
            queries: vec![],
        }
    }

    /// Register a query where it adds a new rule to a group
    ///
    /// ## Panics
    ///
    /// It panics if the group doesn't exist. This usually means that the developer must add the new group
    #[allow(unused)]
    pub(crate) fn insert_rule(
        &mut self,
        rule_name: impl ToString,
        rule_member: JsonMember,
        group: &str,
    ) {
        let group = Group::from_str(group).expect("to be a valid group");

        self.queries.push(Query {
            rule_name: rule_name.to_string(),
            kind: QueryKind::Insert(group),
            rule_member: Some(rule_member),
        })
    }

    /// Register a query where it adds a new rule to a group
    ///
    /// ## Panics
    ///
    /// It panics if the group doesn't exist. This usually means that the developer must add the new group
    #[allow(unused)]
    pub(crate) fn remove_rule(
        &mut self,
        rule_name: impl ToString,
        rule_member: JsonMember,
        group: &str,
    ) {
        let group = Group::from_str(group).expect("to be a valid group");

        self.queries.push(Query {
            rule_name: rule_name.to_string(),
            kind: QueryKind::Remove(group),
            rule_member: Some(rule_member),
        })
    }

    /// Register a query where an existing rule is replaced with a new [JsonMember]
    ///
    /// ## Panics
    ///
    /// It panics if the group doesn't exist. This usually means that the developer must add the new group
    pub(crate) fn replace_rule(
        &mut self,
        rule_name: impl ToString,
        rule_member: JsonMember,
        group: &str,
    ) {
        let group = Group::from_str(group).expect("to be a valid group");

        self.queries.push(Query {
            rule_name: rule_name.to_string(),
            kind: QueryKind::Replace(group),
            rule_member: Some(rule_member),
        })
    }

    /// Register the move of a rule from one group to another
    ///
    /// ## Panics
    ///
    /// It panics if the group doesn't exist. This usually means that the developer must add the new group
    pub(crate) fn move_rule(&mut self, rule_name: &str, from: &str, to: &str) {
        let from_group = Group::from_str(from).expect("to be a valid group");
        let to_group = Group::from_str(to).expect("to be a valid group");

        let mut rule_member = None;
        'outer: for (group, member) in self.groups.iter() {
            if group == &from_group {
                let list = member
                    .value()
                    .ok()
                    .and_then(|n| n.as_json_object_value().cloned())
                    .map(|n| n.json_member_list());
                if let Some(list) = list {
                    for member in list.iter().flatten() {
                        let text = member.name().ok().and_then(|n| n.inner_string_text().ok());

                        if let Some(text) = text {
                            if text.text() == rule_name {
                                rule_member = Some(member);
                                break 'outer;
                            }
                        }
                    }
                }
            }
        }

        self.queries.push(Query {
            rule_name: rule_name.to_string(),
            kind: QueryKind::Move(from_group, to_group),
            rule_member,
        })
    }

    /// Removes a rule from a group, and returns the new member list
    ///
    /// ## Panics
    ///
    /// It panics if the group doesn't exist. This usually means that the developer must add the new group
    fn remove_rule_from_group(
        groups: &mut FxHashMap<Group, JsonMember>,
        rule_name: &str,
        group: &Group,
    ) -> Option<()> {
        if let Some(member) = groups.get_mut(group) {
            let list = member
                .value()
                .ok()?
                .as_json_object_value()?
                .json_member_list();
            let mut new_members = Vec::with_capacity(list.len());
            let mut new_separators = Vec::with_capacity(list.len());

            for item in list.iter() {
                let item = item.ok()?;
                if rule_name != item.name().ok()?.inner_string_text().ok()?.text() {
                    new_members.push(item);
                }
            }

            for index in 0..new_members.len() {
                if index + 1 < new_members.len() {
                    new_separators.push(token(T![,]));
                }
            }

            new_members.shrink_to_fit();
            new_separators.shrink_to_fit();

            *member = group_member(new_members, new_separators, group.as_str());
        } else {
            panic!("The group doesn't exist. This usually means that the developer needs to added to the type.")
        }

        Some(())
    }

    /// It adds a rule to a group
    ///
    /// ## Panics
    ///
    /// It panics if the group doesn't exist. This usually means that the developer must add the new group
    fn add_rule_to_group(
        groups: &mut FxHashMap<Group, JsonMember>,
        rule_member: JsonMember,
        group: &Group,
    ) -> Option<()> {
        if let Some(member) = groups.get_mut(group) {
            let list = member
                .value()
                .ok()?
                .as_json_object_value()?
                .json_member_list();
            let mut new_members = vec![];
            let mut new_separators = vec![];

            for item in list.iter() {
                let item = item.ok()?;
                new_members.push(item);
            }

            for _ in 0..new_members.len() {
                new_separators.push(token(T![,]));
            }

            new_members.push(rule_member);

            *member = group_member(new_members, new_separators, group.as_str());
        } else {
            panic!("The group doesn't exist. This usually means that the developer needs to added to the type.")
        }

        Some(())
    }

    pub(crate) fn run_queries(mut self) -> Option<BatchMutation<JsonLanguage>> {
        let mut mutation = self.root.clone().begin();
        for group in ALL_GROUPS {
            let group_enum = Group::from_str(group).expect("Group to be mapped");
            self.groups
                .entry(group_enum)
                .or_insert_with(|| group_member(vec![], vec![], group));
        }

        let mut groups = self.groups;
        for query in self.queries {
            let Query {
                rule_name,
                rule_member,
                kind,
            } = query;
            let rule_member = if let Some(rule_member) = rule_member {
                rule_member
            } else {
                create_member(
                    rule_name.as_str(),
                    AnyJsonValue::JsonStringValue(json_string_value(json_string_literal("on"))),
                    8,
                )
            };
            match kind {
                QueryKind::Move(from, to) => {
                    RuleMover::remove_rule_from_group(&mut groups, rule_name.as_str(), &from)?;
                    RuleMover::add_rule_to_group(&mut groups, rule_member, &to)?
                }
                QueryKind::Replace(group) => {
                    RuleMover::remove_rule_from_group(&mut groups, rule_name.as_str(), &group)?;
                    RuleMover::add_rule_to_group(&mut groups, rule_member, &group)?
                }
                QueryKind::Remove(group) => {
                    RuleMover::remove_rule_from_group(&mut groups, rule_name.as_str(), &group)?;
                }
                QueryKind::Insert(group) => {
                    RuleMover::add_rule_to_group(&mut groups, rule_member, &group)?
                }
            }
        }

        let mut members = vec![];
        let mut separators = vec![];

        for member in groups.into_values() {
            let list = member
                .value()
                .ok()?
                .as_json_object_value()?
                .json_member_list();
            if !list.is_empty() {
                members.push(member);
            }
        }
        for _ in 0..members.len() - 1 {
            separators.push(token(T![,]))
        }

        let new_linter_member = create_new_linter_member(members, separators);

        let list = self
            .root
            .value()
            .ok()?
            .as_json_object_value()?
            .json_member_list();

        let mut members: Vec<_> = list
            .iter()
            .filter_map(|el| {
                let el = el.ok()?;
                if el.name().ok()?.inner_string_text().ok()?.text() == "linter" {
                    None
                } else {
                    Some(el)
                }
            })
            .collect();
        let mut separators: Vec<_> = list.separators().filter_map(|el| el.ok()).collect();

        members.push(new_linter_member);
        if members.len() > 1 {
            separators.push(token(T![,]));
        }
        mutation.replace_node(list, json_member_list(members, separators));

        Some(mutation)
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

fn group_member(
    members: Vec<JsonMember>,
    separators: Vec<JsonSyntaxToken>,
    group_name: &str,
) -> JsonMember {
    let list = json_member_list(members, separators);
    let object = create_object(list, 8);
    create_member(group_name, AnyJsonValue::JsonObjectValue(object), 6)
}

fn rules_member(
    members: Vec<JsonMember>,
    separators: Vec<JsonSyntaxToken>,
    indentation: usize,
) -> JsonMember {
    let list = json_member_list(members, separators);
    let object = create_object(list, indentation);
    create_member("rules", AnyJsonValue::JsonObjectValue(object), 4)
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
) -> JsonMember {
    let rules = rules_member(members, separators, 4);
    linter_member(vec![rules], vec![], 2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_diagnostics::Error;
    use biome_json_formatter::context::JsonFormatOptions;
    use biome_json_formatter::format_node;
    use biome_json_parser::{parse_json, JsonParse, JsonParserOptions};
    use biome_test_utils::diagnostic_to_string;
    use insta::assert_snapshot;

    fn assert_snapshot(source: JsonParse, mutation: BatchMutation<JsonLanguage>, name: &str) {
        let mut buffer = String::new();

        let result = format_node(JsonFormatOptions::default(), &source.syntax())
            .expect("Should be able to format")
            .print()
            .expect("Should be able to format");

        buffer.push_str("## Source\n\n");
        buffer.push_str("```json\n");
        buffer.push_str(result.as_code());
        buffer.push_str("\n```\n\n");

        buffer.push_str("## Result\n\n");
        buffer.push_str("```\n");
        let new_syntax_node = mutation.commit();
        let result = format_node(JsonFormatOptions::default(), &new_syntax_node)
            .expect("Should be able to format")
            .print()
            .expect("Should be able to format");

        buffer.push_str(result.as_code());
        buffer.push_str("\n```");

        assert_snapshot!(name, buffer);
    }

    #[test]
    fn move_rule_to_new_group() {
        let source = r#"
{
    "linter": {
        "rules": {
            "style": {
                "noVar": "error"
            }
        }
    }
}
        "#;
        let parsed = parse_json(source, JsonParserOptions::default());
        if parsed.has_errors() {
            for diagnostic in parsed.into_diagnostics() {
                let error = diagnostic_to_string("file.json", source, Error::from(diagnostic));
                eprintln!("{:?}", error);
            }
            panic!("Source has errors");
        }
        let root = parsed.tree();
        let mut rule_mover = RuleMover::from_root(root);
        rule_mover.move_rule("noVar", "style", "suspicious");

        let mutation = rule_mover.run_queries().expect("To run queries");

        assert_snapshot(parsed, mutation, "move_rule_to_new_group");
    }

    #[test]
    fn move_rule_with_existing_rules() {
        let source = r#"
{
    "linter": {
        "rules": {
            "style": {
                "noVar": "error",
                "noArguments": "error"
            }
        }
    }
}
        "#;
        let parsed = parse_json(source, JsonParserOptions::default());
        if parsed.has_errors() {
            for diagnostic in parsed.into_diagnostics() {
                let error = diagnostic_to_string("file.json", source, Error::from(diagnostic));
                eprintln!("{:?}", error);
            }
            panic!("Source has errors");
        }
        let root = parsed.tree();
        let mut rule_mover = RuleMover::from_root(root);
        rule_mover.move_rule("noVar", "style", "suspicious");

        let mutation = rule_mover.run_queries().expect("To run queries");

        assert_snapshot(parsed, mutation, "move_rule_with_existing_rules");
    }
}
