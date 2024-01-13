use biome_deserialize::{
    Deserializable, DeserializableValue, DeserializationDiagnostic, DeserializationVisitor, Text,
    VisitableType,
};
use biome_formatter::{LineEnding, LineWidth, QuoteStyle};
use biome_js_formatter::context::{ArrowParentheses, QuoteProperties, Semicolons, TrailingComma};
use biome_service::configuration::{FormatterConfiguration, PlainIndentStyle};
use biome_service::JavascriptFormatter;
use biome_text_size::TextRange;

#[derive(Debug, Eq, PartialEq)]
struct PrettierConfiguration {
    /// https://prettier.io/docs/en/options#print-width
    print_width: u16,
    /// https://prettier.io/docs/en/options#use-tabs
    use_tabs: bool,
    /// https://prettier.io/docs/en/options#trailing-comma
    trailing_comma: PrettierTrailingComma,
    /// https://prettier.io/docs/en/options#tab-width
    tab_width: u8,
    /// https://prettier.io/docs/en/options#semicolons
    semi: bool,
    /// https://prettier.io/docs/en/options#quotes
    single_quote: bool,
    /// https://prettier.io/docs/en/options#bracket-spcing
    bracket_spacing: bool,
    /// https://prettier.io/docs/en/options#bracket-line
    bracket_line: bool,
    /// https://prettier.io/docs/en/options#quote-props
    quote_props: QuoteProps,
    /// https://prettier.io/docs/en/options#jsx-quotes
    jsx_single_quote: bool,
    /// https://prettier.io/docs/en/options#arrow-function-parentheses
    arrow_parens: ArrowParens,
    /// https://prettier.io/docs/en/options#end-of-line
    end_of_line: EndOfLine,
}

#[derive(Debug, Eq, PartialEq, Default)]
enum EndOfLine {
    #[default]
    Lf,
    Crlf,
    Cr,
}

impl Deserializable for EndOfLine {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        match String::deserialize(value, name, diagnostics)?.as_str() {
            "lf" => Some(Self::Lf),
            "crlf" => Some(Self::Crlf),
            "cr" => Some(Self::Cr),
            unknown_variant => {
                const ALLOWED_VARIANTS: &[&str] = &["lf", "crlf", "cr"];
                diagnostics.push(DeserializationDiagnostic::new_unknown_value(
                    unknown_variant,
                    value.range(),
                    ALLOWED_VARIANTS,
                ));
                None
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Default)]
enum ArrowParens {
    #[default]
    Always,
    Avoid,
}

impl Deserializable for ArrowParens {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        match String::deserialize(value, name, diagnostics)?.as_str() {
            "always" => Some(Self::Always),
            "avoid" => Some(Self::Avoid),
            unknown_variant => {
                const ALLOWED_VARIANTS: &[&str] = &["always", "avoid"];
                diagnostics.push(DeserializationDiagnostic::new_unknown_value(
                    unknown_variant,
                    value.range(),
                    ALLOWED_VARIANTS,
                ));
                None
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Default)]
enum PrettierTrailingComma {
    #[default]
    All,
    None,
    Es5,
}

impl Deserializable for PrettierTrailingComma {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        match String::deserialize(value, name, diagnostics)?.as_str() {
            "all" => Some(Self::All),
            "none" => Some(Self::None),
            "es5" => Some(Self::Es5),
            unknown_variant => {
                const ALLOWED_VARIANTS: &[&str] = &["all", "none", "es5"];
                diagnostics.push(DeserializationDiagnostic::new_unknown_value(
                    unknown_variant,
                    value.range(),
                    ALLOWED_VARIANTS,
                ));
                None
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Default)]

enum QuoteProps {
    #[default]
    AsNeeded,
    Preserve,
}

impl Deserializable for QuoteProps {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        match String::deserialize(value, name, diagnostics)?.as_str() {
            "as-needed" => Some(Self::AsNeeded),
            "preserve" => Some(Self::Preserve),
            unknown_variant => {
                const ALLOWED_VARIANTS: &[&str] = &["as-needed", "preserve"];
                diagnostics.push(DeserializationDiagnostic::new_unknown_value(
                    unknown_variant,
                    value.range(),
                    ALLOWED_VARIANTS,
                ));
                None
            }
        }
    }
}

impl TryFrom<&str> for PrettierTrailingComma {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "all" => Ok(Self::All),
            "none" => Ok(Self::None),
            "es5" => Ok(Self::Es5),
            _ => Err("Option not supported".to_string()),
        }
    }
}

