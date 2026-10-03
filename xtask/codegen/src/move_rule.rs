use crate::diagnostic_categories::{
    CategoryChange, apply_category_changes, validate_category_changes,
};
use anyhow::{Context, Result, ensure};
use biome_string_case::Case;
use std::{collections::BTreeSet, env, fs, path::Path, path::PathBuf};

const KNOWN_CATEGORIES: &[&str] = &["lint", "assist", "syntax"];

const KNOWN_GROUPS: &[&str] = &[
    "nursery",
    "a11y",
    "complexity",
    "correctness",
    "performance",
    "security",
    "style",
    "suspicious",
    "source",
];

pub(crate) const ANALYZER_CRATES: &[&str] = &[
    "crates/biome_js_analyze",
    "crates/biome_css_analyze",
    "crates/biome_html_analyze",
    "crates/biome_graphql_analyze",
    "crates/biome_json_analyze",
    "crates/biome_markdown_analyze",
    "crates/biome_yaml_analyze",
];

#[derive(Debug)]
pub(crate) struct RuleMove {
    pub analyzer_path: PathBuf,
    pub category: String,
    pub old_group: String,
    pub new_group: String,
    pub old_name: String,
    pub new_name: String,
    pub source: PathBuf,
    pub destination: PathBuf,
    pub tests: PathBuf,
    pub test_destination: PathBuf,
}

impl RuleMove {
    pub(crate) fn is_noop(&self) -> bool {
        self.source == self.destination && self.tests == self.test_destination
    }

    pub(crate) fn category_change(&self) -> CategoryChange {
        CategoryChange {
            category: self.category.clone(),
            old_group: self.old_group.clone(),
            new_group: self.new_group.clone(),
            old_name: self.old_name.clone(),
            new_name: self.new_name.clone(),
        }
    }
}

pub fn move_rule(rule_name: &str, new_group: &str) -> Result<()> {
    let root = env::current_dir().context("failed to determine the current directory")?;
    let moves = plan_rule_moves(&root, rule_name, new_group, rule_name)?;
    ensure!(!moves.is_empty(), "could not find the rule {rule_name}");
    ensure!(
        moves.iter().any(|rule_move| !rule_move.is_noop()),
        "{rule_name} is already in the {new_group} group"
    );

    for rule_move in &moves {
        if !rule_move.is_noop() {
            println!(
                "Moving {}/{}/{} to {}/{}/{}...",
                rule_move.category,
                rule_move.old_group,
                rule_move.old_name,
                rule_move.category,
                rule_move.new_group,
                rule_move.new_name
            );
        }
    }

    validate_rule_categories(&root, moves.iter())?;
    apply_rule_moves(&moves)?;
    update_rule_categories(&root, moves.iter())
}

pub(crate) fn plan_rule_moves(
    root: &Path,
    rule_name: &str,
    new_group: &str,
    new_name: &str,
) -> Result<Vec<RuleMove>> {
    ensure!(
        KNOWN_GROUPS.contains(&new_group),
        "the group {new_group} doesn't exist; available groups: {}",
        KNOWN_GROUPS.join(", ")
    );

    let rule_name_snake = Case::Snake.convert(rule_name);
    let new_name_snake = Case::Snake.convert(new_name);
    let mut moves = Vec::new();
    let mut destinations = BTreeSet::new();
    let mut test_destinations = BTreeSet::new();

    for analyzer_crate in ANALYZER_CRATES {
        let analyzer_path = root.join(analyzer_crate);
        for category in KNOWN_CATEGORIES {
            for old_group in KNOWN_GROUPS {
                let source = analyzer_path
                    .join("src")
                    .join(category)
                    .join(old_group)
                    .join(format!("{rule_name_snake}.rs"));
                if !source.exists() {
                    continue;
                }

                let destination = analyzer_path
                    .join("src")
                    .join(category)
                    .join(new_group)
                    .join(format!("{new_name_snake}.rs"));
                let tests = analyzer_path
                    .join("tests/specs")
                    .join(old_group)
                    .join(rule_name);
                let test_destination = analyzer_path
                    .join("tests/specs")
                    .join(new_group)
                    .join(new_name);

                ensure!(
                    tests.is_dir(),
                    "the test directory {} does not exist",
                    tests.display()
                );
                if source != destination {
                    ensure!(
                        !destination.exists(),
                        "the destination {} already exists",
                        destination.display()
                    );
                    ensure!(
                        destinations.insert(destination.clone()),
                        "multiple rules would be moved to {}",
                        destination.display()
                    );
                }
                if tests != test_destination {
                    ensure!(
                        !test_destination.exists(),
                        "the destination {} already exists",
                        test_destination.display()
                    );
                    ensure!(
                        test_destinations.insert(test_destination.clone()),
                        "multiple test suites would be moved to {}",
                        test_destination.display()
                    );
                }

                moves.push(RuleMove {
                    analyzer_path: analyzer_path.clone(),
                    category: (*category).to_string(),
                    old_group: (*old_group).to_string(),
                    new_group: new_group.to_string(),
                    old_name: rule_name.to_string(),
                    new_name: new_name.to_string(),
                    source,
                    destination,
                    tests,
                    test_destination,
                });
            }
        }
    }

    Ok(moves)
}

