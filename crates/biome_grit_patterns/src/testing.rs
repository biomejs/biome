use crate::grit_context::GritTargetFile;
use crate::grit_query::GritQuery;
use crate::grit_target_language::GritTargetLanguage;
use crate::{CompilePatternOptions, compile_pattern_with_options};
use biome_grit_parser::parse_grit;
use biome_js_parser::{JsParserOptions, parse};
use biome_languages::JsFileSource;

pub fn compile_js_query(source: &str) -> GritQuery {
    let parsed = parse_grit(source);
    assert!(
        parsed.diagnostics().is_empty(),
        "parse error: {:?}",
        parsed.diagnostics()
    );
    GritQuery::from_node(
        parsed.tree(),
        None,
        GritTargetLanguage::JsTargetLanguage(crate::JsTargetLanguage),
        Vec::new(),
    )
    .expect("compile failed")
}

pub fn compile_js_selector(source: &str) -> GritQuery {
    compile_pattern_with_options(source, CompilePatternOptions::default().as_node_selector())
        .expect("selector compile failed")
}

pub fn make_js_file(code: &str) -> GritTargetFile {
    let parsed = parse(code, JsFileSource::js_module(), JsParserOptions::default());
    GritTargetFile::new("test.js", parsed.into())
}
