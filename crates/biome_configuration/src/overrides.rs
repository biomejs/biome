use crate::analyzer::assist::AssistEnabled;
use crate::analyzer::{LinterEnabled, RuleDomains};
use crate::formatter::{FormatWithErrorsEnabled, FormatterEnabled};
#[cfg(feature = "lang_html")]
use crate::html::HtmlConfiguration;
use crate::max_size::MaxSize;
use crate::{
    Configuration, FilesConfiguration, FormatterConfiguration, GritConfiguration,
    LinterConfiguration, Rules, analyzer::assist::AssistConfiguration,
};
use biome_deserialize::Merge;
use biome_deserialize_macros::{Deserializable, Merge};
use biome_formatter::{
    AttributePosition, BracketSameLine, BracketSpacing, DelimiterSpacing, Expand, IndentStyle,
    IndentWidth, LineEnding, LineWidth, TrailingNewline,
};
#[cfg(feature = "lang_js")]
use biome_js_formatter::context::trailing_commas::TrailingCommas;
#[cfg(feature = "plugins")]
use biome_plugin_loader::Plugins;
#[cfg(feature = "cli")]
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Overrides(pub Vec<OverridePattern>);

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct OverridePattern {
    /// A list of glob patterns. Biome will include files/folders that will
    /// match these patterns.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<OverrideGlobs>,

    /// Specific configuration for the JavaScript language
    #[cfg_attr(feature = "lang_js", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg(feature = "lang_js")]
    pub javascript: Option<crate::JsConfiguration>,

    /// Specific configuration for the Json language
    #[cfg(feature = "lang_json")]
    #[cfg_attr(feature = "lang_json", serde(skip_serializing_if = "Option::is_none"))]
    pub json: Option<crate::JsonConfiguration>,

    /// Specific configuration for the CSS language
    #[cfg_attr(feature = "lang_css", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg(feature = "lang_css")]
    pub css: Option<crate::CssConfiguration>,

    /// Specific configuration for the Graphql language
    #[cfg(feature = "lang_graphql")]
    #[cfg_attr(
        feature = "lang_graphql",
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub graphql: Option<crate::graphql::GraphqlConfiguration>,

    /// Specific configuration for the GritQL language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grit: Option<GritConfiguration>,

    /// Specific configuration for the GritQL language
    #[cfg(feature = "lang_html")]
    #[cfg_attr(feature = "lang_html", serde(skip_serializing_if = "Option::is_none"))]
    pub html: Option<HtmlConfiguration>,

    /// Specific configuration for the Json language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatter: Option<OverrideFormatterConfiguration>,

    /// Specific configuration for the Json language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linter: Option<OverrideLinterConfiguration>,

    /// Specific configuration for the Json language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assist: Option<OverrideAssistConfiguration>,

    /// Specific configuration for the filesystem
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<OverrideFilesConfiguration>,

    /// Specific configuration for additional plugins
    #[cfg(feature = "plugins")]
    #[cfg_attr(feature = "plugins", serde(skip_serializing_if = "Option::is_none"))]
    pub plugins: Option<Plugins>,
}

