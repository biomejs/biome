use super::javascript::PartialJavascriptConfiguration;
use super::json::PartialJsonConfiguration;
use super::PartialCssConfiguration;
use crate::configuration::formatter::{deserialize_line_width, serialize_line_width};
use crate::configuration::{
    partial_css_configuration, partial_javascript_configuration, partial_json_configuration,
    PlainIndentStyle,
};
use crate::settings::{
    to_matcher, FormatSettings, LanguageListSettings, LanguageSettings, LinterSettings,
    OrganizeImportsSettings, OverrideFormatSettings, OverrideLinterSettings,
    OverrideOrganizeImportsSettings, OverrideSettingPattern, OverrideSettings, WorkspaceSettings,
};
use crate::{Rules, WorkspaceError};
use biome_css_syntax::CssLanguage;
use biome_deserialize::StringSet;
use biome_deserialize_macros::{Deserializable, Merge};
use biome_formatter::{AttributePosition, LineEnding, LineWidth};
use biome_js_syntax::JsLanguage;
use biome_json_syntax::JsonLanguage;
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(
    Bpaf, Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Overrides(#[bpaf(hide)] pub Vec<OverridePattern>);

impl FromStr for Overrides {
    type Err = String;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(Self::default())
    }
}

#[derive(
    Bpaf, Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct OverridePattern {
    /// A list of Unix shell style patterns. The formatter will ignore files/folders that will
    /// match these patterns.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(hide)]
    pub ignore: Option<StringSet>,

    /// A list of Unix shell style patterns. The formatter will include files/folders that will
    /// match these patterns.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(hide)]
    pub include: Option<StringSet>,

    /// Specific configuration for the JavaScript language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external(partial_javascript_configuration), optional, hide)]
    pub javascript: Option<PartialJavascriptConfiguration>,

    /// Specific configuration for the Json language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external(partial_json_configuration), optional, hide)]
    pub json: Option<PartialJsonConfiguration>,

    /// Specific configuration for the Css language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external(partial_css_configuration), optional, hide)]
    pub css: Option<PartialCssConfiguration>,

    /// Specific configuration for the Json language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external(override_formatter_configuration), optional, hide)]
    pub formatter: Option<OverrideFormatterConfiguration>,

    /// Specific configuration for the Json language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external(override_linter_configuration), optional, hide)]
    pub linter: Option<OverrideLinterConfiguration>,

    /// Specific configuration for the Json language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external(override_organize_imports_configuration), optional, hide)]
    pub organize_imports: Option<OverrideOrganizeImportsConfiguration>,
}

impl FromStr for OverridePattern {
    type Err = String;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(Self::default())
    }
}

#[derive(
    Bpaf, Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct OverrideFormatterConfiguration {
    // if `false`, it disables the feature. `true` by default
    #[bpaf(hide)]
    pub enabled: Option<bool>,

    /// Stores whether formatting should be allowed to proceed if a given file
    /// has syntax errors
    #[bpaf(hide)]
    pub format_with_errors: Option<bool>,

    /// The indent style.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(long("indent-style"), argument("tab|space"), optional)]
    pub indent_style: Option<PlainIndentStyle>,

    /// The size of the indentation, 2 by default (deprecated, use `indent-width`)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[deserializable(deprecated(use_instead = "formatter.indentWidth"))]
    #[bpaf(long("indent-size"), argument("NUMBER"), optional)]
    pub indent_size: Option<u8>,

    /// The size of the indentation, 2 by default
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(long("indent-width"), argument("NUMBER"), optional)]
    pub indent_width: Option<u8>,

    /// The type of line ending.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(long("line-ending"), argument("lf|crlf|cr"), optional)]
    pub line_ending: Option<LineEnding>,

    /// What's the max width of a line. Defaults to 80.
    #[serde(
        deserialize_with = "deserialize_line_width",
        serialize_with = "serialize_line_width"
    )]
    #[bpaf(long("line-width"), argument("NUMBER"), optional)]
    pub line_width: Option<LineWidth>,

    /// The attribute position style.
    #[bpaf(long("attribute-position"), argument("auto|multiline"), optional)]
    pub attribute_position: Option<AttributePosition>,
}

#[derive(
    Bpaf, Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct OverrideLinterConfiguration {
    /// if `false`, it disables the feature and the linter won't be executed. `true` by default
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(hide)]
    pub enabled: Option<bool>,

    /// List of rules
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(pure(Rules::default()), optional, hide)]
    pub rules: Option<Rules>,
}

