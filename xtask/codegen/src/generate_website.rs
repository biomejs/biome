use rome_cli::rome_command;
use rome_js_formatter::context::JsFormatOptions;
use rome_js_formatter::format_node;
use rome_js_parser::{parse_module, JsParserOptions};
use rome_js_syntax::JsFileSource;
use rome_service::VERSION;
use std::fs;
use xtask::{project_root, Result};

const FRONTMATTER: &str = r#"---
title: VSCode extension
emoji: 💻
category: reference
description: Notes about the Rome's VSCode extension
---
"#;

const SCHEMA_TEMPLATE: &str = r#"// Run `BIOME_VERSION=<version number> cargo codegen-website
// to generate a new schema
import {readFileSync} from "fs";
import {join, resolve} from "path"

export function get() {
	const schemaPath = resolve(join("..", "packages", "@biomejs", "biome", "configuration_schema.json"));
	const schema = readFileSync(schemaPath, "utf8")

	return new Response(schema, {
		status: 200,
		headers: {
			"content-type": "application/json"
		}
	})
}"#;

/// Generates
pub(crate) fn generate_files() -> Result<()> {
    let readme = fs::read_to_string(project_root().join("editors/vscode/README.md"))?;
    fs::remove_file(project_root().join("website/src/pages/vscode.mdx")).ok();
    let page = format!("{FRONTMATTER}{readme}");
    fs::write(project_root().join("website/src/pages/vscode.mdx"), page)?;

    if VERSION != "0.0.0" {
        let parser = rome_command();
        let markdown = parser.render_markdown("rome");
        let mut cli_content = fs::read_to_string(project_root().join("website/src/pages/cli.mdx"))?;

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
            project_root().join("website/src/pages/cli.mdx"),
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
        let node = parse_module(&SCHEMA_TEMPLATE, JsParserOptions::default());
        let result = format_node(
            JsFormatOptions::new(JsFileSource::js_module()),
            &node.syntax(),
        )
        .unwrap();
        fs::write(schema_js_file.clone(), result.print().unwrap().as_code())?;
    }

    Ok(())
}