pub(crate) fn apply_rule_moves(moves: &[RuleMove]) -> Result<()> {
    for rule_move in moves {
        if rule_move.is_noop() {
            continue;
        }
        create_parent(&rule_move.destination)?;
        fs::rename(&rule_move.source, &rule_move.destination).with_context(|| {
            format!(
                "failed to move {} to {}",
                rule_move.source.display(),
                rule_move.destination.display()
            )
        })?;

        create_parent(&rule_move.test_destination)?;
        fs::rename(&rule_move.tests, &rule_move.test_destination).with_context(|| {
            format!(
                "failed to move {} to {}",
                rule_move.tests.display(),
                rule_move.test_destination.display()
            )
        })?;
    }

    remove_empty_groups(moves)
}

pub(crate) fn validate_rule_categories<'a>(
    root: &Path,
    moves: impl IntoIterator<Item = &'a RuleMove>,
) -> Result<()> {
    validate_category_changes(root, moves.into_iter().map(RuleMove::category_change))
}

pub(crate) fn update_rule_categories<'a>(
    root: &Path,
    moves: impl IntoIterator<Item = &'a RuleMove>,
) -> Result<()> {
    apply_category_changes(root, moves.into_iter().map(RuleMove::category_change))
}

fn create_parent(path: &Path) -> Result<()> {
    let parent = path
        .parent()
        .with_context(|| format!("{} has no parent directory", path.display()))?;
    fs::create_dir_all(parent).with_context(|| format!("failed to create {}", parent.display()))?;
    Ok(())
}

fn remove_empty_groups(moves: &[RuleMove]) -> Result<()> {
    let groups: BTreeSet<_> = moves
        .iter()
        .filter(|rule_move| !rule_move.is_noop())
        .map(|rule_move| {
            (
                rule_move.analyzer_path.clone(),
                rule_move.category.clone(),
                rule_move.old_group.clone(),
            )
        })
        .collect();

    for (analyzer_path, category, group) in groups {
        let source_directory = analyzer_path.join("src").join(&category).join(&group);
        if source_directory.is_dir()
            && fs::read_dir(&source_directory)
                .with_context(|| format!("failed to read {}", source_directory.display()))?
                .next()
                .is_none()
        {
            fs::remove_dir(&source_directory)
                .with_context(|| format!("failed to remove {}", source_directory.display()))?;
            let group_module = analyzer_path
                .join("src")
                .join(&category)
                .join(format!("{group}.rs"));
            if group_module.exists() {
                fs::remove_file(&group_module)
                    .with_context(|| format!("failed to remove {}", group_module.display()))?;
            }
        }

        let test_directory = analyzer_path.join("tests/specs").join(&group);
        if test_directory.is_dir()
            && fs::read_dir(&test_directory)
                .with_context(|| format!("failed to read {}", test_directory.display()))?
                .next()
                .is_none()
        {
            fs::remove_dir(&test_directory)
                .with_context(|| format!("failed to remove {}", test_directory.display()))?;
        }
    }
    Ok(())
}
