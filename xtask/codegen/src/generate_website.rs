use crate::generate_schema::generate_configuration_schema;
use biome_cli::biome_command;
use biome_js_formatter::context::JsFormatOptions;
use biome_js_parser::{parse_module, JsParserOptions};
use biome_js_syntax::JsFileSource;
use biome_json_formatter::context::JsonFormatOptions;
use biome_json_formatter::format_node;
use biome_json_parser::{parse_json, JsonParserOptions};
use biome_rowan::AstNode;
use biome_service::{PartialConfiguration, VERSION};
use std::fs;
use xtask::{project_root, Mode, Result};

const CHANGELOG_FRONTMATTER: &str = r#"---
title: Changelog
description: The changelog of Biome
tableOfContents:
    maxHeadingLevel: 2
---
"#;

/// Generates
pub(crate) fn generate_files() -> Result<()> {
    generate_configuration_schema(Mode::Overwrite)?;
    let schema_path_npm = project_root().join("packages/@biomejs/biome/configuration_schema.json");
    let changelog = fs::read_to_string(project_root().join("CHANGELOG.md"))?;
    let default_configuration =
        project_root().join("website/src/components/generated/DefaultConfiguration.mdx");
    fs::remove_file(project_root().join("website/src/content/docs/internals/changelog.mdx")).ok();
    let changelog = format!("{CHANGELOG_FRONTMATTER}{changelog}");

    let configuration_content = serde_json::to_string(&PartialConfiguration::init()).unwrap();
    let tree = parse_json(&configuration_content, JsonParserOptions::default());
    let formatted = format_node(
        JsonFormatOptions::default().with_line_width(60.try_into().unwrap()),
        tree.tree().syntax(),
    )
    .unwrap()
    .print()
    .unwrap();

    let configuration = format!(
        r#"
```json title="biome.json"
{}
```
"#,
        formatted.as_code()
    );

    fs::write(default_configuration, configuration)?;

    fs::write(
        project_root().join("website/src/content/docs/internals/changelog.mdx"),
        changelog,
    )?;

    if VERSION != "0.0.0" {
        let parser = biome_command();
        let markdown = parser.render_markdown("biome");
        let mut cli_content =
            fs::read_to_string(project_root().join("website/src/content/docs/reference/cli.mdx"))?;

        let start = "\n[//]: # (Start-codegen)\n";
        let end = "\n[//]: # (End-codegen)";

        debug_assert!(cli_content.contains(start));
        debug_assert!(cli_content.contains(end));

        let start_index = cli_content
            .find(start)
            .expect("To contain start placeholder")
            + start.len();
        let end_index = cli_content.find(end).expect("To contain end placeholder");

        cli_content.replace_range(start_index..end_index, &markdown);

        fs::write(
            project_root().join("website/src/content/docs/reference/cli.mdx"),
            format!("{cli_content}"),
        )?;
        let schema_root_folder = project_root().join("website/src/pages/schemas");
        let schema_version_folder = schema_root_folder.join(VERSION);
        let schema_js_file = schema_version_folder.join("schema.json.js");
        if schema_version_folder.exists() {
            fs::remove_file(schema_js_file.clone())?;
            fs::remove_dir(schema_version_folder.clone())?;
        }
        fs::create_dir(schema_version_folder.clone())?;
        let mut content = String::new();
        let schema_content = fs::read_to_string(schema_path_npm)?;
        content.push_str(
            r#"// Run `BIOME_VERSION=<version number> cargo codegen-website
// to generate a new schema
export function GET() {"#,
        );
        content.push_str(&format!("const schema  = {};", schema_content));
        content.push_str(
            r#"return new Response(JSON.stringify(schema), {
            status: 200,
            headers: {
                "content-type": "application/json"
            }
        })
    }"#,
        );
        let node = parse_module(&content, JsParserOptions::default());
        let result = biome_js_formatter::format_node(
            JsFormatOptions::new(JsFileSource::js_module()),
            &node.syntax(),
        )
        .unwrap();
        fs::write(schema_js_file.clone(), result.print().unwrap().as_code())?;
    }

    Ok(())
}
