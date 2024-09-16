//! Helpers for handling .editorconfig files.
//!
//! Here's how the options are mapped to the formatter config:
//!
//! | .editorconfig option | biome option |
//! |----------------------|--------------|
//! | indent_style         | indent_style |
//! | indent_size          | indent_width |
//! | end_of_line          | line_ending  |
//! | max_line_length      | line_width   |

use std::{collections::HashMap, str::FromStr};

use biome_diagnostics::{adapters::IniError, Error};
use biome_formatter::{IndentStyle, IndentWidth, LineEnding, LineWidth};
use serde::{Deserialize, Deserializer};

use crate::{
    diagnostics::{EditorConfigDiagnostic, ParseFailedDiagnostic},
    OverrideFormatterConfiguration, OverridePattern, Overrides, PartialConfiguration,
    PartialFormatterConfiguration,
};

pub fn parse_str(s: &str) -> Result<EditorConfig, EditorConfigDiagnostic> {
    // TODO: use serde_path_to_error to emit better parse diagnostics
    serde_ini::from_str(s).map_err(|err| {
        EditorConfigDiagnostic::ParseFailed(ParseFailedDiagnostic {
            source: Some(Error::from(IniError::from(err))),
        })
    })
}

/// Represents a parsed .editorconfig file, containing only the options that are relevant to biome.
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct EditorConfig {
    #[serde(deserialize_with = "deserialize_bool_from_string")]
    root: bool,
    #[serde(flatten)]
    options: HashMap<String, EditorConfigOptions>,
}

impl EditorConfig {
    pub fn to_biome(mut self) -> (Option<PartialConfiguration>, Vec<EditorConfigDiagnostic>) {
        let diagnostics = self.validate();

        let mut config = PartialConfiguration {
            formatter: self.options.remove("*").map(|o| o.to_biome()),
            ..Default::default()
        };
        let mut errors = vec![];
        let overrides: Vec<_> = self
            .options
            .into_iter()
            .map(|(k, v)| {
                let patterns = match expand_unknown_glob_patterns(&k) {
                    Ok(patterns) => patterns.into_iter().map(hack_convert_double_star).collect(),
                    Err(err) => {
                        errors.push(err);
                        vec![k]
                    }
                };

                OverridePattern {
                    include: Some(patterns.into_iter().collect()),
                    formatter: Some(v.to_biome_override()),
                    ..Default::default()
                }
            })
            .collect();
        config.overrides = Some(Overrides(overrides));

        (Some(config), diagnostics)
    }

    fn validate(&self) -> Vec<EditorConfigDiagnostic> {
        let errors: Vec<_> = self.options.values().flat_map(|o| o.validate()).collect();

        errors
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct EditorConfigOptions {
    #[serde(deserialize_with = "deserialize_optional_value_from_string")]
    indent_style: EditorconfigValue<IndentStyle>,
    #[serde(deserialize_with = "deserialize_optional_value_from_string")]
    indent_size: EditorconfigValue<IndentWidth>,
    #[serde(deserialize_with = "deserialize_optional_value_from_string")]
    end_of_line: EditorconfigValue<LineEnding>,
    #[serde(deserialize_with = "deserialize_optional_value_from_string")]
    max_line_length: EditorconfigValue<LineWidth>,
    // Not a biome option, but we need it to emit a diagnostic when this is set to false.
    #[serde(deserialize_with = "deserialize_optional_bool_from_string")]
    insert_final_newline: Option<bool>,
}

impl EditorConfigOptions {
    pub fn to_biome(self) -> PartialFormatterConfiguration {
        PartialFormatterConfiguration {
            indent_style: self.indent_style.into(),
            indent_width: self.indent_size.into(),
            line_ending: self.end_of_line.into(),
            line_width: self.max_line_length.into(),
            ..Default::default()
        }
    }

    pub fn to_biome_override(self) -> OverrideFormatterConfiguration {
        OverrideFormatterConfiguration {
            indent_style: self.indent_style.into(),
            indent_width: self.indent_size.into(),
            line_ending: self.end_of_line.into(),
            line_width: self.max_line_length.into(),
            ..Default::default()
        }
    }

    fn validate(&self) -> Vec<EditorConfigDiagnostic> {
        let mut errors = vec![];
        // `insert_final_newline = false` results in formatting behavior that is incompatible with biome
        if let Some(false) = self.insert_final_newline {
            errors.push(EditorConfigDiagnostic::incompatible(
                "insert_final_newline",
                "Biome always inserts a final newline. Set this option to true.",
            ));
        }
        errors
    }
}

/// Represents a value in an .editorconfig file.
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(untagged)]
pub enum EditorconfigValue<T> {
    /// The value was explicitly specified.
    Explicit(T),
    /// Use the default value for this option. This occurs when the value is `unset`.
    Default,
    /// The value was not specified.
    #[default]
    None,
}

// This is an `Into` because implementing `From` is not possible because you can't implement traits for a type you don't own.
#[allow(clippy::from_over_into)]
impl<T: Default> Into<Option<T>> for EditorconfigValue<T> {
    fn into(self) -> Option<T> {
        match self {
            EditorconfigValue::Explicit(v) => Some(v),
            EditorconfigValue::Default => Some(T::default()),
            EditorconfigValue::None => None,
        }
    }
}

fn deserialize_bool_from_string<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "false" => Ok(false),
        "true" => Ok(true),
        _ => Err(serde::de::Error::custom("expected 'true' or 'false'")),
    }
}