#[derive(
    Bpaf, Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct OverrideOrganizeImportsConfiguration {
    /// if `false`, it disables the feature and the linter won't be executed. `true` by default
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(hide)]
    pub enabled: Option<bool>,
}

pub fn to_override_settings(
    working_directory: Option<PathBuf>,
    overrides: Overrides,
    current_settings: &WorkspaceSettings,
) -> Result<OverrideSettings, WorkspaceError> {
    let mut override_settings = OverrideSettings::default();
    for mut pattern in overrides.0 {
        let formatter = pattern.formatter.take().unwrap_or_default();
        let formatter = to_format_settings(formatter, &current_settings.formatter);

        let linter = pattern.linter.take().unwrap_or_default();
        let linter = to_override_linter_settings(linter, &current_settings.linter);

        let organize_imports = pattern.organize_imports.take().unwrap_or_default();
        let organize_imports =
            to_organize_imports_settings(organize_imports, &current_settings.organize_imports);

        let mut languages = LanguageListSettings::default();
        let javascript = pattern.javascript.take().unwrap_or_default();
        let json = pattern.json.take().unwrap_or_default();
        let css = pattern.css.take().unwrap_or_default();
        languages.javascript =
            to_javascript_language_settings(javascript, &current_settings.languages.javascript);

        languages.json = to_json_language_settings(json, &current_settings.languages.json);
        languages.css = to_css_language_settings(css, &current_settings.languages.css);

        let pattern_setting = OverrideSettingPattern {
            include: to_matcher(working_directory.clone(), pattern.include.as_ref())?,
            exclude: to_matcher(working_directory.clone(), pattern.ignore.as_ref())?,
            formatter,
            linter,
            organize_imports,
            languages,
            ..OverrideSettingPattern::default()
        };

        override_settings.patterns.push(pattern_setting);
    }

    Ok(override_settings)
}

pub(crate) fn to_format_settings(
    conf: OverrideFormatterConfiguration,
    format_settings: &FormatSettings,
) -> OverrideFormatSettings {
    let indent_style = conf
        .indent_style
        .map(Into::into)
        .or(format_settings.indent_style);
    let indent_width = conf
        .indent_width
        .map(Into::into)
        .or(conf.indent_size.map(Into::into))
        .or(format_settings.indent_width);

    let line_ending = conf.line_ending.or(format_settings.line_ending);
    let line_width = conf.line_width.or(format_settings.line_width);
    let format_with_errors = conf
        .format_with_errors
        .unwrap_or(format_settings.format_with_errors);

    OverrideFormatSettings {
        enabled: conf.enabled.or(Some(format_settings.enabled)),
        indent_style,
        indent_width,
        line_ending,
        line_width,
        format_with_errors,
    }
}

fn to_javascript_language_settings(
    mut conf: PartialJavascriptConfiguration,
    parent_settings: &LanguageSettings<JsLanguage>,
) -> LanguageSettings<JsLanguage> {
    let mut language_setting: LanguageSettings<JsLanguage> = LanguageSettings::default();
    let formatter = conf.formatter.take().unwrap_or_default();
    let parent_formatter = &parent_settings.formatter;
    language_setting.formatter.quote_style = formatter.quote_style.or(parent_formatter.quote_style);
    language_setting.formatter.jsx_quote_style = formatter
        .jsx_quote_style
        .or(parent_formatter.jsx_quote_style);
    language_setting.formatter.quote_properties = formatter
        .quote_properties
        .or(parent_formatter.quote_properties);
    language_setting.formatter.trailing_comma =
        formatter.trailing_comma.or(parent_formatter.trailing_comma);
    language_setting.formatter.semicolons = formatter.semicolons.or(parent_formatter.semicolons);
    language_setting.formatter.arrow_parentheses = formatter
        .arrow_parentheses
        .or(parent_formatter.arrow_parentheses);
    language_setting.formatter.bracket_spacing = formatter
        .bracket_spacing
        .map(Into::into)
        .or(parent_formatter.bracket_spacing);
    language_setting.formatter.bracket_same_line = formatter
        .bracket_same_line
        .map(Into::into)
        .or(parent_formatter.bracket_same_line);
    language_setting.formatter.enabled = formatter.enabled.or(parent_formatter.enabled);
    language_setting.formatter.line_width = formatter.line_width.or(parent_formatter.line_width);
    language_setting.formatter.line_ending = formatter.line_ending.or(parent_formatter.line_ending);
    language_setting.formatter.indent_width = formatter
        .indent_width
        .map(Into::into)
        .or(formatter.indent_size.map(Into::into))
        .or(parent_formatter.indent_width);
    language_setting.formatter.indent_style = formatter
        .indent_style
        .map(Into::into)
        .or(parent_formatter.indent_style);

    let parser = conf.parser.take().unwrap_or_default();
    let parent_parser = &parent_settings.parser;
    language_setting.parser.parse_class_parameter_decorators = parser
        .unsafe_parameter_decorators_enabled
        .unwrap_or(parent_parser.parse_class_parameter_decorators);

    let organize_imports = conf.organize_imports;
    if let Some(_organize_imports) = organize_imports {}

    language_setting.globals = conf
        .globals
        .map(StringSet::into_index_set)
        .or_else(|| parent_settings.globals.clone());

    language_setting
}

