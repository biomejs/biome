use rome_cli::rome_command;
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

const CLI_FRONTMATTER: &str = r#"---
title: CLI
emoji: ⌨️
category: reference
description: Available commands and arguments in the Rome CLI.
---
"#;

const SCHEMA_TEMPLATE: &str = r#"// Run `BIOME_VERSION=<version number> cargo codegen-website
// to generate a new schema
import {readFileSync} from "fs";
import {join, resolve} from "path"

export function get() {
	const schemaPath = resolve(join("..", "npm", "rome", "configuration_schema.json"));
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
        fs::write(
            project_root().join("website/src/pages/cli.mdx"),
            format!("{CLI_FRONTMATTER}{markdown}"),
        )?;
        let schema_root_folder = project_root().join("website/src/pages/schemas");
        let schema_version_folder = schema_root_folder.join(VERSION);
        let schema_js_file = schema_version_folder.join("schema.json.js");
        if schema_version_folder.exists() {
            fs::remove_file(schema_js_file.clone())?;
            fs::remove_dir(schema_version_folder.clone())?;
        }
        fs::create_dir(schema_version_folder.clone())?;
        fs::write(schema_js_file.clone(), SCHEMA_TEMPLATE)?;
    }

    Ok(())
}