impl Default for PrettierConfiguration {
    fn default() -> Self {
        Self {
            print_width: 80,

            use_tabs: false,
            trailing_comma: PrettierTrailingComma::default(),
            tab_width: 4,
            semi: false,
            single_quote: true,
            bracket_spacing: true,
            bracket_line: false,
            quote_props: QuoteProps::default(),
            jsx_single_quote: false,
            arrow_parens: ArrowParens::default(),
            end_of_line: EndOfLine::default(),
        }
    }
}

impl Deserializable for PrettierConfiguration {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(PrettierVisitor, name, diagnostics)
    }
}

struct PrettierVisitor;

impl DeserializationVisitor for PrettierVisitor {
    type Output = PrettierConfiguration;
    const EXPECTED_TYPE: VisitableType = VisitableType::MAP;

    fn visit_map(
        self,
        members: impl Iterator<Item = Option<(impl DeserializableValue, impl DeserializableValue)>>,
        _range: TextRange,
        _name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        let mut result = PrettierConfiguration::default();
        for (key, value) in members.flatten() {
            let Some(key_text) = Text::deserialize(&key, "", diagnostics) else {
                continue;
            };
            match key_text.text() {
                "endOfLine" => {
                    if let Some(val) = Deserializable::deserialize(&value, &key_text, diagnostics) {
                        result.end_of_line = val;
                    }
                }
                "arrowParens" => {
                    if let Some(val) = Deserializable::deserialize(&value, &key_text, diagnostics) {
                        result.arrow_parens = val;
                    }
                }
                "useTabs" => {
                    if let Some(val) = Deserializable::deserialize(&value, &key_text, diagnostics) {
                        result.use_tabs = val;
                    }
                }

                "printWidth" => {
                    if let Some(val) = Deserializable::deserialize(&value, &key_text, diagnostics) {
                        result.print_width = val;
                    }
                }

                "trailingComma" => {
                    if let Some(val) = Deserializable::deserialize(&value, &key_text, diagnostics) {
                        result.trailing_comma = val;
                    }
                }

                "quoteProps" => {
                    if let Some(val) = Deserializable::deserialize(&value, &key_text, diagnostics) {
                        result.quote_props = val;
                    }
                }

                "semi" => {
                    if let Some(val) = Deserializable::deserialize(&value, &key_text, diagnostics) {
                        result.semi = val;
                    }
                }

                "bracketSpacing" => {
                    if let Some(val) = Deserializable::deserialize(&value, &key_text, diagnostics) {
                        result.bracket_spacing = val;
                    }
                }

                "bracketLine" => {
                    if let Some(val) = Deserializable::deserialize(&value, &key_text, diagnostics) {
                        result.bracket_line = val;
                    }
                }

                "jsxSingleQuote" => {
                    if let Some(val) = Deserializable::deserialize(&value, &key_text, diagnostics) {
                        result.jsx_single_quote = val;
                    }
                }

                "singleQuote" => {
                    if let Some(val) = Deserializable::deserialize(&value, &key_text, diagnostics) {
                        result.single_quote = val;
                    }
                }

                "tabWidth" => {
                    if let Some(val) = Deserializable::deserialize(&value, &key_text, diagnostics) {
                        result.tab_width = val;
                    }
                }

                _ => {}
            }
        }

        Some(result)
    }
}

impl From<PrettierTrailingComma> for TrailingComma {
    fn from(value: PrettierTrailingComma) -> Self {
        match value {
            PrettierTrailingComma::All => Self::All,
            PrettierTrailingComma::None => Self::None,
            PrettierTrailingComma::Es5 => Self::Es5,
        }
    }
}

