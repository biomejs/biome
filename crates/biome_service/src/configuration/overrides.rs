use crate::configuration::formatter::{deserialize_line_width, serialize_line_width};
use crate::configuration::linter::rules;
use crate::configuration::{
    javascript_configuration, json_configuration, JavascriptConfiguration, JsonConfiguration,
    PlainIndentStyle,
};
use crate::settings::{
    to_matcher, LanguageListSettings, OverrideFormatSettings, OverrideLinterSettings,
    OverrideOrganizeImportsSettings, OverrideSettingPattern, OverrideSettings,
};
use crate::{MergeWith, Rules, WorkspaceError};
use biome_deserialize::StringSet;
use biome_formatter::{IndentStyle, LineEnding, LineWidth};
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
    #[bpaf(external, optional, hide)]
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
) -> Result<OverrideSettings, WorkspaceError> {
    let mut override_settings = OverrideSettings::default();
    for mut pattern in overrides.0 {
        let formatter = pattern.formatter.take().unwrap_or_default();
        let formatter = OverrideFormatSettings::try_from(formatter)?;

        let linter = pattern.linter.take().unwrap_or_default();
        let linter = OverrideLinterSettings::try_from(linter)?;

        let organize_imports = pattern.organize_imports.take().unwrap_or_default();
        let organize_imports = OverrideOrganizeImportsSettings::try_from(organize_imports)?;

        let mut languages = LanguageListSettings::default();
        if let Some(javascript) = pattern.javascript {
            languages.javascript = javascript.into();
        }

        if let Some(json) = pattern.json {
            languages.json = json.into();
        }

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

impl TryFrom<OverrideFormatterConfiguration> for OverrideFormatSettings {
    type Error = WorkspaceError;

    fn try_from(conf: OverrideFormatterConfiguration) -> Result<Self, Self::Error> {
        let indent_style = match conf.indent_style {
            Some(PlainIndentStyle::Tab) => IndentStyle::Tab,
            Some(PlainIndentStyle::Space) => IndentStyle::Space,
            None => IndentStyle::default(),
        };
        let indent_width = conf
            .indent_width
            .map(Into::into)
            .or(conf.indent_size.map(Into::into))
            .unwrap_or_default();

        Ok(Self {
            enabled: conf.enabled,
            indent_style: Some(indent_style),
            indent_width: Some(indent_width),
            line_ending: conf.line_ending,
            line_width: conf.line_width,
            format_with_errors: conf.format_with_errors.unwrap_or_default(),
        })
    }
}

impl TryFrom<OverrideLinterConfiguration> for OverrideLinterSettings {
    type Error = WorkspaceError;

    fn try_from(conf: OverrideLinterConfiguration) -> Result<Self, Self::Error> {
        Ok(Self {
            enabled: conf.enabled,
            rules: conf.rules,
        })
    }
}

impl TryFrom<OverrideOrganizeImportsConfiguration> for OverrideOrganizeImportsSettings {
    type Error = WorkspaceError;

    fn try_from(conf: OverrideOrganizeImportsConfiguration) -> Result<Self, Self::Error> {
        Ok(Self {
            enabled: conf.enabled,
        })
    }
}
