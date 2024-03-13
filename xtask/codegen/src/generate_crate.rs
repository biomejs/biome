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
[package."{name}"]
versioned_files = ["creates/{name}/Cargo.toml"]
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
    knope_contents.push_str(knope_template(crate_name.as_str()).as_str());
    fs::write(knope_config, knope_contents)?;
    Ok(())
}
