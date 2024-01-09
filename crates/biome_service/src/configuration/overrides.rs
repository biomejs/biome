use crate::configuration::formatter::{deserialize_line_width, serialize_line_width};
use crate::configuration::{
    css_configuration, javascript_configuration, json_configuration, CssConfiguration,
    JavascriptConfiguration, JsonConfiguration, PlainIndentStyle,
};
use crate::settings::{
    to_matcher, FormatSettings, LanguageListSettings, LanguageSettings, LinterSettings,
    OrganizeImportsSettings, OverrideFormatSettings, OverrideLinterSettings,
    OverrideOrganizeImportsSettings, OverrideSettingPattern, OverrideSettings, WorkspaceSettings,
};
use crate::{MergeWith, Rules, WorkspaceError};
use biome_css_syntax::CssLanguage;
use biome_deserialize::StringSet;
use biome_formatter::{LineEnding, LineWidth};
use biome_js_syntax::JsLanguage;
use biome_json_syntax::JsonLanguage;
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Default, Serialize, Deserialize, Eq, PartialEq, Clone, Bpaf)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Overrides(#[bpaf(hide)] pub Vec<OverridePattern>);

impl FromStr for Overrides {
    type Err = String;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(Self::default())
    }
}

impl MergeWith<Overrides> for Overrides {
    fn merge_with(&mut self, other: Overrides) {
        let mut self_iter = self.0.iter_mut();
        let mut other_iter = other.0.into_iter();
        while let (Some(self_item), Some(other_item)) = (self_iter.next(), other_iter.next()) {
            self_item.merge_with(other_item);
        }
    }

    fn merge_with_if_not_default(&mut self, other: Overrides)
    where
        Overrides: Default,
    {
        let mut self_iter = self.0.iter_mut();
        let mut other_iter = other.0.into_iter();
        while let (Some(self_item), Some(other_item)) = (self_iter.next(), other_iter.next()) {
            self_item.merge_with_if_not_default(other_item);
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Eq, PartialEq, Clone, Bpaf)]
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
    #[bpaf(external(javascript_configuration), optional, hide)]
    pub javascript: Option<JavascriptConfiguration>,

    /// Specific configuration for the Json language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external(json_configuration), optional, hide)]
    pub json: Option<JsonConfiguration>,

    /// Specific configuration for the Css language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external(css_configuration), optional, hide)]
    pub css: Option<CssConfiguration>,

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

impl MergeWith<OverridePattern> for OverridePattern {
    fn merge_with(&mut self, other: OverridePattern) {
        if let Some(other) = other.ignore {
            let ignore = self.ignore.get_or_insert(StringSet::default());
            ignore.extend(other.into_index_set());
        }

        if let Some(other) = other.include {
            let include = self.include.get_or_insert(StringSet::default());
            include.extend(other.into_index_set());
        }

        if let Some(other) = other.formatter {
            let formatter = self
                .formatter
                .get_or_insert(OverrideFormatterConfiguration::default());
            formatter.merge_with(other);
        }
        if let Some(other) = other.linter {
            let linter = self
                .linter
                .get_or_insert(OverrideLinterConfiguration::default());
            linter.merge_with(other);
        }

        if let Some(other) = other.organize_imports {
            let organize_imports = self
                .organize_imports
                .get_or_insert(OverrideOrganizeImportsConfiguration::default());
            organize_imports.merge_with(other);
        }
        if let Some(other) = other.javascript {
            let javascript = self
                .javascript
                .get_or_insert(JavascriptConfiguration::default());
            javascript.merge_with(other)
        }
        if let Some(other) = other.json {
            let json = self.json.get_or_insert(JsonConfiguration::default());
            json.merge_with(other)
        }
    }
    fn merge_with_if_not_default(&mut self, other: OverridePattern)
    where
        OverridePattern: Default,
    {
        if let Some(other) = other.ignore {
            let ignore = self.ignore.get_or_insert(StringSet::default());
            ignore.extend(other.into_index_set());
        }

        if let Some(other) = other.include {
            let include = self.include.get_or_insert(StringSet::default());
            include.extend(other.into_index_set());
        }

        if let Some(other) = other.formatter {
            let formatter = self
                .formatter
                .get_or_insert(OverrideFormatterConfiguration::default());
            formatter.merge_with_if_not_default(other);
        }
        if let Some(other) = other.linter {
            let linter = self
                .linter
                .get_or_insert(OverrideLinterConfiguration::default());
            linter.merge_with_if_not_default(other);
        }

        if let Some(other) = other.organize_imports {
            let organize_imports = self
                .organize_imports
                .get_or_insert(OverrideOrganizeImportsConfiguration::default());
            organize_imports.merge_with_if_not_default(other);
        }
        if let Some(other) = other.javascript {
            let javascript = self
                .javascript
                .get_or_insert(JavascriptConfiguration::default());
            javascript.merge_with_if_not_default(other)
        }
        if let Some(other) = other.json {
            let json = self.json.get_or_insert(JsonConfiguration::default());
            json.merge_with_if_not_default(other)
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Eq, PartialEq, Clone, Bpaf)]
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
}

impl MergeWith<OverrideFormatterConfiguration> for OverrideFormatterConfiguration {
    fn merge_with(&mut self, other: OverrideFormatterConfiguration) {
        if let Some(enabled) = other.enabled {
            self.enabled = Some(enabled);
        }
        if let Some(indent_size) = other.indent_size {
            self.indent_width = Some(indent_size);
        }
        if let Some(indent_width) = other.indent_width {
            self.indent_width = Some(indent_width);
        }
        if let Some(indent_style) = other.indent_style {
            self.indent_style = Some(indent_style);
        }
        if let Some(line_width) = other.line_width {
            self.line_width = Some(line_width);
        }

        if let Some(format_with_errors) = other.format_with_errors {
            self.format_with_errors = Some(format_with_errors);
        }
    }

