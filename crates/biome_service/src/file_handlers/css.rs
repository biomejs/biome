use super::{ExtensionHandler, Mime};
use crate::file_handlers::{
    AnalyzerCapabilities, Capabilities, FormatterCapabilities, ParserCapabilities,
};
use crate::file_handlers::{DebugCapabilities, Language as LanguageId};
use crate::settings::{
    FormatSettings, Language, LanguageListSettings, LanguageSettings, OverrideSettings,
    SettingsHandle,
};
use crate::workspace::GetSyntaxTreeResult;
use crate::WorkspaceError;
use biome_css_formatter::context::CssFormatOptions;
use biome_css_formatter::{can_format_css_yet, format_node};
use biome_css_parser::CssParserOptions;
use biome_css_syntax::{CssFileSource, CssLanguage, CssRoot, CssSyntaxNode};
use biome_formatter::{
    FormatError, IndentStyle, IndentWidth, LineEnding, LineWidth, Printed, QuoteStyle,
};
use biome_fs::RomePath;
use biome_parser::AnyParse;
use biome_rowan::{FileSource, NodeCache};
use biome_rowan::{TextRange, TextSize, TokenAtOffset};

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CssFormatterSettings {
    pub line_ending: Option<LineEnding>,
    pub line_width: Option<LineWidth>,
    pub indent_width: Option<IndentWidth>,
    pub indent_style: Option<IndentStyle>,
    pub quote_style: Option<QuoteStyle>,
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CssParserSettings {
    pub allow_wrong_line_comments: bool,
}

impl Language for CssLanguage {
    type FormatterSettings = CssFormatterSettings;
    type LinterSettings = ();
    type OrganizeImportsSettings = ();
    type FormatOptions = CssFormatOptions;
    type ParserSettings = CssParserSettings;
    fn lookup_settings(language: &LanguageListSettings) -> &LanguageSettings<Self> {
        &language.css
    }

    fn resolve_format_options(
        global: &FormatSettings,
        overrides: &OverrideSettings,
        language: &Self::FormatterSettings,
        path: &RomePath,
    ) -> Self::FormatOptions {
        let indent_style = if let Some(indent_style) = language.indent_style {
            indent_style
        } else {
            global.indent_style.unwrap_or_default()
        };
        let line_width = if let Some(line_width) = language.line_width {
            line_width
        } else {
            global.line_width.unwrap_or_default()
        };
        let indent_width = if let Some(indent_width) = language.indent_width {
            indent_width
        } else {
            global.indent_width.unwrap_or_default()
        };

        overrides.override_css_format_options(
            path,
            CssFormatOptions::new(path.as_path().try_into().unwrap_or_default())
                .with_indent_style(indent_style)
                .with_indent_width(indent_width)
                .with_line_width(line_width)
                .with_quote_style(language.quote_style.unwrap_or_default()),
        )
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct CssFileHandler;

impl ExtensionHandler for CssFileHandler {
    fn language(&self) -> super::Language {
        super::Language::Css
    }

    fn mime(&self) -> super::Mime {
        Mime::Css
    }

    fn may_use_tabs(&self) -> bool {
        true
    }

    fn capabilities(&self) -> Capabilities {
        Capabilities {
            parser: ParserCapabilities { parse: Some(parse) },
            debug: DebugCapabilities {
                debug_syntax_tree: Some(debug_syntax_tree),
                debug_control_flow: None,
                debug_formatter_ir: Some(debug_formatter_ir),
            },
            analyzer: AnalyzerCapabilities {
                lint: None,
                code_actions: None,
                rename: None,
                fix_all: None,
                organize_imports: None,
            },
            // TODO(faulty): Once the CSS formatter is sufficiently stable, we
            // will unhide its capabilities from services. But in the meantime,
            // we don't want to give the illusion that CSS is supported. Adding
            // the capabilities at all is necessary to support snapshot tests,
            // though, so it needs to exist here when in development.
            formatter: if can_format_css_yet() {
                FormatterCapabilities {
                    format: Some(format),
                    format_range: Some(format_range),
                    format_on_type: Some(format_on_type),
                }
            } else {
                FormatterCapabilities {
                    format: None,
                    format_range: None,
                    format_on_type: None,
                }
            },
        }
    }
}

fn parse(
    rome_path: &RomePath,
    language_hint: LanguageId,
    text: &str,
    settings: SettingsHandle,
    cache: &mut NodeCache,
) -> AnyParse {
    let parser = &settings.as_ref().languages.css.parser;
    let overrides = &settings.as_ref().override_settings;
    let source_type =
        CssFileSource::try_from(rome_path.as_path()).unwrap_or_else(|_| match language_hint {
            LanguageId::Css => CssFileSource::css(),
            _ => CssFileSource::css(),
        });
    let options: CssParserOptions =
        overrides
            .as_css_parser_options(rome_path)
            .unwrap_or(CssParserOptions {
                allow_wrong_line_comments: parser.allow_wrong_line_comments,
            });
    let parse = biome_css_parser::parse_css_with_cache(text, cache, options);
    let root = parse.syntax();
    let diagnostics = parse.into_diagnostics();
    AnyParse::new(
        // SAFETY: the parser should always return a root node
        root.as_send().unwrap(),
        diagnostics,
        source_type.as_any_file_source(),
    )
}

fn debug_syntax_tree(_rome_path: &RomePath, parse: AnyParse) -> GetSyntaxTreeResult {
    let syntax: CssSyntaxNode = parse.syntax();
    let tree: CssRoot = parse.tree();
    GetSyntaxTreeResult {
        cst: format!("{syntax:#?}"),
        ast: format!("{tree:#?}"),
    }
}

fn debug_formatter_ir(
    rome_path: &RomePath,
    parse: AnyParse,
    settings: SettingsHandle,
) -> Result<String, WorkspaceError> {
    let options = settings.format_options::<CssLanguage>(rome_path);

    let tree = parse.syntax();
    let formatted = format_node(options, &tree)?;

    let root_element = formatted.into_document();
    Ok(root_element.to_string())
}

#[tracing::instrument(level = "debug", skip(parse))]
fn format(
    rome_path: &RomePath,
    parse: AnyParse,
    settings: SettingsHandle,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<CssLanguage>(rome_path);

    tracing::debug!("Format with the following options: \n{}", options);

    let tree = parse.syntax();
    let formatted = format_node(options, &tree)?;

    match formatted.print() {
        Ok(printed) => Ok(printed),
        Err(error) => Err(WorkspaceError::FormatError(error.into())),
    }
}

fn format_range(
    rome_path: &RomePath,
    parse: AnyParse,
    settings: SettingsHandle,
    range: TextRange,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<CssLanguage>(rome_path);

    let tree = parse.syntax();
    let printed = biome_css_formatter::format_range(options, &tree, range)?;
    Ok(printed)
}

fn format_on_type(
    rome_path: &RomePath,
    parse: AnyParse,
    settings: SettingsHandle,
    offset: TextSize,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<CssLanguage>(rome_path);

    let tree = parse.syntax();

    let range = tree.text_range();
    if offset < range.start() || offset > range.end() {
        return Err(WorkspaceError::FormatError(FormatError::RangeError {
            input: TextRange::at(offset, TextSize::from(0)),
            tree: range,
        }));
    }

    let token = match tree.token_at_offset(offset) {
        // File is empty, do nothing
        TokenAtOffset::None => panic!("empty file"),
        TokenAtOffset::Single(token) => token,
        // The cursor should be right after the closing character that was just typed,
        // select the previous token as the correct one
        TokenAtOffset::Between(token, _) => token,
    };

    let root_node = match token.parent() {
        Some(node) => node,
        None => panic!("found a token with no parent"),
    };

    let printed = biome_css_formatter::format_sub_tree(options, &root_node)?;
    Ok(printed)
}
