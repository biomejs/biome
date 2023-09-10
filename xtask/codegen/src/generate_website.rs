use biome_cli::biome_command;
use biome_js_syntax::JsFileSource;
use rome_js_formatter::context::JsFormatOptions;
use rome_js_formatter::format_node;
use rome_js_parser::{parse_module, JsParserOptions};
use rome_service::VERSION;
use std::fs;
use xtask::{project_root, Result};

const VSCODE_FRONTMATTER: &str = r#"---
title: VSCode extension
description: Notes about the Biome's VSCode extension
---
"#;

const CHANGELOG_FRONTMATTER: &str = r#"---
title: Changelog
description: The changelog of Biome
tableOfContents:
    maxHeadingLevel: 2
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
    let changelog = fs::read_to_string(project_root().join("CHANGELOG.md"))?;
    fs::remove_file(project_root().join("website/src/content/docs/reference/vscode.mdx")).ok();
    fs::remove_file(project_root().join("website/src/content/docs/internals/changelog.mdx")).ok();
    let vscode = format!("{VSCODE_FRONTMATTER}{readme}");
    let changelog = format!("{CHANGELOG_FRONTMATTER}{changelog}");
    fs::write(
        project_root().join("website/src/content/docs/reference/vscode.mdx"),
        vscode,
    )?;
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
