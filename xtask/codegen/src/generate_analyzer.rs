use std::path::PathBuf;
use std::{collections::BTreeMap, path::Path};

use anyhow::{Context, Ok, Result};
use proc_macro2::{Punct, Spacing, TokenStream};
use quote::{format_ident, quote};
use xtask::{glue::fs2, project_root};

use crate::to_pascal_case;

pub fn generate_analyzer() -> Result<()> {
    generate_js_analyzer()?;
    generate_json_analyzer()?;
    generate_css_analyzer()?;
    Ok(())
}

fn generate_js_analyzer() -> Result<()> {
    let base_path = project_root().join("crates/biome_js_analyze/src");
    let mut analyzers = BTreeMap::new();
    generate_category("analyzers", &mut analyzers, &base_path)?;

    let mut semantic_analyzers = BTreeMap::new();
    generate_category("semantic_analyzers", &mut semantic_analyzers, &base_path)?;

    let mut aria_analyzers = BTreeMap::new();
    generate_category("aria_analyzers", &mut aria_analyzers, &base_path)?;

    let mut assists = BTreeMap::new();
    generate_category("assists", &mut assists, &base_path)?;

    let mut syntax = BTreeMap::new();
    generate_category("syntax", &mut syntax, &base_path)?;

    generate_options(
        &["aria_analyzers", "analyzers", "semantic_analyzers"],
        &base_path,
    )?;

    update_js_registry_builder(
        analyzers,
        semantic_analyzers,
        aria_analyzers,
        assists,
        syntax,
    )
}

fn generate_json_analyzer() -> Result<()> {
    let base_path = project_root().join("crates/biome_json_analyze/src");
    let mut analyzers = BTreeMap::new();
    generate_category("analyzers", &mut analyzers, &base_path)?;

    generate_options(&["analyzers"], &base_path)?;

    update_css_registry_builder(analyzers)
}

fn generate_css_analyzer() -> Result<()> {
    let mut analyzers = BTreeMap::new();
    generate_category(
        "analyzers",
        &mut analyzers,
        &(project_root().join("crates/biome_css_analyze/src")),
    )?;

    update_json_registry_builder(analyzers)
}

fn generate_options(categories: &[&str], base_path: &Path) -> Result<()> {
    let mut category_names = Vec::with_capacity(categories.len());
    let mut rules_options = BTreeMap::new();
    let nl = Punct::new('\n', Spacing::Alone);
    for category in categories {
        let category_path = base_path.join(category);
        let category_name = format_ident!("{}", filename(&category_path)?);
        category_names.push(category_name.clone());
        for group_path in list_entry_paths(&category_path)?.filter(|path| path.is_dir()) {
            let group_name = format_ident!("{}", filename(&group_path)?.to_string());
            for rule_path in list_entry_paths(&group_path)?.filter(|path| !path.is_dir()) {
                let rule_filename = filename(&rule_path)?;
                let rule_name = to_pascal_case(rule_filename);
                let rule_module_name = format_ident!("{}", rule_filename);
                let rule_name = format_ident!("{}", rule_name);
                rules_options.insert(rule_filename.to_string(), quote! {
                    pub type #rule_name = <#category_name::#group_name::#rule_module_name::#rule_name as biome_analyze::Rule>::Options;
                });
            }
        }
    }
    let rules_options = rules_options.values();
    let tokens = xtask::reformat(quote! {
        #( use crate::#category_names; )* #nl #nl

        #( #rules_options )*
    })?;
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
        let group_name = format_ident!("{}", to_pascal_case(file_name));

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

    let category_name = to_pascal_case(name);
    let category_name = format_ident!("{category_name}");

    let kind = match name {
        "syntax" => format_ident!("Syntax"),
        "analyzers" | "semantic_analyzers" | "aria_analyzers" => format_ident!("Lint"),
        "assists" => format_ident!("Action"),
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

        let rule_type = to_pascal_case(file_name);

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

    let group_name = format_ident!("{}", to_pascal_case(group));

    let (rule_imports, rule_names): (Vec<_>, Vec<_>) = rules.into_values().unzip();

    let nl = Punct::new('\n', Spacing::Alone);
    let sp = Punct::new(' ', Spacing::Joint);
    let sp4 = quote! { #sp #sp #sp #sp };
    let tokens = xtask::reformat(quote! {
        use biome_analyze::declare_group;
        #nl #nl
        #( #rule_imports )*
        #nl #nl
        declare_group! { #nl
            #sp4 pub #group_name { #nl
                #sp4 #sp4 name: #group, #nl
                #sp4 #sp4 rules: [ #nl
                    #( #sp4 #sp4 #sp4 #rule_names, #nl )*
                #sp4 #sp4 ] #nl
            #sp4 } #nl
        }
    })?;

    fs2::write(base_path.join(category).join(format!("{group}.rs")), tokens)?;

    Ok(())
}

fn update_js_registry_builder(
    analyzers: BTreeMap<&'static str, TokenStream>,
    semantic_analyzers: BTreeMap<&'static str, TokenStream>,
    aria_analyzers: BTreeMap<&'static str, TokenStream>,
    assists: BTreeMap<&'static str, TokenStream>,
    syntax: BTreeMap<&'static str, TokenStream>,
) -> Result<()> {
    let path = project_root().join("crates/biome_js_analyze/src/registry.rs");

    let categories = analyzers
        .into_iter()
        .chain(semantic_analyzers)
        .chain(aria_analyzers)
        .chain(assists)
        .chain(syntax)
        .map(|(_, tokens)| tokens);

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
