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

use biome_diagnostics::{Error, IniError};
use biome_formatter::{IndentStyle, IndentWidth, LineEnding};
use serde::{Deserialize, Deserializer};

use crate::{
    diagnostics::{EditorConfigDiagnostic, ParseFailedDiagnostic},
    Configuration, FormatterConfiguration, OverrideFormatterConfiguration, OverrideGlobs,
    OverridePattern, Overrides,
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
    pub fn to_biome(mut self) -> (Option<Configuration>, Vec<EditorConfigDiagnostic>) {
        let diagnostics = self.validate();

        let mut config = Configuration {
            formatter: self.options.remove("*").map(|o| o.to_biome()),
            ..Default::default()
        };
        let overrides: Vec<_> = self
            .options
            .into_iter()
            .filter_map(|(k, v)| {
                // Ignore glob patterns that cannot be parsed.
                Some((
                    biome_glob::editorconfig::EditorconfigGlob::try_from(k).ok()?,
                    v,
                ))
            })
            .map(|(glob, v)| OverridePattern {
                includes: Some(OverrideGlobs::EditorconfigGlob(Box::new(glob))),
                formatter: Some(v.to_biome_override()),
                ..Default::default()
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
    // Not a biome option, but we need it to emit a diagnostic when this is set to false.
    #[serde(deserialize_with = "deserialize_optional_value_from_string")]
    insert_final_newline: EditorconfigValue<bool>,
}

impl EditorConfigOptions {
    pub fn to_biome(self) -> FormatterConfiguration {
        FormatterConfiguration {
            indent_style: self.indent_style.into(),
            indent_width: self.indent_size.into(),
            line_ending: self.end_of_line.into(),
            ..Default::default()
        }
    }

    pub fn to_biome_override(self) -> OverrideFormatterConfiguration {
        OverrideFormatterConfiguration {
            indent_style: self.indent_style.into(),
            indent_width: self.indent_size.into(),
            line_ending: self.end_of_line.into(),
            ..Default::default()
        }
    }

    fn validate(&self) -> Vec<EditorConfigDiagnostic> {
        let mut errors = vec![];
        // `insert_final_newline = false` results in formatting behavior that is incompatible with biome
        if let EditorconfigValue::Explicit(false) = self.insert_final_newline {
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
#[expect(clippy::from_over_into)]
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
"#;

        let conf = parse_str(input).expect("Failed to parse editorconfig");
        let (conf, _) = conf.to_biome();
        let conf = conf.expect("Failed to convert editorconfig to biome");
        let formatter = conf.formatter.expect("Formatter not set");
        assert_eq!(formatter.indent_style, Some(IndentStyle::Space));
        assert_eq!(formatter.indent_width.unwrap().value(), 4);
        assert_eq!(formatter.line_ending, Some(LineEnding::Crlf));
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
insert_final_newline = unset
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
    }
}
