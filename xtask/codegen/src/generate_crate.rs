use std::fs;
use xtask::*;

fn cargo_template(name: &str) -> String {
    format!(
        r#"
[package]
authors.workspace    = true
categories.workspace = true
description          = "<DESCRIPTION>"
edition.workspace    = true
homepage.workspace   = true
keywords.workspace   = true
license.workspace    = true
name                 = "{name}"
repository.workspace = true
version              = "0.0.0"

[lints]
workspace = true
"#
    )
}

fn knope_template(name: &str) -> String {
    format!(
        r#"
[packages.{name}]
versioned_files = ["crates/{name}/Cargo.toml"]
changelog = "crates/{name}/CHANGELOG.md"
"#
    )
}

pub fn generate_crate(crate_name: String) -> Result<()> {
    let crate_root = project_root().join("crates").join(crate_name.as_str());
    let cargo_file = crate_root.join("Cargo.toml");
    let knope_config = project_root().join("knope.toml");

    let mut knope_contents = fs::read_to_string(&knope_config)?;
    fs::write(cargo_file, cargo_template(crate_name.as_str()))?;
    let start_content = "## Rust crates. DO NOT CHANGE!\n";
    let end_content = "\n## End of crates. DO NOT CHANGE!";
    debug_assert!(
        knope_contents.contains(start_content),
        "The file knope.toml must contains `{start_content}`"
    );
    debug_assert!(
        knope_contents.contains(end_content),
        "The file knope.toml must contains `{end_content}`"
    );

    let file_start_index = knope_contents.find(start_content).unwrap() + start_content.len();
    let file_end_index = knope_contents.find(end_content).unwrap();
    let crates_text = &knope_contents[file_start_index..file_end_index];
    let template = knope_template(crate_name.as_str());
    let new_crates_text: Vec<_> = crates_text.lines().chain(Some(&template[..])).collect();
    let new_crates_text = new_crates_text.join("\n");

    knope_contents.replace_range(file_start_index..file_end_index, &new_crates_text);
    fs::write(knope_config, knope_contents)?;
    Ok(())
}
