//! Utilities for high-level parsing of js code.

use crate::*;
pub use biome_js_syntax::{
    AnyJsRoot, JsFileSource, JsLanguage, JsModule, JsScript, JsSyntaxNode, ModuleKind,
};
use biome_parser::token_source::Trivia;
use biome_parser::{AnyParse, EmbeddedNodeParse, NodeParse, event::Event};
use biome_rowan::{AstNode, NodeCache, SyntaxNodeWithOffset};
use std::marker::PhantomData;

/// A utility struct for managing the result of a parser job
#[derive(Debug)]
pub struct Parse<T> {
    root: JsSyntaxNode,
    errors: Vec<ParseDiagnostic>,
    _ty: PhantomData<T>,
}

impl<T> Parse<T> {
    pub fn new_module(root: JsSyntaxNode, errors: Vec<ParseDiagnostic>) -> Self {
        Self::new(root, errors)
    }

    pub fn new_script(root: JsSyntaxNode, errors: Vec<ParseDiagnostic>) -> Self {
        Self::new(root, errors)
    }

    pub fn new(root: JsSyntaxNode, errors: Vec<ParseDiagnostic>) -> Self {
        Self {
            root,
            errors,
            _ty: PhantomData,
        }
    }

    pub fn cast<N: AstNode<Language = JsLanguage>>(self) -> Option<Parse<N>> {
        if N::can_cast(self.syntax().kind()) {
            Some(Parse::new(self.root, self.errors))
        } else {
            None
        }
    }

    /// The syntax node represented by this Parse result
    ///
    /// ```
    /// use biome_js_parser::{JsParserOptions, parse_script};
    /// use biome_js_syntax::{JsIfStatement, JsSyntaxKind};
    /// use biome_rowan::{AstNode, AstNodeList};
    ///
    /// let parse = parse_script(
    ///     "
    ///     if (a > 5) {
    ///         /* something */
    ///     }
    /// ",
    ///  JsParserOptions::default()
    /// );
    ///
    /// // The first stmt in the root syntax node (Script) is the if statement.
    /// let if_stmt = parse.tree().statements().first().unwrap();
    ///
    /// assert_eq!(if_stmt.syntax().kind(), JsSyntaxKind::JS_IF_STATEMENT);
    /// ```
    pub fn syntax(&self) -> JsSyntaxNode {
        self.root.clone()
    }

    /// Get the diagnostics which occurred when parsing
    pub fn diagnostics(&self) -> &[ParseDiagnostic] {
        self.errors.as_slice()
    }

    /// Get the diagnostics which occurred when parsing
    pub fn into_diagnostics(self) -> Vec<ParseDiagnostic> {
        self.errors
    }

    /// Returns [true] if the parser encountered some errors during the parsing.
    pub fn has_errors(&self) -> bool {
        self.errors.iter().any(|diagnostic| diagnostic.is_error())
    }
}

impl<T: AstNode<Language = JsLanguage>> Parse<T> {
    /// Convert this parse result into a typed AST node.
    ///
    /// # Panics
    /// Panics if the node represented by this parse result mismatches.
    pub fn tree(&self) -> T {
        self.try_tree().unwrap_or_else(|| {
            panic!(
                "Expected tree to be a {} but root is:\n{:#?}",
                std::any::type_name::<T>(),
                self.syntax()
            )
        })
    }

    /// Try to convert this parse's untyped syntax node into an AST node.
    pub fn try_tree(&self) -> Option<T> {
        T::cast(self.syntax())
    }

    /// Convert this parse into a result
    pub fn ok(self) -> Result<T, Vec<ParseDiagnostic>> {
        if !self.errors.iter().any(|d| d.is_error()) {
            Ok(self.tree())
        } else {
            Err(self.errors)
        }
    }
}