fn to_json_language_settings(
    mut conf: PartialJsonConfiguration,
    parent_settings: &LanguageSettings<JsonLanguage>,
) -> LanguageSettings<JsonLanguage> {
    let mut language_setting: LanguageSettings<JsonLanguage> = LanguageSettings::default();
    let formatter = conf.formatter.take().unwrap_or_default();
    let parent_formatter = &parent_settings.formatter;

    language_setting.formatter.enabled = formatter.enabled.or(parent_formatter.enabled);
    language_setting.formatter.line_width = formatter.line_width.or(parent_formatter.line_width);
    language_setting.formatter.line_ending = formatter.line_ending.or(parent_formatter.line_ending);
    language_setting.formatter.indent_width = formatter
        .indent_width
        .map(Into::into)
        .or(formatter.indent_size.map(Into::into))
        .or(parent_formatter.indent_width);
    language_setting.formatter.indent_style = formatter
        .indent_style
        .map(Into::into)
        .or(parent_formatter.indent_style);

    let parser = conf.parser.take().unwrap_or_default();
    let parent_parser = &parent_settings.parser;
    language_setting.parser.allow_comments = parser
        .allow_comments
        .unwrap_or(parent_parser.allow_comments);

    language_setting.parser.allow_trailing_commas = parser
        .allow_trailing_commas
        .unwrap_or(parent_parser.allow_trailing_commas);

    language_setting
}

fn to_css_language_settings(
    mut conf: PartialCssConfiguration,
    parent_settings: &LanguageSettings<CssLanguage>,
) -> LanguageSettings<CssLanguage> {
    let mut language_setting: LanguageSettings<CssLanguage> = LanguageSettings::default();
    let formatter = conf.formatter.take().unwrap_or_default();
    let parent_formatter = &parent_settings.formatter;

    language_setting.formatter.enabled = formatter.enabled.or(parent_formatter.enabled);
    language_setting.formatter.line_width = formatter.line_width.or(parent_formatter.line_width);
    language_setting.formatter.line_ending = formatter.line_ending.or(parent_formatter.line_ending);
    language_setting.formatter.indent_width = formatter
        .indent_width
        .map(Into::into)
        .or(formatter.indent_size.map(Into::into))
        .or(parent_formatter.indent_width);
    language_setting.formatter.indent_style = formatter
        .indent_style
        .map(Into::into)
        .or(parent_formatter.indent_style);
    language_setting.formatter.quote_style = formatter.quote_style.or(parent_formatter.quote_style);

    let parser = conf.parser.take().unwrap_or_default();
    let parent_parser = &parent_settings.parser;
    language_setting.parser.allow_wrong_line_comments = parser
        .allow_wrong_line_comments
        .unwrap_or(parent_parser.allow_wrong_line_comments);

    language_setting
}

fn to_override_linter_settings(
    conf: OverrideLinterConfiguration,
    lint_settings: &LinterSettings,
) -> OverrideLinterSettings {
    OverrideLinterSettings {
        enabled: conf.enabled.or(Some(lint_settings.enabled)),
        rules: conf.rules.or(lint_settings.rules.clone()),
    }
}

fn to_organize_imports_settings(
    conf: OverrideOrganizeImportsConfiguration,
    settings: &OrganizeImportsSettings,
) -> OverrideOrganizeImportsSettings {
    OverrideOrganizeImportsSettings {
        enabled: conf.enabled.or(Some(settings.enabled)),
    }
}
