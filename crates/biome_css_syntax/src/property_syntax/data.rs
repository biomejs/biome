use biome_diagnostics::{Advices, Diagnostic, LogCategory, Visit};
use biome_formatter::formatter::Formatter;
use biome_formatter::{
    Buffer, Format, FormatContext, FormatOptions, FormatResult, IndentStyle, IndentWidth,
    LineEnding, LineWidth, SourceMapGeneration, TrailingNewline, TransformSourceMap,
    prelude::{PrinterOptions, space, text, token},
    write,
};
use biome_rowan::TextRange;

// #region Data structure definition

/// The result of parsing an `@property` `syntax` descriptor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PropertySyntaxResult {
    /// The declaration has no `syntax` descriptor.
    Missing,
    /// The descriptor value does not conform to the registered property syntax grammar.
    Error(PropertySyntaxParseDiagnostic),
    /// The parsed and normalized descriptor value.
    Value(PropertySyntax),
}

impl PropertySyntaxResult {
    /// Compares parsed descriptor semantics without considering source ranges.
    pub fn semantic_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Missing, Self::Missing) => true,
            (Self::Error(left), Self::Error(right)) => left.kind() == right.kind(),
            (
                Self::Value(PropertySyntax::Universal { .. }),
                Self::Value(PropertySyntax::Universal { .. }),
            ) => true,
            (
                Self::Value(PropertySyntax::Components(left)),
                Self::Value(PropertySyntax::Components(right)),
            ) => {
                left.len() == right.len()
                    && left.iter().zip(right).all(|(left, right)| {
                        left.name == right.name && left.multiplier == right.multiplier
                    })
            }
            _ => false,
        }
    }

    pub const fn is_valid(&self) -> bool {
        matches!(self, Self::Value(_))
    }

    pub fn as_valid(&self) -> Option<&PropertySyntax> {
        match self {
            Self::Value(value) => Some(value),
            _ => None,
        }
    }
}

/// Describes a syntax error and its absolute source range.
#[derive(Clone, Debug, Diagnostic, Eq, PartialEq)]
#[diagnostic(category = "parse", severity = Error)]
pub struct PropertySyntaxParseDiagnostic {
    /// The reason parsing failed.
    #[message]
    #[description]
    #[advice]
    kind: PropertySyntaxErrorKind,
    /// The absolute source range that caused the error.
    #[location(span)]
    range: TextRange,
}

impl PropertySyntaxParseDiagnostic {
    /// Creates a parse diagnostic for `kind` at an absolute source `range`.
    pub const fn new(kind: PropertySyntaxErrorKind, range: TextRange) -> Self {
        Self { kind, range }
    }

    /// Returns the reason parsing failed.
    pub const fn kind(&self) -> PropertySyntaxErrorKind {
        self.kind
    }

    /// Returns the absolute source range that caused the error.
    pub const fn range(&self) -> TextRange {
        self.range
    }
}

/// The reason an `@property` `syntax` descriptor could not be parsed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PropertySyntaxErrorKind {
    /// The descriptor contains no syntax components.
    Empty,
    /// A component is missing where the grammar requires one.
    ExpectedComponent,
    /// The descriptor value is not a CSS string.
    ExpectedString,
    /// Adjacent components are not separated by `|`.
    ExpectedPipe,
    /// A data type name is missing, malformed, or unsupported.
    ExpectedTypeName,
    /// A custom identifier is malformed or is a CSS-wide keyword.
    InvalidCustomIdentifier,
    /// The universal syntax is combined with another component or a multiplier.
    InvalidUniversalSyntax,
    /// A multiplier follows the pre-multiplied `<transform-list>` type.
    MultiplierAfterTransformList,
    /// Whitespace occurs where the syntax grammar does not permit it.
    UnexpectedWhitespace,
}

