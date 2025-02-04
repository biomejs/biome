use biome_json_factory::make::{
    json_member, json_member_list, json_member_name, json_object_value, json_string_literal,
    json_string_value, token,
};
use biome_json_syntax::{
    AnyJsonValue, JsonLanguage, JsonMember, JsonMemberList, JsonObjectValue, JsonRoot,
    JsonSyntaxToken, T,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutation, BatchMutationExt, TriviaPieceKind};
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
    "source",
];

#[derive(Debug, Eq, Hash, PartialEq)]
pub(crate) enum Category {
    Linter(LinterGroup),
    Assist(AssistGroup),
}

impl FromStr for Category {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(group) = LinterGroup::from_str(s) {
            Ok(Category::Linter(group))
        } else if let Ok(group) = AssistGroup::from_str(s) {
            Ok(Category::Assist(group))
        } else {
            Err(())
        }
    }
}

impl Category {
    pub(crate) fn as_str<'a>(&self) -> &'a str {
        match self {
            Category::Linter(g) => g.as_str(),
            Category::Assist(g) => g.as_str(),
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub(crate) enum LinterGroup {
    Style,
    Suspicious,
    Nursery,
    A11y,
    Security,
    Complexity,
    Correctness,
    Performance,
}

impl LinterGroup {
    pub(crate) fn as_str<'a>(&self) -> &'a str {
        match self {
            LinterGroup::Style => "style",
            LinterGroup::Suspicious => "suspicious",
            LinterGroup::Nursery => "nursery",
            LinterGroup::A11y => "a11y",
            LinterGroup::Security => "security",
            LinterGroup::Complexity => "complexity",
            LinterGroup::Correctness => "correctness",
            LinterGroup::Performance => "performance",
        }
    }
}

impl FromStr for LinterGroup {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "style" => LinterGroup::Style,
            "suspicious" => LinterGroup::Suspicious,
            "nursery" => LinterGroup::Nursery,
            "a11y" => LinterGroup::A11y,
            "security" => LinterGroup::Security,
            "complexity" => LinterGroup::Complexity,
            "correctness" => LinterGroup::Correctness,
            "performance" => LinterGroup::Performance,
            _ => return Err(()),
        })
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub(crate) enum AssistGroup {
    Source,
}

impl AssistGroup {
    pub(crate) fn as_str<'a>(&self) -> &'a str {
        match self {
            AssistGroup::Source => "source",
        }
    }
}

impl FromStr for AssistGroup {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "source" => AssistGroup::Source,
            _ => return Err(()),
        })
    }
}

pub(crate) struct AnalyzerMover {
    groups: FxHashMap<Category, JsonMember>,
    root: JsonRoot,
    queries: Vec<Query>,
    linter_member: Option<JsonMember>,
    assist_member: Option<JsonMember>,
}

pub(crate) struct Query {
    rule_name: String,
    kind: QueryKind,
    rule_member: Option<JsonMember>,
}

#[derive(Debug)]
enum QueryKind {
    Move(Category, Category),
    Replace(Category),
    Remove(Category),
}

impl AnalyzerMover {
    /// Attempts to find  `linter`, `linter.rules` or `linter.rules.<group>`
    pub(crate) fn from_root(root: JsonRoot) -> Self {
        let mut groups = FxHashMap::default();
        let mut linter_member = None;
        let mut assist_member = None;

        let object = root
            .value()
            .ok()
            .and_then(|n| n.as_json_object_value().cloned());

        if let Some(object) = object {
            for member in object.json_member_list().iter().flatten() {
                let Some(name) = member.name().ok().and_then(|n| n.inner_string_text().ok()) else {
                    continue;
                };

                if name.text() == "linter" {
                    linter_member = Some(member);
                } else if name.text() == "assist" {
                    assist_member = Some(member);
                }
            }
        };

        if let Some(member) = &linter_member {
            Self::track_groups(member, &mut groups, "rules");
        }

        if let Some(member) = &assist_member {
            Self::track_groups(member, &mut groups, "actions");
        }

        Self {
            root,
            groups,
            queries: vec![],
            assist_member,
            linter_member,
        }
    }

