use biome_json_syntax::{AnyJsonValue, JsonObjectValue};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

use super::sorters::sort_object_by_key_order;

/// See https://docs.npmjs.com/misc/scripts
const DEFAULT_NPM_SCRIPTS: &[&str] = &[
    "install",
    "pack",
    "prepare",
    "publish",
    "restart",
    "shrinkwrap",
    "start",
    "stop",
    "test",
    "uninstall",
    "version",
];

pub fn transform(
    value: &AnyJsonValue,
    package_json_root: &JsonObjectValue,
) -> Option<AnyJsonValue> {
    let object = value.as_json_object_value()?;
    let members = object.json_member_list();

    let mut script_names = Vec::new();
    for m in &members {
        if let Ok(member) = m
            && let Ok(name) = member.name().and_then(|n| n.inner_string_text())
        {
            script_names.push(name.text().to_string());
        }
    }

    if has_sequential_script(package_json_root) {
        return None;
    }

    let base_names = normalize_script_names(&script_names);
    let sorted_names = sort_script_names(base_names, "");
    let expanded_names = expand_with_lifecycle_scripts(&sorted_names, &script_names);

    let expanded_refs: Vec<&str> = expanded_names.iter().map(|s| s.as_str()).collect();
    let sorted = sort_object_by_key_order(object, &expanded_refs)?;

    Some(AnyJsonValue::from(sorted))
}

fn has_sequential_script(package_json: &JsonObjectValue) -> bool {
    let members = package_json.json_member_list();

    let has_npm_run_all =
        has_dev_dependency("npm-run-all", &members) || has_dev_dependency("npm-run-all2", &members);

    if !has_npm_run_all {
        return false;
    }

    let script_values = get_all_script_values(package_json);

    script_values
        .iter()
        .any(|script| is_sequential_script(script))
}

fn has_dev_dependency(dep_name: &str, members: &biome_json_syntax::JsonMemberList) -> bool {
    for member in members {
        if let Ok(m) = member
            && let Ok(name) = m.name().and_then(|n| n.inner_string_text())
            && name.text() == "devDependencies"
            && let Ok(value) = m.value()
            && let Some(obj) = value.as_json_object_value()
        {
            for dep_member in obj.json_member_list() {
                if let Ok(dep) = dep_member
                    && let Ok(dep_name_token) = dep.name().and_then(|n| n.inner_string_text())
                    && dep_name_token.text() == dep_name
                {
                    return true;
                }
            }
        }
    }
    false
}

fn get_all_script_values(package_json: &JsonObjectValue) -> Vec<String> {
    let members = package_json.json_member_list();
    let mut values = Vec::new();

    for field in ["scripts", "betterScripts"] {
        for member in &members {
            if let Ok(m) = member
                && let Ok(name) = m.name().and_then(|n| n.inner_string_text())
                && name.text() == field
                && let Ok(value) = m.value()
                && let Some(obj) = value.as_json_object_value()
            {
                for script_member in obj.json_member_list() {
                    if let Ok(sm) = script_member
                        && let Ok(sv) = sm.value()
                        && let Some(s) = sv.as_json_string_value()
                        && let Ok(text) = s.inner_string_text()
                    {
                        values.push(text.text().to_string());
                    }
                }
            }
        }
    }

    values
}

/// Regex to detect sequential scripts using run-s or npm-run-all with -s/--sequential/--serial
/// Copied over from https://github.com/keithamus/sort-package-json/blob/e3f2370982fdd22fbf6d68ab8c9690f3a767f834/index.js#L319
static RUN_S_REGEXP: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?:^|[\s&;<>|(])(?:run-s|npm-run-all2? .*(?:--sequential|--serial|-s))(?:$|[\s&;<>|)])",
    )
    .unwrap()
});

fn is_sequential_script(command: &str) -> bool {
    command.contains('*') && RUN_S_REGEXP.is_match(command)
}

