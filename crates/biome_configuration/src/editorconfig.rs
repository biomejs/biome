//! Helpers for handling .editorconfig files.
//!
//! Here's how the options are mapped to the formatter config:
//!
//! | .editorconfig option | biome option |
//! |----------------------|--------------|
//! | indent_style         | indent_style |
//! | indent_size          | indent_width |
//! | end_of_line          | line_ending  |

use crate::{
    Configuration, FormatterConfiguration, OverrideFormatterConfiguration, OverrideGlobs,
    OverridePattern, Overrides, diagnostics::EditorConfigDiagnostic,
};
use biome_formatter::{IndentStyle, IndentWidth, LineEnding, ParseFormatNumberError};
use biome_rowan::TextRange;
use std::fmt::{Debug, Display};
use std::num::ParseIntError;
use std::str::FromStr;

/// Error type returned when parsing a [IndentSize] value
pub enum ParseIndentSizeError {
    /// The value could not be parsed to a number or `tab`
    ParseError(ParseIntError),
    /// The `u8` value of the string is not a valid [IndentSize]
    TryFromU8Error(IndentSizeFromIntError),
}

impl From<ParseIntError> for ParseIndentSizeError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseError(value)
    }
}

impl From<IndentSizeFromIntError> for ParseIndentSizeError {
    fn from(value: IndentSizeFromIntError) -> Self {
        Self::TryFromU8Error(value)
    }
}

impl Debug for ParseIndentSizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl std::error::Error for ParseIndentSizeError {}

impl std::fmt::Display for ParseIndentSizeError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseError(err) => std::fmt::Display::fmt(err, fmt),
            Self::TryFromU8Error(err) => std::fmt::Display::fmt(err, fmt),
        }
    }
}

/// Config for `.editorconfig`, it can set to a number or string value `tab`.
/// The difference between [IndentSize] with [IndentWidth] is that the latter does not support `tab`.
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum IndentSize {
    Tab,
    Value(u8),
}

impl IndentSize {
    pub const MIN: u8 = 0;
    pub const MAX: u8 = 24;
}

impl Default for IndentSize {
    fn default() -> Self {
        Self::Value(2)
    }
}

/// Error type returned when converting a u8 to a [IndentSize] fails
#[derive(Clone, Copy, Debug)]
pub struct IndentSizeFromIntError(pub u8);

impl std::fmt::Display for IndentSizeFromIntError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "The indent_size should be between {} and {}, got {}",
            IndentSize::MIN,
            IndentSize::MAX,
            self.0,
        )
    }
}

impl std::error::Error for IndentSizeFromIntError {}

/// Error type returned when converting an invalid string to a [IndentSize]
#[derive(Clone, Debug)]
pub struct InvalidIndentSize(pub &'static str);

impl std::fmt::Display for InvalidIndentSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Invalid `indent_size` value: a positive integer between `0` and `24` or `tab` is expected."
        )
    }
}

impl FromStr for IndentSize {
    type Err = ParseIndentSizeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "tab" => Ok(Self::Tab),
            s => {
                let val = u8::from_str(s).map_err(ParseIndentSizeError::ParseError)?;

                if (Self::MIN..=Self::MAX).contains(&val) {
                    Ok(Self::Value(val))
                } else {
                    Err(ParseIndentSizeError::TryFromU8Error(
                        IndentSizeFromIntError(val),
                    ))
                }
            }
        }
    }
}

impl From<u8> for IndentSize {
    fn from(value: u8) -> Self {
        Self::Value(value)
    }
}

impl Display for IndentSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tab => std::write!(f, "Tab"),
            Self::Value(val) => f.write_str(&std::format!("{val}")),
        }
    }
}

impl TryFrom<IndentSize> for IndentWidth {
    type Error = ParseFormatNumberError;

    fn try_from(value: IndentSize) -> Result<Self, Self::Error> {
        match value {
            IndentSize::Tab => Self::try_from(4).map_err(ParseFormatNumberError::TryFromU8Error),
            IndentSize::Value(val) => {
                Self::try_from(val).map_err(ParseFormatNumberError::TryFromU8Error)
            }
        }
    }
}

#[derive(Debug, biome_diagnostics::Diagnostic)]
#[diagnostic(category = "configuration", severity = Error)]
pub struct EditorConfigError {
    #[description]
    #[message]
    pub kind: EditorConfigErrorKind,
    #[location(span)]
    pub span: TextRange,
}

