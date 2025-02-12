use std::path::PathBuf;
use std::{collections::BTreeMap, path::Path};

use anyhow::{Context, Ok, Result};
use biome_string_case::Case;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use xtask::{glue::fs2, project_root, reformat};

pub fn generate_analyzer() -> Result<()> {
    generate_js_analyzer()?;
    generate_json_analyzer()?;
    generate_css_analyzer()?;
    generate_graphql_analyzer()?;
    Ok(())
}

fn generate_js_analyzer() -> Result<()> {
    let base_path = project_root().join("crates/biome_js_analyze/src");
    let mut analyzers = BTreeMap::new();
    generate_category("lint", &mut analyzers, &base_path)?;
    generate_category("assist", &mut analyzers, &base_path)?;
    generate_category("syntax", &mut analyzers, &base_path)?;

    generate_options(&base_path)?;

    update_js_registry_builder(analyzers)
}

fn generate_json_analyzer() -> Result<()> {
    let base_path = project_root().join("crates/biome_json_analyze/src");
    let mut analyzers = BTreeMap::new();
    generate_category("lint", &mut analyzers, &base_path)?;
    generate_category("assist", &mut analyzers, &base_path)?;

    generate_options(&base_path)?;
    update_json_registry_builder(analyzers)
}

fn generate_css_analyzer() -> Result<()> {
    let base_path = project_root().join("crates/biome_css_analyze/src");
    let mut analyzers = BTreeMap::new();
    generate_category("lint", &mut analyzers, &base_path)?;
    generate_category("assist", &mut analyzers, &base_path)?;

    generate_options(&base_path)?;
    update_css_registry_builder(analyzers)
}

fn generate_graphql_analyzer() -> Result<()> {
    let base_path = project_root().join("crates/biome_graphql_analyze/src");
    let mut analyzers = BTreeMap::new();
    generate_category("lint", &mut analyzers, &base_path)?;
    generate_options(&base_path)?;
    update_graphql_registry_builder(analyzers)
}

fn generate_options(base_path: &Path) -> Result<()> {
    let mut rules_options = BTreeMap::new();
    let mut crates = vec![];
    for category in ["lint", "assist"] {
        let category_path = base_path.join(category);
        if !category_path.exists() {
            continue;
        }
        let category_name = format_ident!("{}", filename(&category_path)?);
        for group_path in list_entry_paths(&category_path)?.filter(|path| path.is_dir()) {
            let group_name = format_ident!("{}", filename(&group_path)?.to_string());
            for rule_path in list_entry_paths(&group_path)?.filter(|path| !path.is_dir()) {
                let rule_filename = filename(&rule_path)?;
                let rule_name = Case::Pascal.convert(rule_filename);
                let rule_module_name = format_ident!("{}", rule_filename);
                let rule_name = format_ident!("{}", rule_name);
                rules_options.insert(rule_filename.to_string(), quote! {
                    pub type #rule_name = <#category_name::#group_name::#rule_module_name::#rule_name as biome_analyze::Rule>::Options;
                });
            }
        }
        if category == "lint" {
            crates.push(quote! {
                use crate::lint;
            })
        } else if category == "assist" {
            crates.push(quote! {
                use crate::assist;
            })
        }
    }
    let rules_options = rules_options.values();
    let tokens = xtask::reformat(quote! {
        #( #crates )*

        #( #rules_options )*
    })?;
    let tokens = reformat(tokens)?;
    fs2::write(base_path.join("options.rs"), tokens)?;

    Ok(())
}

fn generate_category(
    name: &'static str,
    entries: &mut BTreeMap<&'static str, TokenStream>,
    base_path: &Path,
) -> Result<()> {
    let path = base_path.join(name);

    let mut groups = BTreeMap::new();
    for entry in fs2::read_dir(path)? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }

        let entry = entry.path();
        let file_name = entry
            .file_stem()
            .context("path has no file name")?
            .to_str()
            .context("could not convert file name to string")?;

        generate_group(name, file_name, base_path)?;

        let module_name = format_ident!("{}", file_name);
        let group_name = format_ident!("{}", Case::Pascal.convert(file_name));

        groups.insert(
            file_name.to_string(),
            (
                quote! {
                   pub mod #module_name;
                },
                quote! {
                    self::#module_name::#group_name
                },
            ),
        );
    }

    let key = name;
    let module_name = format_ident!("{name}");

    let category_name = Case::Pascal.convert(name);
    let category_name = format_ident!("{category_name}");

    let kind = match name {
        "syntax" => format_ident!("Syntax"),
        "lint" => format_ident!("Lint"),
        "assist" => format_ident!("Action"),
        _ => panic!("unimplemented analyzer category {name:?}"),
    };

    entries.insert(
        key,
        quote! {
            registry.record_category::<crate::#module_name::#category_name>();
        },
    );

    let (modules, paths): (Vec<_>, Vec<_>) = groups.into_values().unzip();
    let tokens = xtask::reformat(quote! {
        #( #modules )*
        ::biome_analyze::declare_category! {
            pub #category_name {
                kind: #kind,
                groups: [
                    #( #paths, )*
                ]
            }
        }
    })?;

    let tokens = reformat(tokens)?;
    fs2::write(base_path.join(format!("{name}.rs")), tokens)?;

    Ok(())
}

