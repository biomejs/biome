//! Auto generate markdown documentation for the configuration schema.
use std::fs::File;
use std::io::Write;

use biome_service::PartialConfiguration;
use schemars::schema_for;
use serde_json::{to_string_pretty, Value};
use xtask::Result;

pub(crate) fn generate_configuration_markdown() -> Result<()> {
    let schema = schema_for!(PartialConfiguration);
    let json_schema = to_string_pretty(&schema)?;
    let schema: Value = serde_json::from_str(&json_schema)?;
    let mut markdown = File::create("website/src/content/docs/reference/configuration.mdx")?;

    writeln!(markdown, "{}", generate_markdown_hearer())?;

    if let Some(schema_object) = schema.as_object() {
        if let Some(properties) = schema_object.get("properties").and_then(Value::as_object) {
            for (name, details) in properties {
                writeln!(markdown, "## `{}`", name)?;
                writeln!(markdown)?;
                if let Some(description) = details.get("description").and_then(Value::as_str) {
                    if description.to_string().contains("```json") {
                        let detail = format_code_block(description)?;
                        writeln!(markdown, "{}", detail)?;
                    } else {
                        writeln!(markdown, "{}", description)?;
                    }
                }
                writeln!(markdown, "\n")?;
            }
        }
    }

    Ok(())
}

fn generate_markdown_hearer() -> String {
    let header = r#"---
title: Configuration
emoji: ⚙️
category: reference
description: How to customize and configure Biome with biome.json.
---

import LintGroups from "@/components/generated/Groups.astro";

{/** Make sure to update the redirect in `static/_redirects` when changing the configuration title --> **/}
"#;

    header.to_string()
}

pub fn format_code_block(description: &str) -> Result<String, serde_json::Error> {
    let mut formatted_description = String::new();

    for line in description.split('\n') {
        let line = line.trim();

        if line.starts_with("```json title=") {
            formatted_description.push_str(&format_json_block(line)?);
        } else {
            formatted_description.push_str(&format_list_line(line));
        }
        formatted_description.push('\n');
    }

    Ok(formatted_description)
}

fn format_json_block(line: &str) -> Result<String, serde_json::Error> {
    let title = extract_title(line);
    let json_body = extract_json_body(line);

    let parsed_json = serde_json::from_str::<Value>(&json_body)?;
    let pretty_json = serde_json::to_string_pretty(&parsed_json)?;

    Ok(format!("{}\n{}\n", title, pretty_json))
}

fn extract_title(line: &str) -> String {
    line.split_once('\"')
        .and_then(|(_, rest)| rest.split_once('\"'))
        .map_or(String::new(), |(title, _)| {
            format!("```json title=\"{}\"", title)
        })
}

fn extract_json_body(line: &str) -> String {
    line.split_once('{')
        .and_then(|(_, rest)| rest.rsplit_once('}'))
        .map_or(String::new(), |(json_body, _)| format!("{{{}}}", json_body))
}

fn format_list_line(line: &str) -> String {
    if line.contains(": -") || line.contains(". -") {
        format_list(line)
    } else {
        line.to_string()
    }
}

fn format_list(description: &str) -> String {
    if let Some((header, items)) = description.split_once(':') {
        let formatted_header = format!("{}:\n", header.trim());
        let formatted_items = items
            .split("; -")
            .enumerate()
            .map(|(index, item)| format_list_item(item, index > 0))
            .collect::<Vec<String>>()
            .join("\n");

        format!("{}{}", formatted_header, formatted_items)
    } else {
        description.to_string()
    }
}

fn format_list_item(item: &str, needs_bullet_point: bool) -> String {
    let trimmed_item = item.trim();
    let bullet_point = if needs_bullet_point { "- " } else { "" };
    format!("{}{}", bullet_point, trimmed_item)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_code_block() {
        let description = r#"```json title="biome.json" { "key": "value" } ```"#;
        let expected = r#"```json title="biome.json"
{
  "key": "value"
}
"#;
        assert_eq!(format_code_block(description), expected);

        let description =
            r#"```json title="config.json" { "key1": "value1", "key2": "value2" } ```"#;
        let expected = r#"```json title="config.json"
{
  "key1": "value1",
  "key2": "value2"
}
"#;
        assert_eq!(format_code_block(description), expected);

        let description = "This is a test string without JSON.";
        assert_eq!(format_code_block(description), description);

        let description = r#"```json title="invalid.json" { invalid JSON } ```"#;
        assert_eq!(format_code_block(description), description);
    }
}
