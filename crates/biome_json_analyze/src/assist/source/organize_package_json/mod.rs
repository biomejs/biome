use crate::JsonRuleAction;
use crate::utils::is_package_json;
use biome_analyze::{
    Ast, FixKind, Rule, RuleAction, RuleDiagnostic, context::RuleContext, declare_source_rule,
};
use biome_console::markup;
use biome_diagnostics::category;
use biome_json_factory::make;
use biome_json_syntax::{AnyJsonValue, JsonMember, JsonMemberList, JsonObjectValue, JsonRoot, T};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt};
use std::collections::HashMap;

mod field_order;
mod sort_dependencies;
mod sorters;

use field_order::{FieldTransformer, get_field_index, get_field_transformer};

declare_source_rule! {
    /// Organize package.json fields according to established conventions.
    ///
    /// Sorts fields following the same conventions as the popular
    /// [sort-package-json](https://github.com/keithamus/sort-package-json) tool.
    ///
    /// Field organization:
    /// - 110 predefined fields in conventional order
    /// - Unknown fields alphabetically after
    /// - Private fields (starting with `_`) alphabetically at end
    ///
    /// See https://github.com/keithamus/sort-package-json/blob/main/defaultRules.md
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```json,expect_diagnostic
    /// {
    ///   "dependencies": {
    ///     "lodash": "^4.0.0"
    ///   },
    ///   "name": "my-package",
    ///   "version": "1.0.0"
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```json
    /// {
    ///   "name": "my-package",
    ///   "version": "1.0.0",
    ///   "dependencies": {
    ///     "lodash": "^4.0.0"
    ///   }
    /// }
    /// ```
    pub OrganizePackageJson {
        version: "next",
        name: "organizePackageJson",
        language: "json",
        fix_kind: FixKind::Safe,
    }
}

impl Rule for OrganizePackageJson {
    type Query = Ast<JsonRoot>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let path = ctx.file_path();
        if !is_package_json(path) {
            return None;
        }

        let root = ctx.query();
        let value = root.value().ok()?;
        let object = value.as_json_object_value()?;
        let members = object.json_member_list();

