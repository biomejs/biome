use biome_rowan::TextRange;

/// Source-ordered Markdown constructs needed to process rule documentation.
///
/// The representation omits prose and inline formatting. It retains headings,
/// fenced code blocks, and raw HTML because rule validation and website
/// generation assign those constructs additional behavior. The first summary
/// paragraph is lowered separately by [`crate::parse_rule_summary`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuleDocument {
    items: Vec<RuleDocumentItem>,
}

impl RuleDocument {
    pub(crate) fn new(items: Vec<RuleDocumentItem>) -> Self {
        Self { items }
    }

    /// Returns the lowered constructs in the order in which they occur in the
    /// Markdown source.
    pub fn items(&self) -> &[RuleDocumentItem] {
        &self.items
    }

    /// Transfers ownership of the lowered constructs and iterates over them in
    /// source order.
    pub fn into_items(self) -> impl Iterator<Item = RuleDocumentItem> {
        self.items.into_iter()
    }
}

/// A Markdown construct with special meaning in rule documentation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RuleDocumentItem {
    /// A heading used to delimit independently analyzed documentation sections.
    Heading(RuleDocHeading),
    /// A fenced code example or configuration block.
    FencedCodeBlock(RuleDocCodeBlock),
    /// Raw block-level or inline HTML.
    RawHtml(RuleDocHtml),
}

/// A Markdown heading that can delimit a group of rule documentation tests.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuleDocHeading {
    /// The heading's range in the original Markdown source, including attached
    /// trivia.
    pub range: TextRange,
    /// The heading level from 1 through 6.
    pub level: usize,
}

/// A fenced code block lowered for documentation processing.
///
/// The info string has Markdown escapes and character references resolved. The
/// body uses `\n` line endings and excludes the opening line break, block quote
/// prefixes, list indentation, and opening-fence indentation. Indentation that
/// belongs to the code itself is preserved.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuleDocCodeBlock {
    /// The complete fenced block's range in the original Markdown source,
    /// including attached trivia.
    pub range: TextRange,
    /// The normalized text following the opening fence.
    pub info: String,
    /// The normalized code between the opening and closing fences.
    pub body: String,
}

/// Raw HTML preserved from the rule documentation source.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuleDocHtml {
    /// The HTML node's range in the original Markdown source, including
    /// attached trivia.
    pub range: TextRange,
    /// Whether the Markdown parser classified the HTML as block-level or inline.
    pub kind: RuleDocHtmlKind,
    /// Lossless source fragments in emission order.
    ///
    /// Block HTML has one fragment per physical source line, matching the event
    /// boundaries used by rule documentation validation. Inline HTML has one
    /// fragment containing the complete construct.
    pub source_fragments: Vec<String>,
}

/// The Markdown context in which raw HTML appeared.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuleDocHtmlKind {
    /// An HTML block recognized by the CommonMark block parser.
    Block,
    /// An HTML construct embedded in inline Markdown content.
    Inline,
}

/// The semantic contents of the first paragraph in rule documentation.
///
/// Emphasis and strong-emphasis containers are transparent in this
/// representation. Callers retain inline code as a distinct fragment so they
/// can escape or decorate it for their output format.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RuleSummary {
    fragments: Vec<RuleSummaryFragment>,
}

impl RuleSummary {
    pub(crate) fn new(fragments: Vec<RuleSummaryFragment>) -> Self {
        Self { fragments }
    }

    /// Returns the normalized paragraph fragments in source order.
    pub fn fragments(&self) -> &[RuleSummaryFragment] {
        &self.fragments
    }
}

/// Content that contributes to a generated rule summary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RuleSummaryFragment {
    /// Plain text with Markdown escapes and character references resolved.
    Text(String),
    /// Normalized code-span content without its backtick delimiters.
    InlineCode(String),
    /// A source line break that Markdown renders as ordinary whitespace.
    SoftBreak,
}
