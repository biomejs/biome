use crate::diagnostics::MigrationDiagnostic;
use crate::CliDiagnostic;
use biome_console::{markup, Console, ConsoleExt};
use biome_deserialize::json::deserialize_from_json_str;
use biome_deserialize::{
    Deserializable, DeserializableValue, DeserializationDiagnostic, DeserializationVisitor,
    NoneState, StringSet, Text, VisitableType,
};
use biome_diagnostics::{DiagnosticExt, PrintDiagnostic};
use biome_formatter::{LineEnding, LineWidth, QuoteStyle};
use biome_fs::{FileSystem, OpenOptions};
use biome_js_formatter::context::{ArrowParentheses, QuoteProperties, Semicolons, TrailingComma};
use biome_json_parser::JsonParserOptions;
use biome_service::configuration::{
    FormatterConfiguration, JavascriptConfiguration, PlainIndentStyle,
};
use biome_service::{Configuration, DynRef, JavascriptFormatter};
use biome_text_size::TextRange;
use indexmap::IndexSet;
use std::path::{Path, PathBuf};

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct PrettierConfiguration {
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

impl Default for PrettierConfiguration {
    fn default() -> Self {
        Self {
            print_width: 80,

            use_tabs: false,
            trailing_comma: PrettierTrailingComma::default(),
            tab_width: 2,
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

#[derive(Debug, Eq, PartialEq, Default, Clone)]
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

#[derive(Debug, Eq, PartialEq, Default, Clone)]
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

#[derive(Debug, Default, Eq, PartialEq, Clone)]
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

#[derive(Debug, Eq, Default, PartialEq, Clone)]
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
        let line_width = LineWidth::try_from(value.print_width).map_err(|err| err.to_string())?;
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

impl From<PrettierConfiguration> for JavascriptFormatter {
    fn from(value: PrettierConfiguration) -> Self {
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
        Self {
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
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct FromPrettierConfiguration {
    /// Path of the Prettier configuration file
    configuration_path: Option<PathBuf>,
    /// Path of the `.prettierignore` file
    #[allow(unused)]
    ignore_path: Option<PathBuf>,

    /// The translated Biome configuration, from the Prettier configuration
    formatter_configuration: Option<FormatterConfiguration>,

    /// The translated Biome configuration, from the Prettier configuration
    javascript_formatter_configuration: Option<JavascriptFormatter>,
}

impl FromPrettierConfiguration {
    pub(crate) fn store_configuration(
        &mut self,
        (formatter, javascript_formatter): (FormatterConfiguration, JavascriptFormatter),
    ) {
        self.formatter_configuration = Some(formatter);
        self.javascript_formatter_configuration = Some(javascript_formatter);
    }

    pub(crate) fn store_ignored_globs(&mut self, value: StringSet) {
        let formatter_configuration =
            self.formatter_configuration
                .get_or_insert(FormatterConfiguration {
                    ..NoneState::none()
                });

        formatter_configuration.ignore = Some(value);
    }

    pub(crate) fn store_configuration_path(&mut self, path: impl Into<PathBuf>) {
        self.configuration_path = Some(path.into());
    }

    pub(crate) fn store_ignore_path(&mut self, path: impl Into<PathBuf>) {
        self.ignore_path = Some(path.into())
    }

    pub(crate) fn as_biome_configuration(&self) -> Configuration {
        let mut configuration = Configuration {
            ..NoneState::none()
        };
        configuration.formatter = self.formatter_configuration.clone();
        if self.javascript_formatter_configuration.is_some() {
            configuration.javascript = Some(JavascriptConfiguration {
                formatter: self.javascript_formatter_configuration.clone(),
                ..NoneState::none()
            });
        }

        configuration
    }

    pub(crate) fn has_configuration(&self) -> bool {
        self.formatter_configuration.is_some() || self.javascript_formatter_configuration.is_some()
    }

    pub(crate) fn get_configuration_path(&self) -> Option<&Path> {
        self.configuration_path.as_deref()
    }
}

const PRETTIER_CONFIG_FILES: [&str; 2] = [".prettierrc", ".prettierrc.json"];
const PRETTIER_IGNORE_FILE: &str = ".prettierignore";
/// This function is in charge of reading prettier files, deserialize its contents and convert them in a Biome configuration type
pub(crate) fn read_prettier_files(
    fs: &DynRef<'_, dyn FileSystem>,
    console: &mut dyn Console,
) -> Result<FromPrettierConfiguration, CliDiagnostic> {
    let mut from_prettier_configuration = FromPrettierConfiguration::default();
    let mut prettier_config_content = String::new();
    for config_name in PRETTIER_CONFIG_FILES {
        let open_options = OpenOptions::default().read(true);
        let path = Path::new(config_name);
        let file = fs.open_with_options(path, open_options);
        match file {
            Ok(mut file) => {
                let result = file.read_to_string(&mut prettier_config_content);
                if let Err(err) = result {
                    return Err(CliDiagnostic::io_error(err));
                }
                from_prettier_configuration.store_configuration_path(path);

                break;
            }
            Err(_) => {
                continue;
            }
        }
    }

    if prettier_config_content.is_empty() {
        return Err(CliDiagnostic::MigrateError(MigrationDiagnostic {
            reason: "Biome couldn't find a Prettier configuration file.".to_string(),
        }));
    }

    let deserialized = deserialize_from_json_str::<PrettierConfiguration>(
        prettier_config_content.as_str(),
        JsonParserOptions::default()
            .with_allow_trailing_commas()
            .with_allow_comments(),
        "",
    );

    if deserialized.has_errors() {
        let diagnostics = deserialized.into_diagnostics();
        for diagnostic in diagnostics {
            let diagnostic =
                if let Some(path) = from_prettier_configuration.get_configuration_path() {
                    diagnostic.with_file_path(path.display().to_string())
                } else {
                    diagnostic
                };
            console.error(markup! {{PrintDiagnostic::simple(&diagnostic)}});
        }
        return Err(CliDiagnostic::MigrateError(MigrationDiagnostic {
            reason: "Could not deserialize the Prettier configuration file".to_string(),
        }));
    } else {
        let prettier_configuration = deserialized.into_deserialized();

        if let Some(prettier_configuration) = prettier_configuration {
            let formatter_configuration = prettier_configuration
                .clone()
                .try_into()
                .map_err(|err| CliDiagnostic::MigrateError(MigrationDiagnostic { reason: err }))?;
            let javascript_configuration = prettier_configuration.into();
            from_prettier_configuration
                .store_configuration((formatter_configuration, javascript_configuration));
        }
    }

    // to be safe, let's drop this content
    drop(prettier_config_content);

    let mut ignore_file_content = String::new();
    let open_options = OpenOptions::default().read(true);
    let path = Path::new(PRETTIER_IGNORE_FILE);
    let file = fs.open_with_options(path, open_options);
    if let Ok(mut file) = file {
        let result = file.read_to_string(&mut ignore_file_content);
        if let Err(err) = result {
            return Err(CliDiagnostic::io_error(err));
        }
        from_prettier_configuration.store_ignore_path(path)
    }

    if !ignore_file_content.is_empty() {
        let lines = StringSet::new(
            ignore_file_content
                .lines()
                .filter(|line| !line.is_empty())
                .filter(|line| !line.starts_with('#'))
                .map(String::from)
                .collect::<IndexSet<_>>(),
        );
        from_prettier_configuration.store_ignored_globs(lines);
    }

    Ok(from_prettier_configuration)
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
