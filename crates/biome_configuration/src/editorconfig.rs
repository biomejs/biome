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

use std::str::FromStr;

use biome_diagnostics::Error;
use biome_formatter::{IndentStyle, IndentWidth, LineEnding, ParseFormatNumberError};

use crate::{
    Configuration, FormatterConfiguration, OverrideFormatterConfiguration, OverrideGlobs,
    OverridePattern, Overrides,
    diagnostics::{EditorConfigDiagnostic, ParseFailedDiagnostic},
};

pub fn parse_str(s: &str) -> Result<EditorConfig, EditorConfigDiagnostic> {
    EditorConfig::from_str(s).map_err(|err| {
        EditorConfigDiagnostic::ParseFailed(ParseFailedDiagnostic {
            source: Some(Error::from(err)),
        })
    })
}

#[derive(Debug)]
pub enum EditorConfigErrorKind {
    StaticStr(&'static str),
    ParseBoolError(std::str::ParseBoolError),
    ParseFormatNumberError(ParseFormatNumberError),
    // A section must end with `]`
    MissingSectionEnd,
}
impl std::fmt::Display for EditorConfigErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StaticStr(s) => f.write_str(s),
            Self::ParseBoolError(error) => {
                write!(f, "{}", error)
            }
            Self::ParseFormatNumberError(error) => {
                write!(f, "{}", error)
            }
            Self::MissingSectionEnd => f.write_str("a section must be closed with `]`"),
        }
    }
}

#[derive(Debug)]
pub struct EditorConfigError {
    line_number: u32,
    kind: EditorConfigErrorKind,
}
impl biome_diagnostics::Diagnostic for EditorConfigError {
    fn category(&self) -> Option<&'static biome_diagnostics::Category> {
        Some(biome_diagnostics::category!("configuration"))
    }

    fn severity(&self) -> crate::Severity {
        crate::Severity::Error
    }

    fn description(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self)
    }

    fn message(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_markup(biome_console::markup!({ AsConsoleDisplay(&self) }))
    }
}
impl biome_console::fmt::Display for EditorConfigError {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        write!(fmt, "{}", self)
    }
}
impl std::fmt::Display for EditorConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line {}: {}", self.line_number, self.kind)
    }
}

struct AsConsoleDisplay<'a, T>(&'a T);
impl<T: std::fmt::Display> biome_console::fmt::Display for AsConsoleDisplay<'_, T> {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_fmt(format_args!("{}", self.0))
    }
}

/// Represents a parsed .editorconfig file, containing only the options that are relevant to biome.
#[derive(Debug, Clone, Default)]
pub struct EditorConfig {
    pub root: bool,
    pub sections: Vec<EditorConfigSection>,
}

#[derive(Debug, Clone, Default)]
pub struct EditorConfigSection {
    pub glob: String,
    pub options: EditorConfigOptions,
}

impl FromStr for EditorConfig {
    type Err = EditorConfigError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut root = false;
        let mut sections = Vec::new();
        let mut last_section: Option<(&str, EditorConfigOptions)> = None;
        for (line_number, line) in (1..).zip(s.lines()) {
            let line = line.trim_ascii();
            match line.bytes().next() {
                Some(b'#' | b';') => {
                    // Ignore comments
                }
                Some(b'[') => {
                    if let Some((section_name, options)) = last_section.take() {
                        if !options.is_all_none() {
                            sections.push(EditorConfigSection {
                                glob: section_name.to_string(),
                                options,
                            });
                        }
                    }
                    if let Some(section_name) = line[1..].strip_suffix(']') {
                        last_section = Some((section_name, EditorConfigOptions::default()))
                    } else {
                        return Err(EditorConfigError {
                            line_number,
                            kind: EditorConfigErrorKind::MissingSectionEnd,
                        });
                    }
                }
                Some(b'a'..=b'z' | b'A'..=b'Z') => {
                    if let Some((key, value)) = line.split_once('=') {
                        let key = key.trim_ascii_end();
                        let value = value.trim_ascii();
                        if let Some((_, last_options)) = &mut last_section {
                            if key.eq_ignore_ascii_case("indent_style") {
                                last_options.indent_style = EditorconfigValue::from_str(value)
                                    .map_err(|err| EditorConfigError {
                                        line_number,
                                        kind: EditorConfigErrorKind::StaticStr(err),
                                    })?;
                            } else if key.eq_ignore_ascii_case("indent_size") {
                                last_options.indent_size = EditorconfigValue::from_str(value)
                                    .map_err(|err| EditorConfigError {
                                        line_number,
                                        kind: EditorConfigErrorKind::ParseFormatNumberError(err),
                                    })?;
                            } else if key.eq_ignore_ascii_case("end_of_line") {
                                last_options.end_of_line = EditorconfigValue::from_str(value)
                                    .map_err(|err| EditorConfigError {
                                        line_number,
                                        kind: EditorConfigErrorKind::StaticStr(err),
                                    })?;
                            } else if key.eq_ignore_ascii_case("insert_final_newline") {
                                last_options.insert_final_newline =
                                    EditorconfigValue::from_str(value).map_err(|err| {
                                        EditorConfigError {
                                            line_number,
                                            kind: EditorConfigErrorKind::ParseBoolError(err),
                                        }
                                    })?;
                            }
                        } else if key.eq_ignore_ascii_case("root") {
                            root = bool::from_str(value).map_err(|err| EditorConfigError {
                                line_number,
                                kind: EditorConfigErrorKind::ParseBoolError(err),
                            })?;
                        }
                    }
                }
                _ => {}
            }
        }
        if let Some((section_name, options)) = last_section.take() {
            if !options.is_all_none() {
                sections.push(EditorConfigSection {
                    glob: section_name.to_string(),
                    options,
                });
            }
        }
        Ok(Self { root, sections })
    }
}

