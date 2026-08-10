use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_markdown_syntax::{
    AnyMdHeader, MarkdownSyntaxKind::*, MdHtmlBlock, MdRoot,
};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::use_single_top_level_heading::UseSingleTopLevelHeadingOptions;

declare_lint_rule! {
    /// Enforce that a Markdown document has a single top-level heading.
    ///
    /// A Markdown document should have a single top-level heading (level 1 by default) that
    /// acts as the document's title. Subsequent headings should use lower levels (h2, h3, etc).
    /// Multiple top-level headings confuse document outlines, tables of contents, and the
    /// heading structure produced when the file is converted to HTML.
    ///
    /// This rule only reports extra top-level headings when the *first* matching heading is
    /// itself the document's title, i.e. nothing but blank lines and HTML comments precede it.
    /// If other content (a paragraph, a different heading level, etc.) comes before the first
    /// matching heading, the rule assumes the document doesn't follow the single-title
    /// convention at all and stays silent.
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
        let title = root
            .syntax()
            .descendants()
            .skip(1)
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

/// Returns whether a header can serve as the document title.
///
/// A document title may be preceded only by blank lines, containers that contain the header, and
/// HTML comments.
fn is_document_title(root: &MdRoot, header: &AnyMdHeader) -> bool {
    root.syntax()
        .descendants()
        .skip(1)
        .take_while(|node| node != header.syntax())
        .all(|node| match node.kind() {
            MD_BLOCK_LIST | MD_QUOTE | MD_BULLET_LIST_ITEM | MD_ORDERED_LIST_ITEM
            | MD_BULLET_LIST | MD_BULLET => header
                .syntax()
                .ancestors()
                .any(|ancestor| ancestor == node),
            MD_LIST_MARKER_PREFIX
            | MD_INDENT_TOKEN_LIST
            | MD_QUOTE_INDENT_LIST
            | MD_NEWLINE
            | MD_QUOTE_PREFIX
            | MD_QUOTE_INDENT
            | MD_INDENT_TOKEN
            | MD_CONTINUATION_INDENT => true,
            MD_HTML_CONTENT => node
                .parent()
                .and_then(MdHtmlBlock::cast)
                .is_some_and(|block| block.is_html_comment()),
            MD_HTML_BLOCK => {
                MdHtmlBlock::cast(node.clone()).is_some_and(|block| block.is_html_comment())
            }
            _ => false,
        })
}