fn generate_group(category: &'static str, group: &str, base_path: &Path) -> Result<()> {
    let path = base_path.join(category).join(group);

    let mut rules = BTreeMap::new();
    for entry in fs2::read_dir(path)? {
        let entry = entry?.path();
        let file_name = entry
            .file_stem()
            .context("path has no file name")?
            .to_str()
            .context("could not convert file name to string")?;

        let rule_type = Case::Pascal.convert(file_name);

        let key = rule_type.clone();
        let module_name = format_ident!("{}", file_name);
        let rule_type = format_ident!("{}", rule_type);

        rules.insert(
            key,
            (
                quote! {
                    pub mod #module_name;
                },
                quote! {
                    self::#module_name::#rule_type
                },
            ),
        );
    }

    let group_name = format_ident!("{}", Case::Pascal.convert(group));

    let (rule_imports, rule_names): (Vec<_>, Vec<_>) = rules.into_values().unzip();

    let (import_macro, use_macro) = match category {
        "lint" => (
            quote!(
                use biome_analyze::declare_lint_group
            ),
            quote!(declare_lint_group),
        ),
        "assist" => (
            quote!(
                use biome_analyze::declare_assist_group
            ),
            quote!(declare_assist_group),
        ),
        "syntax" => (
            quote!(
                use biome_analyze::declare_syntax_group
            ),
            quote!(declare_syntax_group),
        ),

        _ => panic!("Category not supported: {category}"),
    };
    let tokens = xtask::reformat(quote! {
        #import_macro;

        #( #rule_imports )*


        #use_macro! {
            pub #group_name {
                name: #group,
                 rules: [
                    #( #rule_names,  )*
                 ]
            }
        }
    })?;

    let tokens = reformat(tokens)?;
    fs2::write(base_path.join(category).join(format!("{group}.rs")), tokens)?;

    Ok(())
}

fn update_js_registry_builder(analyzers: BTreeMap<&'static str, TokenStream>) -> Result<()> {
    let path = project_root().join("crates/biome_js_analyze/src/registry.rs");

    let categories = analyzers.into_values();

    let tokens = xtask::reformat(quote! {
        use biome_analyze::RegistryVisitor;
        use biome_js_syntax::JsLanguage;

        pub fn visit_registry<V: RegistryVisitor<JsLanguage>>(registry: &mut V) {
            #( #categories )*
        }
    })?;

    fs2::write(path, tokens)?;

    Ok(())
}

fn update_json_registry_builder(analyzers: BTreeMap<&'static str, TokenStream>) -> Result<()> {
    let path = project_root().join("crates/biome_json_analyze/src/registry.rs");

    let categories = analyzers.into_values();

    let tokens = xtask::reformat(quote! {
        use biome_analyze::RegistryVisitor;
        use biome_json_syntax::JsonLanguage;

        pub fn visit_registry<V: RegistryVisitor<JsonLanguage>>(registry: &mut V) {
            #( #categories )*
        }
    })?;

    let tokens = reformat(tokens)?;
    fs2::write(path, tokens)?;

    Ok(())
}

fn update_css_registry_builder(analyzers: BTreeMap<&'static str, TokenStream>) -> Result<()> {
    let path = project_root().join("crates/biome_css_analyze/src/registry.rs");

    let categories = analyzers.into_values();

    let tokens = xtask::reformat(quote! {
        use biome_analyze::RegistryVisitor;
        use biome_css_syntax::CssLanguage;

        pub fn visit_registry<V: RegistryVisitor<CssLanguage>>(registry: &mut V) {
            #( #categories )*
        }
    })?;

    let tokens = reformat(tokens)?;
    fs2::write(path, tokens)?;

    Ok(())
}

fn update_graphql_registry_builder(analyzers: BTreeMap<&'static str, TokenStream>) -> Result<()> {
    let path = project_root().join("crates/biome_graphql_analyze/src/registry.rs");

    let categories = analyzers.into_values();

    let tokens = xtask::reformat(quote! {
        use biome_analyze::RegistryVisitor;
        use biome_graphql_syntax::GraphqlLanguage;

        pub fn visit_registry<V: RegistryVisitor<GraphqlLanguage>>(registry: &mut V) {
            #( #categories )*
        }
    })?;

    fs2::write(path, tokens)?;

    Ok(())
}

/// Returns file paths of the given directory.
fn list_entry_paths(dir: &Path) -> Result<impl Iterator<Item = PathBuf>> {
    Ok(fs2::read_dir(dir)
        .context("A directory is expected")?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path()))
}

/// Returns filename if any.
fn filename(file: &Path) -> Result<&str> {
    file.file_stem()
        .context("path has no file name")?
        .to_str()
        .context("could not convert file name to string")
}