impl std::fmt::Display for PropertySyntaxErrorKind {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => fmt.write_str("The property syntax cannot be empty."),
            Self::ExpectedComponent => fmt.write_str("Expected a property syntax component."),
            Self::ExpectedString => fmt.write_str("Use a string for the property syntax."),
            Self::ExpectedPipe => fmt.write_str("Separate property syntax components with `|`."),
            Self::ExpectedTypeName => fmt.write_str("Use a supported property syntax type."),
            Self::InvalidCustomIdentifier => {
                fmt.write_str("Use a custom identifier that isn't reserved.")
            }
            Self::InvalidUniversalSyntax => {
                fmt.write_str("The `*` can't appear with other syntax components.")
            }
            Self::MultiplierAfterTransformList => {
                fmt.write_str("Remove the multiplier after `<transform-list>`.")
            }
            Self::UnexpectedWhitespace => {
                fmt.write_str("Remove whitespace inside the type name or before the multiplier.")
            }
        }
    }
}

impl biome_console::fmt::Display for PropertySyntaxErrorKind {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        match self {
            Self::Empty => fmt.write_str("The property syntax cannot be empty."),
            Self::ExpectedComponent => fmt.write_str("Expected a property syntax component."),
            Self::ExpectedString => fmt.write_str("Use a string for the property syntax."),
            Self::ExpectedPipe => fmt.write_markup(biome_console::markup! {
                "Separate property syntax components with "<Emphasis>"|"</Emphasis>"."
            }),
            Self::ExpectedTypeName => fmt.write_str("Use a supported property syntax type."),
            Self::InvalidCustomIdentifier => {
                fmt.write_str("Use a custom identifier that isn't reserved.")
            }
            Self::InvalidUniversalSyntax => fmt.write_markup(biome_console::markup! {
                "The "<Emphasis>"*"</Emphasis>" can't appear with other syntax components."
            }),
            Self::MultiplierAfterTransformList => fmt.write_markup(biome_console::markup! {
                "Remove the multiplier after "<Emphasis>"<transform-list>"</Emphasis>"."
            }),
            Self::UnexpectedWhitespace => {
                fmt.write_str("Remove whitespace inside the type name or before the multiplier.")
            }
        }
    }
}

impl Advices for PropertySyntaxErrorKind {
    fn record(&self, visitor: &mut dyn Visit) -> std::io::Result<()> {
        match self {
            Self::Empty => visitor.record_log(
                LogCategory::Info,
                &biome_console::markup! {
                    "Add a syntax component, such as "<Emphasis>"<length>"</Emphasis>
                    ", or use "<Emphasis>"*"</Emphasis>" to accept any value."
                },
            ),
            Self::ExpectedComponent => visitor.record_log(
                LogCategory::Info,
                &biome_console::markup! {
                    "Add a data type, such as "<Emphasis>"<length>"</Emphasis>
                    ", or a custom identifier at this position."
                },
            ),
            Self::ExpectedString => Ok(()),
            Self::ExpectedTypeName => {
                visitor.record_log(
                    LogCategory::Info,
                    &biome_console::markup! { "Supported types:" },
                )?;
                let types = PropertySyntaxType::ALL;
                let types = types
                    .iter()
                    .map(|ty| ty as &dyn biome_console::fmt::Display)
                    .collect::<Vec<_>>();
                visitor.record_list(&types)
            }
            Self::InvalidCustomIdentifier => {
                visitor.record_log(
                    LogCategory::Info,
                    &biome_console::markup! { "Reserved identifiers:" },
                )?;
                let identifiers = RESERVED_CUSTOM_IDENTIFIERS.map(FormatReservedCustomIdentifier);
                let identifiers = identifiers
                    .iter()
                    .map(|identifier| identifier as &dyn biome_console::fmt::Display)
                    .collect::<Vec<_>>();
                visitor.record_list(&identifiers)
            }
            Self::InvalidUniversalSyntax => visitor.record_log(
                LogCategory::Info,
                &biome_console::markup! {
                    "Remove "<Emphasis>"*"</Emphasis>" or remove the other syntax components."
                },
            ),
            _ => Ok(()),
        }
    }
}

pub(crate) const RESERVED_CUSTOM_IDENTIFIERS: [&str; 6] = [
    "default",
    "inherit",
    "initial",
    "revert",
    "revert-layer",
    "unset",
];

/// The parsed value of an `@property` `syntax` descriptor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PropertySyntax {
    /// The universal syntax, which accepts any token sequence.
    Universal {
        /// The absolute source range of the `*` token.
        range: TextRange,
    },
    /// An ordered list of syntax components separated by `|`.
    Components(Box<[PropertySyntaxComponent]>),
}

