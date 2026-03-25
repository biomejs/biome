use anyhow::Context;
use proc_macro2::Literal;
use quote::quote;
use serde::Deserialize;
use std::collections::BTreeMap;
use ureq::get;
use xtask_codegen::update;
use xtask_glue::{Mode, Result, project_root};

const WEB_FEATURES_DATA_URL: &str = "https://unpkg.com/web-features/data.json";

#[derive(Debug, Deserialize)]
struct WebFeaturesData {
    features: BTreeMap<String, FeatureEntry>,
}

/// A single entry in `features`. We only care about `"feature"` kind entries.
#[derive(Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
enum FeatureEntry {
    Feature(Feature),
    /// Redirected to another feature — ignored.
    #[serde(other)]
    Other,
}

#[derive(Debug, Deserialize)]
struct Feature {
    status: FeatureStatus,
    #[serde(default)]
    compat_features: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct FeatureStatus {
    baseline: BaselineValue,
    baseline_low_date: Option<String>,
}

/// The `baseline` field can be `"high"`, `"low"`, or `false`.
#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(untagged)]
enum BaselineValue {
    Named(BaselineNamed),
    Limited(bool), // always false when present
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
enum BaselineNamed {
    High,
    Low,
}

/// Extract the year from an optional baseline_low_date string.
fn extract_year(low_date: Option<&str>) -> Option<u16> {
    let date = low_date?;
    // Dates are formatted as "YYYY-MM-DD" or "≤YYYY-MM-DD"
    let date = date.trim_start_matches('≤');
    date.get(..4).and_then(|y| y.parse::<u16>().ok())
}

#[derive(Debug, Clone, Copy)]
struct Status {
    tier: Tier,
    year: Option<u16>,
}

#[derive(Debug, Clone, Copy)]
enum Tier {
    Limited,
    Newly,
    Widely,
}

fn map_status(baseline: BaselineValue, low_date: Option<&str>) -> Status {
    let tier = match baseline {
        BaselineValue::Named(BaselineNamed::High) => Tier::Widely,
        BaselineValue::Named(BaselineNamed::Low) => Tier::Newly,
        BaselineValue::Limited(_) => Tier::Limited,
    };
    Status {
        tier,
        year: extract_year(low_date),
    }
}

/// Regex-free pattern matching for CSS compat keys.
/// Returns `None` if the key doesn't match the given category.
fn match_css_property(key: &str) -> Option<&str> {
    // css.properties.<property>  (no further dots — excludes value sub-keys)
    let rest = key.strip_prefix("css.properties.")?;
    if rest.contains('.') {
        return None;
    }
    Some(rest)
}

fn match_css_property_value(key: &str) -> Option<(&str, &str)> {
    // css.properties.<property>.<value>  (exactly one more level, no underscores in value)
    let rest = key.strip_prefix("css.properties.")?;
    let (prop, val) = rest.split_once('.')?;
    // ESLint skips keys where value contains underscore (those are descriptions, not syntax)
    if val.contains('_') {
        return None;
    }
    // Exclude any further nesting
    if val.contains('.') {
        return None;
    }
    Some((prop, val))
}

fn match_css_at_rule(key: &str) -> Option<&str> {
    // css.at-rules.<name>  (no further dots)
    let rest = key.strip_prefix("css.at-rules.")?;
    if rest.contains('.') {
        return None;
    }
    Some(rest)
}

fn match_css_media_condition(key: &str) -> Option<&str> {
    // css.at-rules.media.<condition>  (exactly one more level)
    let rest = key.strip_prefix("css.at-rules.media.")?;
    if rest.contains('.') {
        return None;
    }
    Some(rest)
}

fn match_css_selector(key: &str) -> Option<&str> {
    // css.selectors.<name>  (no further dots)
    let rest = key.strip_prefix("css.selectors.")?;
    if rest.contains('.') {
        return None;
    }
    Some(rest)
}

fn match_css_function(key: &str) -> Option<&str> {
    // css.types.<optional-prefix.><name>
    // We only want entries that correspond to actual CSS functions (end in "()")
    // in the mdn-data CSS functions list — but we don't have mdn-data here.
    // ESLint uses `mdn-data` to check if `name + "()"` is a known CSS function.
    // We replicate by checking the known set of CSS type keys that are functions.
    // The key format is css.types.<something>  or css.types.<parent>.<something>
    let rest = key.strip_prefix("css.types.")?;
    // Take the last segment
    let name = rest.split('.').next_back()?;
    // Heuristic: if it ends with no underscore and is a single segment,
    // it's a candidate. We'll validate against known function names below.
    Some(name)
}

// ── Main extraction ──────────────────────────────────────────────────────────

struct CssFeatures {
    /// (property_name, status)  — sorted by name
    properties: Vec<(String, Status)>,
    /// (property_name, value_name, status)  — sorted by (prop, value)
    property_values: Vec<(String, String, Status)>,
    /// (at_rule_name, status)  — sorted by name
    at_rules: Vec<(String, Status)>,
    /// (media_condition_name, status)  — sorted by name
    media_conditions: Vec<(String, Status)>,
    /// (function_name, status)  — sorted by name
    functions: Vec<(String, Status)>,
    /// (selector_name, status)  — sorted by name
    selectors: Vec<(String, Status)>,
}

/// Properties that aren't considered baseline but have wide support.
/// https://github.com/web-platform-dx/web-features/issues/1038#issuecomment-2627370691
const WIDE_SUPPORT_PROPERTIES: &[&str] = &["cursor"];

/// Compare baseline tiers: Widely > Newly > Limited.
fn tier_rank(tier: Tier) -> u8 {
    match tier {
        Tier::Limited => 0,
        Tier::Newly => 1,
        Tier::Widely => 2,
    }
}

/// Insert or update the map to keep the highest-tier status.
fn update_best<K: Ord>(map: &mut BTreeMap<K, Status>, key: K, status: Status) {
    map.entry(key)
        .and_modify(|existing| {
            if tier_rank(status.tier) > tier_rank(existing.tier) {
                *existing = status;
            }
        })
        .or_insert(status);
}

fn extract_css_features(data: &WebFeaturesData) -> CssFeatures {
    let mut properties: BTreeMap<String, Status> = BTreeMap::new();
    let mut property_values: BTreeMap<(String, String), Status> = BTreeMap::new();
    let mut at_rules: BTreeMap<String, Status> = BTreeMap::new();
    let mut media_conditions: BTreeMap<String, Status> = BTreeMap::new();
    let mut functions_map: BTreeMap<String, Status> = BTreeMap::new();
    let mut selectors: BTreeMap<String, Status> = BTreeMap::new();

    for entry in data.features.values() {
        let feature = match entry {
            FeatureEntry::Feature(f) => f,
            FeatureEntry::Other => continue,
        };

        // Use the feature-level status for all compat keys.
        // The per-compat-key status may differ and is less reliable.
        // ESLint's generate-baseline.js uses the feature-level status.
        let feature_status = map_status(
            feature.status.baseline,
            feature.status.baseline_low_date.as_deref(),
        );

        for key in &feature.compat_features {
            if !key.starts_with("css.") {
                continue;
            }
            let status = feature_status;

            if let Some(prop) = match_css_property(key) {
                if !WIDE_SUPPORT_PROPERTIES.contains(&prop) {
                    update_best(&mut properties, prop.to_string(), status);
                }
                continue;
            }

            if let Some((prop, val)) = match_css_property_value(key) {
                if !WIDE_SUPPORT_PROPERTIES.contains(&prop) {
                    update_best(
                        &mut property_values,
                        (prop.to_string(), val.to_string()),
                        status,
                    );
                }
                continue;
            }

            if let Some(at_rule) = match_css_at_rule(key) {
                update_best(&mut at_rules, at_rule.to_string(), status);
                continue;
            }

            if let Some(condition) = match_css_media_condition(key) {
                update_best(&mut media_conditions, condition.to_string(), status);
                continue;
            }

            if let Some(name) = match_css_function(key) {
                // Only include if it appears to be a function (not a generic type)
                // We use the same set ESLint uses from mdn-data css.functions
                if is_known_css_function(name) {
                    update_best(&mut functions_map, name.to_string(), status);
                }
                continue;
            }

            if let Some(sel) = match_css_selector(key) {
                update_best(&mut selectors, sel.to_string(), status);
            }
        } // inner loop over compat_features
    } // outer loop over features

    CssFeatures {
        properties: properties.into_iter().collect(),
        property_values: property_values
            .into_iter()
            .map(|((p, v), s)| (p, v, s))
            .collect(),
        at_rules: at_rules.into_iter().collect(),
        media_conditions: media_conditions.into_iter().collect(),
        functions: functions_map.into_iter().collect(),
        selectors: selectors.into_iter().collect(),
    }
}

// This mirrors what ESLint extracts from mdn-data's css.functions list.
// These are function names (without parentheses) that exist as CSS value functions.

fn is_known_css_function(name: &str) -> bool {
    // This list is derived from mdn-data/css/functions.json keys (strip parens)
    const KNOWN: &[&str] = &[
        "abs",
        "acos",
        "annotation",
        "asin",
        "atan",
        "atan2",
        "attr",
        "blur",
        "brightness",
        "calc",
        "calc-size",
        "character-variant",
        "circle",
        "clamp",
        "color",
        "color-mix",
        "conic-gradient",
        "contrast",
        "cos",
        "counter",
        "counters",
        "cross-fade",
        "cubic-bezier",
        "device-cmyk",
        "drop-shadow",
        "element",
        "ellipse",
        "env",
        "exp",
        "fit-content",
        "format",
        "grayscale",
        "hsl",
        "hsla",
        "hue-rotate",
        "hwb",
        "hypot",
        "image",
        "image-set",
        "inset",
        "invert",
        "lab",
        "layer",
        "lch",
        "leader",
        "light-dark",
        "linear",
        "linear-gradient",
        "local",
        "log",
        "matrix",
        "matrix3d",
        "max",
        "min",
        "minmax",
        "mod",
        "oklab",
        "oklch",
        "opacity",
        "ornaments",
        "paint",
        "path",
        "perspective",
        "polygon",
        "pow",
        "radial-gradient",
        "ray",
        "rect",
        "rem",
        "repeat",
        "repeating-conic-gradient",
        "repeating-linear-gradient",
        "repeating-radial-gradient",
        "rgb",
        "rgba",
        "rotate",
        "rotate3d",
        "rotateX",
        "rotateY",
        "rotateZ",
        "round",
        "saturate",
        "scale",
        "scale3d",
        "scaleX",
        "scaleY",
        "scaleZ",
        "sepia",
        "sign",
        "sin",
        "skew",
        "skewX",
        "skewY",
        "sqrt",
        "steps",
        "string",
        "styleset",
        "stylistic",
        "supports",
        "swash",
        "symbols",
        "tan",
        "target-counter",
        "target-counters",
        "target-text",
        "translate",
        "translate3d",
        "translateX",
        "translateY",
        "translateZ",
        "url",
        "var",
        "xywh",
        "anchor",
        "anchor-size",
    ];
    KNOWN.contains(&name)
}

fn tier_tokens(tier: Tier) -> proc_macro2::TokenStream {
    match tier {
        Tier::Widely => quote! { BaselineTier::Widely },
        Tier::Newly => quote! { BaselineTier::Newly },
        Tier::Limited => quote! { BaselineTier::Limited },
    }
}

fn year_tokens(year: Option<u16>) -> proc_macro2::TokenStream {
    match year {
        Some(y) => quote! { BaselineYear::Known(#y) },
        None => quote! { BaselineYear::Unknown },
    }
}

fn status_entry(status: &Status) -> proc_macro2::TokenStream {
    let tier = tier_tokens(status.tier);
    let year = year_tokens(status.year);
    quote! { BaselineStatus { tier: #tier, year: #year } }
}

fn phf_map_entries(items: &[(String, Status)]) -> Vec<proc_macro2::TokenStream> {
    items
        .iter()
        .map(|(name, status)| {
            let name_lit = Literal::string(name);
            let entry = status_entry(status);
            quote! { #name_lit => #entry }
        })
        .collect()
}

fn generate_code(features: CssFeatures) -> proc_macro2::TokenStream {
    let properties = phf_map_entries(&features.properties);
    let at_rules = phf_map_entries(&features.at_rules);
    let media_conditions = phf_map_entries(&features.media_conditions);
    let functions = phf_map_entries(&features.functions);
    let selectors = phf_map_entries(&features.selectors);

    // Property values: flat map keyed by "property:value".
    let property_values: Vec<_> = features
        .property_values
        .iter()
        .map(|(prop, val, status)| {
            let key_lit = Literal::string(&format!("{prop}:{val}"));
            let entry = status_entry(status);
            quote! { #key_lit => #entry }
        })
        .collect();

    quote! {
        /// The Baseline availability tier of a CSS feature.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum BaselineTier {
            /// Not yet supported in all core browsers.
            Limited,
            /// Supported in all core browsers for less than 30 months.
            Newly,
            /// Supported in all core browsers for at least 30 months.
            Widely,
        }

        /// The year a CSS feature became Baseline newly-available.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum BaselineYear {
            /// The year the feature became newly available.
            Known(u16),
            /// No date is recorded for when this feature became available.
            Unknown,
        }

        /// The combined Baseline status of a CSS feature.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct BaselineStatus {
            pub tier: BaselineTier,
            pub year: BaselineYear,
        }

        /// Look up the Baseline status of a feature by name.
        pub fn find_baseline(
            table: &phf::Map<&'static str, BaselineStatus>,
            name: &str,
        ) -> Option<BaselineStatus> {
            table.get(name).copied()
        }

        /// Look up the Baseline status of a specific CSS property value.
        /// The key format is `"property:value"` (lowercase).
        pub fn find_property_value_baseline(property: &str, value: &str) -> Option<BaselineStatus> {
            let key = format!("{property}:{value}");
            BASELINE_PROPERTY_VALUES.get(key.as_str()).copied()
        }

        /// Baseline status for CSS properties.
        pub static BASELINE_PROPERTIES: phf::Map<&'static str, BaselineStatus> = phf::phf_map! {
            #( #properties ),*
        };

        /// Baseline status for CSS property values (identifier keywords only).
        /// Keys are in `"property:value"` format (lowercase).
        pub static BASELINE_PROPERTY_VALUES: phf::Map<&'static str, BaselineStatus> = phf::phf_map! {
            #( #property_values ),*
        };

        /// Baseline status for CSS at-rules.
        pub static BASELINE_AT_RULES: phf::Map<&'static str, BaselineStatus> = phf::phf_map! {
            #( #at_rules ),*
        };

        /// Baseline status for CSS media query conditions.
        pub static BASELINE_MEDIA_CONDITIONS: phf::Map<&'static str, BaselineStatus> = phf::phf_map! {
            #( #media_conditions ),*
        };

        /// Baseline status for CSS value functions.
        pub static BASELINE_FUNCTIONS: phf::Map<&'static str, BaselineStatus> = phf::phf_map! {
            #( #functions ),*
        };