        if is_organized(&members) {
            return None;
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let root = ctx.query();
        let value = root.value().ok()?;
        let object = value.as_json_object_value()?;

        Some(RuleDiagnostic::new(
            category!("assist/source/organizePackageJson"),
            object.range(),
            markup! {
                "package.json fields can be organized."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsonRuleAction> {
        let root = ctx.query();
        let value = root.value().ok()?;
        let object = value.as_json_object_value()?;
        let members = object.json_member_list();

        let organized_members = organize_members(&members, object)?;

        let mut mutation = ctx.root().begin();
        mutation.replace_node_discard_trivia(members, organized_members);

        Some(RuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
                "Organize package.json fields."
            },
            mutation,
        ))
    }
}

fn is_organized(members: &JsonMemberList) -> bool {
    let field_names: Vec<String> = members
        .iter()
        .filter_map(|member| {
            member
                .ok()?
                .name()
                .ok()?
                .inner_string_text()
                .ok()
                .map(|text| text.text().to_string())
        })
        .collect();

    if field_names.is_empty() {
        return true;
    }

    let sorted_names = get_sorted_field_order(&field_names);
    if field_names != sorted_names {
        return false;
    }

    for member in members.iter().filter_map(|m| m.ok()) {
        if let Ok(name) = member.name().and_then(|n| n.inner_string_text()) {
            let field_name = name.text();
            let transformer = get_field_transformer(field_name);

            if transformer != FieldTransformer::None {
                if needs_transformation(&member, transformer) {
                    return false;
                }
            }
        }
    }

    true
}

fn needs_transformation(member: &JsonMember, transformer: FieldTransformer) -> bool {
    let Ok(value) = member.value() else {
        return false;
    };

    match transformer {
        FieldTransformer::None => false,

        FieldTransformer::SortObject
        | FieldTransformer::SortPeopleObject
        | FieldTransformer::SortURLObject
        | FieldTransformer::SortBugsObject
        | FieldTransformer::SortDirectories
        | FieldTransformer::SortVolta
        | FieldTransformer::SortBinary
        | FieldTransformer::SortGitHooks
        | FieldTransformer::SortVSCodeBadgeObject => {
            if let Some(obj) = value.as_json_object_value() {
                let members = obj.json_member_list();
                let current_keys: Vec<String> = members
                    .iter()
                    .filter_map(|m| {
                        m.ok()?
                            .name()
                            .ok()?
                            .inner_string_text()
                            .ok()
                            .map(|t| t.text().to_string())
                    })
                    .collect();

                let sorted_keys = get_expected_sorted_keys(&current_keys, transformer);
                current_keys != sorted_keys
            } else {
                false
            }
        }

        FieldTransformer::SortDependencies => false,

        FieldTransformer::UniqArray | FieldTransformer::UniqAndSortArray => false,
    }
}

fn get_expected_sorted_keys(keys: &[String], transformer: FieldTransformer) -> Vec<String> {
    let mut sorted = keys.to_vec();

    match transformer {
        FieldTransformer::SortObject => {
            sorted.sort();
        }
        FieldTransformer::SortPeopleObject => {
            sort_by_key_order(&mut sorted, &["name", "email", "url"]);
        }
        FieldTransformer::SortURLObject => {
            sort_by_key_order(&mut sorted, &["type", "url"]);
        }
        FieldTransformer::SortBugsObject => {
            sort_by_key_order(&mut sorted, &["url", "email"]);
        }
        FieldTransformer::SortDirectories => {
            sort_by_key_order(
                &mut sorted,
                &["lib", "bin", "man", "doc", "example", "test"],
            );
        }
        FieldTransformer::SortVolta => {
            sort_by_key_order(&mut sorted, &["node", "npm", "yarn"]);
        }
        FieldTransformer::SortBinary => {
            sort_by_key_order(
                &mut sorted,
                &[
                    "module_name",
                    "module_path",
                    "remote_path",
                    "package_name",
                    "host",
                ],
            );
        }
        FieldTransformer::SortGitHooks => {
            const GIT_HOOKS_ORDER: &[&str] = &[
                "applypatch-msg",
                "pre-applypatch",
                "post-applypatch",
                "pre-commit",
                "pre-merge-commit",
                "prepare-commit-msg",
                "commit-msg",
                "post-commit",
                "pre-rebase",
                "post-checkout",
                "post-merge",
                "pre-push",
                "pre-receive",
                "update",
                "proc-receive",
                "post-receive",
                "post-update",
                "reference-transaction",
                "push-to-checkout",
                "pre-auto-gc",
                "post-rewrite",
                "sendemail-validate",
                "fsmonitor-watchman",
                "p4-changelist",
                "p4-prepare-changelist",
                "p4-post-changelist",
                "p4-pre-submit",
                "post-index-change",
            ];
            sort_by_key_order(&mut sorted, GIT_HOOKS_ORDER);
        }
        FieldTransformer::SortDependencies => {
            sorted.sort();
        }
        _ => {}
    }

    sorted
}

fn sort_by_key_order(keys: &mut Vec<String>, order: &[&str]) {
    keys.sort_by(|a, b| {
        let a_idx = order.iter().position(|&k| k == a);
        let b_idx = order.iter().position(|&k| k == b);

        match (a_idx, b_idx) {
            (Some(a_i), Some(b_i)) => a_i.cmp(&b_i),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => a.cmp(b),
        }
    });
}

fn organize_members(
    members: &JsonMemberList,
    root_object: &JsonObjectValue,
) -> Option<JsonMemberList> {
    let member_map: HashMap<String, JsonMember> = members
        .iter()
        .filter_map(|member| {
            let member = member.ok()?;
            let name = member
                .name()
                .ok()?
                .inner_string_text()
                .ok()?
                .text()
                .to_string();
            Some((name, member))
        })
        .collect();

    if member_map.is_empty() {
        return None;
    }

    let field_names: Vec<String> = member_map.keys().cloned().collect();
    let sorted_names = get_sorted_field_order(&field_names);

    let mut elements = Vec::new();
    let mut separators = Vec::new();

    for (i, field_name) in sorted_names.iter().enumerate() {
        if let Some(member) = member_map.get(field_name) {
            let transformed_member = apply_field_transformer(member, field_name, root_object);
            elements.push(transformed_member);

            if i < sorted_names.len() - 1 {
                separators.push(make::token(T![,]));
            }
        }
    }

    Some(make::json_member_list(elements, separators))
}

fn apply_field_transformer(
    member: &JsonMember,
    field_name: &str,
    root_object: &JsonObjectValue,
) -> JsonMember {
    let transformer = get_field_transformer(field_name);

    if transformer == FieldTransformer::None {
        return member.clone();
    }

    let Ok(value) = member.value() else {
        return member.clone();
    };

    let transformed_value = match transformer {
        FieldTransformer::None => return member.clone(),

        FieldTransformer::SortDependencies => {
            sort_dependencies::transform(&value, root_object).unwrap_or_else(|| value.clone())
        }

        FieldTransformer::SortObject => {
            if let Some(obj) = value.as_json_object_value() {
                sorters::sort_alphabetically(obj)
                    .map(|sorted| AnyJsonValue::from(sorted))
                    .unwrap_or_else(|| value.clone())
            } else {
                value.clone()
            }
        }

        FieldTransformer::SortPeopleObject => {
            if let Some(obj) = value.as_json_object_value() {
                sorters::sort_people_object(obj)
                    .map(|sorted| AnyJsonValue::from(sorted))
                    .unwrap_or_else(|| value.clone())
            } else {
                value.clone()
            }
        }

        FieldTransformer::SortURLObject => {
            if let Some(obj) = value.as_json_object_value() {
                sorters::sort_url_object(obj)
                    .map(|sorted| AnyJsonValue::from(sorted))
                    .unwrap_or_else(|| value.clone())
            } else {
                value.clone()
            }
        }

        FieldTransformer::SortBugsObject => {
            if let Some(obj) = value.as_json_object_value() {
                sorters::sort_bugs_object(obj)
                    .map(|sorted| AnyJsonValue::from(sorted))
                    .unwrap_or_else(|| value.clone())
            } else {
                value.clone()
            }
        }

        FieldTransformer::SortDirectories => {
            if let Some(obj) = value.as_json_object_value() {
                sorters::sort_directories(obj)
                    .map(|sorted| AnyJsonValue::from(sorted))
                    .unwrap_or_else(|| value.clone())
            } else {
                value.clone()
            }
        }

        FieldTransformer::SortVolta => {
            if let Some(obj) = value.as_json_object_value() {
                sorters::sort_volta(obj)
                    .map(|sorted| AnyJsonValue::from(sorted))
                    .unwrap_or_else(|| value.clone())
            } else {
                value.clone()
            }
        }

        FieldTransformer::SortBinary => {
            if let Some(obj) = value.as_json_object_value() {
                sorters::sort_binary(obj)
                    .map(|sorted| AnyJsonValue::from(sorted))
                    .unwrap_or_else(|| value.clone())
            } else {
                value.clone()
            }
        }

        FieldTransformer::SortGitHooks => {
            if let Some(obj) = value.as_json_object_value() {
                sorters::sort_git_hooks(obj)
                    .map(|sorted| AnyJsonValue::from(sorted))
                    .unwrap_or_else(|| value.clone())
            } else {
                value.clone()
            }
        }

        FieldTransformer::SortVSCodeBadgeObject => {
            if let Some(obj) = value.as_json_object_value() {
                sorters::sort_vscode_badge_object(obj)
                    .map(|sorted| AnyJsonValue::from(sorted))
                    .unwrap_or_else(|| value.clone())
            } else {
                value.clone()
            }
        }

        FieldTransformer::UniqArray | FieldTransformer::UniqAndSortArray => value.clone(),
    };

    member.clone().with_value(transformed_value)
}

fn get_sorted_field_order(field_names: &[String]) -> Vec<String> {
    let mut known_fields = Vec::new();
    let mut unknown_fields = Vec::new();
    let mut private_fields = Vec::new();

    for name in field_names {
        if name.starts_with('_') {
            private_fields.push(name.clone());
        } else if get_field_index(name).is_some() {
            known_fields.push(name.clone());
        } else {
            unknown_fields.push(name.clone());
        }
    }

    known_fields.sort_by_key(|name| get_field_index(name).unwrap_or(usize::MAX));
    unknown_fields.sort();
    private_fields.sort();

    let mut result = Vec::new();
    result.extend(known_fields);
    result.extend(unknown_fields);
    result.extend(private_fields);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sorted_field_order() {
        let fields = vec![
            "dependencies".to_string(),
            "name".to_string(),
            "version".to_string(),
            "unknown".to_string(),
            "_private".to_string(),
        ];

        let sorted = get_sorted_field_order(&fields);

        assert_eq!(
            sorted,
            vec!["name", "version", "dependencies", "unknown", "_private"]
        );
    }

    #[test]
    fn test_get_sorted_field_order_private_last() {
        let fields = vec![
            "_z".to_string(),
            "name".to_string(),
            "_a".to_string(),
            "version".to_string(),
        ];

        let sorted = get_sorted_field_order(&fields);

        assert_eq!(sorted, vec!["name", "version", "_a", "_z"]);
    }

    #[test]
    fn test_get_sorted_field_order_unknown_alphabetical() {
        let fields = vec![
            "zebra".to_string(),
            "name".to_string(),
            "apple".to_string(),
            "version".to_string(),
        ];

        let sorted = get_sorted_field_order(&fields);

        assert_eq!(sorted, vec!["name", "version", "apple", "zebra"]);
    }
}