impl OverridePattern {
    /// Applies the configuration values of this override to an existing configuration.
    ///
    /// Matching is handled by the caller. `includes` and fields that runtime settings don't apply
    /// per file are excluded, while JavaScript globals replace the inherited set and plugin lists
    /// retain their append-merge behavior. Fields that runtime override settings inherit per
    /// pattern are reset to `base_configuration` when the override omits them.
    pub fn apply_to_configuration(
        &self,
        configuration: &mut Configuration,
        base_configuration: &Configuration,
    ) {
        let Self {
            includes: _,
            #[cfg(feature = "lang_js")]
            mut javascript,
            #[cfg(feature = "lang_json")]
            mut json,
            #[cfg(feature = "lang_css")]
            mut css,
            #[cfg(feature = "lang_graphql")]
            mut graphql,
            mut grit,
            #[cfg(feature = "lang_html")]
            mut html,
            mut formatter,
            mut linter,
            mut assist,
            files,
            #[cfg(feature = "plugins")]
            plugins,
        } = self.clone();

        #[cfg(feature = "lang_js")]
        if let Some(javascript) = javascript.as_mut() {
            javascript.resolver = None;
            javascript.experimental_embedded_snippets_enabled = None;
            if let Some(parser) = javascript.parser.as_mut() {
                parser.grit_metavariables = None;
                parser.jsx_everywhere = None;
            }
            if javascript
                .parser
                .as_ref()
                .is_some_and(|parser| parser.unsafe_parameter_decorators_enabled.is_none())
            {
                javascript.parser = None;
            }
        }
        #[cfg(feature = "lang_css")]
        if let Some(css) = css.as_mut() {
            css.globals = None;
        }
        #[cfg(feature = "lang_html")]
        if let Some(html) = html.as_mut() {
            html.experimental_full_support_enabled = None;
        }

        if let Some(formatter) = formatter.as_mut() {
            formatter.enabled = formatter.enabled.or_else(|| {
                base_configuration
                    .formatter
                    .as_ref()
                    .and_then(|formatter| formatter.enabled)
            });
        }
        if let Some(linter) = linter.as_mut() {
            linter.enabled = linter.enabled.or_else(|| {
                base_configuration
                    .linter
                    .as_ref()
                    .and_then(|linter| linter.enabled)
            });
        }
        if let Some(assist) = assist.as_mut() {
            assist.enabled = assist.enabled.or_else(|| {
                base_configuration
                    .assist
                    .as_ref()
                    .and_then(|assist| assist.enabled)
            });
        }

        #[cfg(feature = "lang_js")]
        if let Some(global) = formatter.as_ref() {
            let formatter = javascript
                .get_or_insert_with(Default::default)
                .formatter
                .get_or_insert_with(Default::default);
            formatter.enabled = formatter
                .enabled
                .or(global.enabled.map(|enabled| enabled.value().into()));
            formatter.indent_style = formatter.indent_style.or(global.indent_style);
            formatter.indent_width = formatter.indent_width.or(global.indent_width);
            formatter.line_ending = formatter.line_ending.or(global.line_ending);
            formatter.line_width = formatter.line_width.or(global.line_width);
            formatter.trailing_commas = formatter.trailing_commas.or(global.trailing_commas);
            formatter.bracket_spacing = formatter.bracket_spacing.or(global.bracket_spacing);
            formatter.delimiter_spacing = formatter.delimiter_spacing.or(global.delimiter_spacing);
            formatter.expand = formatter.expand.or(global.expand);
            formatter.attribute_position =
                formatter.attribute_position.or(global.attribute_position);
            formatter.trailing_newline = formatter.trailing_newline.or(global.trailing_newline);
        }
        #[cfg(feature = "lang_json")]
        if let Some(global) = formatter.as_ref() {
            let formatter = json
                .get_or_insert_with(Default::default)
                .formatter
                .get_or_insert_with(Default::default);
            formatter.enabled = formatter
                .enabled
                .or(global.enabled.map(|enabled| enabled.value().into()));
            formatter.indent_style = formatter.indent_style.or(global.indent_style);
            formatter.indent_width = formatter.indent_width.or(global.indent_width);
            formatter.line_ending = formatter.line_ending.or(global.line_ending);
            formatter.line_width = formatter.line_width.or(global.line_width);
            formatter.expand = formatter.expand.or(global.expand);
            formatter.bracket_spacing = formatter.bracket_spacing.or(global.bracket_spacing);
            formatter.delimiter_spacing = formatter.delimiter_spacing.or(global.delimiter_spacing);
            formatter.trailing_newline = formatter.trailing_newline.or(global.trailing_newline);
        }
        #[cfg(feature = "lang_css")]
        if let Some(global) = formatter.as_ref() {
            let formatter = css
                .get_or_insert_with(Default::default)
                .formatter
                .get_or_insert_with(Default::default);
            formatter.enabled = formatter
                .enabled
                .or(global.enabled.map(|enabled| enabled.value().into()));
            formatter.indent_style = formatter.indent_style.or(global.indent_style);
            formatter.indent_width = formatter.indent_width.or(global.indent_width);
            formatter.line_ending = formatter.line_ending.or(global.line_ending);
            formatter.line_width = formatter.line_width.or(global.line_width);
            formatter.delimiter_spacing = formatter.delimiter_spacing.or(global.delimiter_spacing);
            formatter.trailing_newline = formatter.trailing_newline.or(global.trailing_newline);
        }
        #[cfg(feature = "lang_graphql")]
        if let Some(global) = formatter.as_ref() {
            let formatter = graphql
                .get_or_insert_with(Default::default)
                .formatter
                .get_or_insert_with(Default::default);
            formatter.enabled = formatter
                .enabled
                .or(global.enabled.map(|enabled| enabled.value().into()));
            formatter.indent_style = formatter.indent_style.or(global.indent_style);
            formatter.indent_width = formatter.indent_width.or(global.indent_width);
            formatter.line_ending = formatter.line_ending.or(global.line_ending);
            formatter.line_width = formatter.line_width.or(global.line_width);
            formatter.bracket_spacing = formatter.bracket_spacing.or(global.bracket_spacing);
            formatter.trailing_newline = formatter.trailing_newline.or(global.trailing_newline);
        }
        if let Some(global) = formatter.as_ref() {
            let formatter = grit
                .get_or_insert_with(Default::default)
                .formatter
                .get_or_insert_with(Default::default);
            formatter.enabled = formatter
                .enabled
                .or(global.enabled.map(|enabled| enabled.value().into()));
            formatter.indent_style = formatter.indent_style.or(global.indent_style);
            formatter.indent_width = formatter.indent_width.or(global.indent_width);
            formatter.line_ending = formatter.line_ending.or(global.line_ending);
            formatter.line_width = formatter.line_width.or(global.line_width);
            formatter.trailing_newline = formatter.trailing_newline.or(global.trailing_newline);
        }
        #[cfg(feature = "lang_html")]
        if let Some(global) = formatter.as_ref() {
            let formatter = html
                .get_or_insert_with(Default::default)
                .formatter
                .get_or_insert_with(Default::default);
            formatter.enabled = formatter
                .enabled
                .or(global.enabled.map(|enabled| enabled.value().into()));
            formatter.indent_style = formatter.indent_style.or(global.indent_style);
            formatter.indent_width = formatter.indent_width.or(global.indent_width);
            formatter.line_ending = formatter.line_ending.or(global.line_ending);
            formatter.line_width = formatter.line_width.or(global.line_width);
            formatter.bracket_same_line = formatter.bracket_same_line.or(global.bracket_same_line);
            formatter.attribute_position =
                formatter.attribute_position.or(global.attribute_position);
            formatter.trailing_newline = formatter.trailing_newline.or(global.trailing_newline);
        }

        if let Some(global) = linter.as_ref() {
            #[cfg(feature = "lang_js")]
            {
                let linter = javascript
                    .get_or_insert_with(Default::default)
                    .linter
                    .get_or_insert_with(Default::default);
                linter.enabled = linter
                    .enabled
                    .or(global.enabled.map(|enabled| enabled.value().into()));
            }
            #[cfg(feature = "lang_json")]
            {
                let linter = json
                    .get_or_insert_with(Default::default)
                    .linter
                    .get_or_insert_with(Default::default);
                linter.enabled = linter
                    .enabled
                    .or(global.enabled.map(|enabled| enabled.value().into()));
            }
            #[cfg(feature = "lang_css")]
            {
                let linter = css
                    .get_or_insert_with(Default::default)
                    .linter
                    .get_or_insert_with(Default::default);
                linter.enabled = linter
                    .enabled
                    .or(global.enabled.map(|enabled| enabled.value().into()));
            }
            #[cfg(feature = "lang_graphql")]
            {
                let linter = graphql
                    .get_or_insert_with(Default::default)
                    .linter
                    .get_or_insert_with(Default::default);
                linter.enabled = linter
                    .enabled
                    .or(global.enabled.map(|enabled| enabled.value().into()));
            }
            let grit_linter = grit
                .get_or_insert_with(Default::default)
                .linter
                .get_or_insert_with(Default::default);
            grit_linter.enabled = grit_linter
                .enabled
                .or(global.enabled.map(|enabled| enabled.value().into()));
            #[cfg(feature = "lang_html")]
            {
                let linter = html
                    .get_or_insert_with(Default::default)
                    .linter
                    .get_or_insert_with(Default::default);
                linter.enabled = linter
                    .enabled
                    .or(global.enabled.map(|enabled| enabled.value().into()));
            }
        }

        if let Some(global) = assist.as_ref() {
            #[cfg(feature = "lang_js")]
            {
                let assist = javascript
                    .get_or_insert_with(Default::default)
                    .assist
                    .get_or_insert_with(Default::default);
                assist.enabled = assist
                    .enabled
                    .or(global.enabled.map(|enabled| enabled.value().into()));
            }
            #[cfg(feature = "lang_json")]
            {
                let assist = json
                    .get_or_insert_with(Default::default)
                    .assist
                    .get_or_insert_with(Default::default);
                assist.enabled = assist
                    .enabled
                    .or(global.enabled.map(|enabled| enabled.value().into()));
            }
            #[cfg(feature = "lang_css")]
            {
                let assist = css
                    .get_or_insert_with(Default::default)
                    .assist
                    .get_or_insert_with(Default::default);
                assist.enabled = assist
                    .enabled
                    .or(global.enabled.map(|enabled| enabled.value().into()));
            }
            #[cfg(feature = "lang_graphql")]
            {
                let assist = graphql
                    .get_or_insert_with(Default::default)
                    .assist
                    .get_or_insert_with(Default::default);
                assist.enabled = assist
                    .enabled
                    .or(global.enabled.map(|enabled| enabled.value().into()));
            }
            let grit_assist = grit
                .get_or_insert_with(Default::default)
                .assist
                .get_or_insert_with(Default::default);
            grit_assist.enabled = grit_assist
                .enabled
                .or(global.enabled.map(|enabled| enabled.value().into()));
            #[cfg(feature = "lang_html")]
            {
                let assist = html
                    .get_or_insert_with(Default::default)
                    .assist
                    .get_or_insert_with(Default::default);
                assist.enabled = assist
                    .enabled
                    .or(global.enabled.map(|enabled| enabled.value().into()));
            }
        }

        #[cfg(feature = "lang_js")]
        let javascript_globals = javascript
            .as_ref()
            .and_then(|javascript| javascript.globals.clone());

        #[cfg(feature = "lang_js")]
        {
            let base_unsafe_parameter_decorators_enabled = base_configuration
                .javascript
                .as_ref()
                .and_then(|javascript| javascript.parser.as_ref())
                .and_then(|parser| parser.unsafe_parameter_decorators_enabled);
            if base_unsafe_parameter_decorators_enabled.is_some()
                || configuration
                    .javascript
                    .as_ref()
                    .and_then(|javascript| javascript.parser.as_ref())
                    .is_some()
            {
                configuration
                    .javascript
                    .get_or_insert_with(Default::default)
                    .parser
                    .get_or_insert_with(Default::default)
                    .unsafe_parameter_decorators_enabled = base_unsafe_parameter_decorators_enabled;
            }
            if let Some(base_value) = base_configuration
                .javascript
                .as_ref()
                .and_then(|javascript| javascript.jsx_runtime)
            {
                let javascript = javascript.get_or_insert_with(Default::default);
                javascript.jsx_runtime = javascript.jsx_runtime.or(Some(base_value));
            }
        }

        #[cfg(feature = "lang_json")]
        {
            let base_parser = base_configuration
                .json
                .as_ref()
                .and_then(|json| json.parser.as_ref());
            let base_allow_comments = base_parser.and_then(|parser| parser.allow_comments);
            let base_allow_trailing_commas =
                base_parser.and_then(|parser| parser.allow_trailing_commas);
            if base_allow_comments.is_some() || base_allow_trailing_commas.is_some() {
                let parser = json
                    .get_or_insert_with(Default::default)
                    .parser
                    .get_or_insert_with(Default::default);
                parser.allow_comments = parser.allow_comments.or(base_allow_comments);
                parser.allow_trailing_commas =
                    parser.allow_trailing_commas.or(base_allow_trailing_commas);
            }
        }

        #[cfg(feature = "lang_css")]
        {
            let base_parser = base_configuration
                .css
                .as_ref()
                .and_then(|css| css.parser.as_ref());
            let base_allow_wrong_line_comments =
                base_parser.and_then(|parser| parser.allow_wrong_line_comments);
            let base_css_modules = base_parser.and_then(|parser| parser.css_modules);
            let base_tailwind_directives =
                base_parser.and_then(|parser| parser.tailwind_directives);
            if base_allow_wrong_line_comments.is_some()
                || base_css_modules.is_some()
                || base_tailwind_directives.is_some()
            {
                let parser = css
                    .get_or_insert_with(Default::default)
                    .parser
                    .get_or_insert_with(Default::default);
                parser.allow_wrong_line_comments = parser
                    .allow_wrong_line_comments
                    .or(base_allow_wrong_line_comments);
                parser.css_modules = parser.css_modules.or(base_css_modules);
                parser.tailwind_directives =
                    parser.tailwind_directives.or(base_tailwind_directives);
            }
        }

        let formatter = formatter.map(|formatter| FormatterConfiguration {
            enabled: formatter.enabled,
            format_with_errors: formatter.format_with_errors.or_else(|| {
                base_configuration
                    .formatter
                    .as_ref()
                    .and_then(|formatter| formatter.format_with_errors)
            }),
            indent_style: formatter.indent_style,
            indent_width: formatter.indent_width,
            line_ending: formatter.line_ending,
            line_width: formatter.line_width,
            attribute_position: formatter.attribute_position,
            bracket_same_line: formatter.bracket_same_line,
            bracket_spacing: formatter.bracket_spacing,
            delimiter_spacing: formatter.delimiter_spacing,
            expand: formatter.expand,
            trailing_newline: formatter.trailing_newline,
            ..FormatterConfiguration::default()
        });
        let linter = linter.map(|linter| LinterConfiguration {
            enabled: linter.enabled,
            rules: linter.rules,
            domains: linter.domains,
            ..LinterConfiguration::default()
        });
        let assist = assist.map(|assist| AssistConfiguration {
            enabled: assist.enabled,
            actions: assist.actions,
            ..AssistConfiguration::default()
        });
        let files = files.map(|files| FilesConfiguration {
            max_size: files.max_size,
            ..FilesConfiguration::default()
        });

        configuration.merge_with(Configuration {
            files,
            formatter,
            linter,
            #[cfg(feature = "lang_js")]
            javascript,
            #[cfg(feature = "lang_json")]
            json,
            #[cfg(feature = "lang_css")]
            css,
            #[cfg(feature = "lang_graphql")]
            graphql,
            grit,
            #[cfg(feature = "lang_html")]
            html,
            #[cfg(feature = "plugins")]
            plugins,
            assist,
            ..Configuration::default()
        });

        #[cfg(feature = "lang_js")]
        if let Some(globals) = javascript_globals {
            configuration
                .javascript
                .get_or_insert_with(Default::default)
                .globals = Some(globals);
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum OverrideGlobs {
    Globs(Box<[biome_glob::NormalizedGlob]>),
    EditorconfigGlob(Box<biome_glob::editorconfig::EditorconfigGlob>),
}
impl OverrideGlobs {
    /// Normalize `path` and match it against the list of globs.
    pub fn is_match_candidate(&self, path: &biome_glob::CandidatePath) -> bool {
        match self {
            Self::Globs(globs) => path.matches_with_exceptions(globs),
            Self::EditorconfigGlob(glob) => glob.is_match_candidate(path),
        }
    }
}
impl biome_deserialize::Deserializable for OverrideGlobs {
    fn deserialize(
        ctx: &mut dyn biome_deserialize::DeserializationContext,
        value: &impl biome_deserialize::DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        biome_deserialize::Deserializable::deserialize(ctx, value, name).map(OverrideGlobs::Globs)
    }
}
#[cfg(feature = "schema")]
impl schemars::JsonSchema for OverrideGlobs {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("OverrideGlobs")
    }
    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        Vec::<biome_glob::Glob>::json_schema(generator)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct OverrideFormatterConfiguration {
    // if `false`, it disables the feature. `true` by default
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "cli", bpaf(hide))]
    pub enabled: Option<FormatterEnabled>,

    /// Stores whether formatting should be allowed to proceed if a given file
    /// has syntax errors
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "cli", bpaf(hide))]
    pub format_with_errors: Option<FormatWithErrorsEnabled>,

    /// The indent style.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "cli", bpaf(long("indent-style"), argument("tab|space")))]
    pub indent_style: Option<IndentStyle>,

    /// The size of the indentation, 2 by default (deprecated, use `indent-width`)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[deserializable(deprecated(use_instead = "formatter.indentWidth"))]
    #[cfg_attr(feature = "cli", bpaf(long("indent-size"), argument("NUMBER")))]
    pub indent_size: Option<IndentWidth>,

    /// The size of the indentation, 2 by default
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "cli", bpaf(long("indent-width"), argument("NUMBER")))]
    pub indent_width: Option<IndentWidth>,

    /// The type of line ending.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "cli",
        bpaf(long("line-ending"), argument("lf|crlf|cr|auto"))
    )]
    pub line_ending: Option<LineEnding>,

    /// What's the max width of a line. Defaults to 80.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "cli", bpaf(long("line-width"), argument("NUMBER")))]
    pub line_width: Option<LineWidth>,

    /// The attribute position style.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "cli",
        bpaf(long("attribute-position"), argument("multiline|auto"))
    )]
    pub attribute_position: Option<AttributePosition>,

    /// Put the `>` of a multi-line HTML or JSX element at the end of the last line instead of being alone on the next line (does not apply to self closing elements).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "cli",
        bpaf(long("bracket-same-line"), argument("true|false"))
    )]
    pub bracket_same_line: Option<BracketSameLine>,

    /// Whether to insert spaces around brackets in object literals. Defaults to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "cli", bpaf(long("bracket-spacing"), argument("true|false")))]
    pub bracket_spacing: Option<BracketSpacing>,

    /// Controls spaces immediately inside supported delimiters when their content fits on one line.
    /// It doesn't add spaces before opening delimiters or inside empty delimiters.
    ///
    /// The affected delimiters vary by language. If unset, uses the configured formatter setting.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "cli",
        bpaf(long("delimiter-spacing"), argument("true|false"))
    )]
    pub delimiter_spacing: Option<DelimiterSpacing>,

    /// Controls whether arrays and objects are formatted on one line or multiple lines.
    ///
    /// `auto` formats objects on multiple lines if the first property has a newline, and arrays on
    /// one line if they fit.
    ///
    /// `always` formats arrays and objects on multiple lines.
    ///
    /// `never` formats arrays and objects on one line if they fit.
    ///
    /// If unset, uses the configured formatter setting.
    ///
    /// When formatting `package.json`, Biome uses `always` unless configured otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "cli",
        bpaf(long("object-wrap"), argument("auto|always|never"))
    )]
    pub expand: Option<Expand>,

    /// Print trailing commas wherever possible in multi-line comma-separated syntactic structures.
    #[cfg(feature = "lang_js")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "cli",
        bpaf(long("trailing-commas"), argument("all|es5|none"))
    )]
    pub trailing_commas: Option<TrailingCommas>,

    /// Whether to add a trailing newline at the end of the file.
    ///
    /// Setting this option to `false` is **highly discouraged** because it could cause many problems with other tools:
    /// - https://thoughtbot.com/blog/no-newline-at-end-of-file
    /// - https://callmeryan.medium.com/no-newline-at-end-of-file-navigating-gits-warning-for-android-developers-af14e73dd804
    /// - https://unix.stackexchange.com/questions/345548/how-to-cat-files-together-adding-missing-newlines-at-end-of-some-files
    ///
    /// Disable the option at your own risk.
    ///
    /// Defaults to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "cli",
        bpaf(long("trailing-newline"), argument("true|false"))
    )]
    pub trailing_newline: Option<TrailingNewline>,
}

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct OverrideLinterConfiguration {
    /// if `false`, it disables the feature and the linter won't be executed. `true` by default
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "cli", bpaf(hide))]
    pub enabled: Option<LinterEnabled>,

    /// List of rules
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "cli", bpaf(pure(Default::default()), hide))]
    pub rules: Option<Rules>,

    /// List of rules
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "cli", bpaf(pure(Default::default()), optional, hide))]
    pub domains: Option<RuleDomains>,
}

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OverrideFilesConfiguration {
    /// File size limit in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size: Option<MaxSize>,
}

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct OverrideAssistConfiguration {
    /// if `false`, it disables the feature and the assist won't be executed. `true` by default
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "cli", bpaf(hide))]
    pub enabled: Option<AssistEnabled>,

    /// List of actions
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "cli",
        bpaf(pure(crate::analyzer::assist::Actions::default()), optional, hide)
    )]
    pub actions: Option<crate::analyzer::assist::Actions>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn applies_override_values_without_deserializing_configuration() {
        let mut configuration = Configuration {
            formatter: Some(FormatterConfiguration {
                enabled: Some(true.into()),
                ..FormatterConfiguration::default()
            }),
            ..Configuration::default()
        };
        let pattern = OverridePattern {
            formatter: Some(OverrideFormatterConfiguration {
                enabled: Some(false.into()),
                ..OverrideFormatterConfiguration::default()
            }),
            ..OverridePattern::default()
        };

        let base_configuration = configuration.clone();
        pattern.apply_to_configuration(&mut configuration, &base_configuration);

        assert_eq!(
            configuration
                .formatter
                .and_then(|formatter| formatter.enabled),
            Some(false.into())
        );
    }

    #[test]
    fn resets_runtime_inherited_values_for_each_override() {
        let base_configuration = Configuration {
            formatter: Some(FormatterConfiguration {
                format_with_errors: Some(false.into()),
                ..FormatterConfiguration::default()
            }),
            ..Configuration::default()
        };
        let mut configuration = base_configuration.clone();
        let first_pattern = OverridePattern {
            formatter: Some(OverrideFormatterConfiguration {
                format_with_errors: Some(true.into()),
                ..OverrideFormatterConfiguration::default()
            }),
            ..OverridePattern::default()
        };
        let second_pattern = OverridePattern {
            formatter: Some(OverrideFormatterConfiguration {
                line_width: Some(LineWidth::try_from(120).expect("valid line width")),
                ..OverrideFormatterConfiguration::default()
            }),
            ..OverridePattern::default()
        };

        first_pattern.apply_to_configuration(&mut configuration, &base_configuration);
        second_pattern.apply_to_configuration(&mut configuration, &base_configuration);

        let formatter = configuration.formatter.expect("formatter configuration");
        assert_eq!(formatter.format_with_errors, Some(false.into()));
        assert_eq!(
            formatter.line_width,
            Some(LineWidth::try_from(120).expect("valid line width"))
        );
    }

    #[cfg(all(feature = "lang_js", feature = "lang_json", feature = "lang_css"))]
    #[test]
    fn resets_language_runtime_fallbacks_for_each_override() {
        use crate::{
            CssConfiguration, CssParserConfiguration, JsConfiguration, JsonConfiguration,
            javascript::{JsParserConfiguration, JsxRuntime},
            json::JsonParserConfiguration,
        };

        let base_configuration = Configuration {
            javascript: Some(JsConfiguration {
                parser: Some(JsParserConfiguration {
                    unsafe_parameter_decorators_enabled: Some(true.into()),
                    ..JsParserConfiguration::default()
                }),
                jsx_runtime: Some(JsxRuntime::ReactClassic),
                ..JsConfiguration::default()
            }),
            json: Some(JsonConfiguration {
                parser: Some(JsonParserConfiguration {
                    allow_comments: Some(true.into()),
                    ..JsonParserConfiguration::default()
                }),
                ..JsonConfiguration::default()
            }),
            css: Some(CssConfiguration {
                parser: Some(CssParserConfiguration {
                    css_modules: Some(true.into()),
                    ..CssParserConfiguration::default()
                }),
                ..CssConfiguration::default()
            }),
            ..Configuration::default()
        };
        let mut configuration = base_configuration.clone();
        let first_pattern = OverridePattern {
            javascript: Some(JsConfiguration {
                parser: Some(JsParserConfiguration {
                    unsafe_parameter_decorators_enabled: Some(false.into()),
                    ..JsParserConfiguration::default()
                }),
                jsx_runtime: Some(JsxRuntime::Transparent),
                ..JsConfiguration::default()
            }),
            json: Some(JsonConfiguration {
                parser: Some(JsonParserConfiguration {
                    allow_comments: Some(false.into()),
                    ..JsonParserConfiguration::default()
                }),
                ..JsonConfiguration::default()
            }),
            css: Some(CssConfiguration {
                parser: Some(CssParserConfiguration {
                    css_modules: Some(false.into()),
                    ..CssParserConfiguration::default()
                }),
                ..CssConfiguration::default()
            }),
            ..OverridePattern::default()
        };

        first_pattern.apply_to_configuration(&mut configuration, &base_configuration);
        OverridePattern::default().apply_to_configuration(&mut configuration, &base_configuration);

        let javascript = configuration.javascript.expect("JavaScript configuration");
        assert_eq!(
            javascript
                .parser
                .and_then(|parser| parser.unsafe_parameter_decorators_enabled),
            Some(true.into())
        );
        assert_eq!(javascript.jsx_runtime, Some(JsxRuntime::ReactClassic));
        assert_eq!(
            configuration
                .json
                .and_then(|json| json.parser)
                .and_then(|parser| parser.allow_comments),
            Some(true.into())
        );
        assert_eq!(
            configuration
                .css
                .and_then(|css| css.parser)
                .and_then(|parser| parser.css_modules),
            Some(true.into())
        );
    }

    #[cfg(feature = "lang_js")]
    #[test]
    fn clears_unsafe_parameter_decorators_without_a_base_fallback() {
        use crate::{JsConfiguration, javascript::JsParserConfiguration};

        let base_configuration = Configuration::default();
        let mut configuration = base_configuration.clone();
        let first_pattern = OverridePattern {
            javascript: Some(JsConfiguration {
                parser: Some(JsParserConfiguration {
                    unsafe_parameter_decorators_enabled: Some(true.into()),
                    ..JsParserConfiguration::default()
                }),
                ..JsConfiguration::default()
            }),
            ..OverridePattern::default()
        };

        first_pattern.apply_to_configuration(&mut configuration, &base_configuration);
        OverridePattern::default().apply_to_configuration(&mut configuration, &base_configuration);

        assert_eq!(
            configuration
                .javascript
                .and_then(|javascript| javascript.parser)
                .and_then(|parser| parser.unsafe_parameter_decorators_enabled),
            None
        );
    }
}