impl PropertySyntax {
    pub const fn is_universal(&self) -> bool {
        matches!(self, Self::Universal { .. })
    }
}

/// A data type or custom identifier with an optional multiplier.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PropertySyntaxComponent {
    /// The value matched by the component.
    pub name: PropertySyntaxComponentName,
    /// The repetition behavior applied to the component.
    pub multiplier: PropertySyntaxMultiplier,
    /// The absolute source range from the component name through its multiplier, if present.
    pub range: TextRange,
}

/// The value matched by a property syntax component.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PropertySyntaxComponentName {
    /// A registered property data type enclosed in angle brackets.
    Type(PropertySyntaxType),
    /// A case-sensitive custom identifier with CSS escapes resolved.
    CustomIdentifier(Box<str>),
}

/// The repetition behavior of a property syntax component.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PropertySyntaxMultiplier {
    /// The component matches exactly one value.
    #[default]
    None,
    /// The `+` multiplier, which matches one or more space-separated values.
    SpaceSeparated,
    /// The `#` multiplier, which matches one or more comma-separated values.
    CommaSeparated,
}

/// A data type supported by the registered property syntax grammar.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PropertySyntaxType {
    /// The `<angle>` data type.
    Angle,
    /// The `<color>` data type.
    Color,
    /// The `<custom-ident>` data type.
    CustomIdent,
    /// The `<image>` data type.
    Image,
    /// The `<integer>` data type.
    Integer,
    /// The `<length>` data type.
    Length,
    /// The `<length-percentage>` data type.
    LengthPercentage,
    /// The `<number>` data type.
    Number,
    /// The `<percentage>` data type.
    Percentage,
    /// The `<resolution>` data type.
    Resolution,
    /// The `<string>` data type.
    String,
    /// The `<time>` data type.
    Time,
    /// The `<transform-function>` data type.
    TransformFunction,
    /// The pre-multiplied `<transform-list>` data type.
    TransformList,
    /// The `<url>` data type.
    Url,
}

impl PropertySyntaxType {
    pub(crate) const ALL: [Self; 15] = [
        Self::Angle,
        Self::Color,
        Self::CustomIdent,
        Self::Image,
        Self::Integer,
        Self::Length,
        Self::LengthPercentage,
        Self::Number,
        Self::Percentage,
        Self::Resolution,
        Self::String,
        Self::Time,
        Self::TransformFunction,
        Self::TransformList,
        Self::Url,
    ];

    pub(crate) const fn name(self) -> &'static str {
        match self {
            Self::Angle => "angle",
            Self::Color => "color",
            Self::CustomIdent => "custom-ident",
            Self::Image => "image",
            Self::Integer => "integer",
            Self::Length => "length",
            Self::LengthPercentage => "length-percentage",
            Self::Number => "number",
            Self::Percentage => "percentage",
            Self::Resolution => "resolution",
            Self::String => "string",
            Self::Time => "time",
            Self::TransformFunction => "transform-function",
            Self::TransformList => "transform-list",
            Self::Url => "url",
        }
    }
}

impl biome_console::fmt::Display for PropertySyntaxType {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_markup(biome_console::markup! {
            <Emphasis>"<"{self.name()}">"</Emphasis>
        })
    }
}

/// Formats an identifier that cannot be used as a custom syntax component.
struct FormatReservedCustomIdentifier<'a>(&'a str);

impl biome_console::fmt::Display for FormatReservedCustomIdentifier<'_> {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_markup(biome_console::markup! {
            <Emphasis>{self.0}</Emphasis>
        })
    }
}

// #endregion

// #region Formatting

/// Formatting options for normalized property syntax output.
#[derive(Debug, Default)]
pub struct PropertyFmtOptions;

impl FormatOptions for PropertyFmtOptions {
    fn indent_style(&self) -> IndentStyle {
        IndentStyle::default()
    }

    fn indent_width(&self) -> IndentWidth {
        IndentWidth::default()
    }

    fn line_width(&self) -> LineWidth {
        LineWidth::default()
    }

    fn line_ending(&self) -> LineEnding {
        LineEnding::default()
    }

    fn trailing_newline(&self) -> TrailingNewline {
        false.into()
    }