fn deserialize_optional_bool_from_string<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    deserialize_bool_from_string(deserializer).map(Some)
}

fn deserialize_optional_value_from_string<'de, D, T>(
    deserializer: D,
) -> Result<EditorconfigValue<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: std::fmt::Display,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "unset" | "off" => Ok(EditorconfigValue::Default),
        _ => T::from_str(s.as_str())
            .map_err(serde::de::Error::custom)
            .map(EditorconfigValue::Explicit),
    }
}

/// Turn an unknown glob pattern into a list of known glob patterns. This is part of a hack to support all editorconfig patterns.
///
/// TODO: remove in biome 2.0
fn expand_unknown_glob_patterns(pattern: &str) -> Result<Vec<String>, EditorConfigDiagnostic> {
    struct Variants {
        /// index of the { character
        start: usize,
        /// index of the } character
        end: usize,
        variants: Option<VariantType>,
    }

    impl Variants {
        fn new(start: usize) -> Self {
            Self {
                start,
                end: start,
                variants: None,
            }
        }

        fn parse_to_variants(&mut self, s: &str) -> Result<(), EditorConfigDiagnostic> {
            let s = s.trim_start_matches('{').trim_end_matches('}');
            if s.contains("..") {
                let mut parts = s.split("..");
                let start = parts.next().ok_or_else(|| {
                    EditorConfigDiagnostic::invalid_glob_pattern(
                        s,
                        "Range pattern must have exactly two parts",
                    )
                })?;
                let end = parts.next().ok_or_else(|| {
                    EditorConfigDiagnostic::invalid_glob_pattern(
                        s,
                        "Range pattern must have exactly two parts",
                    )
                })?;
                if parts.next().is_some() {
                    return Err(EditorConfigDiagnostic::invalid_glob_pattern(
                        s,
                        "Range pattern must have exactly two parts",
                    ));
                }

                let start = start.parse().map_err(|err| {
                    EditorConfigDiagnostic::invalid_glob_pattern(
                        s,
                        format!("Error parsing the start of the range: {err}"),
                    )
                })?;
                let end = end.parse().map_err(|err| {
                    EditorConfigDiagnostic::invalid_glob_pattern(
                        s,
                        format!("Error parsing the end of the range: {err}"),
                    )
                })?;
                self.variants = Some(VariantType::Range((start, end)));
            } else {
                self.variants = Some(VariantType::List(
                    s.split(',').map(|s| s.to_string()).collect(),
                ));
            }

            Ok(())
        }

        fn variants(&self) -> Vec<String> {
            match &self.variants {
                Some(VariantType::List(ref list)) => list.clone(),
                Some(VariantType::Range((start, end))) => {
                    let mut variants = vec![];
                    for i in *start..=*end {
                        variants.push(i.to_string());
                    }
                    variants
                }
                None => vec![],
            }
        }
    }

    enum VariantType {
        List(Vec<String>),
        Range((i64, i64)),
    }

    let mut all_variants = vec![];
    let mut current_variants = None;
    for (i, c) in pattern.chars().enumerate() {
        match c {
            '{' => {
                if current_variants.is_none() {
                    current_variants = Some(Variants::new(i));
                } else {
                    // TODO: error, recursive brace expansion is not supported
                }
            }
            '}' => {
                if let Some(mut v) = current_variants.take() {
                    v.end = i;
                    v.parse_to_variants(&pattern[v.start..=v.end])?;
                    all_variants.push(v);
                }
            }
            _ => {}
        }
    }

    if all_variants.is_empty() {
        return Ok(vec![pattern.to_string()]);
    }

    let mut expanded_patterns = vec![];
    for variants in all_variants.iter().rev() {
        if expanded_patterns.is_empty() {
            for variant in &variants.variants() {
                let mut pattern = pattern.to_string();
                pattern.replace_range(variants.start..=variants.end, variant);
                expanded_patterns.push(pattern);
            }
        } else {
            let mut new_patterns = vec![];
            for existing in &expanded_patterns {
                for variant in &variants.variants() {
                    let mut pattern = existing.clone();
                    pattern.replace_range(variants.start..=variants.end, variant);
                    new_patterns.push(pattern);
                }
            }
            expanded_patterns = new_patterns;
        }
    }

    Ok(expanded_patterns)
}

