use biome_analyze::{RuleMetadata, RuleSource};
use convert_case::{Case, Casing};
use std::collections::BTreeMap;
use std::io::Write;
use xtask::*;

pub(crate) fn generate_rule_sources(
    rules: BTreeMap<&str, BTreeMap<&'static str, RuleMetadata>>,
) -> Result<Vec<u8>> {
    let mut buffer = vec![];

    writeln!(
        buffer,
        r#"---
title: Linter sources
description: A page that maps lint rules from other sources to Biome
---
    "#
    )?;

    let rules = rules
        .into_iter()
        .flat_map(|(_, rule)| rule)
        .collect::<BTreeMap<&str, RuleMetadata>>();

    eslint_to_biome(&rules, &mut buffer)?;
    typescript_to_biome(&rules, &mut buffer)?;
    eslint_plugins_to_biome(&rules, &mut buffer)?;

    Ok(buffer)
}

fn eslint_to_biome(map: &BTreeMap<&str, RuleMetadata>, buffer: &mut Vec<u8>) -> Result<()> {
    writeln!(buffer, "## ESlint rules to Biome")?;

    writeln!(buffer, r#"| ESLint Rule name | Biome Rule name |"#)?;
    writeln!(buffer, r#"| ---- | ---- |"#)?;
    let mut eslint_to_biome = BTreeMap::new();

    for (rule_name, metadata) in map {
        if let Some(source) = &metadata.source {
            if source.is_eslint() {
                eslint_to_biome.insert(source, rule_name);
            }
        }
    }

    push_to_table(eslint_to_biome, buffer)?;
    Ok(())
}

fn typescript_to_biome(map: &BTreeMap<&str, RuleMetadata>, buffer: &mut Vec<u8>) -> Result<()> {
    writeln!(buffer, "## `typescript-eslint` rules to Biome")?;
    writeln!(
        buffer,
        r#"| `typescript-eslint` rule name | Biome rule name |"#
    )?;
    writeln!(buffer, r#"| ---- | ---- |"#)?;
    let mut typescript_to_biome = BTreeMap::new();

    for (rule_name, metadata) in map {
        if let Some(source) = &metadata.source {
            if source.is_eslint_typescript() {
                typescript_to_biome.insert(source, rule_name);
            }
        }
    }

    push_to_table(typescript_to_biome, buffer)?;

    Ok(())
}

fn eslint_plugins_to_biome(map: &BTreeMap<&str, RuleMetadata>, buffer: &mut Vec<u8>) -> Result<()> {
    writeln!(buffer, "## ESlint plugin rules to Biome")?;
    writeln!(
        buffer,
        r#"| Plugin name | Plugin rule name | Biome rule name |"#
    )?;
    writeln!(buffer, r#"| ---- | ---- | --- |"#)?;
    let mut eslint_plugin_to_biome = BTreeMap::new();

    for (rule_name, metadata) in map {
        if let Some(source) = &metadata.source {
            if source.is_eslint_plugin() {
                eslint_plugin_to_biome.insert(source, rule_name);
            }
        }
    }

    push_to_table(eslint_plugin_to_biome, buffer)?;

    Ok(())
}

fn push_to_table(map: BTreeMap<&RuleSource, &&str>, buffer: &mut Vec<u8>) -> Result<()> {
    for (rule_source, rule_name) in map {
        let biome_link = rule_name.to_case(Case::Kebab);
        let source_rule_name = rule_source.as_rule_name();
        let source_link = rule_source.as_rule_url();

        if rule_source.is_eslint_plugin() {
            writeln!(
                buffer,
                "| `{}` | [{}]({}) |[{}](/linter/rules/{})  |",
                rule_source, source_rule_name, source_link, rule_name, biome_link
            )?;
        } else {
            writeln!(
                buffer,
                "| [{}]({}) |[{}](/linter/rules/{})  |",
                source_rule_name, source_link, rule_name, biome_link
            )?;
        }
    }

    Ok(())
}
