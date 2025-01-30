//! Utilities to extract known information from JSON AST for the `biome.json` configuration

use biome_json_factory::make::{
    json_member, json_member_list, json_member_name, json_object_value, json_string_literal, token,
};
use biome_json_syntax::{AnyJsonValue, JsonMember, JsonMemberList, JsonRoot, JsonSyntaxToken, T};
use biome_rowan::{AstNode, AstSeparatedList, TokenText, TriviaPieceKind};

/// If the member belongs to a rule, it returns its severity as a [TokenText]. Returns [None] otherwise.
pub fn get_severity_from_member(member: &JsonMember) -> Option<TokenText> {
    let value = member.value().ok()?;
    match value {
        // This matches
        AnyJsonValue::JsonObjectValue(object) => {
            for member in object.json_member_list().iter().flatten() {
                let name = member.name().ok()?.inner_string_text().ok()?;
                if name.text() == "severity" {
                    let value = member.value().ok()?;
                    let value = value.as_json_string_value()?;
                    return value.inner_string_text().ok();
                }
            }
        }
        AnyJsonValue::JsonStringValue(string_literal) => {
            return string_literal.inner_string_text().ok()
        }

        _ => return None,
    }

    None
}

pub fn traverse_linter_from_root(root: &JsonRoot, group: &str) -> Option<TraverseResult> {
    Some(match seek_linter_configuration_downwards(root) {
        None => return None,
        Some(linter) => {
            let rules_member = seek_rules_configuration(&linter);

            match rules_member {
                Some(member_found) => {
                    let group_member = seek_group_configuration(&member_found, group);
                    if let Some(style_member) = group_member {
                        TraverseResult::Group(style_member)
                    } else {
                        TraverseResult::Rules(member_found)
                    }
                }
                None => TraverseResult::Linter(linter),
            }
        }
    })
}

/// Finds the `linter` member from the root
pub fn seek_linter_configuration_downwards(root: &JsonRoot) -> Option<JsonMember> {
    let root = root.value().ok()?;
    let root = root.as_json_object_value()?;

    root.json_member_list().iter().flatten().find_map(|member| {
        if member.name().ok()?.inner_string_text().ok()?.text() == "linter" {
            Some(member)
        } else {
            None
        }
    })
}

/// Finds the `rules` member
pub fn seek_rules_configuration(member: &JsonMember) -> Option<JsonMember> {
    member
        .value()
        .ok()?
        .as_json_object_value()?
        .json_member_list()
        .iter()
        .flatten()
        .find_map(|member| {
            if member.name().ok()?.inner_string_text().ok()?.text() == "rules" {
                Some(member)
            } else {
                None
            }
        })
}

// Finds the group member
pub fn seek_group_configuration(member: &JsonMember, group: &str) -> Option<JsonMember> {
    member
        .value()
        .ok()?
        .as_json_object_value()?
        .json_member_list()
        .iter()
        .flatten()
        .find_map(|member| {
            if member.name().ok()?.inner_string_text().ok()?.text() == group {
                Some(member)
            } else {
                None
            }
        })
}

pub fn replace_value_to_member(member: &JsonMember, new_value: AnyJsonValue) -> Option<JsonMember> {
    let value = member.value().ok()?;
    Some(json_member(
        member.name().ok()?,
        member.colon_token().ok()?,
        new_value
            .with_leading_trivia_pieces(value.syntax().first_leading_trivia()?.pieces())?
            .with_trailing_trivia_pieces(value.syntax().last_trailing_trivia()?.pieces())?,
    ))
}

#[derive(Debug)]
pub enum TraverseResult {
    /// The top-level `linter` member
    /// ```json
    /// {
    ///     "linter" :{}
    /// }
    /// ```
    Linter(JsonMember),
    /// The `rules` member
    /// ```json
    /// {
    ///     "linter" :{
    ///         "rules": {}
    ///     }
    /// }
    /// ```
    Rules(JsonMember),
    /// The group member, e.g. `"styles"`
    /// ```json
    /// {
    ///     "linter" :{
    ///         "rules": {
    ///             "styles": {}
    ///         }
    ///     }
    /// }
    /// ```
    Group(JsonMember),
}

pub fn create_formatted_object_value(list: Vec<JsonMember>, spaces: usize) -> AnyJsonValue {
    let separators = create_separator(list.as_slice());

    create_object(json_member_list(list, separators), spaces)
}

/// Given a list of members, it returns a list of separators to use when creating a [JsonMemberList]
pub fn create_separator(list: &[JsonMember]) -> Vec<JsonSyntaxToken> {
    let mut separators = vec![];
    for _ in 0..list.len().saturating_sub(1) {
        separators.push(token(T![,]));
    }

    separators
}

/// Creates a formatted "linter" member
pub fn create_linter_member(list: Vec<JsonMember>) -> JsonMember {
    let separators = create_separator(list.as_slice());
    let list = json_member_list(list, separators);

    create_member("linter", create_object(list, 2), 2)
}

/// Creates a formatted "linter -> rules" member
pub fn create_rules_member(list: Vec<JsonMember>) -> JsonMember {
    let separators = create_separator(list.as_slice());
    let list = json_member_list(list, separators);

    create_member("rules", create_object(list, 4), 4)
}

/// Creates a formatted "linter -> rules -> <group>" member
pub fn create_group_member(group_name: &str, list: Vec<JsonMember>) -> JsonMember {
    let separators = create_separator(list.as_slice());
    let list = json_member_list(list, separators);
    create_member(group_name, create_object(list, 6), 6)
}

/// Creates a [JsonMember] wit the formatted name, and given value
pub fn create_member(text: &str, value: AnyJsonValue, spaces: usize) -> JsonMember {
    json_member(
        json_member_name(json_string_literal(text).with_leading_trivia(vec![
            (TriviaPieceKind::Newline, "\n"),
            (TriviaPieceKind::Whitespace, " ".repeat(spaces).as_str()),
        ])),
        token(T![:]),
        value,
    )
}

pub fn create_object(list: JsonMemberList, spaces: usize) -> AnyJsonValue {
    AnyJsonValue::JsonObjectValue(json_object_value(
        token(T!['{']).with_leading_trivia(vec![(TriviaPieceKind::Whitespace, " ")]),
        list,
        token(T!['}']).with_leading_trivia(vec![
            (TriviaPieceKind::Newline, "\n"),
            (TriviaPieceKind::Whitespace, " ".repeat(spaces).as_str()),
        ]),
    ))
}
// #endregion
