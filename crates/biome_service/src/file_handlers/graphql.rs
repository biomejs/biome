use super::{
    DocumentFileSource, ExtensionHandler, LintParams, LintResults, ParseResult, SearchCapabilities,
};
use crate::file_handlers::DebugCapabilities;
use crate::file_handlers::{
    AnalyzerCapabilities, Capabilities, FormatterCapabilities, ParserCapabilities,
};
use crate::settings::{
    FormatSettings, LanguageListSettings, LanguageSettings, LinterSettings, OverrideSettings,
    ServiceLanguage, Settings,
};
use crate::workspace::GetSyntaxTreeResult;
use biome_analyze::{AnalyzerConfiguration, AnalyzerOptions};
use biome_formatter::SimpleFormatOptions;
use biome_fs::BiomePath;
use biome_graphql_parser::parse_graphql_with_cache;
use biome_graphql_syntax::{GraphqlLanguage, GraphqlRoot, GraphqlSyntaxNode};
use biome_parser::AnyParse;
use biome_rowan::NodeCache;

impl ServiceLanguage for GraphqlLanguage {
    type FormatterSettings = ();
    type LinterSettings = ();
    type OrganizeImportsSettings = ();
    type FormatOptions = SimpleFormatOptions;
    type ParserSettings = ();
    type EnvironmentSettings = ();

    fn lookup_settings(language: &LanguageListSettings) -> &LanguageSettings<Self> {
        &language.graphql
    }

    fn resolve_format_options(
        _global: Option<&FormatSettings>,
        _overrides: Option<&OverrideSettings>,
        _language: Option<&()>,
        _path: &BiomePath,
        _document_file_source: &DocumentFileSource,
    ) -> Self::FormatOptions {
        SimpleFormatOptions::default()
    }

    fn resolve_analyzer_options(
        _global: Option<&Settings>,
        _linter: Option<&LinterSettings>,
        _overrides: Option<&OverrideSettings>,
        _language: Option<&Self::LinterSettings>,
        path: &BiomePath,
        _file_source: &DocumentFileSource,
    ) -> AnalyzerOptions {
        AnalyzerOptions {
            configuration: AnalyzerConfiguration::default(),
            file_path: path.to_path_buf(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct GraphqlFileHandler;

impl ExtensionHandler for GraphqlFileHandler {
    fn capabilities(&self) -> Capabilities {
        Capabilities {
            parser: ParserCapabilities { parse: Some(parse) },
            debug: DebugCapabilities {
                debug_syntax_tree: Some(debug_syntax_tree),
                debug_control_flow: None,
                debug_formatter_ir: None,
            },
            analyzer: AnalyzerCapabilities {
                lint: Some(lint),
                code_actions: None,
                rename: None,
                fix_all: None,
                organize_imports: None,
            },
            formatter: FormatterCapabilities {
                format: None,
                format_range: None,
                format_on_type: None,
            },
            search: SearchCapabilities { search_file: None },
        }
    }
}

fn parse(
    _biome_path: &BiomePath,
    file_source: DocumentFileSource,
    text: &str,
    _settings: Option<&Settings>,
    cache: &mut NodeCache,
) -> ParseResult {
    let parse = parse_graphql_with_cache(text, cache);

    ParseResult {
        any_parse: parse.into(),
        language: Some(file_source),
    }
}

fn debug_syntax_tree(_rome_path: &BiomePath, parse: AnyParse) -> GetSyntaxTreeResult {
    let syntax: GraphqlSyntaxNode = parse.syntax();
    let tree: GraphqlRoot = parse.tree();
    GetSyntaxTreeResult {
        cst: format!("{syntax:#?}"),
        ast: format!("{tree:#?}"),
    }
}

fn lint(params: LintParams) -> LintResults {
    let diagnostics = params.parse.into_diagnostics();
    LintResults {
        errors: diagnostics.len(),
        diagnostics,
        skipped_diagnostics: 0,
    }
}