impl<T> From<Parse<T>> for AnyParse {
    fn from(parse: Parse<T>) -> Self {
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

impl From<JsOffsetParse> for AnyParse {
    fn from(parse: JsOffsetParse) -> Self {
        let root = parse.syntax();
        let diagnostics = parse.into_diagnostics();
        EmbeddedNodeParse::new(
            // SAFETY: the parser should always return a root node
            root.as_embedded_send(),
            diagnostics,
        )
        .into()
    }
}

fn parse_common(
    text: &str,
    source_type: JsFileSource,
    options: JsParserOptions,
) -> (Vec<Event<JsSyntaxKind>>, Vec<ParseDiagnostic>, Vec<Trivia>) {
    let mut parser = JsParser::new(text, source_type, options);
    syntax::program::parse(&mut parser);

    let (events, trivia, errors) = parser.finish();

    (events, errors, trivia)
}

/// Parse text into a [`Parse`](Parse) which can then be turned into an untyped root [`JsSyntaxNode`](JsSyntaxNode).
/// Or turned into a typed [`JsScript`](JsScript) with [`tree`](Parse::tree).
///
/// ```
/// use biome_js_parser::{JsParserOptions, parse_script};
/// use biome_js_syntax::{JsSyntaxToken, JsFileSource, JsSyntaxList, JsComputedMemberExpression};
/// use biome_rowan::{AstNode, Direction};
///
/// let parse = parse_script("foo.bar[2]", JsParserOptions::default());
/// // Parse returns a JS Root which contains two lists, the directives and the statements, let's get the statements
/// let stmt = parse.syntax().children().nth(1).unwrap();
/// // The untyped syntax node of `foo.bar[2]`, the root node is `Script`.
/// let untyped_expr_node = stmt.first_child().unwrap();
///
/// // SyntaxNodes can be turned into a nice string representation.
/// println!("{:#?}", untyped_expr_node);
///
/// // You can then cast syntax nodes into a typed AST node.
/// let typed_ast_node = JsComputedMemberExpression::cast(untyped_expr_node.first_child().unwrap()).unwrap();
///
/// // Everything on every ast node is optional because of error recovery.
/// let prop = dbg!(typed_ast_node.member()).unwrap();
///
/// // You can then go back to an untyped SyntaxNode and get its range, text, parents, children, etc.
/// assert_eq!(prop.syntax().text_with_trivia(), "2");
///
/// // Util has a function for yielding all tokens of a node.
/// let tokens = untyped_expr_node.descendants_tokens(Direction::Next).map(|token| token.text_trimmed().to_string()).collect::<Vec<_>>();
///
/// assert_eq!(&tokens, &vec!["foo", ".", "bar", "[", "2", "]"]);
/// ```
pub fn parse_script(text: &str, options: JsParserOptions) -> Parse<JsScript> {
    parse(
        text,
        JsFileSource::js_module().with_module_kind(ModuleKind::Script),
        options,
    )
    .cast::<JsScript>()
    .unwrap()
}

/// Same as [parse_script] but configures the parser to parse an ECMAScript module instead of a script
///
/// ### Examples
///
/// Check the diagnostics emitted by the code
/// ```
/// use biome_js_parser::{JsParserOptions, parse_module};
/// let source = r#"
/// import { someModule } from "./someModule.js";
///
/// someModule();
/// "#;
///
/// let parse = parse_module(source, JsParserOptions::default());
///
/// // Retrieve the diagnostics emitted
/// assert_eq!(parse.diagnostics().len(), 0);
/// ```
///
/// Retrieve the emitted AST and check its kind:
/// ```
/// use biome_js_parser::{JsParserOptions, parse_module};
/// use biome_js_syntax::JsSyntaxKind;
/// use biome_rowan::AstNode;
/// let source = r#"
/// import { someModule } from "./someModule.js";
///
/// someModule();
/// "#;
/// let parse = parse_module(source, JsParserOptions::default());
///
/// let tree = parse.tree();
///
/// assert_eq!(tree.syntax().kind(), JsSyntaxKind::JS_MODULE);
/// ```
///
pub fn parse_module(text: &str, options: JsParserOptions) -> Parse<JsModule> {
    parse(text, JsFileSource::js_module(), options)
        .cast::<JsModule>()
        .unwrap()
}

/// Parses the provided string as a EcmaScript program using the provided syntax features.
///
/// ### Examples
///
/// ```
/// use biome_js_parser::{JsParserOptions, parse};
/// use biome_js_syntax::{LanguageVariant, LanguageVersion, ModuleKind, JsFileSource};
/// // parse source text as TypeScript
/// let mut module = JsFileSource::ts();
/// let mut parsed = parse("type F = {}", module, JsParserOptions::default());
/// assert_eq!(parsed.diagnostics().len(), 0);
/// // parse source text as JSX
/// module = JsFileSource::jsx();
/// parsed = parse("<Component></Component>", module, JsParserOptions::default());
/// assert_eq!(parsed.diagnostics().len(), 0);
/// // parse source text with granular control
/// module = JsFileSource::default()
///   .with_version(LanguageVersion::ESNext)
///   .with_module_kind(ModuleKind::Module)
///   .with_variant(LanguageVariant::Jsx);
/// parsed = parse("foo[bar]", module, JsParserOptions::default());
/// assert_eq!(parsed.diagnostics().len(), 0);
/// ```
pub fn parse(text: &str, source_type: JsFileSource, options: JsParserOptions) -> Parse<AnyJsRoot> {
    let mut cache = NodeCache::default();
    parse_js_with_cache(text, source_type, options, &mut cache)
}

/// Parses the provided string as a EcmaScript program using the provided syntax features and node cache.
///
/// ### Examples
///
/// ```
/// use biome_js_parser::{JsParserOptions, parse_js_with_cache};
/// use biome_js_syntax::JsFileSource;
/// use biome_rowan::NodeCache;
///
/// let source_type = JsFileSource::js_module();
/// let mut cache = NodeCache::default();
/// let mut source = "function f() { return 2 }";
///
/// let parsed = parse_js_with_cache(source, source_type, JsParserOptions::default(), &mut cache);
/// assert_eq!(parsed.diagnostics().len(), 0);
///
/// source = "function bar() { return 3 }";
/// let parsed  = parse_js_with_cache(source, source_type, JsParserOptions::default(), &mut cache);
/// assert_eq!(parsed.diagnostics().len(), 0);
/// ```
pub fn parse_js_with_cache(
    text: &str,
    source_type: JsFileSource,
    options: JsParserOptions,
    cache: &mut NodeCache,
) -> Parse<AnyJsRoot> {
    let (events, errors, tokens) = parse_common(text, source_type, options);
    let mut tree_sink = JsLosslessTreeSink::with_cache(text, &tokens, cache);
    biome_parser::event::process(&mut tree_sink, events, errors);
    let (green, parse_errors) = tree_sink.finish();
    Parse::new(green, parse_errors)
}

/// A utility struct for managing the result of an offset-aware parser job
#[derive(Clone, Debug)]
pub struct JsOffsetParse {
    root: SyntaxNodeWithOffset<JsLanguage>,
    diagnostics: Vec<ParseDiagnostic>,
}

impl JsOffsetParse {
    pub fn new(root: SyntaxNodeWithOffset<JsLanguage>, diagnostics: Vec<ParseDiagnostic>) -> Self {
        Self { root, diagnostics }
    }

    /// The offset-aware syntax node represented by this Parse result
    pub fn syntax(&self) -> SyntaxNodeWithOffset<JsLanguage> {
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
    pub fn tree(&self) -> AnyJsRoot {
        AnyJsRoot::unwrap_cast(self.root.inner().clone())
    }

    /// Get the base offset applied to this parse result
    pub fn base_offset(&self) -> biome_rowan::TextSize {
        self.root.base_offset()
    }

    /// Convert back to the underlying parse result, discarding offset information
    pub fn into_inner(self) -> Parse<JsLanguage> {
        Parse::new(self.root.into_inner(), self.diagnostics)
    }
}

/// Parses JavaScript/TypeScript code with an offset for embedded content.
///
/// This function is designed for parsing embedded JavaScript content (e.g., in HTML `<script>` tags)
/// where the source positions need to be adjusted relative to the parent document.
///
/// # Arguments
/// * `text` - The JavaScript/TypeScript source code to parse
/// * `base_offset` - The offset to apply to all source positions
/// * `source_type` - The file source configuration (JS, TS, JSX, etc.)
/// * `options` - Parser options
///
/// # Examples
/// ```
/// use biome_js_parser::{JsParserOptions, parse_js_with_offset};
/// use biome_js_syntax::JsFileSource;
/// use biome_rowan::TextSize;
///
/// // Parsing embedded JavaScript starting at position 100 in an HTML document
/// let js_code = "console.log('Hello, world!');";
/// let offset = TextSize::from(100);
/// let parse = parse_js_with_offset(
///     js_code,
///     offset,
///     JsFileSource::js_module(),
///     JsParserOptions::default()
/// );
///
/// // All text ranges in the resulting AST will be offset by 100
/// assert_eq!(parse.base_offset(), offset);
/// ```
pub fn parse_js_with_offset(
    text: &str,
    base_offset: biome_rowan::TextSize,
    source_type: JsFileSource,
    options: JsParserOptions,
) -> JsOffsetParse {
    let mut cache = NodeCache::default();
    parse_js_with_offset_and_cache(text, base_offset, source_type, options, &mut cache)
}

/// Parses JavaScript/TypeScript code with an offset and cache for embedded content.
///
/// This is the cache-enabled version of [`parse_js_with_offset`] for improved performance
/// when parsing multiple embedded code blocks.
pub fn parse_js_with_offset_and_cache(
    text: &str,
    base_offset: biome_rowan::TextSize,
    source_type: JsFileSource,
    options: JsParserOptions,
    cache: &mut NodeCache,
) -> JsOffsetParse {
    let (events, errors, tokens) = parse_common(text, source_type, options);
    let mut tree_sink =
        crate::JsOffsetLosslessTreeSink::with_cache(text, &tokens, cache, base_offset);
    biome_parser::event::process(&mut tree_sink, events, errors);
    let (offset_node, parse_errors) = tree_sink.finish();
    JsOffsetParse::new(offset_node, parse_errors)
}

/// Parse JavaScript script text with an offset for embedded content.
///
/// This is the offset-aware version of [`parse_script`] for embedded script tags.
pub fn parse_script_with_offset(
    text: &str,
    base_offset: biome_rowan::TextSize,
    options: JsParserOptions,
) -> JsOffsetParse {
    let parse = parse_js_with_offset(
        text,
        base_offset,
        JsFileSource::js_module().with_module_kind(ModuleKind::Script),
        options,
    );
    JsOffsetParse::new(parse.root, parse.diagnostics)
}

/// Parse JavaScript module text with an offset for embedded content.
///
/// This is the offset-aware version of [`parse_module`] for embedded script tags.
pub fn parse_module_with_offset(
    text: &str,
    base_offset: biome_rowan::TextSize,
    options: JsParserOptions,
) -> JsOffsetParse {
    let parse = parse_js_with_offset(text, base_offset, JsFileSource::js_module(), options);
    JsOffsetParse::new(parse.root, parse.diagnostics)
}

#[cfg(test)]
mod tests {
    use crate::{JsFileSource, JsParserOptions, parse_js_with_cache, parse_js_with_offset};
    use biome_rowan::TextSize;

    #[test]
    fn test_offset_parsing_basic() {
        let js_code = "console.log('hello');";
        let base_offset = TextSize::from(100);

        let parse = parse_js_with_offset(
            js_code,
            base_offset,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );

        // Verify no parsing errors
        assert!(!parse.has_errors(), "Parse should not have errors");

        // Verify the base offset is correctly set
        assert_eq!(parse.base_offset(), base_offset);

        // Verify the syntax tree text ranges are offset
        let syntax = parse.syntax();
        let root_range = syntax.text_range_with_trivia();

        // The root should start at the base offset
        assert_eq!(root_range.start(), base_offset);

        // The end should be base_offset + length of the text
        let expected_end = base_offset + TextSize::from(js_code.len() as u32);
        assert_eq!(root_range.end(), expected_end);
    }

    #[test]
    fn test_offset_parsing_vs_regular_parsing() {
        let js_code = "function test() { return 42; }";
        let base_offset = TextSize::from(50);

        // Parse with offset
        let offset_parse = parse_js_with_offset(
            js_code,
            base_offset,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );

        // Parse normally
        let normal_parse = parse_js_with_cache(
            js_code,
            JsFileSource::js_module(),
            JsParserOptions::default(),
            &mut biome_rowan::NodeCache::default(),
        );

        // Both should have same number of errors (hopefully none)
        assert_eq!(offset_parse.has_errors(), normal_parse.has_errors());

        // The offset parse should have all ranges shifted by base_offset
        let offset_range = offset_parse.syntax().text_range_with_trivia();
        let normal_range = normal_parse.syntax().text_range_with_trivia();

        assert_eq!(offset_range.start(), normal_range.start() + base_offset);
        assert_eq!(offset_range.end(), normal_range.end() + base_offset);

        // The text content should be identical
        assert_eq!(
            offset_parse.syntax().inner().text_with_trivia().to_string(),
            normal_parse.syntax().text_with_trivia().to_string()
        );
    }
}