    fn as_print_options(&self) -> PrinterOptions {
        PrinterOptions {
            indent_width: self.indent_width(),
            print_width: self.line_width().into(),
            line_ending: self.line_ending(),
            indent_style: self.indent_style(),
            source_map_generation: SourceMapGeneration::Disabled,
        }
    }
}

/// The formatter context used to serialize property syntax values.
#[derive(Debug, Default)]
pub struct PropertyFmtContext {
    /// The options used to print the normalized syntax.
    options: PropertyFmtOptions,
}

impl FormatContext for PropertyFmtContext {
    type Options = PropertyFmtOptions;

    fn options(&self) -> &Self::Options {
        &self.options
    }

    fn source_map(&self) -> Option<&TransformSourceMap> {
        None
    }
}

impl Format<PropertyFmtContext> for PropertySyntax {
    fn fmt(&self, f: &mut Formatter<PropertyFmtContext>) -> FormatResult<()> {
        match self {
            Self::Universal { .. } => write!(f, [token("*")]),
            Self::Components(components) => {
                for (index, component) in components.iter().enumerate() {
                    if index > 0 {
                        write!(f, [space(), token("|"), space()])?;
                    }
                    write!(f, [component])?;
                }
                Ok(())
            }
        }
    }
}

impl Format<PropertyFmtContext> for PropertySyntaxComponent {
    fn fmt(&self, f: &mut Formatter<PropertyFmtContext>) -> FormatResult<()> {
        match &self.name {
            PropertySyntaxComponentName::Type(ty) => {
                write!(f, [token("<"), token(ty.name()), token(">")])?;
            }
            PropertySyntaxComponentName::CustomIdentifier(identifier) => {
                write!(f, [FormatCustomIdentifier(identifier)])?;
            }
        }
        match self.multiplier {
            PropertySyntaxMultiplier::None => Ok(()),
            PropertySyntaxMultiplier::SpaceSeparated => write!(f, [token("+")]),
            PropertySyntaxMultiplier::CommaSeparated => write!(f, [token("#")]),
        }
    }
}

/// Serializes a decoded custom identifier as a valid CSS identifier.
///
/// Characters that cannot appear literally are escaped according to CSSOM's
/// [identifier serialization algorithm](https://drafts.csswg.org/cssom/#serialize-an-identifier).
/// Hexadecimal escapes include a trailing space so a following hexadecimal
/// digit cannot become part of the escape sequence. A lone hyphen is escaped
/// because it cannot start an identifier by itself.
struct FormatCustomIdentifier<'a>(&'a str);

impl Format<PropertyFmtContext> for FormatCustomIdentifier<'_> {
    fn fmt(&self, f: &mut Formatter<PropertyFmtContext>) -> FormatResult<()> {
        let bytes = self.0.as_bytes();
        // Unescaped spans share the input string and produce one formatter element each.
        let mut start = 0;
        let mut index = 0;
        while index < bytes.len() {
            let byte = bytes[index];
            let (code_point, code_point_len) = if byte.is_ascii() {
                (u32::from(byte), 1)
            } else {
                debug_assert!(self.0.is_char_boundary(index));
                let character = self.0[index..]
                    .chars()
                    .next()
                    .expect("the index should point to a character");
                (character as u32, character.len_utf8())
            };
            let requires_hex_escape = byte <= 0x1f
                || byte == 0x7f
                || (index == 0 && byte.is_ascii_digit())
                || (index == 1 && bytes[0] == b'-' && byte.is_ascii_digit());
            let requires_simple_escape = (bytes.len() == 1 && byte == b'-')
                || (byte.is_ascii()
                    && !byte.is_ascii_alphanumeric()
                    && !matches!(byte, b'-' | b'_')
                    && !requires_hex_escape);

            if requires_hex_escape || requires_simple_escape {
                if start < index {
                    write!(f, [text(&self.0[start..index], None)])?;
                }
                if requires_hex_escape {
                    let escaped = std::format!("\\{code_point:x} ");
                    write!(f, [text(&escaped, None)])?;
                } else {
                    write!(f, [token("\\"), text(&self.0[index..index + 1], None)])?;
                }
                index += code_point_len;
                start = index;
            } else {
                index += code_point_len;
            }
        }
        if start < bytes.len() {
            write!(f, [text(&self.0[start..], None)])?;
        }
        Ok(())
    }
}

// #endregion
