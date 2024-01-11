use std::collections::BTreeMap;
use std::path::PathBuf;

use anyhow::{Context, Ok, Result};
use case::CaseExt;
use proc_macro2::{Punct, Spacing, TokenStream};
use quote::{format_ident, quote};
use xtask::{glue::fs2, project_root};

pub fn generate_analyzer() -> Result<()> {
    generate_js_analyzer()?;
    generate_json_analyzer()?;
    generate_css_analyzer()?;
    Ok(())
}

fn generate_js_analyzer() -> Result<()> {
    let base_path = project_root().join("crates/biome_js_analyze/src");
    let mut analyzers = BTreeMap::new();
    generate_category("analyzers", &mut analyzers, base_path.clone())?;

    let mut semantic_analyzers = BTreeMap::new();
    generate_category(
        "semantic_analyzers",
        &mut semantic_analyzers,
        base_path.clone(),
    )?;

    let mut aria_analyzers = BTreeMap::new();
    generate_category("aria_analyzers", &mut aria_analyzers, base_path.clone())?;

    let mut assists = BTreeMap::new();
    generate_category("assists", &mut assists, base_path.clone())?;

    let mut syntax = BTreeMap::new();
    generate_category("syntax", &mut syntax, base_path)?;

    update_js_registry_builder(
        analyzers,
        semantic_analyzers,
        aria_analyzers,
        assists,
        syntax,
    )
}

fn generate_json_analyzer() -> Result<()> {
    let mut analyzers = BTreeMap::new();
    generate_category(
        "analyzers",
        &mut analyzers,
        project_root().join("crates/biome_json_analyze/src"),
    )?;

    update_css_registry_builder(analyzers)
}

fn generate_css_analyzer() -> Result<()> {
    let mut analyzers = BTreeMap::new();
    generate_category(
        "analyzers",
        &mut analyzers,
        project_root().join("crates/biome_css_analyze/src"),
    )?;

    update_json_registry_builder(analyzers)
}

fn generate_category(
    name: &'static str,
    entries: &mut BTreeMap<&'static str, TokenStream>,
    base_path: PathBuf,
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

        generate_group(name, file_name, base_path.clone())?;

        let module_name = format_ident!("{}", file_name);
        let group_name = format_ident!("{}", to_pascal_case(file_name)?);

        groups.insert(
            file_name.to_string(),
            (
                quote! {
                   pub(crate) mod #module_name;
                },
                quote! {
                    self::#module_name::#group_name
                },
            ),
        );
    }

    let key = name;
    let module_name = format_ident!("{name}");

    let category_name = to_pascal_case(name).unwrap();
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
            pub(crate) #category_name {
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

fn generate_group(category: &'static str, group: &str, base_path: PathBuf) -> Result<()> {
    let path = base_path.join(category).join(group);

    let mut rules = BTreeMap::new();
    for entry in fs2::read_dir(path)? {
        let entry = entry?.path();
        let file_name = entry
            .file_stem()
            .context("path has no file name")?
            .to_str()
            .context("could not convert file name to string")?;

        let rule_type = file_name.to_camel();

        let key = rule_type.clone();
        let module_name = format_ident!("{}", file_name);
        let rule_type = format_ident!("{}", rule_type);

        rules.insert(
            key,
            (
                quote! {
                    pub(crate) mod #module_name;
                },
                quote! {
                    self::#module_name::#rule_type
                },
            ),
        );
    }

    let group_name = format_ident!("{}", to_pascal_case(group)?);

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
            #sp4 pub(crate) #group_name { #nl
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

fn to_pascal_case(input: &str) -> Result<String> {
    let mut result = String::new();
    let mut chars = input.char_indices();

    while let Some((index, mut char)) = chars.next() {
        if index == 0 {
            char = char.to_ascii_uppercase();
        }

        if char == '_' {
            let (_, next_char) = chars.next().context("iterator is empty")?;
            char = next_char.to_ascii_uppercase();
        }

        result.push(char);
    }

    Ok(result)
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
