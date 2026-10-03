use anyhow::{Context, Result, ensure};
use biome_string_case::Case;
use regex::Regex;
use std::{collections::BTreeSet, fs, path::Path};

pub(crate) const CATEGORIES_PATH: &str = "crates/biome_diagnostics_categories/src/categories.rs";

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct CategoryChange {
    pub category: String,
    pub old_group: String,
    pub new_group: String,
    pub old_name: String,
    pub new_name: String,
}

pub(crate) fn validate_category_changes(
    root: &Path,
    changes: impl IntoIterator<Item = CategoryChange>,
) -> Result<()> {
    let changes: BTreeSet<_> = changes.into_iter().collect();
    let path = root.join(CATEGORIES_PATH);
    let contents =
        fs::read_to_string(&path).with_context(|| format!("failed to read {}", path.display()))?;
    validate_changes(&contents, &changes)
}

pub(crate) fn apply_category_changes(
    root: &Path,
    changes: impl IntoIterator<Item = CategoryChange>,
) -> Result<()> {
    let changes: BTreeSet<_> = changes.into_iter().collect();
    if changes.is_empty() {
        return Ok(());
    }

    let path = root.join(CATEGORIES_PATH);
    let mut contents =
        fs::read_to_string(&path).with_context(|| format!("failed to read {}", path.display()))?;
    validate_changes(&contents, &changes)?;

    for change in &changes {
        let old_category = format!(
            "{}/{}/{}",
            change.category, change.old_group, change.old_name
        );
        let new_category = format!(
            "{}/{}/{}",
            change.category, change.new_group, change.new_name
        );
        contents = contents.replacen(&old_category, &new_category, 1);

        if change.old_name != change.new_name {
            let line_start = contents
                .find(&format!("\"{new_category}\""))
                .with_context(|| format!("failed to find the category {new_category}"))?;
            let line_end = contents[line_start..]
                .find('\n')
                .map_or(contents.len(), |offset| line_start + offset);
            let url_prefix = match change.category.as_str() {
                "lint" => Some("https://biomejs.dev/linter/rules/"),
                "assist" => Some("https://biomejs.dev/assist/actions/"),
                "syntax" => None,
                _ => anyhow::bail!("the category {} is not supported", change.category),
            };
            if let Some(url_prefix) = url_prefix {
                let pattern = Regex::new(&format!(r#"{}[^"]+"#, regex::escape(url_prefix)))?;
                let old_line = &contents[line_start..line_end];
                ensure!(
                    pattern.find_iter(old_line).count() == 1,
                    "expected one documentation URL for {new_category}"
                );
                let new_url = format!("{url_prefix}{}", Case::Kebab.convert(&change.new_name));
                let line = pattern.replace(old_line, new_url).into_owned();
                contents.replace_range(line_start..line_end, &line);
            }
        }
    }

    for category in changes.iter().map(|change| change.category.as_str()) {
        sort_category_section(&mut contents, category)?;
    }

    fs::write(&path, contents).with_context(|| format!("failed to write {}", path.display()))?;
    Ok(())
}

fn validate_changes(contents: &str, changes: &BTreeSet<CategoryChange>) -> Result<()> {
    for change in changes {
        let old_category = format!(
            "{}/{}/{}",
            change.category, change.old_group, change.old_name
        );
        let new_category = format!(
            "{}/{}/{}",
            change.category, change.new_group, change.new_name
        );
        ensure!(
            contents.matches(&format!("\"{old_category}\"")).count() == 1,
            "expected one diagnostic category for {old_category}"
        );
        if old_category != new_category {
            ensure!(
                !contents.contains(&format!("\"{new_category}\"")),
                "the diagnostic category {new_category} already exists"
            );
        }
    }
    Ok(())
}

pub(crate) fn insert_category_line(
    contents: &mut String,
    category: &str,
    line: &str,
) -> Result<()> {
    let (_, end_marker) = category_markers(category)?;
    let insert_at = contents
        .find(end_marker)
        .with_context(|| format!("failed to find the end of {category} categories"))?;
    contents.insert_str(insert_at, &format!("\n{line}"));
    sort_category_section(contents, category)
}

pub(crate) fn sort_category_section(contents: &mut String, category: &str) -> Result<()> {
    let (start_marker, end_marker) = category_markers(category)?;
    let start = contents
        .find(start_marker)
        .with_context(|| format!("failed to find the start of {category} categories"))?
        + start_marker.len();
    let end = contents
        .find(end_marker)
        .with_context(|| format!("failed to find the end of {category} categories"))?;
    let mut entries: Vec<_> = contents[start..end].lines().collect();
    // Lexicographic ordering reduces conflicts between rule changes developed in parallel.
    entries.sort_unstable();
    contents.replace_range(start..end, &entries.join("\n"));
    Ok(())
}

fn category_markers(category: &str) -> Result<(&'static str, &'static str)> {
    match category {
        "lint" => Ok(("define_categories! {\n", "\n    // end lint rules\n")),
        "assist" => Ok((
            "    // start assist actions\n",
            "\n    // end assist actions\n",
        )),
        "syntax" => Ok((
            "    ; // start syntax rules\n",
            "\n    // end syntax rules\n",
        )),
        _ => anyhow::bail!("the category {category} is not supported"),
    }
}