fn normalize_script_names(names: &[String]) -> Vec<String> {
    let prefixable: HashSet<String> = names
        .iter()
        .filter_map(|name| {
            let stripped = name
                .strip_prefix("pre")
                .or_else(|| name.strip_prefix("post"))?;
            if DEFAULT_NPM_SCRIPTS.contains(&stripped) || names.iter().any(|n| n == stripped) {
                Some(stripped.to_string())
            } else {
                None
            }
        })
        .collect();

    names
        .iter()
        .map(|name| {
            if let Some(stripped) = name
                .strip_prefix("pre")
                .or_else(|| name.strip_prefix("post"))
                && prefixable.contains(stripped)
            {
                return stripped.to_string();
            }
            name.clone()
        })
        .collect()
}

fn sort_script_names(names: Vec<String>, prefix: &str) -> Vec<String> {
    let mut group_map: HashMap<String, Vec<String>> = HashMap::new();

    for key in &names {
        // Safely extract the rest after prefix
        let rest = if prefix.is_empty() {
            key.as_str()
        } else {
            let prefix_with_colon = format!("{}:", prefix);
            key.strip_prefix(&prefix_with_colon).unwrap_or(key.as_str())
        };

        if let Some(idx) = rest.find(':') {
            // Safely compute base by finding where rest starts in key
            let rest_offset = key.len() - rest.len();
            let base = &key[..rest_offset + idx];
            group_map
                .entry(base.to_string())
                .or_default()
                .push(key.clone());
        } else {
            group_map.entry(key.clone()).or_default().push(key.clone());
        }
    }

    let mut sorted_keys: Vec<String> = group_map.keys().cloned().collect();
    sorted_keys.sort();

    sorted_keys
        .into_iter()
        .flat_map(|group_key| {
            let children = &group_map[&group_key];

            if children.len() > 1
                && children
                    .iter()
                    .any(|k| k != &group_key && k.starts_with(&format!("{}:", group_key)))
            {
                let mut direct: Vec<String> = children
                    .iter()
                    .filter(|k| *k == &group_key || !k.starts_with(&format!("{}:", group_key)))
                    .cloned()
                    .collect();
                direct.sort();

                let nested: Vec<String> = children
                    .iter()
                    .filter(|k| k.starts_with(&format!("{}:", group_key)))
                    .cloned()
                    .collect();

                let mut result = direct;
                result.extend(sort_script_names(nested, &group_key));
                result
            } else {
                let mut sorted = children.clone();
                sorted.sort();
                sorted
            }
        })
        .collect()
}

fn expand_with_lifecycle_scripts(base_names: &[String], all_scripts: &[String]) -> Vec<String> {
    let mut result = Vec::new();

    for name in base_names {
        let pre_name = format!("pre{}", name);
        let post_name = format!("post{}", name);

        if all_scripts.contains(&pre_name) {
            result.push(pre_name);
        }
        result.push(name.clone());
        if all_scripts.contains(&post_name) {
            result.push(post_name);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_sequential_script() {
        assert!(is_sequential_script("run-s \"lint:*\""));
        assert!(is_sequential_script("npm-run-all -s \"lint:*\""));
        assert!(is_sequential_script("npm-run-all --sequential \"lint:*\""));
        assert!(is_sequential_script("npm-run-all --serial \"lint:*\""));
        assert!(!is_sequential_script("run-s lint:a lint:b"));
        assert!(!is_sequential_script("npm-run-all *"));
    }

    #[test]
    fn test_default_npm_scripts() {
        assert!(DEFAULT_NPM_SCRIPTS.contains(&"test"));
        assert!(DEFAULT_NPM_SCRIPTS.contains(&"install"));
        assert!(DEFAULT_NPM_SCRIPTS.contains(&"start"));
        assert_eq!(DEFAULT_NPM_SCRIPTS.len(), 11);
    }
}
