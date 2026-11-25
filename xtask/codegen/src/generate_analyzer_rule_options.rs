use crate::update;
use biome_string_case::Case;
use proc_macro2::{Ident, Span};
use quote::quote;
use std::path::PathBuf;
use xtask_glue::*;

/// The path to the analyzer rule options directory.
pub fn get_analyzer_rule_options_path() -> PathBuf {
    project_root().join("crates/biome_rule_options/src/")
}

/// Generates the options for a new analyzer rule.
/// This function creates the options named `<RuleName>Options` and saves them in the
/// `biome_rule_options` crate, as an alias of the options shared by the rules without options.
pub fn generate_analyzer_rule_options(
    rule_name: &str,
    mode: Mode,
    register_mod: bool,
) -> Result<()> {
    let struct_name = Ident::new(
        &format!("{}Options", Case::Pascal.convert(rule_name)),
        Span::call_site(),
    );

    // A rule has nothing to configure until its author adds options,
    // so it shares the options type of the rules without options.
    let content = quote! {
        pub type #struct_name = crate::EmptyRuleOptions;
    };

    let snake_rule_name = Case::Snake.convert(rule_name);
    let file_name = format!("{snake_rule_name}.rs");
    let file_path = get_analyzer_rule_options_path().join(file_name);

    update(
        file_path.as_path(),
        &reformat_without_preamble(content)?,
        &mode,
    )?;

    if register_mod {
        register_analyzer_rule_options(&snake_rule_name)?;
    }

    Ok(())
}

/// Registers the new analyzer rule options module in the `lib.rs` file of the `biome_rule_options` crate.
/// This is a temporal addition to let configuration codegen properly run afterward.
/// Configuration codegen will re-create the `lib.rs` file and put mods in the right order.
fn register_analyzer_rule_options(mod_name: &str) -> Result<()> {
    let lib_path = get_analyzer_rule_options_path().join("lib.rs");
    let current_lib_content = std::fs::read_to_string(&lib_path)?;
    let mod_declaration = format!("pub mod {mod_name};");
    if current_lib_content
        .lines()
        .any(|line| line.trim() == mod_declaration)
    {
        return Ok(());
    }
    std::fs::write(
        &lib_path,
        format!("{current_lib_content}{mod_declaration}\n"),
    )?;
    Ok(())
}
