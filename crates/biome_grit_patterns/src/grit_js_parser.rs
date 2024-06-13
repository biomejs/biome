use crate::{grit_analysis_ext::GritAnalysisExt, grit_tree::GritTree};
use biome_js_parser::{parse, JsParserOptions};
use biome_js_syntax::JsFileSource;
use grit_util::{AnalysisLogs, FileOrigin, Parser, SnippetTree};
use std::path::Path;

pub struct GritJsParser;

impl Parser for GritJsParser {
    type Tree = GritTree;

    fn parse_file(
        &mut self,
        body: &str,
        path: Option<&Path>,
        logs: &mut AnalysisLogs,
        _old_tree: FileOrigin<'_, GritTree>,
    ) -> Option<GritTree> {
        let parse_result = parse(body, JsFileSource::tsx(), JsParserOptions::default());

        for diagnostic in parse_result.diagnostics() {
            logs.push(diagnostic.to_log(path));
        }

        Some(GritTree::new(parse_result.syntax().into()))
    }

    fn parse_snippet(
        &mut self,
        prefix: &'static str,
        source: &str,
        postfix: &'static str,
    ) -> SnippetTree<GritTree> {
        let context = format!("{prefix}{source}{postfix}");

        let len = if cfg!(target_arch = "wasm32") {
            |src: &str| src.chars().count() as u32
        } else {
            |src: &str| src.len() as u32
        };

        let parse_result = parse(&context, JsFileSource::tsx(), JsParserOptions::default());

        SnippetTree {
            tree: GritTree::new(parse_result.syntax().into()),
            source: source.to_owned(),
            prefix,
            postfix,
            snippet_start: (len(prefix) + len(source) - len(source.trim_start())),
            snippet_end: (len(prefix) + len(source.trim_end())),
        }
    }
}
