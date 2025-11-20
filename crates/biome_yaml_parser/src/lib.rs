use biome_parser::{AnyParse, NodeParse, prelude::ParseDiagnostic, tree_sink::LosslessTreeSink};
use biome_rowan::{AstNode, NodeCache};
use biome_yaml_factory::YamlSyntaxFactory;
use biome_yaml_syntax::{YamlLanguage, YamlRoot, YamlSyntaxNode};
use parser::{YamlParser, parse_root};

mod lexer;
mod parser;
mod token_source;

pub(crate) type YamlLosslessTreeSink<'source> =
    LosslessTreeSink<'source, YamlLanguage, YamlSyntaxFactory>;

pub fn parse_yaml(source: &str) -> YamlParse {
    let mut cache = NodeCache::default();
    parse_yaml_with_cache(source, &mut cache)
}

pub fn parse_yaml_with_cache(source: &str, cache: &mut NodeCache) -> YamlParse {
    let mut parser = YamlParser::new(source);

    parse_root(&mut parser);

    let (events, diagnostics, trivia) = parser.finish();

    let mut tree_sink = YamlLosslessTreeSink::with_cache(source, &trivia, cache);
    biome_parser::event::process(&mut tree_sink, events, diagnostics);
    let (green, diagnostics) = tree_sink.finish();

    YamlParse::new(green, diagnostics)
}

/// A utility struct for managing the result of a parser job
#[derive(Debug)]
pub struct YamlParse {
    root: YamlSyntaxNode,
    diagnostics: Vec<ParseDiagnostic>,
}

impl YamlParse {
    pub fn new(root: YamlSyntaxNode, diagnostics: Vec<ParseDiagnostic>) -> YamlParse {
        YamlParse { root, diagnostics }
    }

    /// The syntax node represented by this Parse result
    ///
    /// ```
    /// # use biome_yaml_parser::parse_yaml;
    /// # use biome_yaml_syntax::YamlSyntaxKind;
    /// # use biome_rowan::{AstNode, AstNodeList, SyntaxError};
    ///
    /// # fn main() -> Result<(), SyntaxError> {
    /// use biome_yaml_syntax::YamlSyntaxKind;
    /// let parse = parse_yaml(r#""#);
    ///
    /// let root_value = parse.tree().documents();
    ///
    /// assert_eq!(root_value.syntax().kind(), YamlSyntaxKind::YAML_DOCUMENT_LIST);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn syntax(&self) -> YamlSyntaxNode {
        self.root.clone()
    }

    /// Get the diagnostics which occurred when parsing
    pub fn diagnostics(&self) -> &[ParseDiagnostic] {
        &self.diagnostics
    }

    /// Get the diagnostics which occurred when parsing
    pub fn into_diagnostics(self) -> Vec<ParseDiagnostic> {
        self.diagnostics
    }

    /// Returns [true] if the parser encountered some errors during the parsing.
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.is_error())
    }

    /// Convert this parse result into a typed AST node.
    ///
    /// # Panics
    /// Panics if the node represented by this parse result mismatches.
    pub fn tree(&self) -> YamlRoot {
        YamlRoot::unwrap_cast(self.syntax())
    }
}

impl From<YamlParse> for AnyParse {
    fn from(parse: YamlParse) -> Self {
        let root = parse.syntax();
        let diagnostics = parse.into_diagnostics();
        NodeParse::new(
            // SAFETY: the parser should always return a root node
            root.as_send().unwrap(),
            diagnostics,
        )
        .into()
    }
}

#[cfg(test)]
mod tests {
    use crate::parse_yaml;

    #[test]
    fn parser_smoke_test() {
        let src = r#"
"#;

        let _yaml = parse_yaml(src);
    }
}