        /// Baseline status for CSS pseudo-class and pseudo-element selectors.
        pub static BASELINE_SELECTORS: phf::Map<&'static str, BaselineStatus> = phf::phf_map! {
            #( #selectors ),*
        };
    }
}

pub fn generate_css_baseline(mode: Mode) -> Result<()> {
    eprintln!("Fetching web-features data from {WEB_FEATURES_DATA_URL}");

    let mut response = get(WEB_FEATURES_DATA_URL)
        .call()
        .context("Failed to fetch web-features data")?;

    let data: WebFeaturesData = response
        .body_mut()
        .read_json()
        .context("Failed to parse web-features JSON")?;

    eprintln!("Loaded {} features", data.features.len());

    let features = extract_css_features(&data);

    eprintln!(
        "Extracted: {} properties, {} property values, {} at-rules, {} media conditions, {} functions, {} selectors",
        features.properties.len(),
        features.property_values.len(),
        features.at_rules.len(),
        features.media_conditions.len(),
        features.functions.len(),
        features.selectors.len(),
    );

    let tokens = generate_code(features);
    let output_path = project_root().join("crates/biome_css_analyze/src/baseline_data.rs");

    let content = &xtask_glue::reformat_with_command(tokens.to_string(), "baseline")?;

    update(output_path.as_path(), content, &mode)?;

    Ok(())
}
