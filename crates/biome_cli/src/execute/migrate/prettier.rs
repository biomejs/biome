use super::{eslint_eslint::ShorthandVec, node};
use crate::diagnostics::MigrationDiagnostic;
use crate::CliDiagnostic;
use biome_configuration::javascript::JsFormatterConfiguration;
use biome_console::{markup, Console, ConsoleExt};
use biome_deserialize::json::deserialize_from_json_str;
use biome_deserialize_macros::Deserializable;
use biome_diagnostics::{DiagnosticExt, PrintDiagnostic};
use biome_formatter::{
    AttributePosition, BracketSpacing, IndentWidth, LineEnding, LineWidth,
    ObjectWrap as BiomeObjectWrap, ParseFormatNumberError, QuoteStyle,
};
use biome_fs::{FileSystem, OpenOptions};
use biome_js_formatter::context::{ArrowParentheses, QuoteProperties, Semicolons, TrailingCommas};
use biome_json_parser::JsonParserOptions;
use camino::Utf8Path;

#[derive(Debug, Default, Deserializable)]
#[deserializable(unknown_fields = "allow")]
pub(crate) struct PrettierPackageJson {
    pub(crate) prettier: Option<PrettierConfiguration>,
}

#[derive(Debug)]
pub(crate) struct Config {
    /// Path of the ESlint config file
    pub(crate) path: &'static str,
    /// Resolved ESlint config
    pub(crate) data: PrettierConfiguration,
}

#[derive(Debug, Deserializable)]
#[deserializable(unknown_fields = "allow")]
pub(crate) struct PrettierConfiguration {
    /// https://prettier.io/docs/en/options#print-width
    print_width: u16,
    /// https://prettier.io/docs/en/options#use-tabs
    use_tabs: bool,
    /// https://prettier.io/docs/en/options#trailing-commas
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
    /// https://prettier.io/docs/options#object-wrap
    object_wrap: ObjectWrap,
    /// https://prettier.io/docs/en/configuration.html#configuration-overrides
    overrides: Vec<Override>,
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
            object_wrap: ObjectWrap::default(),
            overrides: vec![],
        }
    }
}

#[derive(Debug, Default, Deserializable)]
pub(crate) struct Override {
    files: ShorthandVec<String>,
    options: OverrideOptions,
}

#[derive(Clone, Debug, Default, Deserializable, Eq, PartialEq)]
#[deserializable(unknown_fields = "allow")]
pub(crate) struct OverrideOptions {
    /// https://prettier.io/docs/en/options#print-width
    print_width: Option<u16>,
    /// https://prettier.io/docs/en/options#use-tabs
    use_tabs: Option<bool>,
    /// https://prettier.io/docs/en/options#trailing-commas
    trailing_comma: Option<PrettierTrailingComma>,
    /// https://prettier.io/docs/en/options#tab-width
    tab_width: Option<u8>,
    /// https://prettier.io/docs/en/options#semicolons
    semi: Option<bool>,
    /// https://prettier.io/docs/en/options#quotes
    single_quote: Option<bool>,
    /// https://prettier.io/docs/en/options#bracket-spcing
    bracket_spacing: Option<bool>,
    /// https://prettier.io/docs/en/options#bracket-line
    bracket_line: Option<bool>,
    /// https://prettier.io/docs/en/options#quote-props
    quote_props: Option<QuoteProps>,
    /// https://prettier.io/docs/en/options#jsx-quotes
    jsx_single_quote: Option<bool>,
    /// https://prettier.io/docs/en/options#arrow-function-parentheses
    arrow_parens: Option<ArrowParens>,
    /// https://prettier.io/docs/en/options#end-of-line
    end_of_line: Option<EndOfLine>,
    /// https://prettier.io/docs/options#object-wrap
    object_wrap: ObjectWrap,
}

#[derive(Clone, Debug, Default, Deserializable, Eq, PartialEq)]
enum EndOfLine {
    #[default]
    Lf,
    Crlf,
    Cr,
    Auto,
}

