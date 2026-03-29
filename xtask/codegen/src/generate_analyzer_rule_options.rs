use crate::update;
use biome_string_case::Case;
use proc_macro2::{Ident, Span};
use quote::quote;
use std::path::{Path, PathBuf};
use xtask_glue::*;

/// The path to the analyzer rule options directory.
pub fn get_analyzer_rule_options_path() -> PathBuf {
    get_analyzer_rule_options_path_from(project_root().as_path())
}

fn get_analyzer_rule_options_path_from(root: &Path) -> PathBuf {
    root.join("crates/biome_rule_options/src/")
}

/// Generates the options struct for a new analyzer rule.
/// This function creates a new struct with the name `<RuleName>Options` and saves it in the
/// `biome_rule_options` crate.
pub fn generate_analyzer_rule_options(
    rule_name: &str,
    mode: Mode,
    register_mod: bool,
) -> Result<()> {
    let root = project_root();
    generate_analyzer_rule_options_at(root.as_path(), rule_name, mode, register_mod)
}

fn generate_analyzer_rule_options_at(
    root: &Path,
    rule_name: &str,
    mode: Mode,
    register_mod: bool,
) -> Result<()> {
    let struct_name = Ident::new(
        &format!("{}Options", Case::Pascal.convert(rule_name)),
        Span::call_site(),
    );

    let content = quote! {
        use biome_deserialize_macros::{Deserializable, Merge};
        use serde::{Deserialize, Serialize};

        #[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
        #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
        #[serde(rename_all = "camelCase", deny_unknown_fields, default)]
        pub struct #struct_name {}
    };

    let snake_rule_name = Case::Snake.convert(rule_name);
    let file_name = format!("{snake_rule_name}.rs");
    let file_path = get_analyzer_rule_options_path_from(root).join(file_name);

    if file_path.exists() {
        if register_mod {
            register_analyzer_rule_options_at(root, &snake_rule_name)?;
        }
        return Ok(());
    }

    update(
        file_path.as_path(),
        &reformat_without_preamble(content)?,
        &mode,
    )?;

    if register_mod {
        register_analyzer_rule_options_at(root, &snake_rule_name)?;
    }

    Ok(())
}

/// Registers the new analyzer rule options module in the `lib.rs` file of the `biome_rule_options` crate.
/// This is a temporal addition to let configuration codegen properly run afterward.
/// Configuration codegen will re-create the `lib.rs` file and put mods in the right order.
fn register_analyzer_rule_options(mod_name: &str) -> Result<()> {
    register_analyzer_rule_options_at(project_root().as_path(), mod_name)
}

fn register_analyzer_rule_options_at(root: &Path, mod_name: &str) -> Result<()> {
    let lib_path = get_analyzer_rule_options_path_from(root).join("lib.rs");
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs,
        sync::atomic::{AtomicU64, Ordering},
    };

    static NEXT_ID: AtomicU64 = AtomicU64::new(0);

    struct TestDir(PathBuf);

    impl TestDir {
        fn new(name: &str) -> Self {
            let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!("biome-rule-options-{name}-{id}"));
            if path.exists() {
                let _ = fs::remove_dir_all(&path);
            }
            fs::create_dir_all(path.join("crates/biome_rule_options/src")).unwrap();
            Self(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TestDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn existing_option_modules_are_preserved() {
        let temp = TestDir::new("preserve-existing");
        let rule_options_dir = temp.path().join("crates/biome_rule_options/src");
        let lib_path = rule_options_dir.join("lib.rs");
        let options_path = rule_options_dir.join("use_sorted_attributes.rs");

        fs::write(&lib_path, "pub mod use_sorted_attributes;\n").unwrap();
        fs::write(
            &options_path,
            "pub struct UseSortedAttributesOptions { preserved: bool }\n",
        )
        .unwrap();

        generate_analyzer_rule_options_at(
            temp.path(),
            "useSortedAttributes",
            Mode::Overwrite,
            true,
        )
        .unwrap();

        assert_eq!(
            fs::read_to_string(options_path).unwrap(),
            "pub struct UseSortedAttributesOptions { preserved: bool }\n"
        );
        assert_eq!(
            fs::read_to_string(lib_path).unwrap(),
            "pub mod use_sorted_attributes;\n"
        );
    }

    #[test]
    fn missing_option_modules_are_generated() {
        let temp = TestDir::new("generate-missing");
        let rule_options_dir = temp.path().join("crates/biome_rule_options/src");
        let lib_path = rule_options_dir.join("lib.rs");
        let options_path = rule_options_dir.join("use_sorted_attributes.rs");

        fs::write(&lib_path, "").unwrap();

        generate_analyzer_rule_options_at(
            temp.path(),
            "useSortedAttributes",
            Mode::Overwrite,
            true,
        )
        .unwrap();

        let options = fs::read_to_string(options_path).unwrap();
        assert!(options.contains("pub struct UseSortedAttributesOptions {}"));
        assert_eq!(
            fs::read_to_string(lib_path).unwrap(),
            "pub mod use_sorted_attributes;\n"
        );
    }
}
