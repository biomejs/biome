use anyhow::{Context, Result, bail};
use biome_string_case::Case;
use camino::Utf8PathBuf;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use std::collections::BTreeMap;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitStr, Token, parse_macro_input};

/// Input for the declare_group_from_fs macro
struct GroupInput {
    category: String,
    group: String,
}

impl Parse for GroupInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut category = None;
        let mut group = None;

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![:]>()?;
            let value: LitStr = input.parse()?;

            match key.to_string().as_str() {
                "category" => category = Some(value.value()),
                "group" => group = Some(value.value()),
                _ => {
                    return Err(syn::Error::new_spanned(
                        key,
                        "Expected 'category' or 'group'",
                    ));
                }
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(GroupInput {
            category: category.ok_or_else(|| {
                syn::Error::new(input.span(), "Missing required 'category' field")
            })?,
            group: group
                .ok_or_else(|| syn::Error::new(input.span(), "Missing required 'group' field"))?,
        })
    }
}

pub fn declare_group_from_fs_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as GroupInput);

    match generate_group_code(&input.category, &input.group) {
        Ok(tokens) => tokens,
        Err(err) => {
            let error_msg = format!("Failed to generate group code: {:#}", err);
            quote! {
                compile_error!(#error_msg);
            }
            .into()
        }
    }
}

fn generate_group_code(category: &str, group: &str) -> Result<TokenStream> {
    // Use CARGO_MANIFEST_DIR to get the base path since proc_macro::Span::source_file
    // is not available on stable Rust
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").context("CARGO_MANIFEST_DIR not set")?;
    let base_path = Utf8PathBuf::from(manifest_dir).join("src").join(category);

    // Discover all rules in the group directory
    let group_dir = base_path.join(group);
    let mut rules = BTreeMap::new();

    for entry in std::fs::read_dir(&group_dir)
        .with_context(|| format!("Failed to read directory: {}", group_dir))?
    {
        let entry = entry?.path();

        // Check if it's a file and has .rs extension
        if !entry.is_file() || entry.extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }

        let file_stem = entry
            .file_stem()
            .context("path has no file name")?
            .to_str()
            .context("could not convert file name to string")?;

        // Skip mod.rs files
        if file_stem == "mod" {
            continue;
        }

        let file_name = file_stem;

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

    if rules.is_empty() {
        bail!("No rules found in directory: {}", group_dir);
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
        _ => bail!("Category not supported: {}", category),
    };

    let tokens = quote! {
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
    };

    // Format the code using prettyplease since we can't call rustfmt in proc macros
    let formatted = format_code(tokens)?;

    // Parse the formatted string back into a TokenStream
    // We need to go through proc_macro2::TokenStream first, then convert to proc_macro::TokenStream
    let token_stream = formatted
        .parse::<proc_macro2::TokenStream>()
        .map_err(|e| anyhow::anyhow!("Failed to parse formatted code as TokenStream: {}", e))?;

    Ok(token_stream.into())
}

fn format_code(tokens: proc_macro2::TokenStream) -> Result<String> {
    // Parse the token stream into a syn file
    let file = syn::parse2::<syn::File>(tokens).context("Failed to parse tokens as syn::File")?;

    // Use prettyplease to format the code
    Ok(prettyplease::unparse(&file))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input: syn::Result<GroupInput> =
            syn::parse_str(r#"category: "lint", group: "nursery""#);
        assert!(input.is_ok());
        let input = input.unwrap();
        assert_eq!(input.category, "lint");
        assert_eq!(input.group, "nursery");
    }

    #[test]
    fn test_parse_input_missing_category() {
        let input: syn::Result<GroupInput> = syn::parse_str(r#"group: "nursery""#);
        assert!(input.is_err());
    }

    #[test]
    fn test_parse_input_missing_group() {
        let input: syn::Result<GroupInput> = syn::parse_str(r#"category: "lint""#);
        assert!(input.is_err());
    }
}
