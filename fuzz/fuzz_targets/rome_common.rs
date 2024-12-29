//! Common functionality between different fuzzers. Look here if you need to inspect implementation
//! details for the fuzzer harnesses!

#![expect(dead_code)]

use biome_analyze::{AnalysisFilter, AnalyzerOptions, ControlFlow, RuleFilter};
use biome_diagnostics::Diagnostic;
use biome_formatter::format_node;
use biome_js_analyze::analyze;
use biome_js_formatter::context::JsFormatOptions;
use biome_js_formatter::JsFormatLanguage;
use biome_js_parser::parse;
use biome_js_syntax::JsFileSource;
use biome_json_formatter::context::JsonFormatOptions;
use biome_json_formatter::JsonFormatLanguage;
use biome_json_parser::parse_json;
use biome_service::Rules;
use libfuzzer_sys::Corpus;
use similar::TextDiff;
use std::fmt::{Display, Formatter};

pub fn fuzz_js_parser_with_source_type(data: &[u8], source: JsFileSource) -> Corpus {
    let Ok(code1) = std::str::from_utf8(data) else {
        return Corpus::Reject;
    };

    let parse1 = parse(code1, source);
    if !parse1.has_errors() {
        let syntax1 = parse1.syntax();
        let code2 = syntax1.to_string();
        assert_eq!(code1, code2, "unparse output differed");
    }

    Corpus::Keep
}

static mut ANALYSIS_RULES: Option<Rules> = None;
static mut ANALYSIS_RULE_FILTERS: Option<Vec<RuleFilter>> = None;
static mut ANALYSIS_OPTIONS: Option<AnalyzerOptions> = None;

struct DiagnosticDescriptionExtractor<'a, D> {
    diagnostic: &'a D,
}

impl<'a, D> DiagnosticDescriptionExtractor<'a, D> {
    pub fn new(diagnostic: &'a D) -> Self {
        Self { diagnostic }
    }
}

impl<'a, D> Display for DiagnosticDescriptionExtractor<'a, D>
where
    D: Diagnostic,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.diagnostic.description(f)
    }
}

pub fn fuzz_js_formatter_with_source_type(data: &[u8], source: JsFileSource) -> Corpus {
    let Ok(code1) = std::str::from_utf8(data) else {
        return Corpus::Reject;
    };

    // TODO: replace with OnceLock when upgrading to 1.70
    let rule_filters = if let Some(rules) = unsafe { ANALYSIS_RULE_FILTERS.as_ref() } {
        rules
    } else {
        let rules = unsafe {
            ANALYSIS_RULES.get_or_insert_with(|| Rules {
                all: Some(true),
                ..Default::default()
            })
        };
        let rules = rules.as_enabled_rules().into_iter().collect::<Vec<_>>();
        unsafe {
            ANALYSIS_RULE_FILTERS = Some(rules);
            ANALYSIS_RULE_FILTERS.as_ref().unwrap_unchecked()
        }
    };
    let options = unsafe { ANALYSIS_OPTIONS.get_or_insert_with(AnalyzerOptions::default) };

    let parse1 = parse(code1, source);
    if !parse1.has_errors() {
        let language = JsFormatLanguage::new(JsFormatOptions::new(source));
        let tree1 = parse1.tree();
        let mut linter_errors = Vec::new();
        let _ = analyze(
            &tree1,
            AnalysisFilter::from_enabled_rules(Some(rule_filters)),
            options,
            source,
            |e| -> ControlFlow<()> {
                if let Some(diagnostic) = e.diagnostic() {
                    linter_errors
                        .push(DiagnosticDescriptionExtractor::new(&diagnostic).to_string());
                }

                ControlFlow::Continue(())
            },
        );
        let syntax1 = parse1.syntax();
        if let Ok(formatted1) = format_node(&syntax1, language.clone()) {
            if let Ok(printed1) = formatted1.print() {
                let code2 = printed1.as_code();
                let parse2 = parse(code2, source);
                assert!(
                    !parse2.has_errors(),
                    "formatter introduced errors:\n{}",
                    TextDiff::from_lines(code1, code2)
                        .unified_diff()
                        .header("original code", "formatted")
                );
                let tree2 = parse2.tree();
                let (maybe_diagnostic, _) = analyze(
                    &tree2,
                    AnalysisFilter::from_enabled_rules(Some(rule_filters)),
                    options,
                    source,
                    |e| {
                        if let Some(diagnostic) = e.diagnostic() {
                            let new_error =
                                DiagnosticDescriptionExtractor::new(&diagnostic).to_string();
                            if let Some(idx) = linter_errors.iter().position(|e| *e == new_error) {
                                linter_errors.remove(idx);
                            } else {
                                return ControlFlow::Break(new_error);
                            }
                        }

                        ControlFlow::Continue(())
                    },
                );
                if let Some(diagnostic) = maybe_diagnostic {
                    panic!(
                        "formatter introduced linter failure: {} (expected one of: {})\n{}",
                        diagnostic,
                        linter_errors.join(", "),
                        TextDiff::from_lines(code1, code2)
                            .unified_diff()
                            .header("original code", "formatted")
                    );
                }
                let syntax2 = parse2.syntax();
                let formatted2 = format_node(&syntax2, language)
                    .expect("formatted code could not be reformatted");
                let printed2 = formatted2
                    .print()
                    .expect("reformatted code could not be printed");
                let code3 = printed2.as_code();
                assert_eq!(
                    code2,
                    code3,
                    "format results differ:\n{}",
                    TextDiff::from_lines(code2, code3)
                        .unified_diff()
                        .header("formatted", "reformatted")
                )
            }
        }
    }

    Corpus::Keep
}

pub fn fuzz_json_parser(data: &[u8]) -> Corpus {
    let Ok(code1) = std::str::from_utf8(data) else {
        return Corpus::Reject;
    };

    let parse1 = parse_json(code1);
    if !parse1.has_errors() {
        let syntax1 = parse1.syntax();
        let code2 = syntax1.to_string();
        assert_eq!(code1, code2, "unparse output differed");
    }

    Corpus::Keep
}

pub fn fuzz_json_formatter(data: &[u8]) -> Corpus {
    let Ok(code1) = std::str::from_utf8(data) else {
        return Corpus::Reject;
    };

    let parse1 = parse_json(code1);
    if !parse1.has_errors() {
        let language = JsonFormatLanguage::new(JsonFormatOptions::default());
        let syntax1 = parse1.syntax();
        if let Ok(formatted1) = format_node(&syntax1, language.clone()) {
            if let Ok(printed1) = formatted1.print() {
                let code2 = printed1.as_code();
                let parse2 = parse_json(code2);
                assert!(
                    !parse2.has_errors(),
                    "formatter introduced errors:\n{}",
                    TextDiff::from_lines(code1, code2)
                        .unified_diff()
                        .header("original code", "formatted")
                );
                let syntax2 = parse2.syntax();
                let formatted2 = format_node(&syntax2, language)
                    .expect("formatted code could not be reformatted");
                let printed2 = formatted2
                    .print()
                    .expect("reformatted code could not be printed");
                assert_eq!(
                    code2,
                    printed2.as_code(),
                    "format results differ:\n{}",
                    TextDiff::from_lines(code1, code2)
                        .unified_diff()
                        .header("formatted", "reformatted")
                )
            }
        }
    }

    Corpus::Keep
}