    fn merge_with_if_not_default(&mut self, other: OverrideFormatterConfiguration)
    where
        OverrideFormatterConfiguration: Default,
    {
        if other != OverrideFormatterConfiguration::default() {
            self.merge_with(other)
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Eq, PartialEq, Clone, Bpaf)]
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

impl MergeWith<OverrideLinterConfiguration> for OverrideLinterConfiguration {
    fn merge_with(&mut self, other: OverrideLinterConfiguration) {
        if let Some(enabled) = other.enabled {
            self.enabled = Some(enabled);
        }
        if let Some(rules) = other.rules {
            self.rules = Some(rules);
        }
    }

    fn merge_with_if_not_default(&mut self, other: OverrideLinterConfiguration)
    where
        OverrideLinterConfiguration: Default,
    {
        if other != OverrideLinterConfiguration::default() {
            self.merge_with(other)
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Eq, PartialEq, Clone, Bpaf)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct OverrideOrganizeImportsConfiguration {
    /// if `false`, it disables the feature and the linter won't be executed. `true` by default
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(hide)]
    pub enabled: Option<bool>,
}

impl MergeWith<OverrideOrganizeImportsConfiguration> for OverrideOrganizeImportsConfiguration {
    fn merge_with(&mut self, other: OverrideOrganizeImportsConfiguration) {
        if let Some(enabled) = other.enabled {
            self.enabled = Some(enabled);
        }
    }

    fn merge_with_if_not_default(&mut self, other: OverrideOrganizeImportsConfiguration)
    where
        OverrideOrganizeImportsConfiguration: Default,
    {
        if other != OverrideOrganizeImportsConfiguration::default() {
            self.merge_with(other)
        }
    }
}

pub fn to_override_settings(
    overrides: Overrides,
    vcs_base_path: Option<PathBuf>,
    gitignore_matches: &[String],
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
            include: to_matcher(
                pattern.include.as_ref(),
                vcs_base_path.clone(),
                gitignore_matches,
            )?,
            exclude: to_matcher(
                pattern.ignore.as_ref(),
                vcs_base_path.clone(),
                gitignore_matches,
            )?,
            formatter,
            linter,
            organize_imports,

            languages,
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
    mut conf: JavascriptConfiguration,
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
        .map(|global| global.into_index_set())
        .or(parent_settings.globals.clone());

    language_setting
}

fn to_json_language_settings(
    mut conf: JsonConfiguration,
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
    mut conf: CssConfiguration,
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
