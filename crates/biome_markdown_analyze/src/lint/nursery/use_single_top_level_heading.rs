use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_markdown_syntax::{AnyMdHeader, MarkdownSyntaxKind::*, MdHtmlBlock, MdRoot};
use biome_rowan::{AstNode, Direction, TextRange};
use biome_rule_options::use_single_top_level_heading::UseSingleTopLevelHeadingOptions;

declare_lint_rule! {
    /// Enforce that a Markdown document has a single top-level heading.
    ///
    /// A Markdown document should have a single top-level heading (level 1 by default) that
    /// acts as the document's title. Subsequent headings should use lower levels (h2, h3, etc).
    /// Multiple top-level headings break document outlines, tables of contents, and the
    /// heading structure produced when the file is converted to HTML.
    ///
    /// The rule only looks at headings that are direct children of the document. A heading
    /// nested in a blockquote or in a list item is ignored entirely: it never counts as the
    /// title, and it is never reported as an extra top-level heading.
    ///
    /// The rule doesn't report diagnostics when the first heading at the configured level is
    /// preceded by:
    ///
    /// - a heading at another level
    /// - other block content, such as a paragraph or a front matter block
    ///
    /// In these cases, the rule assumes the document doesn't follow the single-title convention.
    /// Blank lines and HTML comments are ignored when determining whether the heading is the
    /// document title.
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
        version: "next",
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
        let block_list = root.value();

        if header.syntax().parent().as_ref() != Some(block_list.syntax()) {
            return None;
        }

        let title = block_list
            .syntax()
            .children()
            .filter_map(AnyMdHeader::cast)
            .find(|header| header.level() == level)?;

        if title.syntax() == header.syntax() || !is_document_title(&root, &title) {
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
/// A document title must be a direct child of the root block list and may be preceded only by blank
/// lines and HTML comments.
fn is_document_title(root: &MdRoot, header: &AnyMdHeader) -> bool {
    if header.syntax().grand_parent().as_ref() != Some(root.syntax()) {
        return false;
    }

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
