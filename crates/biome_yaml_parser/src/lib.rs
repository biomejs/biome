use biome_parser::tree_sink::{LosslessTreeSink, OffsetLosslessTreeSink};
use biome_parser::{AnyParse, EmbeddedNodeParse, NodeParse, prelude::ParseDiagnostic};
use biome_rowan::{AstNode, NodeCache, SyntaxNodeWithOffset, TextSize};
use biome_yaml_factory::YamlSyntaxFactory;
use biome_yaml_syntax::{YamlLanguage, YamlRoot, YamlSyntaxNode};
use parser::{YamlParser, parse_root};

mod lexer;
mod parser;
mod token_source;

pub(crate) type YamlLosslessTreeSink<'source> =
    LosslessTreeSink<'source, YamlLanguage, YamlSyntaxFactory>;
pub(crate) type YamlOffsetLosslessTreeSink<'source> =
    OffsetLosslessTreeSink<'source, YamlLanguage, YamlSyntaxFactory>;

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

/// Parses YAML `source` with a `base_offset` for embedded content.
pub fn parse_yaml_with_offset(source: &str, base_offset: TextSize) -> YamlOffsetParse {
    parse_yaml_with_offset_and_cache(source, base_offset, &mut NodeCache::default())
}

/// Parses YAML `source` with a `base_offset` and `cache` for embedded content.
pub fn parse_yaml_with_offset_and_cache(
    source: &str,
    base_offset: TextSize,
    cache: &mut NodeCache,
) -> YamlOffsetParse {
    let mut parser = YamlParser::new(source);

    parse_root(&mut parser);

    let (events, diagnostics, trivia) = parser.finish();

    let mut tree_sink = YamlOffsetLosslessTreeSink::with_cache(source, &trivia, cache, base_offset);
    biome_parser::event::process(&mut tree_sink, events, diagnostics);
    let (root, diagnostics) = tree_sink.finish();

    YamlOffsetParse::new(root, diagnostics)
}

/// A utility struct for managing the result of an offset-aware YAML parser job.
#[derive(Clone, Debug)]
pub struct YamlOffsetParse {
    root: SyntaxNodeWithOffset<YamlLanguage>,
    diagnostics: Vec<ParseDiagnostic>,
}

impl YamlOffsetParse {
    pub fn new(
        root: SyntaxNodeWithOffset<YamlLanguage>,
        diagnostics: Vec<ParseDiagnostic>,
    ) -> Self {
        Self { root, diagnostics }
    }

    /// Returns the offset-aware syntax node represented by this parse result.
    pub fn syntax(&self) -> SyntaxNodeWithOffset<YamlLanguage> {
        self.root.clone()
    }

    /// Returns the diagnostics which occurred when parsing.
    pub fn diagnostics(&self) -> &[ParseDiagnostic] {
        &self.diagnostics
    }

    /// Retrieves the diagnostics which occurred when parsing.
    pub fn into_diagnostics(self) -> Vec<ParseDiagnostic> {
        self.diagnostics
    }

    /// Returns `true` if the parser encountered errors.
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.is_error())
    }

    /// Converts this parse into a typed AST node.
    ///
    /// # Panics
    /// Panics if the node represented by this parse result mismatches.
    pub fn tree(&self) -> YamlRoot {
        YamlRoot::unwrap_cast(self.root.inner().clone())
    }

    /// Returns the base offset applied to this parse result.
    pub fn base_offset(&self) -> TextSize {
        self.root.base_offset()
    }

    /// Converts back to the underlying parse result, discarding offset information.
    pub fn into_inner(self) -> YamlParse {
        YamlParse::new(self.root.into_inner(), self.diagnostics)
    }
}

impl From<YamlOffsetParse> for AnyParse {
    fn from(parse: YamlOffsetParse) -> Self {
        let root = parse.syntax();
        let diagnostics = parse.into_diagnostics();
        EmbeddedNodeParse::new(root.as_embedded_send(), diagnostics).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_yaml, parse_yaml_with_offset};
    use biome_rowan::TextSize;

    #[test]
    fn parser_smoke_test() {
        let src = r#"
"#;

        let _yaml = parse_yaml(src);
    }

    #[test]
    fn offset_parse_ranges_start_at_base_offset() {
        let source = "key: value";
        let base_offset = TextSize::from(42);
        let parse = parse_yaml_with_offset(source, base_offset);

        assert!(!parse.has_errors());
        assert_eq!(parse.base_offset(), base_offset);
        assert_eq!(parse.syntax().text_range_with_trivia().start(), base_offset);
        assert_eq!(
            parse.syntax().text_range_with_trivia().end(),
            base_offset + TextSize::from(source.len() as u32)
        );
    }
}