    fn track_groups(
        linter_member: &JsonMember,
        groups: &mut FxHashMap<Category, JsonMember>,
        parent_group_name: &str,
    ) -> Option<()> {
        let rules_list = linter_member
            .value()
            .ok()
            .and_then(|n| n.as_json_object_value().cloned())?
            .json_member_list()
            .iter()
            .flatten()
            .find_map(|member| {
                let name = member
                    .name()
                    .ok()
                    .and_then(|n| n.inner_string_text().ok())?;
                if name.text() == parent_group_name {
                    Some(member)
                } else {
                    None
                }
            })
            .and_then(|member| {
                Some(
                    member
                        .value()
                        .ok()?
                        .as_json_object_value()?
                        .json_member_list(),
                )
            })?;

        for item in rules_list.iter().flatten() {
            let name = item.name().ok().and_then(|n| n.inner_string_text().ok())?;
            if let Ok(group) = Category::from_str(name.text()) {
                groups.insert(group, item);
            }
        }

        Some(())
    }

    /// Register a query where it adds a new rule to a group
    ///
    /// ## Panics
    ///
    /// It panics if the group doesn't exist. This usually means that the developer must add the new group
    #[expect(unused)]
    pub(crate) fn remove_rule(
        &mut self,
        rule_name: impl ToString,
        rule_member: JsonMember,
        group: &str,
    ) {
        let category = Category::from_str(group).expect("to be a valid group");

        self.queries.push(Query {
            rule_name: rule_name.to_string(),
            kind: QueryKind::Remove(category),
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
        let category = Category::from_str(group).expect("to be a valid group");

        self.queries.push(Query {
            rule_name: rule_name.to_string(),
            kind: QueryKind::Replace(category),
            rule_member: Some(rule_member),
        })
    }

    /// Register the move of a rule from one group to another
    ///
    /// ## Panics
    ///
    /// It panics if the group doesn't exist. This usually means that the developer must add the new group
    pub(crate) fn move_rule(&mut self, rule_name: &str, from: &str, to: &str) {
        let from_group = Category::from_str(from).expect("to be a valid group");
        let to_group = Category::from_str(to).expect("to be a valid group");

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

    fn replace_rule_in_group(
        groups: &mut FxHashMap<Category, JsonMember>,
        rule_name: &str,
        rule_member: JsonMember,
        group: &Category,
    ) -> Option<()> {
        if let Some(member) = groups.get_mut(group) {
            let list = member
                .value()
                .ok()?
                .as_json_object_value()?
                .json_member_list();

            let mut new_members = Vec::with_capacity(list.len());
            let mut new_separators = Vec::with_capacity(list.len());
            let mut rule_member_added = false;
            for member in list.iter().flatten() {
                if rule_name == member.name().ok()?.inner_string_text().ok()?.text() {
                    new_members.push(rule_member.clone());
                    rule_member_added = true;
                } else {
                    new_members.push(member);
                }
            }

            if !rule_member_added {
                new_members.push(rule_member);
            }
            for _ in 0..new_members.len().saturating_sub(1) {
                new_separators.push(token(T![,]));
            }

            *member = group_member(new_members, new_separators, group.as_str());
        }

        Some(())
    }

    /// Removes a rule from a group, and returns the new member list
    ///
    /// ## Panics
    ///
    /// It panics if the group doesn't exist. This usually means that the developer must add the new group
    fn remove_rule_from_group(
        groups: &mut FxHashMap<Category, JsonMember>,
        rule_name: &str,
        group: &Category,
    ) -> Option<()> {
        if let Some(member) = groups.get_mut(group) {
            let list = member
                .value()
                .ok()?
                .as_json_object_value()?
                .json_member_list();
            let mut new_members = Vec::with_capacity(list.len());
            let mut new_separators = Vec::with_capacity(list.len());

            for item in list.iter().flatten() {
                if rule_name != item.name().ok()?.inner_string_text().ok()?.text() {
                    new_members.push(item);
                }
            }

            for _ in 0..new_members.len().saturating_sub(1) {
                new_separators.push(token(T![,]));
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
        groups: &mut FxHashMap<Category, JsonMember>,
        rule_member: JsonMember,
        group: &Category,
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
            let group_enum = Category::from_str(group).expect("Group to be mapped");
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
                    AnalyzerMover::remove_rule_from_group(&mut groups, rule_name.as_str(), &from)?;
                    AnalyzerMover::add_rule_to_group(&mut groups, rule_member, &to)?
                }
                QueryKind::Replace(group) => {
                    AnalyzerMover::replace_rule_in_group(
                        &mut groups,
                        rule_name.as_str(),
                        rule_member,
                        &group,
                    )?;
                }
                QueryKind::Remove(group) => {
                    AnalyzerMover::remove_rule_from_group(&mut groups, rule_name.as_str(), &group)?;
                }
            }
        }

        let mut linter_members = vec![];
        let mut linter_separators = vec![];

        let mut assist_members = vec![];
        let mut assist_separators = vec![];

        for (category, member) in groups {
            let list = member
                .value()
                .ok()?
                .as_json_object_value()?
                .json_member_list();
            if !list.is_empty() {
                match category {
                    Category::Linter(_) => {
                        linter_members.push(member);
                    }
                    Category::Assist(_) => {
                        assist_members.push(member);
                    }
                }
            }
        }

        if linter_members.len() > 1 {
            for _ in 0..linter_members.len().saturating_sub(1) {
                linter_separators.push(token(T![,]));
            }
        }

        if assist_members.len() > 1 {
            for _ in 0..assist_members.len().saturating_sub(1) {
                assist_separators.push(token(T![,]));
            }
        }

        let list = self
            .root
            .value()
            .ok()?
            .as_json_object_value()?
            .json_member_list();

        let mut separators = vec![];
        let mut top_level_members = vec![];

        // AKAK: linter: {}
        let linter_member =
            self.linter_member
                .unwrap_or(create_new_linter_member(vec![], vec![], 4));

        // AKAK: assist: {}
        let assist_memebr = self
            .assist_member
            .unwrap_or(create_new_assist_member(vec![], vec![]));

        let mut linter_member_added = false;
        let mut assist_member_added = false;

        for top_level_member in list.iter().flatten() {
            let name = top_level_member
                .name()
                .ok()
                .and_then(|n| n.inner_string_text().ok());

            if let Some(name) = name {
                if name.text() == "linter" {
                    if linter_members.is_empty() {
                        top_level_members.push(top_level_member.clone());
                    } else {
                        let new_linter_member = add_members_to_linter_member(
                            &linter_member,
                            linter_members.clone(),
                            linter_separators.clone(),
                        );
                        top_level_members.push(new_linter_member);
                    }
                    linter_member_added = true;
                } else if name.text() == "assist" {
                    if assist_members.is_empty() {
                        top_level_members.push(top_level_member.clone());
                    } else {
                        let new_assist_member = add_members_to_assist_member(
                            &assist_memebr,
                            assist_members.clone(),
                            assist_separators.clone(),
                        );
                        top_level_members.push(new_assist_member);
                    }
                    assist_member_added = true;
                } else {
                    top_level_members.push(top_level_member.clone());
                }
            }
        }

        if !assist_member_added && !assist_members.is_empty() {
            top_level_members.push(add_members_to_assist_member(
                &assist_memebr,
                assist_members.clone(),
                assist_separators.clone(),
            ));
        }

        if !linter_member_added && !linter_members.is_empty() {
            top_level_members.push(add_members_to_linter_member(
                &linter_member,
                linter_members.clone(),
                linter_separators.clone(),
            ));
        }

        for _ in 0..top_level_members.len().saturating_sub(1) {
            separators.push(token(T![,]));
        }

        mutation.replace_node(list, json_member_list(top_level_members, separators));

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
    let object = create_object(list, 6);
    create_member(group_name, AnyJsonValue::JsonObjectValue(object), 6)
}

fn rules_member(
    members: Vec<JsonMember>,
    separators: Vec<JsonSyntaxToken>,
    white_space: usize,
) -> JsonMember {
    let list = json_member_list(members, separators);
    let object = create_object(list, white_space);
    create_member("rules", AnyJsonValue::JsonObjectValue(object), white_space)
}

fn actions_member(members: Vec<JsonMember>, separators: Vec<JsonSyntaxToken>) -> JsonMember {
    let list = json_member_list(members, separators);
    let object = create_object(list, 4);
    create_member("actions", AnyJsonValue::JsonObjectValue(object), 4)
}

fn linter_member(
    members: Vec<JsonMember>,
    separators: Vec<JsonSyntaxToken>,
    white_space: usize,
) -> JsonMember {
    let list = json_member_list(members, separators);
    let object = create_object(list, white_space);
    create_member("linter", AnyJsonValue::JsonObjectValue(object), white_space)
}

fn assist_member(members: Vec<JsonMember>, separators: Vec<JsonSyntaxToken>) -> JsonMember {
    let list = json_member_list(members, separators);
    let object = create_object(list, 2);
    create_member("assist", AnyJsonValue::JsonObjectValue(object), 2)
}

fn create_new_linter_member(
    members: Vec<JsonMember>,
    separators: Vec<JsonSyntaxToken>,
    white_space: usize,
) -> JsonMember {
    let rules = rules_member(members, separators, white_space + 2);
    linter_member(vec![rules], vec![], white_space)
}

fn add_members_to_linter_member(
    linter_member: &JsonMember,
    members: Vec<JsonMember>,
    separators: Vec<JsonSyntaxToken>,
) -> JsonMember {
    let mut new_list = vec![];
    let mut new_separators = vec![];
    let mut has_rules_member = false;

    let rules_member_indentation = linter_member
        .syntax()
        .first_token()
        .map(|token| token.leading_trivia().pieces())
        .and_then(|mut pieces| pieces.find(|piece| piece.kind() == TriviaPieceKind::Whitespace))
        .map_or(4, |piece| piece.text().len() + 2);

    let list = linter_member
        .value()
        .ok()
        .and_then(|value| value.as_json_object_value().cloned())
        .map(|o| o.json_member_list());
    if let Some(list) = list {
        for item in list.iter().flatten() {
            let name = item.name().ok().and_then(|n| n.inner_string_text().ok());
            if !matches!(name.as_deref(), Some("rules")) {
                new_list.push(item);
            } else {
                let rules_members = rules_member(
                    members.clone(),
                    separators.clone(),
                    rules_member_indentation,
                );
                new_list.push(rules_members);
                has_rules_member = true;
            }
        }
    }

    if !has_rules_member {
        let rules_members = rules_member(
            members.clone(),
            separators.clone(),
            rules_member_indentation,
        );
        new_list.push(rules_members);
    }

    for _ in 0..new_list.len().saturating_sub(1) {
        new_separators.push(token(T![,]));
    }

    create_member(
        "linter",
        AnyJsonValue::JsonObjectValue(create_object(json_member_list(new_list, new_separators), 2)),
        2,
    )
}

fn add_members_to_assist_member(
    assist_member: &JsonMember,
    members: Vec<JsonMember>,
    separators: Vec<JsonSyntaxToken>,
) -> JsonMember {
    let mut new_list = vec![];
    let mut new_separators = vec![];
    let mut has_actions_member = false;
    let list = assist_member
        .value()
        .ok()
        .and_then(|value| value.as_json_object_value().cloned())
        .map(|o| o.json_member_list());
    if let Some(list) = list {
        for item in list.iter().flatten() {
            let name = item.name().ok().and_then(|n| n.inner_string_text().ok());
            if !matches!(name.as_deref(), Some("actions")) {
                new_list.push(item);
            } else {
                let actions_member = actions_member(members.clone(), separators.clone());
                new_list.push(actions_member);
                has_actions_member = true;
            }
        }
    }

    if !has_actions_member {
        let actions_member = actions_member(members.clone(), separators.clone());
        new_list.push(actions_member);
    }

    for _ in 0..new_list.len().saturating_sub(1) {
        new_separators.push(token(T![,]));
    }

    create_member(
        "assist",
        AnyJsonValue::JsonObjectValue(create_object(json_member_list(new_list, new_separators), 2)),
        2,
    )
}

fn create_new_assist_member(
    members: Vec<JsonMember>,
    separators: Vec<JsonSyntaxToken>,
) -> JsonMember {
    let actions = actions_member(members, separators);
    assist_member(vec![actions], vec![])
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

        eprintln!("{}", result.as_code());

        buffer.push_str(result.as_code());
        buffer.push_str("\n```");

        assert_snapshot!(name, buffer);
    }

    #[test]
    fn move_rule_to_new_group() {
        let source = r#"
{
    "linter": {
        "includes": [""],
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
        let mut rule_mover = AnalyzerMover::from_root(root);
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
        let mut rule_mover = AnalyzerMover::from_root(root);
        rule_mover.move_rule("noVar", "style", "suspicious");

        let mutation = rule_mover.run_queries().expect("To run queries");

        assert_snapshot(parsed, mutation, "move_rule_with_existing_rules");
    }
}
