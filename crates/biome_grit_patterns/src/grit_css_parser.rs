use crate::{
    grit_analysis_ext::GritAnalysisExt, grit_target_language::GritTargetParser,
    grit_tree::GritTargetTree,
};
use biome_css_parser::{parse_css, CssParserOptions};
use biome_css_syntax::CssLanguage;
use biome_parser::AnyParse;
use camino::Utf8Path;
use grit_util::{AnalysisLogs, FileOrigin, Parser, SnippetTree};
use std::path::Path;

pub struct GritCssParser;

impl GritTargetParser for GritCssParser {
    fn from_cached_parse_result(
        &self,
        parse: &AnyParse,
        path: Option<&Path>,
        logs: &mut AnalysisLogs,
    ) -> Option<GritTargetTree> {
        for diagnostic in parse.diagnostics() {
            logs.push(diagnostic.to_log(path));
        }

        Some(GritTargetTree::new(parse.syntax::<CssLanguage>().into()))
    }

    fn parse_with_path(&self, source: &str, _path: &Utf8Path) -> AnyParse {
        parse_css(source, CssParserOptions::default()).into()
    }
}

impl Parser for GritCssParser {
    type Tree = GritTargetTree;

    fn parse_file(
        &mut self,
        body: &str,
        path: Option<&Path>,
        logs: &mut AnalysisLogs,
        _old_tree: FileOrigin<'_, GritTargetTree>,
    ) -> Option<GritTargetTree> {
        let parse_result = parse_css(body, CssParserOptions::default().allow_metavariables());

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

        let parse_result = parse_css(&context, CssParserOptions::default().allow_metavariables());

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
