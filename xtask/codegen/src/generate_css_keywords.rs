use anyhow::{Context, bail};
use biome_string_case::StrLikeExtension;
use proc_macro2::Literal;
use quote::{format_ident, quote};
use serde::Deserialize;
use std::collections::BTreeSet;
use ureq::get;
use xtask_codegen::update;
use xtask_glue::{Mode, Result, project_root};

const WEBREF_CSS_VERSION: &str = "8.7.1";
const WEBREF_CSS_URL: &str = "https://unpkg.com/@webref/css@8.7.1/css.json";

/// At-rules whose blocks contain only descriptors listed by webref.
///
/// `@page` is excluded because its block also accepts properties, which webref does not list as
/// its descriptors. `@position-try` is excluded because webref lists no descriptors for it.
const DESCRIPTOR_AT_RULES: &[&str] = &[
    "@color-profile",
    "@counter-style",
    "@font-face",
    "@font-palette-values",
    "@property",
    "@view-transition",
];

#[derive(Debug, Deserialize)]
struct CssData {
    atrules: Vec<CssAtRule>,
    properties: Vec<CssProperty>,
    types: Vec<CssType>,
}

#[derive(Debug, Deserialize)]
struct CssAtRule {
    name: String,
    descriptors: Vec<CssDescriptor>,
}

#[derive(Debug, Deserialize)]
struct CssDescriptor {
    name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CssProperty {
    name: String,
    legacy_alias_of: Option<String>,
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

struct AtRuleDescriptors {
    /// The at-rule name without `@`, such as `font-face`.
    name: &'static str,
    descriptors: BTreeSet<String>,
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

/// Returns the lowercase form of `name`, or an error when `name` is not a plain CSS identifier.
fn to_identifier(name: &str) -> Result<String> {
    if name.is_empty()
        || !name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
    {
        bail!("Expected a CSS identifier, found `{name}`");
    }
    Ok(name.to_ascii_lowercase_cow().into_owned())
}

fn extract_keywords(syntax: &str) -> Result<BTreeSet<String>> {
    syntax
        .split('|')
        .map(str::trim)
        .map(to_identifier)
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

/// Collects the descriptors of each at-rule in [DESCRIPTOR_AT_RULES].
///
/// A property that webref marks as a legacy name alias of a descriptor's name is collected as a
/// descriptor too. For example, `font-stretch` is collected for `@font-face`, which defines the
/// `font-width` descriptor. Webref records legacy name aliases only for properties, but browsers
/// also accept them as descriptors.
fn extract_at_rule_descriptors(data: &CssData) -> Result<Vec<AtRuleDescriptors>> {
    DESCRIPTOR_AT_RULES
        .iter()
        .map(|&at_rule_name| {
            let at_rule = data
                .atrules
                .iter()
                .find(|at_rule| at_rule.name == at_rule_name)
                .with_context(|| format!("Missing `{at_rule_name}` CSS at-rule"))?;
            let mut descriptors = at_rule
                .descriptors
                .iter()
                .map(|descriptor| to_identifier(&descriptor.name))
                .collect::<Result<BTreeSet<_>>>()?;
            if descriptors.is_empty() {
                bail!("Missing descriptors for `{at_rule_name}` CSS at-rule");
            }
            for property in &data.properties {
                if property
                    .legacy_alias_of
                    .as_ref()
                    .is_some_and(|alias_of| descriptors.contains(alias_of))
                {
                    descriptors.insert(to_identifier(&property.name)?);
                }
            }
            Ok(AtRuleDescriptors {
                name: at_rule_name.trim_start_matches('@'),
                descriptors,
            })
        })
        .collect()
}

fn literals(values: &BTreeSet<String>) -> Vec<Literal> {
    values.iter().map(|value| Literal::string(value)).collect()
}

fn generate_code(
    keywords: ColorKeywords,
    at_rules: &[AtRuleDescriptors],
) -> proc_macro2::TokenStream {
    let named = literals(&keywords.named);
    let system = literals(&keywords.system);
    let all = literals(&keywords.all);
    let descriptors = at_rules.iter().map(|at_rule| {
        let ident = format_ident!(
            "{}_DESCRIPTORS",
            at_rule.name.to_ascii_uppercase().replace('-', "_")
        );
        let doc = format!(" Descriptors of the CSS `@{}` at-rule.", at_rule.name);
        let values = literals(&at_rule.descriptors);
        quote! {
            #[doc = #doc]
            pub const #ident: &[&str] = &[#(#values),*];
        }
    });
    quote! {
        /// CSS `<named-color>` keywords.
        pub const NAMED_COLORS: &[&str] = &[#(#named),*];

        /// CSS `<system-color>` keywords, including deprecated system colors.
        pub const SYSTEM_COLORS: &[&str] = &[#(#system),*];

        /// Identifier keywords accepted by the CSS `<color>` type.
        pub const COLOR_KEYWORDS: &[&str] = &[#(#all),*];

        #(#descriptors)*
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
    let at_rules = extract_at_rule_descriptors(&data)?;
    let tokens = generate_code(keywords, &at_rules);
    let content = xtask_glue::reformat_with_command(tokens, "just gen-css-keywords")?;
    let output_path = project_root().join("crates/biome_css_syntax/src/keywords/generated.rs");

    update(&output_path, &content, &mode)?;
    Ok(())
}
