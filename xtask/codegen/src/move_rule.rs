use biome_string_case::Case;
use std::env;
use std::fs;
use std::path::Path;

const KNOWN_CATEGORIES: &[&str] = &["lint", "assist", "syntax"];

const KNOWN_GROUPS: &[&str] = &[
    // any
    "nursery",
    // lint group
    "a11y",
    "complexity",
    "correctness",
    "performance",
    "security",
    "style",
    "suspicious",
    // assist group
    "source",
];

const KNOWN_PATHS: &[&str] = &[
    "crates/biome_js_analyze",
    "crates/biome_css_analyze",
    "crates/biome_html_analyze",
    "crates/biome_graphql_analyze",
    "crates/biome_json_analyze",
];

pub fn move_rule(rule_name: &str, new_group: &str) {
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
    let mut has_moved_rules = false;
    for known_path in KNOWN_PATHS {
        for category in KNOWN_CATEGORIES {
            for group in KNOWN_GROUPS {
                let local_rule_path = current_dir
                    .join(known_path)
                    .join("src")
                    .join(category)
                    .join(group)
                    .join(format!("{rule_name_snake}.rs"));
                if local_rule_path.exists() {
                    if *group != new_group {
                        println!(
                            "Moving {category}/{group}/{rule_name} to {category}/{new_group}/{rule_name}..."
                        );
                        move_to_group(
                            &local_rule_path,
                            Path::new(known_path),
                            category,
                            rule_name,
                            group,
                            new_group,
                        );
                        has_moved_rules = true;
                    } else {
                        eprintln!(
                            "{category}/{group}/{rule_name} cannot be moved because it is already in the correct group."
                        );
                    }
                }
            }
        }
    }

    if !has_moved_rules {
        panic!("Couldn't find the rule {rule_name}");
    }
}

fn move_to_group(
    rule_path: &Path,
    analyzers_path: &Path,
    category: &str,
    rule_name: &str,
    old_group: &str,
    new_group: &str,
) {
    let rule_name_snake = Case::Snake.convert(rule_name);

    let new_group_src_path = analyzers_path.join("src").join(category).join(new_group);
    let new_rule_path = new_group_src_path.join(format!("{rule_name_snake}.rs"));
    let new_group_test_path = analyzers_path.join("tests/specs").join(new_group);

    let categories_path = "crates/biome_diagnostics_categories/src/categories.rs";
    let categories = std::fs::read_to_string(categories_path).unwrap();

    let mut categories = categories.replace(
        &format!("{category}/{old_group}/{rule_name}"),
        &format!("{category}/{new_group}/{rule_name}"),
    );

    // We sort rules to reduce conflicts between contributions made in parallel.
    let lint_start = "define_categories! {\n";
    let (category_start, category_end) = match category {
        "lint" => ("define_categories! {\n", "\n    // end lint rules\n"),
        "assist" => (
            "    // start assist actions|n",
            "\n    // end assist actions\n",
        ),
        "syntax" => (
            "    ; // start syntax rules\n",
            "\n    // end syntax rules\n",
        ),
        _ => {
            panic!("The category '{category}' is not handled.",)
        }
    };
    debug_assert!(categories.contains(category_start));
    debug_assert!(categories.contains(category_end));
    let lint_start_index = categories.find(category_start).unwrap() + lint_start.len();
    let lint_end_index = categories.find(category_end).unwrap();
    let lint_rule_text = &categories[lint_start_index..lint_end_index];
    let mut lint_rules: Vec<_> = lint_rule_text.lines().collect();
    lint_rules.sort_unstable();
    let new_lint_rule_text = lint_rules.join("\n");
    categories.replace_range(lint_start_index..lint_end_index, &new_lint_rule_text);

    if !new_group_src_path.exists() {
        fs::create_dir(&new_group_src_path).expect("To create the group source folder");
    }
    fs::rename(rule_path, &new_rule_path).expect("To move rule file");
    std::fs::write(categories_path, categories).unwrap();

    if !new_group_test_path.exists() {
        fs::create_dir(&new_group_test_path).expect("To create the group test folder");
    }
    let old_test_path = analyzers_path
        .join("tests/specs")
        .join(old_group)
        .join(rule_name);
    let new_test_path = new_group_test_path.join(rule_name);
    fs::rename(old_test_path, new_test_path).expect("To move rule test folder");
}