#[derive(Debug, Default)]
pub enum EditorConfigErrorKind {
    ParseFormatNumberError(ParseFormatNumberError),
    // A section must end with `]`
    #[default]
    MissingSectionEnd,
    InvalidBooleanValue,
    InvalidEndOfLineValue,
    InvalidIndentStyleValue,
    InvalidIndentSizeValue(ParseIndentSizeError),
}
impl biome_console::fmt::Display for EditorConfigErrorKind {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        write!(fmt, "{self}",)
    }
}
impl std::fmt::Display for EditorConfigErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseFormatNumberError(error) => {
                write!(f, "{error}",)
            }
            Self::MissingSectionEnd => f.write_str("A section must be closed with `]`."),
            Self::InvalidBooleanValue => {
                f.write_str("Invalid boolean value: `true` or `false` is expected.")
            }
            Self::InvalidEndOfLineValue => {
                f.write_str("Invalid `end_of_line` value: `lf`, `cr`, or `crlf` is expected.")
            }
            Self::InvalidIndentStyleValue => {
                f.write_str("Invalid `ident_style` value: `space` or `tab` is expected.")
            }
            Self::InvalidIndentSizeValue(error) => {
                write!(f, "{error}",)
            }
        }
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
        let mut offset = 0;
        let mut root = false;
        let mut sections = Vec::new();
        let mut last_section: Option<(&str, EditorConfigOptions)> = None;
        // We use `s.split('\n')` instead of `s.lines()` because the latter splits on `\n` and `\r\n`.
        // This could be an issue for updating `offset`.
        for line in s.split('\n') {
            match line.trim_ascii_start().bytes().next() {
                Some(b'#' | b';') => {
                    // Ignore comments
                }
                Some(b'[') => {
                    if let Some((section_name, options)) = last_section.take()
                        && !options.is_all_none()
                    {
                        sections.push(EditorConfigSection {
                            glob: section_name.to_string(),
                            options,
                        });
                    }
                    if let Some(section_name) = line[1..].trim_ascii().strip_suffix(']') {
                        last_section = Some((section_name, EditorConfigOptions::default()))
                    } else {
                        let end = offset + line.len() as u32;
                        return Err(EditorConfigError {
                            kind: EditorConfigErrorKind::MissingSectionEnd,
                            span: TextRange::new(offset.into(), end.into()),
                        });
                    }
                }
                Some(b'a'..=b'z' | b'A'..=b'Z') => {
                    if let Some((key, val)) = line.split_once('=') {
                        let val = val.trim_ascii_end();
                        let val_len = val.len() as u32;
                        let val = val.trim_ascii_start();
                        // `+ 1` for the equal sign `=`.
                        let val_end = offset + key.len() as u32 + 1 + val_len;
                        let val_start = val_end - val.len() as u32;
                        let key = key.trim_ascii();
                        if let Some((_, last_options)) = &mut last_section {
                            if key.eq_ignore_ascii_case("indent_style") {
                                last_options.indent_style = EditorconfigValue::from_str(val)
                                    .map_err(|_| EditorConfigError {
                                        kind: EditorConfigErrorKind::InvalidIndentStyleValue,
                                        span: TextRange::new(val_start.into(), val_end.into()),
                                    })?;
                            } else if key.eq_ignore_ascii_case("indent_size") {
                                last_options.indent_size = EditorconfigValue::from_str(val)
                                    .map_err(|error| EditorConfigError {
                                        kind: EditorConfigErrorKind::InvalidIndentSizeValue(error),
                                        span: TextRange::new(val_start.into(), val_end.into()),
                                    })?;
                            } else if key.eq_ignore_ascii_case("end_of_line") {
                                last_options.end_of_line = EditorconfigValue::from_str(val)
                                    .map_err(|_| EditorConfigError {
                                        kind: EditorConfigErrorKind::InvalidEndOfLineValue,
                                        span: TextRange::new(val_start.into(), val_end.into()),
                                    })?;
                            } else if key.eq_ignore_ascii_case("insert_final_newline") {
                                last_options.insert_final_newline =
                                    EditorconfigValue::from_str(val).map_err(|_| {
                                        EditorConfigError {
                                            kind: EditorConfigErrorKind::InvalidBooleanValue,
                                            span: TextRange::new(val_start.into(), val_end.into()),
                                        }
                                    })?;
                            }
                        } else if key.eq_ignore_ascii_case("root") {
                            root = bool::from_str(val).map_err(|_| EditorConfigError {
                                kind: EditorConfigErrorKind::InvalidBooleanValue,
                                span: TextRange::new(val_start.into(), val_end.into()),
                            })?;
                        }
                    }
                }
                _ => {}
            }
            // `+ 1` for the newline `\n`
            offset += line.len() as u32 + 1;
        }
        if let Some((section_name, options)) = last_section.take()
            && !options.is_all_none()
        {
            sections.push(EditorConfigSection {
                glob: section_name.to_string(),
                options,
            });
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
    pub indent_size: EditorconfigValue<IndentSize>,
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
        let indent_size: Option<IndentSize> = self.indent_size.into();
        let indent_width = indent_size.map(IndentWidth::try_from).and_then(Result::ok);

        FormatterConfiguration {
            indent_style: self.indent_style.into(),
            indent_width,
            line_ending: self.end_of_line.into(),
            ..Default::default()
        }
    }

    pub fn to_biome_override(self) -> OverrideFormatterConfiguration {
        let indent_size: Option<IndentSize> = self.indent_size.into();
        let indent_width = indent_size.map(IndentWidth::try_from).and_then(Result::ok);

        OverrideFormatterConfiguration {
            indent_style: self.indent_style.into(),
            indent_width,
            line_ending: self.end_of_line.into(),
            ..Default::default()
        }
    }

    fn validate(&self) -> Vec<EditorConfigDiagnostic> {
        let mut diagnostics = vec![];
        // `insert_final_newline = false` results in formatting behavior that is incompatible with biome
        if let EditorconfigValue::Explicit(false) = self.insert_final_newline {
            diagnostics.push(EditorConfigDiagnostic::incompatible(
                "insert_final_newline",
                "Biome always inserts a final newline. Set this option to true.",
            ));
        }
        diagnostics
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
            "unset" | "off" => Ok(Self::Default),
            _ => T::from_str(s).map(EditorconfigValue::Explicit),
        }
    }
}

// This is an `Into` because implementing `From` is not possible because you can't implement traits for a type you don't own.
#[expect(clippy::from_over_into)]
impl<T: Default> Into<Option<T>> for EditorconfigValue<T> {
    fn into(self) -> Option<T> {
        match self {
            Self::Explicit(v) => Some(v),
            Self::Default => Some(T::default()),
            Self::None => None,
        }
    }
}
