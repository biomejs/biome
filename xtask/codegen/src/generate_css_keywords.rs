use anyhow::{Context, bail};
use biome_string_case::StrLikeExtension;
use proc_macro2::Literal;
use quote::quote;
use serde::Deserialize;
use std::collections::BTreeSet;
use ureq::get;
use xtask_codegen::update;
use xtask_glue::{Mode, Result, project_root};

const WEBREF_CSS_VERSION: &str = "8.7.1";
const WEBREF_CSS_URL: &str = "https://unpkg.com/@webref/css@8.7.1/css.json";

#[derive(Debug, Deserialize)]
struct CssData {
    types: Vec<CssType>,
}

#[derive(Debug, Deserialize)]
struct CssType {
    name: String,
    syntax: Option<String>,
}

struct ColorKeywords {
    named: BTreeSet<String>,
    system: BTreeSet<String>,
    all: BTreeSet<String>,
}

fn type_syntax<'a>(data: &'a CssData, name: &str) -> Result<&'a str> {
    data.types
        .iter()
        .find(|ty| ty.name == name)
        .with_context(|| format!("Missing `{name}` CSS type"))?
        .syntax
        .as_deref()
        .with_context(|| format!("Missing syntax for `{name}` CSS type"))
}

fn extract_keywords(syntax: &str) -> Result<BTreeSet<String>> {
    syntax
        .split('|')
        .map(str::trim)
        .map(|keyword| {
            if keyword.is_empty()
                || !keyword
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
            {
                bail!("Expected a CSS keyword, found `{keyword}`");
            }
            Ok(keyword.to_ascii_lowercase_cow().into_owned())
        })
        .collect()
}

fn extract_color_keywords(data: &CssData) -> Result<ColorKeywords> {
    let named = extract_keywords(type_syntax(data, "named-color")?)?;
    let deprecated = extract_keywords(type_syntax(data, "deprecated-color")?)?;

    let mut system = BTreeSet::new();
    for alternative in type_syntax(data, "system-color")?.split('|').map(str::trim) {
        if alternative == "<deprecated-color>" {
            system.extend(deprecated.iter().cloned());
        } else {
            system.extend(extract_keywords(alternative)?);
        }
    }

    let mut all = named.union(&system).cloned().collect::<BTreeSet<_>>();
    all.insert("currentcolor".to_string());

    Ok(ColorKeywords { named, system, all })
}

fn literals(values: &BTreeSet<String>) -> Vec<Literal> {
    values.iter().map(|value| Literal::string(value)).collect()
}

fn generate_code(keywords: ColorKeywords) -> proc_macro2::TokenStream {
    let named = literals(&keywords.named);
    let system = literals(&keywords.system);
    let all = literals(&keywords.all);
    quote! {
        /// CSS `<named-color>` keywords.
        pub const NAMED_COLORS: &[&str] = &[#(#named),*];

        /// CSS `<system-color>` keywords, including deprecated system colors.
        pub const SYSTEM_COLORS: &[&str] = &[#(#system),*];

        /// Identifier keywords accepted by the CSS `<color>` type.
        pub const COLOR_KEYWORDS: &[&str] = &[#(#all),*];

    }
}

pub fn generate_css_keywords(mode: Mode) -> Result<()> {
    eprintln!("Fetching @webref/css {WEBREF_CSS_VERSION} data from {WEBREF_CSS_URL}");

    let mut response = get(WEBREF_CSS_URL)
        .call()
        .context("Failed to fetch @webref/css data")?;
    let data: CssData = response
        .body_mut()
        .read_json()
        .context("Failed to parse @webref/css JSON")?;
    let keywords = extract_color_keywords(&data)?;
    let tokens = generate_code(keywords);
    let content = xtask_glue::reformat_with_command(tokens, "just gen-css-keywords")?;
    let output_path = project_root().join("crates/biome_css_syntax/src/keywords/generated.rs");

    update(&output_path, &content, &mode)?;
    Ok(())
}
