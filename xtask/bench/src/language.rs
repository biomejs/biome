use crate::test_case::TestCase;
use biome_analyze::{AnalysisFilter, AnalyzerOptions, ControlFlow, Never, RuleCategories};
use biome_css_formatter::context::{CssFormatContext, CssFormatOptions};
use biome_css_parser::CssParserOptions;
use biome_css_syntax::CssSyntaxNode;
use biome_formatter::{FormatResult, Formatted, PrintResult, Printed};
use biome_js_analyze::analyze;
use biome_js_formatter::context::{JsFormatContext, JsFormatOptions};
use biome_js_parser::JsParserOptions;
use biome_js_syntax::{AnyJsRoot, JsFileSource, JsSyntaxNode};
use biome_json_formatter::context::{JsonFormatContext, JsonFormatOptions};
use biome_json_parser::JsonParserOptions;
use biome_json_syntax::JsonSyntaxNode;
use biome_parser::prelude::ParseDiagnostic;
use biome_rowan::NodeCache;
use criterion::black_box;

pub enum Parse<'a> {
    JavaScript(JsFileSource, &'a str),
    Json(&'a str),
    Css(&'a str),
}

impl<'a> Parse<'a> {
    pub fn try_from_case(case: &TestCase) -> Option<Parse> {
        match JsFileSource::try_from(case.path()) {
            Ok(source_type) => Some(Parse::JavaScript(source_type, case.code())),
            Err(_) => match case.extension() {
                "json" => Some(Parse::Json(case.code())),
                "css" => Some(Parse::Css(case.code())),
                _ => None,
            },
        }
    }

    pub fn parse(&self) -> Parsed {
        match self {
            Parse::JavaScript(source_type, code) => Parsed::JavaScript(
                biome_js_parser::parse(code, *source_type, JsParserOptions::default()),
                *source_type,
            ),
            Parse::Json(code) => Parsed::Json(biome_json_parser::parse_json(
                code,
                JsonParserOptions::default(),
            )),
            Parse::Css(code) => Parsed::Css(biome_css_parser::parse_css(
                code,
                CssParserOptions::default().allow_wrong_line_comments(),
            )),
        }
    }

    pub fn parse_with_cache(&self, cache: &mut NodeCache) -> Parsed {
        match self {
            Parse::JavaScript(source_type, code) => Parsed::JavaScript(
                biome_js_parser::parse_js_with_cache(
                    code,
                    *source_type,
                    JsParserOptions::default(),
                    cache,
                ),
                *source_type,
            ),
            Parse::Json(code) => Parsed::Json(biome_json_parser::parse_json_with_cache(
                code,
                cache,
                JsonParserOptions::default(),
            )),
            Parse::Css(code) => Parsed::Css(biome_css_parser::parse_css_with_cache(
                code,
                cache,
                CssParserOptions::default().allow_wrong_line_comments(),
            )),
        }
    }
}

pub enum Parsed {
    JavaScript(biome_js_parser::Parse<AnyJsRoot>, JsFileSource),
    Json(biome_json_parser::JsonParse),
    Css(biome_css_parser::CssParse),
}

impl Parsed {
    pub fn format_node(&self) -> Option<FormatNode> {
        match self {
            Parsed::JavaScript(parse, source_type) => {
                Some(FormatNode::JavaScript(parse.syntax(), *source_type))
            }
            Parsed::Json(parse) => Some(FormatNode::Json(parse.syntax())),
            Parsed::Css(_) => None,
        }
    }

    pub fn analyze(&self) -> Option<Analyze> {
        match self {
            Parsed::JavaScript(parse, _) => Some(Analyze::JavaScript(parse.tree())),
            Parsed::Json(_) => None,
            Parsed::Css(_) => None,
        }
    }

    pub fn into_diagnostics(self) -> Vec<ParseDiagnostic> {
        match self {
            Parsed::JavaScript(parse, _) => parse.into_diagnostics(),
            Parsed::Json(parse) => parse.into_diagnostics(),
            Parsed::Css(parse) => parse.into_diagnostics(),
        }
    }
}

pub enum FormatNode {
    JavaScript(JsSyntaxNode, JsFileSource),
    Json(JsonSyntaxNode),
    Css(CssSyntaxNode),
}

impl FormatNode {
    pub fn format_node(&self) -> FormatResult<FormattedNode> {
        match self {
            Self::JavaScript(root, source_type) => {
                biome_js_formatter::format_node(JsFormatOptions::new(*source_type), root)
                    .map(FormattedNode::JavaScript)
            }
            Self::Json(root) => {
                biome_json_formatter::format_node(JsonFormatOptions::default(), root)
                    .map(FormattedNode::Json)
            }
            Self::Css(root) => biome_css_formatter::format_node(CssFormatOptions::default(), root)
                .map(FormattedNode::Css),
        }
    }
}

pub enum FormattedNode {
    JavaScript(Formatted<JsFormatContext>),
    Json(Formatted<JsonFormatContext>),
    Css(Formatted<CssFormatContext>),
}

impl FormattedNode {
    pub fn print(&self) -> PrintResult<Printed> {
        match self {
            FormattedNode::JavaScript(formatted) => formatted.print(),
            FormattedNode::Json(formatted) => formatted.print(),
            FormattedNode::Css(formatted) => formatted.print(),
        }
    }
}

pub enum Analyze {
    JavaScript(AnyJsRoot),
}

impl Analyze {
    pub fn analyze(&self) {
        match self {
            Analyze::JavaScript(root) => {
                let filter = AnalysisFilter {
                    categories: RuleCategories::SYNTAX | RuleCategories::LINT,
                    ..AnalysisFilter::default()
                };
                let options = AnalyzerOptions::default();
                analyze(
                    root,
                    filter,
                    &options,
                    JsFileSource::default(),
                    None,
                    |event| {
                        black_box(event.diagnostic());
                        black_box(event.actions());
                        ControlFlow::<Never>::Continue(())
                    },
                );
            }
        }
    }
}
