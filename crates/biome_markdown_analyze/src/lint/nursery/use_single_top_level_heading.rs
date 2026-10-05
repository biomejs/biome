use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_markdown_syntax::{AnyMdHeader, MarkdownSyntaxKind::*, MdHtmlBlock};
use biome_rowan::{AstNode, Direction, TextRange};
use biome_rule_options::use_single_top_level_heading::UseSingleTopLevelHeadingOptions;

declare_lint_rule! {
    /// Enforce that a Markdown document has a single top-level heading.
    ///
    /// A Markdown document should have a single top-level heading that
    /// acts as the document's title. By default, level 1. Subsequent headings should use lower levels (h2, h3, etc).
    ///
    /// The rule reports a document only when it follows this convention, so it stays silent
    /// when:
    ///
    /// - the document starts with a front matter block (only parsed when
    ///   `markdown.parser.frontmatter` is enabled): the front matter may already carry the
    ///   document title, so a heading in the body isn't necessarily a duplicate;
    /// - the heading isn't a direct child of the document: a heading nested in a blockquote or
    ///   in a list item never counts as the title, and is never reported as an extra top-level
    ///   heading;
    /// - the first heading at the configured level isn't the document's title, because it is
    ///   preceded by something other than blank lines and HTML comments, such as a heading at
    ///   another level or a paragraph.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```md,expect_diagnostic
    /// Title
    /// =====
    ///
    /// Another top-level heading
    /// =========================
    /// ```
    ///
    /// ### Valid
    ///
    /// ```md
    /// Title
    /// =====
    ///
    /// ## Heading
    ///
    /// ## Another heading
    /// ```
    ///
    /// ```md
    /// # Title
    ///
    /// > # Quoted heading
    /// ```
    ///
    /// ## Options
    ///
    /// ### `level`
    ///
    /// Use the `level` option to change which heading level is treated as the top-level one.
    /// This is useful when an external tool (a static site generator, for example) already
    /// injects an `h1` for the page title, so the Markdown source is expected to start at `h2`.
    /// The value must be between `1` and `6`.
    ///
    /// Default: `1`.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "level": 2
    ///     }
    /// }
    /// ```
    ///
    /// ```md,use_options,expect_diagnostic
    /// ## Section
    ///
    /// ## Another top-level section
    /// ```
    ///
    pub UseSingleTopLevelHeading {
        version: "2.5.12",
        name: "useSingleTopLevelHeading",
        language: "md",
        recommended: true,
        sources: &[RuleSource::MarkdownLint("md025", "single-title").inspired()],
    }
}

impl Rule for UseSingleTopLevelHeading {
    type Query = Ast<AnyMdHeader>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = UseSingleTopLevelHeadingOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let header = ctx.query();
        let level = ctx.options().level() as usize;

        if header.level() != level {
            return None;
        }

        let root = ctx.root();

        // Front matter isn't a sibling in the block list, so `is_document_title` can't see it.
        // A document with front matter may already carry its title there, so body headings
        // aren't necessarily duplicates: stay silent rather than risk a false positive.
        if root.frontmatter().is_some() {
            return None;
        }

        let block_list = root.value();

        if header.syntax().parent().as_ref() != Some(block_list.syntax()) {
            return None;
        }

        let title = header
            .syntax()
            .siblings(Direction::Prev)
            .skip(1)
            .filter_map(AnyMdHeader::cast)
            .filter(|header| header.level() == level)
            .last()?;

        if !is_document_title(&title) {
            return None;
        }

        Some(title.range())
    }

    fn diagnostic(ctx: &RuleContext<Self>, title_range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "This document has more than one top-level heading."
                },
            )
            .detail(
                *title_range,
                markup! {
                    "The other top-level heading is here."
                },
            )
            .note(markup! {
                "A single top-level heading acts as the document's title. Additional ones break document outlines, tables of contents, and the structure produced when converting to HTML."
            })
            .note(markup! {
                "Demote this heading to a lower level, or move its section into its own document."
            }),
        )
    }
}

/// Returns whether a header is a top-level document title.
///
/// A document title may be preceded only by blank lines and HTML comments.
fn is_document_title(header: &AnyMdHeader) -> bool {
    header
        .syntax()
        .siblings(Direction::Prev)
        .skip(1)
        .all(|sibling| match sibling.kind() {
            MD_NEWLINE => true,
            MD_HTML_BLOCK => {
                MdHtmlBlock::cast(sibling).is_some_and(|block| block.is_html_comment())
            }
            _ => false,
        })
}