#[derive(Clone, Debug, Default, Deserializable, Eq, PartialEq)]
enum ArrowParens {
    #[default]
    Always,
    Avoid,
}

#[derive(Clone, Debug, Default, Deserializable, Eq, PartialEq)]
enum PrettierTrailingComma {
    #[default]
    All,
    None,
    Es5,
}

#[derive(Clone, Debug, Default, Deserializable, Eq, PartialEq)]
enum QuoteProps {
    #[default]
    #[deserializable(rename = "as-needed")]
    AsNeeded,
    Preserve,
}

#[derive(Clone, Debug, Default, Deserializable, Eq, PartialEq)]
enum ObjectWrap {
    #[default]
    Preserve,
    Collapse,
}

impl From<PrettierTrailingComma> for TrailingCommas {
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
            EndOfLine::Auto => LineEnding::default(),
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

impl From<ObjectWrap> for BiomeObjectWrap {
    fn from(value: ObjectWrap) -> Self {
        match value {
            ObjectWrap::Preserve => Self::Preserve,
            ObjectWrap::Collapse => Self::Collapse,
        }
    }
}

impl TryFrom<PrettierConfiguration> for biome_configuration::Configuration {
    type Error = ParseFormatNumberError;
    fn try_from(value: PrettierConfiguration) -> Result<Self, Self::Error> {
        let mut result = biome_configuration::Configuration::default();

        let line_width = LineWidth::try_from(value.print_width)?;
        let indent_width = IndentWidth::try_from(value.tab_width)?;
        let indent_style = if value.use_tabs {
            biome_formatter::IndentStyle::Tab
        } else {
            biome_formatter::IndentStyle::Space
        };
        let formatter = biome_configuration::FormatterConfiguration {
            indent_width: Some(indent_width),
            line_width: Some(line_width),
            indent_style: Some(indent_style),
            line_ending: Some(value.end_of_line.into()),
            bracket_same_line: Some(value.bracket_line.into()),
            attribute_position: Some(AttributePosition::default()),
            bracket_spacing: Some(BracketSpacing::default()),
            object_wrap: Some(value.object_wrap.into()),
            format_with_errors: Some(false.into()),
            includes: None,
            enabled: Some(true.into()),
            // editorconfig support is intentionally set to true, because prettier always reads the editorconfig file
            // see: https://github.com/prettier/prettier/issues/15255
            use_editorconfig: Some(true.into()),
        };
        result.formatter = Some(formatter);

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
        let js_formatter = JsFormatterConfiguration {
            indent_width: None,
            line_width: None,
            indent_style: None,
            line_ending: None,
            object_wrap: None,
            enabled: None,
            // js ones
            bracket_same_line: Some(value.bracket_line.into()),
            arrow_parentheses: Some(value.arrow_parens.into()),
            semicolons: Some(semicolons),
            trailing_commas: Some(value.trailing_comma.into()),
            quote_style: Some(quote_style),
            quote_properties: Some(value.quote_props.into()),
            bracket_spacing: Some(value.bracket_spacing.into()),
            jsx_quote_style: Some(jsx_quote_style),
            attribute_position: Some(AttributePosition::default()),
        };
        let js_config = biome_configuration::JsConfiguration {
            formatter: Some(js_formatter),
            ..Default::default()
        };
        result.javascript = Some(js_config);
        if !value.overrides.is_empty() {
            let mut overrides = biome_configuration::Overrides::default();
            for override_elt in value.overrides {
                overrides.0.push(override_elt.try_into()?);
            }
            result.overrides = Some(overrides);
        }
        Ok(result)
    }
}

impl TryFrom<Override> for biome_configuration::OverridePattern {
    type Error = ParseFormatNumberError;
    fn try_from(Override { files, options }: Override) -> Result<Self, Self::Error> {
        let mut result = biome_configuration::OverridePattern {
            includes: Some(biome_configuration::OverrideGlobs::Globs(
                files
                    .into_iter()
                    .filter_map(|glob| glob.parse().ok())
                    .collect(),
            )),
            ..Default::default()
        };
        if options.print_width.is_some()
            || options.use_tabs.is_some()
            || options.tab_width.is_some()
            || options.end_of_line.is_some()
        {
            // are global options are set
            let line_width = if let Some(print_width) = options.print_width {
                Some(LineWidth::try_from(print_width)?)
            } else {
                None
            };
            // are global options are set
            let indent_width = if let Some(indent_width) = options.tab_width {
                Some(IndentWidth::try_from(indent_width)?)
            } else {
                None
            };
            let indent_style = options.use_tabs.map(|use_tabs| {
                if use_tabs {
                    biome_formatter::IndentStyle::Tab
                } else {
                    biome_formatter::IndentStyle::Space
                }
            });
            let formatter = biome_configuration::OverrideFormatterConfiguration {
                indent_width,
                line_width,
                indent_style,
                line_ending: options.end_of_line.map(|end_of_line| end_of_line.into()),
                ..Default::default()
            };
            result.formatter = Some(formatter);
        }
        if options.semi.is_none()
            && options.single_quote.is_none()
            && options.jsx_single_quote.is_none()
            && options.bracket_line.is_none()
            && options.arrow_parens.is_none()
            && options.trailing_comma.is_none()
            && options.quote_props.is_none()
            && options.bracket_spacing.is_none()
        {
            // no js option are set
            return Ok(result);
        }
        // js config
        let semicolons = options.semi.map(|semi| {
            if semi {
                Semicolons::Always
            } else {
                Semicolons::AsNeeded
            }
        });
        let quote_style = options.single_quote.map(|single_quote| {
            if single_quote {
                QuoteStyle::Single
            } else {
                QuoteStyle::Double
            }
        });
        let jsx_quote_style = options.jsx_single_quote.map(|jsx_single_quote| {
            if jsx_single_quote {
                QuoteStyle::Single
            } else {
                QuoteStyle::Double
            }
        });
        let js_formatter = JsFormatterConfiguration {
            bracket_same_line: options.bracket_line.map(Into::into),
            arrow_parentheses: options.arrow_parens.map(|arrow_parens| arrow_parens.into()),
            semicolons,
            trailing_commas: options
                .trailing_comma
                .map(|trailing_comma| trailing_comma.into()),
            quote_style,
            quote_properties: options.quote_props.map(|quote_props| quote_props.into()),
            jsx_quote_style,
            ..Default::default()
        };
        let js_config = biome_configuration::JsConfiguration {
            formatter: Some(js_formatter),
            ..Default::default()
        };
        result.javascript = Some(js_config);
        Ok(result)
    }
}

/// A Prettier config can be embedded in `package.json`
const PACKAGE_JSON: &str = "package.json";

/// Prettie config files ordered by precedence
const CONFIG_FILES: [&str; 8] = [
    ".prettierrc",
    ".prettierrc.json",
    // Prefixed with `./` to ensure that it is loadable via Node.js's `import()`
    "./.prettierrc.js",
    "./prettier.config.js",
    "./.prettierrc.mjs",
    "./prettier.config.mjs",
    "./.prettierrc.cjs",
    "./prettier.config.cjs",
];

/// Prettier Ignore file. Use the same syntax as gitignore.
pub(crate) const IGNORE_FILE: &str = ".prettierignore";

/// This function is in charge of reading prettier files, deserialize its contents
pub(crate) fn read_config_file(
    fs: &dyn FileSystem,
    console: &mut dyn Console,
) -> Result<Config, CliDiagnostic> {
    // We don't report an error if Prettier config is not embedded in `PACKAGE_JSON`.
    if let Ok(data) = load_config(fs, Utf8Path::new(PACKAGE_JSON), console) {
        return Ok(Config {
            path: PACKAGE_JSON,
            data,
        });
    }
    for config_name in CONFIG_FILES {
        let path = Utf8Path::new(config_name);
        if fs.path_exists(path) {
            return Ok(Config {
                path: config_name,
                data: load_config(fs, path, console)?,
            });
        }
    }
    Err(CliDiagnostic::MigrateError(MigrationDiagnostic {
        reason: "Biome couldn't find a Prettier configuration file.".to_string(),
    }))
}

fn load_config(
    fs: &dyn FileSystem,
    path: &Utf8Path,
    console: &mut dyn Console,
) -> Result<PrettierConfiguration, CliDiagnostic> {
    let (deserialized, diagnostics) = match path.extension() {
        None | Some("json") => {
            let mut file = fs.open_with_options(path, OpenOptions::default().read(true))?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            if path.file_name().is_some_and(|name| name == PACKAGE_JSON) {
                let (deserialized, _) = deserialize_from_json_str::<PrettierPackageJson>(
                    &content,
                    JsonParserOptions::default()
                        .with_allow_trailing_commas()
                        .with_allow_comments(),
                    "",
                )
                .consume();
                (
                    deserialized.and_then(|packagejson| packagejson.prettier),
                    vec![],
                )
            } else {
                deserialize_from_json_str::<PrettierConfiguration>(
                    &content,
                    JsonParserOptions::default()
                        .with_allow_trailing_commas()
                        .with_allow_comments(),
                    "",
                )
                .consume()
            }
        }
        Some("js" | "mjs" | "cjs") => {
            let node::Resolution { content, .. } = node::load_config(path.as_ref())?;
            deserialize_from_json_str::<PrettierConfiguration>(
                &content,
                JsonParserOptions::default(),
                "",
            )
            .consume()
        }
        Some(ext) => {
            return Err(CliDiagnostic::MigrateError(MigrationDiagnostic {
                reason: format!(
                    "Prettier configuration ending with the extension `{ext}` are not supported."
                ),
            }))
        }
    };
    let path_str = path.to_string();
    // Heuristic: the Prettier config file is considered a YAML file if:
    // - desrialization failed
    // - there are at least 3 diagnostics
    // - the configuration file has no extension.
    // In this case we skip emitting the diagnostics
    if !(deserialized.is_none() && diagnostics.len() > 2 || path.extension().is_none()) {
        for diagnostic in diagnostics.into_iter().filter(|diag| {
            matches!(
                diag.severity(),
                biome_diagnostics::Severity::Fatal
                    | biome_diagnostics::Severity::Error
                    | biome_diagnostics::Severity::Warning
            )
        }) {
            let diagnostic = diagnostic.with_file_path(path_str.to_string());
            console.error(markup! {{PrintDiagnostic::simple(&diagnostic)}});
        }
    }
    if let Some(result) = deserialized {
        if result.end_of_line == EndOfLine::Auto {
            console.log(markup! {
                <Warn>"Prettier's `\"endOfLine\": \"auto\"` option is not supported in Biome. The default `\"lf\"` option is used instead."</Warn>
            });
        }
        Ok(result)
    } else if path.extension().is_none() {
        // The Prettier config file may be a YAML file.
        Err(CliDiagnostic::MigrateError(MigrationDiagnostic {
            reason: "Could not deserialize the Prettier configuration file.\nOnly JSON configurations are supported.".to_string(),
        }))
    } else {
        Err(CliDiagnostic::MigrateError(MigrationDiagnostic {
            reason: "Could not deserialize the Prettier configuration file".to_string(),
        }))
    }
}

#[cfg(test)]
mod tests {
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

        assert!(matches!(
            configuration,
            PrettierConfiguration { use_tabs: true, .. }
        ))
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

        assert!(matches!(
            configuration,
            PrettierConfiguration {
                use_tabs: true,
                print_width: 100,
                semi: true,
                single_quote: true,
                tab_width: 2,
                trailing_comma: PrettierTrailingComma::Es5,
                jsx_single_quote: true,
                ..
            }
        ))
    }
}
