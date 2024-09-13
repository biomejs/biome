use biome_string_case::Case;
use std::env;
use std::fs;
use std::path::PathBuf;

const KNOWN_GROUPS: [&str; 7] = [
    "a11y",
    "suspicious",
    "correctness",
    "performance",
    "security",
    "style",
    "complexity",
];

const KNOWN_PATHS: &[&str] = &[
    "crates/biome_js_analyze",
    "crates/biome_css_analyze",
    "crates/biome_json_analyze",
    "crates/biome_graphql_analyze",
];
pub fn promote_rule(rule_name: &str, new_group: &str) {
    let current_dir = env::current_dir().ok().unwrap();

    if !KNOWN_GROUPS.contains(&new_group) {
        panic!(
            "The group '{}' doesn't exist. Available groups: {}",
            new_group,
            KNOWN_GROUPS.join(", ")
        )
    }

    let rule_name_snake = Case::Snake.convert(rule_name);

    // look for the rule in the source code
    let mut rule_path = None;
    let mut analyzers_path = None;
    for known_path in KNOWN_PATHS {
        let local_rule_path = current_dir
            .join(known_path)
            .join("src/lint/nursery")
            .join(format!("{rule_name_snake}.rs"));
        if local_rule_path.exists() {
            rule_path = Some(local_rule_path);
            analyzers_path = Some(PathBuf::from(known_path));
            break;
        }
    }

    if let (Some(rule_path), Some(analyzers_path)) = (rule_path, analyzers_path) {
        // rule found!
        let new_group_src_path = analyzers_path.join("src/lint").join(new_group);
        let new_rule_path = new_group_src_path.join(format!("{rule_name_snake}.rs"));
        let new_group_test_path = analyzers_path.join("tests/specs").join(new_group);

        let categories_path = "crates/biome_diagnostics_categories/src/categories.rs";
        let categories = std::fs::read_to_string(categories_path).unwrap();

        let mut categories = categories.replace(
            &format!("lint/nursery/{rule_name}"),
            &format!("lint/{new_group}/{rule_name}"),
        );

        // We sort rules to reduce conflicts between contributions made in parallel.
        let lint_start = "define_categories! {\n";
        let lint_end = "\n    // end lint rules\n";
        debug_assert!(categories.contains(lint_start));
        debug_assert!(categories.contains(lint_end));
        let lint_start_index = categories.find(lint_start).unwrap() + lint_start.len();
        let lint_end_index = categories.find(lint_end).unwrap();
        let lint_rule_text = &categories[lint_start_index..lint_end_index];
        let mut lint_rules: Vec<_> = lint_rule_text.lines().collect();
        lint_rules.sort_unstable();
        let new_lint_rule_text = lint_rules.join("\n");
        categories.replace_range(lint_start_index..lint_end_index, &new_lint_rule_text);

        if !new_group_src_path.exists() {
            fs::create_dir(&new_group_src_path).expect("To create the group source folder");
        }
        fs::rename(&rule_path, &new_rule_path).expect("To move rule file");
        std::fs::write(categories_path, categories).unwrap();

        if !new_group_test_path.exists() {
            fs::create_dir(&new_group_test_path).expect("To create the group test folder");
        }
        let old_test_path = analyzers_path.join("tests/specs/nursery").join(rule_name);
        let new_test_path = new_group_test_path.join(rule_name);
        fs::rename(old_test_path, new_test_path).expect("To move rule test folder");
    } else {
        panic!("Couldn't find the rule {rule_name}");
    }
}