impl EditorConfig {
    pub fn to_biome(mut self) -> (Option<Configuration>, Vec<EditorConfigDiagnostic>) {
        let diagnostics = self.validate();

        let global_index = self
            .sections
            .iter()
            .position(|section| &*section.glob == "*");
        let mut config = Configuration {
            formatter: global_index.map(|index| self.sections.remove(index).options.to_biome()),
            ..Default::default()
        };
        let overrides: Vec<_> = self
            .sections
            .into_iter()
            .filter_map(|section| {
                // Ignore glob patterns that cannot be parsed.
                Some((
                    biome_glob::editorconfig::EditorconfigGlob::try_from(section.glob).ok()?,
                    section.options,
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
        self.sections
            .iter()
            .flat_map(|section| section.options.validate())
            .collect()
    }
}

#[derive(Debug, Clone, Default)]
pub struct EditorConfigOptions {
    pub indent_style: EditorconfigValue<IndentStyle>,
    pub indent_size: EditorconfigValue<IndentWidth>,
    pub end_of_line: EditorconfigValue<LineEnding>,
    // Not a biome option, but we need it to emit a diagnostic when this is set to false.
    pub insert_final_newline: EditorconfigValue<bool>,
}

impl EditorConfigOptions {
    pub fn is_all_none(&self) -> bool {
        self.indent_style.is_none()
            && self.indent_size.is_none()
            && self.end_of_line.is_none()
            && self.insert_final_newline.is_none()
    }

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
#[derive(Debug, Clone, Default)]
pub enum EditorconfigValue<T> {
    /// The value was explicitly specified.
    Explicit(T),
    /// Use the default value for this option. This occurs when the value is `unset`.
    Default,
    /// The value was not specified.
    #[default]
    None,
}
impl<T> EditorconfigValue<T> {
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}
impl<T: FromStr> FromStr for EditorconfigValue<T> {
    type Err = T::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "unset" | "off" => Ok(EditorconfigValue::Default),
            _ => T::from_str(s).map(EditorconfigValue::Explicit),
        }
    }
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
        let global_options = conf
            .sections
            .into_iter()
            .find(|section| section.glob == "*")
            .expect("has section")
            .options;
        assert!(matches!(
            global_options.indent_style,
            EditorconfigValue::Default
        ));
        assert!(matches!(
            global_options.indent_size,
            EditorconfigValue::Default
        ));
        assert!(matches!(
            global_options.end_of_line,
            EditorconfigValue::Default
        ));
    }

    #[test]
    fn test_inner_section_brackets() {
        let input = r#"
root = true

[a[abc]a]
indent_style=space
"#;

        let conf = parse_str(input).expect("Failed to parse editorconfig");
        let options = conf
            .sections
            .into_iter()
            .find(|section| section.glob == "a[abc]a")
            .expect("has section")
            .options;
        assert!(matches!(
            options.indent_style,
            EditorconfigValue::Explicit(IndentStyle::Space)
        ));
    }
}
