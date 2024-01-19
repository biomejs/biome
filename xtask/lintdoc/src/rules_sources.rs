use biome_analyze::RuleMetadata;
use convert_case::{Case, Casing};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;
use xtask::*;

#[derive(Debug, Eq, PartialEq)]
struct SourceSet {
    source_rule_name: String,
    source_link: String,
    biome_rule_name: String,
    biome_link: String,
    inspired: bool,
}

impl Ord for SourceSet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.source_rule_name.cmp(&other.source_rule_name)
    }
}

impl PartialOrd for SourceSet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub(crate) fn generate_rule_sources(
    rules: BTreeMap<&str, BTreeMap<&'static str, RuleMetadata>>,
) -> Result<Vec<u8>> {
    let mut buffer = vec![];

    writeln!(
        buffer,
        r#"---
title: Rules sources
description: A page that maps lint rules from other sources to Biome
---
    "#
    )?;

    let rules = rules
        .into_iter()
        .flat_map(|(_, rule)| rule)
        .collect::<BTreeMap<&str, RuleMetadata>>();

    let mut rules_by_source = BTreeMap::<String, BTreeSet<SourceSet>>::new();
    let mut exclusive_biome_rules = BTreeSet::<(String, String)>::new();

    for (rule_name, metadata) in rules {
        if let Some(source) = &metadata.source {
            let set = rules_by_source.get_mut(&format!("{source}"));
            if let Some(set) = set {
                set.insert(SourceSet {
                    biome_rule_name: rule_name.to_string(),
                    biome_link: format!("/linter/rules/{}", rule_name.to_case(Case::Kebab)),
                    source_link: source.to_rule_url(),
                    source_rule_name: source.as_rule_name().to_string(),
                    inspired: metadata
                        .source_kind
                        .map(|kind| kind.is_inspired())
                        .unwrap_or(false),
                });
            } else {
                let mut set = BTreeSet::new();
                set.insert(SourceSet {
                    biome_rule_name: rule_name.to_string(),
                    biome_link: format!("/linter/rules/{}", rule_name.to_case(Case::Kebab)),
                    source_link: source.to_rule_url(),
                    source_rule_name: source.as_rule_name().to_string(),
                    inspired: metadata
                        .source_kind
                        .map(|kind| kind.is_inspired())
                        .unwrap_or(true),
                });
                rules_by_source.insert(format!("{source}"), set);
            }
        } else {
            exclusive_biome_rules.insert((
                rule_name.to_string(),
                format!("/linter/rules/{}", rule_name.to_case(Case::Kebab)),
            ));
        }
    }

    writeln!(buffer, "## Biome exclusive rules",)?;
    for (rule, link) in exclusive_biome_rules {
        writeln!(buffer, "- [{}]({}) ", rule, link)?;
    }

    writeln!(buffer, "## Rules from other sources",)?;
    writeln!(
        buffer,
        r#":::note
Some **Biome** rules might **not** have options, compared to the original rule.
:::"#
    )?;

    for (source, rules) in rules_by_source {
        writeln!(buffer, "### {source}")?;
        writeln!(buffer, r#"| {source} rule name | Biome rule name |"#)?;
        writeln!(buffer, r#"| ---- | ---- |"#)?;

        push_to_table(rules, &mut buffer)?;
    }

    Ok(buffer)
}

fn push_to_table(source_set: BTreeSet<SourceSet>, buffer: &mut Vec<u8>) -> Result<u8> {
    let mut footnotes = 0;
    for source_set in source_set {
        write!(
            buffer,
            "| [{}]({}) |[{}]({})",
            source_set.source_rule_name,
            source_set.source_link,
            source_set.biome_rule_name,
            source_set.biome_link
        )?;

        if source_set.inspired {
            footnotes += 1;
            write!(buffer, " (inspired)")?;
        }
        writeln!(buffer, " |")?;
    }

    Ok(footnotes)
}
