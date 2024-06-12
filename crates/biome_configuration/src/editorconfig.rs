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

use biome_deserialize::StringSet;
use biome_diagnostics::{adapters::IniError, Error};
use biome_formatter::{IndentWidth, LineEnding, LineWidth};
use indexmap::IndexSet;
use serde::{Deserialize, Deserializer};

use crate::{
    diagnostics::{EditorConfigDiagnostic, ParseFailedDiagnostic},
    OverrideFormatterConfiguration, OverridePattern, Overrides, PartialConfiguration,
    PartialFormatterConfiguration, PlainIndentStyle,
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
        let overrides: Vec<_> = self
            .options
            .into_iter()
            .map(|(k, v)| OverridePattern {
                include: Some(StringSet::new(IndexSet::from([k]))),
                formatter: Some(v.to_biome_override()),
                ..Default::default()
            })
            .collect();
        config.overrides = Some(Overrides(overrides));

        (Some(config), diagnostics)
    }

    fn validate(&self) -> Vec<EditorConfigDiagnostic> {
        let mut errors: Vec<_> = self.options.values().flat_map(|o| o.validate()).collect();

        // biome doesn't currently support all the glob patterns that .editorconfig does
        errors.extend(
            self.options
                .keys()
                .filter(|k| k.contains('{') || k.contains('}'))
                .map(|pattern| EditorConfigDiagnostic::unknown_glob_pattern(pattern.clone())),
        );

        errors
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct EditorConfigOptions {
    indent_style: Option<PlainIndentStyle>,
    #[serde(deserialize_with = "deserialize_optional_indent_width_from_string")]
    indent_size: Option<IndentWidth>,
    end_of_line: Option<LineEnding>,
    #[serde(deserialize_with = "deserialize_optional_line_width_from_string")]
    max_line_length: Option<LineWidth>,
    // Not a biome option, but we need it to emit a diagnostic when this is set to false.
    #[serde(deserialize_with = "deserialize_optional_bool_from_string")]
    insert_final_newline: Option<bool>,
}

impl EditorConfigOptions {
    pub fn to_biome(self) -> PartialFormatterConfiguration {
        PartialFormatterConfiguration {
            indent_style: self.indent_style,
            indent_width: self.indent_size,
            line_ending: self.end_of_line,
            line_width: self.max_line_length,
            ..Default::default()
        }
    }

    pub fn to_biome_override(self) -> OverrideFormatterConfiguration {
        OverrideFormatterConfiguration {
            indent_style: self.indent_style,
            indent_width: self.indent_size,
            line_ending: self.end_of_line,
            line_width: self.max_line_length,
            ..Default::default()
        }
    }

    fn validate(&self) -> Vec<EditorConfigDiagnostic> {
        let mut errors = vec![];
        // `insert_final_newline = false` results in formatting behavior that is incompatible with biome
        if let Some(false) = self.insert_final_newline {
            errors.push(EditorConfigDiagnostic::incompatible(
                "insert_final_newline",
                "Biome always inserts a final newline.",
            ));
        }
        errors
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

fn deserialize_optional_indent_width_from_string<'de, D>(
    deserializer: D,
) -> Result<Option<IndentWidth>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    IndentWidth::from_str(s.as_str())
        .map_err(serde::de::Error::custom)
        .map(Some)
}

fn deserialize_optional_line_width_from_string<'de, D>(
    deserializer: D,
) -> Result<Option<LineWidth>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    LineWidth::from_str(s.as_str())
        .map_err(serde::de::Error::custom)
        .map(Some)
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
        assert_eq!(formatter.indent_style, Some(PlainIndentStyle::Space));
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
    fn should_emit_diagnostic_glob_pattern() {
        let input = r#"
root = true

[{package.json,.travis.yml}]
indent_style = space
"#;

        let conf = parse_str(input).expect("Failed to parse editorconfig");
        let (_, errors) = conf.to_biome();
        assert_eq!(errors.len(), 1);
        assert!(matches!(
            errors[0],
            EditorConfigDiagnostic::UnknownGlobPattern(_)
        ));
    }
}