impl From<ArrowParens> for ArrowParentheses {
    fn from(value: ArrowParens) -> Self {
        match value {
            ArrowParens::Always => Self::Always,
            ArrowParens::Avoid => Self::AsNeeded,
        }
    }
}

impl From<EndOfLine> for LineEnding {
    fn from(value: EndOfLine) -> Self {
        match value {
            EndOfLine::Lf => LineEnding::Lf,
            EndOfLine::Crlf => LineEnding::Crlf,
            EndOfLine::Cr => LineEnding::Cr,
        }
    }
}

impl From<QuoteProps> for QuoteProperties {
    fn from(value: QuoteProps) -> Self {
        match value {
            QuoteProps::AsNeeded => Self::AsNeeded,
            QuoteProps::Preserve => Self::Preserve,
        }
    }
}

impl TryFrom<PrettierConfiguration> for FormatterConfiguration {
    type Error = String;
    fn try_from(value: PrettierConfiguration) -> Result<Self, Self::Error> {
        // TODO: handle error
        let line_width = LineWidth::try_from(value.print_width).unwrap();
        let indent_style = if value.use_tabs {
            PlainIndentStyle::Tab
        } else {
            PlainIndentStyle::Space
        };
        Ok(Self {
            indent_width: Some(value.tab_width),
            line_width: Some(line_width),
            indent_style: Some(indent_style),
            line_ending: Some(value.end_of_line.into()),
            format_with_errors: Some(false),
            ignore: None,
            include: None,
            enabled: Some(true),
            // deprecated
            indent_size: None,
        })
    }
}

impl TryFrom<PrettierConfiguration> for JavascriptFormatter {
    type Error = String;
    fn try_from(value: PrettierConfiguration) -> Result<Self, Self::Error> {
        let semicolons = if value.semi {
            Semicolons::Always
        } else {
            Semicolons::AsNeeded
        };
        let quote_style = if value.single_quote {
            QuoteStyle::Single
        } else {
            QuoteStyle::Double
        };
        let jsx_quote_style = if value.jsx_single_quote {
            QuoteStyle::Single
        } else {
            QuoteStyle::Double
        };
        Ok(Self {
            indent_width: None,
            line_width: None,
            indent_style: None,
            line_ending: None,
            enabled: None,
            // deprecated
            indent_size: None,

            // js ones
            bracket_same_line: Some(value.bracket_line),
            arrow_parentheses: Some(value.arrow_parens.into()),
            semicolons: Some(semicolons),
            trailing_comma: Some(value.trailing_comma.into()),
            quote_style: Some(quote_style),
            quote_properties: Some(value.quote_props.into()),
            bracket_spacing: Some(value.bracket_spacing),
            jsx_quote_style: Some(jsx_quote_style),
        })
    }
}

#[cfg(test)]
mod test {
    use crate::execute::migrate::prettier::{PrettierConfiguration, PrettierTrailingComma};
    use biome_deserialize::json::deserialize_from_json_str;
    use biome_json_parser::JsonParserOptions;

    #[test]
    fn ok() {
        let configuration = deserialize_from_json_str::<PrettierConfiguration>(
            r#"{ "useTabs": true }"#,
            JsonParserOptions::default(),
            "",
        )
        .into_deserialized()
        .unwrap();

        assert_eq!(
            configuration,
            PrettierConfiguration {
                use_tabs: true,
                ..PrettierConfiguration::default()
            }
        )
    }

    #[test]
    fn some_properties() {
        let configuration = deserialize_from_json_str::<PrettierConfiguration>(
            r#"
{
  "printWidth": 100,
  "semi": true,
  "singleQuote": true,
  "tabWidth": 2,
  "trailingComma": "es5",
  "useTabs": true,
  "jsxSingleQuote": true
}
            "#,
            JsonParserOptions::default(),
            "",
        )
        .into_deserialized()
        .unwrap();

        assert_eq!(
            configuration,
            PrettierConfiguration {
                use_tabs: true,
                print_width: 100,
                semi: true,
                single_quote: true,
                tab_width: 2,
                trailing_comma: PrettierTrailingComma::Es5,
                jsx_single_quote: true,
                ..PrettierConfiguration::default()
            }
        )
    }
}
