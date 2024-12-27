use crate::{
    grit_analysis_ext::GritAnalysisExt, grit_target_language::GritTargetParser,
    grit_tree::GritTargetTree,
};
use biome_js_parser::{parse, JsParserOptions};
use biome_js_syntax::{JsFileSource, JsLanguage};
use biome_parser::AnyParse;
use camino::Utf8Path;
use grit_util::{AnalysisLogs, FileOrigin, Parser, SnippetTree};
use std::path::Path;

pub struct GritJsParser;

impl GritTargetParser for GritJsParser {
    fn from_cached_parse_result(
        &self,
        parse: &AnyParse,
        path: Option<&Path>,
        logs: &mut AnalysisLogs,
    ) -> Option<GritTargetTree> {
        for diagnostic in parse.diagnostics() {
            logs.push(diagnostic.to_log(path));
        }

        Some(GritTargetTree::new(parse.syntax::<JsLanguage>().into()))
    }

    fn parse_with_path(&self, source: &str, path: &Utf8Path) -> AnyParse {
        let source_type = match path.extension() {
            Some("d.ts") => JsFileSource::d_ts(),
            Some("js") => JsFileSource::js_module(),
            Some("jsx") => JsFileSource::jsx(),
            Some("tsx") => JsFileSource::tsx(),
            _ => JsFileSource::ts(),
        };

        parse(source, source_type, JsParserOptions::default()).into()
    }
}

impl Parser for GritJsParser {
    type Tree = GritTargetTree;

    fn parse_file(
        &mut self,
        body: &str,
        path: Option<&Path>,
        logs: &mut AnalysisLogs,
        _old_tree: FileOrigin<'_, GritTargetTree>,
    ) -> Option<GritTargetTree> {
        let parse_result = parse(
            body,
            JsFileSource::tsx(),
            JsParserOptions::default().with_metavariables(),
        );

        for diagnostic in parse_result.diagnostics() {
            logs.push(diagnostic.to_log(path));
        }

        Some(GritTargetTree::new(parse_result.syntax().into()))
    }

    fn parse_snippet(
        &mut self,
        prefix: &'static str,
        source: &str,
        postfix: &'static str,
    ) -> SnippetTree<GritTargetTree> {
        let context = format!("{prefix}{source}{postfix}");

        let len = if cfg!(target_arch = "wasm32") {
            |src: &str| src.chars().count() as u32
        } else {
            |src: &str| src.len() as u32
        };

        let parse_result = parse(
            &context,
            JsFileSource::tsx(),
            JsParserOptions::default().with_metavariables(),
        );

        SnippetTree {
            tree: GritTargetTree::new(parse_result.syntax().into()),
            source: source.to_owned(),
            prefix,
            postfix,
            snippet_start: (len(prefix) + len(source) - len(source.trim_start())),
            snippet_end: (len(prefix) + len(source.trim_end())),
        }
    }
}