/// The EditorConfig spec allows for patterns like `**.yml`, which is not supported by biome. This function corrects such patterns so that they can be parsed by biome's glob parser.
fn hack_convert_double_star(pattern: impl Into<String>) -> String {
    pattern
        .into()
        .split('/')
        .map(|component| {
            if component == "**" {
                component.to_string()
            } else if component.contains("**") {
                component.replace("**", "**/*")
            } else {
                component.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_editorconfig() {
        // the example from https://editorconfig.org/
        let input = r#"
# EditorConfig is awesome: https://EditorConfig.org

# top-most EditorConfig file
root = true

# Unix-style newlines with a newline ending every file
[*]
end_of_line = lf
insert_final_newline = true

# Matches multiple files with brace expansion notation
# Set default charset
[*.{js,py}]
charset = utf-8

# 4 space indentation
[*.py]
indent_style = space
indent_size = 4

# Tab indentation (no size specified)
[Makefile]
indent_style = tab

# Indentation override for all JS under lib directory
[lib/**.js]
indent_style = space
indent_size = 2

# Matches the exact files either package.json or .travis.yml
[{package.json,.travis.yml}]
indent_style = space
indent_size = 2
"#;

        let conf = parse_str(input).expect("Failed to parse editorconfig");
        assert!(conf.root);
    }

    #[test]
    fn should_convert_to_biome_root_settings() {
        let input = r#"
root = true

[*]
insert_final_newline = true
end_of_line = crlf
indent_style = space
indent_size = 4
max_line_length = 80
"#;

        let conf = parse_str(input).expect("Failed to parse editorconfig");
        let (conf, _) = conf.to_biome();
        let conf = conf.expect("Failed to convert editorconfig to biome");
        let formatter = conf.formatter.expect("Formatter not set");
        assert_eq!(formatter.indent_style, Some(IndentStyle::Space));
        assert_eq!(formatter.indent_width.unwrap().value(), 4);
        assert_eq!(formatter.line_ending, Some(LineEnding::Crlf));
        assert_eq!(formatter.line_width.map(|v| v.value()), Some(80));
    }

    #[test]
    fn should_emit_diagnostic_incompatible() {
        let input = r#"
root = true

[*]
insert_final_newline = false
"#;

        let conf = parse_str(input).expect("Failed to parse editorconfig");
        let (_, errors) = conf.to_biome();
        assert_eq!(errors.len(), 1);
        assert!(matches!(errors[0], EditorConfigDiagnostic::Incompatible(_)));
    }

    #[test]
    fn should_parse_editorconfig_with_unset_values() {
        let input = r#"
root = true

[*]
indent_style = unset
indent_size = unset
end_of_line = unset
max_line_length = unset
"#;

        let conf = parse_str(input).expect("Failed to parse editorconfig");
        assert!(matches!(
            conf.options["*"].indent_style,
            EditorconfigValue::Default
        ));
        assert!(matches!(
            conf.options["*"].indent_size,
            EditorconfigValue::Default
        ));
        assert!(matches!(
            conf.options["*"].end_of_line,
            EditorconfigValue::Default
        ));
        assert!(matches!(
            conf.options["*"].max_line_length,
            EditorconfigValue::Default
        ));
    }

    #[test]
    fn should_parse_editorconfig_with_max_line_length_off() {
        let input = r#"
root = true

[*]
max_line_length = off
"#;

        let conf = parse_str(input).expect("Failed to parse editorconfig");
        assert!(matches!(
            conf.options["*"].max_line_length,
            EditorconfigValue::Default,
        ));
    }

    #[test]
    fn should_expand_glob_pattern_list() {
        let pattern = "package.json";
        let mut expanded =
            expand_unknown_glob_patterns(pattern).expect("Failed to expand glob pattern");
        expanded.sort();
        assert_eq!(expanded, vec!["package.json"]);

        let pattern = "{package.json,.travis.yml}";
        let mut expanded =
            expand_unknown_glob_patterns(pattern).expect("Failed to expand glob pattern");
        expanded.sort();
        assert_eq!(expanded, vec![".travis.yml", "package.json"]);
    }

    #[test]
    fn should_expand_glob_pattern_list_2() {
        let pattern = "**/{foo,bar}.{test,spec}.js";
        let mut expanded =
            expand_unknown_glob_patterns(pattern).expect("Failed to expand glob pattern");
        expanded.sort();
        assert_eq!(
            expanded,
            vec![
                "**/bar.spec.js",
                "**/bar.test.js",
                "**/foo.spec.js",
                "**/foo.test.js",
            ]
        );
    }

    #[test]
    fn should_expand_glob_pattern_range() {
        let pattern = "**/bar.{1..4}.js";
        let mut expanded =
            expand_unknown_glob_patterns(pattern).expect("Failed to expand glob pattern");
        expanded.sort();
        assert_eq!(
            expanded,
            vec!["**/bar.1.js", "**/bar.2.js", "**/bar.3.js", "**/bar.4.js",]
        );
    }

    #[test]
    fn should_correct_double_star() {
        let pattern = "**.yml";
        let corrected = hack_convert_double_star(pattern);
        assert_eq!(corrected, "**/*.yml",);

        let pattern = "**/*.yml";
        let corrected = hack_convert_double_star(pattern);
        assert_eq!(corrected, "**/*.yml",);
    }
}
