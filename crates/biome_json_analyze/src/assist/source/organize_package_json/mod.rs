use crate::utils::is_package_json;
use crate::JsonRuleAction;
use biome_analyze::{
    context::RuleContext, declare_source_rule, Ast, FixKind, Rule, RuleAction, RuleDiagnostic,
};
use biome_console::markup;
use biome_diagnostics::category;
use biome_json_factory::make;
use biome_json_syntax::{JsonMember, JsonMemberList, JsonRoot, T};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt};
use std::collections::HashMap;

mod field_order;

use field_order::{get_field_index, is_private_field};

declare_source_rule! {
    /// Organize package.json fields according to established conventions.
    ///
    /// This assist action sorts package.json fields following the same conventions
    /// as the popular [sort-package-json](https://github.com/keithamus/sort-package-json) tool.
    ///
    /// Fields are organized in priority order:
    /// 1. 110 predefined fields (e.g., name, version, description, scripts, dependencies)
    /// 2. Unknown fields (alphabetically)
    /// 3. Private fields starting with `_` (alphabetically)
    ///
    /// For complete sorting rules, see:
    /// https://github.com/keithamus/sort-package-json/blob/main/defaultRules.md
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
        // Only run on package.json files
        let path = ctx.file_path();
        if !is_package_json(path) {
            return None;
        }

        let root = ctx.query();
        let value = root.value().ok()?;
        let object = value.as_json_object_value()?;
        let members = object.json_member_list();

        // Check if already organized
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

        // Organize the members
        let organized_members = organize_members(&members)?;

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

/// Check if members are already in the correct order
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

    // Get the sorted order
    let sorted_names = get_sorted_field_order(&field_names);

    // Compare with current order
    field_names == sorted_names
}

/// Organize members according to package.json conventions
fn organize_members(members: &JsonMemberList) -> Option<JsonMemberList> {
    // Extract all members into a map
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

    // Get field names
    let field_names: Vec<String> = member_map.keys().cloned().collect();

    // Get sorted order
    let sorted_names = get_sorted_field_order(&field_names);

    // Build new member list in sorted order
    let mut elements = Vec::new();
    let mut separators = Vec::new();

    for (i, field_name) in sorted_names.iter().enumerate() {
        if let Some(member) = member_map.get(field_name) {
            elements.push(member.clone());

            // Add comma separator (except for last element)
            if i < sorted_names.len() - 1 {
                separators.push(make::token(T![,]));
            }
        }
    }

    // Create new member list
    Some(make::json_member_list(elements, separators))
}

/// Get the sorted field order for given field names
fn get_sorted_field_order(field_names: &[String]) -> Vec<String> {
    // Categorize fields
    let mut known_fields = Vec::new();
    let mut unknown_fields = Vec::new();
    let mut private_fields = Vec::new();

    for name in field_names {
        if is_private_field(name) {
            private_fields.push(name.clone());
        } else if get_field_index(name).is_some() {
            known_fields.push(name.clone());
        } else {
            unknown_fields.push(name.clone());
        }
    }

    // Sort known fields by predefined order
    known_fields.sort_by_key(|name| get_field_index(name).unwrap_or(usize::MAX));

    // Sort unknown and private fields alphabetically
    unknown_fields.sort();
    private_fields.sort();

    // Combine: known → unknown → private
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
