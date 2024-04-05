use crate::diagnostics::MigrationDiagnostic;
use crate::CliDiagnostic;
use biome_console::{markup, Console, ConsoleExt};
use biome_deserialize::json::deserialize_from_json_str;
use biome_deserialize_macros::Deserializable;
use biome_diagnostics::{DiagnosticExt, PrintDiagnostic};
use biome_formatter::{AttributePosition, LineEnding, LineWidth, QuoteStyle};
use biome_fs::{FileSystem, OpenOptions};
use biome_js_formatter::context::{ArrowParentheses, QuoteProperties, Semicolons, TrailingComma};
use biome_json_parser::JsonParserOptions;
use biome_service::DynRef;
use std::path::Path;

use super::node;

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

#[derive(Clone, Debug, Deserializable, Eq, PartialEq)]
#[deserializable(unknown_fields = "allow")]
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

impl TryFrom<PrettierConfiguration> for biome_configuration::PartialConfiguration {
    type Error = String;
    fn try_from(value: PrettierConfiguration) -> Result<Self, Self::Error> {
        let mut result = biome_configuration::PartialConfiguration::default();

        let line_width = LineWidth::try_from(value.print_width).map_err(|err| err.to_string())?;
        let indent_style = if value.use_tabs {
            biome_configuration::PlainIndentStyle::Tab
        } else {
            biome_configuration::PlainIndentStyle::Space
        };
        let formatter = biome_configuration::PartialFormatterConfiguration {
            indent_width: Some(value.tab_width),
            line_width: Some(line_width),
            indent_style: Some(indent_style),
            line_ending: Some(value.end_of_line.into()),
            attribute_position: Some(AttributePosition::default()),
            format_with_errors: Some(false),
            ignore: None,
            include: None,
            enabled: Some(true),
            // deprecated
            indent_size: None,
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
        let js_formatter = biome_configuration::PartialJavascriptFormatter {
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
            attribute_position: Some(AttributePosition::default()),
        };
        let js_config = biome_configuration::PartialJavascriptConfiguration {
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
    // Prefixed with `./` to ensure that it is loadable via NodeJS's `import()`
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
    fs: &DynRef<'_, dyn FileSystem>,
    console: &mut dyn Console,
) -> Result<Config, CliDiagnostic> {
    // We don't report an error if Prettier config is not embedded in `PACKAGE_JSON`.
    if let Ok(data) = load_config(fs, Path::new(PACKAGE_JSON), console) {
        return Ok(Config {
            path: PACKAGE_JSON,
            data,
        });
    }
    for config_name in CONFIG_FILES {
        let path = Path::new(config_name);
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
    fs: &DynRef<'_, dyn FileSystem>,
    path: &Path,
    console: &mut dyn Console,
) -> Result<PrettierConfiguration, CliDiagnostic> {
    let (deserialized, diagnostics) = match path.extension().and_then(|file_ext| file_ext.to_str())
    {
        Some("json") | None => {
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
            let node::Resolution { content, .. } = node::load_config(&path.to_string_lossy())?;
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
    let path_str = path.to_string_lossy();
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
    if let Some(result) = deserialized {
        if result.end_of_line == EndOfLine::Auto {
            console.log(markup! {
                <Warn>"Prettier's `\"endOfLine\": \"auto\"` option is not supported in Biome. The default `\"lf\"` option is used instead."</Warn>
            });
        }
        Ok(result)
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
