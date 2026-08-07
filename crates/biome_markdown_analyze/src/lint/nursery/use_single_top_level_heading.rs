use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_markdown_syntax::{
    MarkdownSyntaxKind::*, MarkdownSyntaxNode, MdHeader, MdHtmlBlock, MdRoot, MdSetextHeader,
};
use biome_rowan::{AstNode, TextRange, declare_node_union};
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
    /// :::note
    /// This rule does not support markdownlint's `front_matter_title` option: Biome's Markdown
    /// parser doesn't model YAML front matter, so a `title:` key can't be recognized as the
    /// document's title.
    /// :::
    ///
    pub UseSingleTopLevelHeading {
        version: "next",
        name: "useSingleTopLevelHeading",
        language: "md",
        recommended: true,
        sources: &[RuleSource::Markdownlint("MD025").inspired()],
    }
}

declare_node_union! {
    /// Any kind of Markdown heading: ATX (`# Heading`) or setext (`Heading` underlined with `=`/`-`).
    pub(crate) AnyMdHeading = MdHeader | MdSetextHeader
}

impl AnyMdHeading {
    fn level(&self) -> usize {
        match self {
            Self::MdHeader(header) => header.level(),
            Self::MdSetextHeader(header) => header.level(),
        }
    }
}

pub struct UseSingleTopLevelHeadingState {
    range: TextRange,
    title_range: TextRange,
}

impl Rule for UseSingleTopLevelHeading {
    type Query = Ast<MdRoot>;
    type State = UseSingleTopLevelHeadingState;
    type Signals = Box<[Self::State]>;
    type Options = UseSingleTopLevelHeadingOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let root = ctx.query();
        let level = ctx.options().level() as usize;

        let mut matching_headings = root
            .syntax()
            .descendants()
            .filter_map(AnyMdHeading::cast)
            .filter(|heading| heading.level() == level);

        let Some(title) = matching_headings.next() else {
            return [].into();
        };

        if !is_document_title(root, &title) {
            return [].into();
        }

        let title_range = title.range();

        matching_headings
            .map(|heading| UseSingleTopLevelHeadingState {
                range: heading.range(),
                title_range,
            })
            .collect()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "This document has more than one top-level heading."
                },
            )
            .detail(
                state.title_range,
                markup! {
                    "The first top-level heading is here."
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

/// Mirrors markdownlint's MD025 check that the document's title is only recognized as such if
/// nothing but non-content material precedes it: blank lines, structural containers (the ones
/// wrapping the heading itself, e.g. a surrounding block quote or list item), and HTML comments.
fn is_document_title(root: &MdRoot, heading: &AnyMdHeading) -> bool {
    root.syntax()
        .descendants()
        .take_while(|node| node != heading.syntax())
        .all(|node| is_non_content(&node, heading))
}

fn is_non_content(node: &MarkdownSyntaxNode, heading: &AnyMdHeading) -> bool {
    match node.kind() {
        MD_ROOT | MD_BLOCK_LIST | MD_QUOTE | MD_BULLET_LIST_ITEM | MD_ORDERED_LIST_ITEM
        | MD_BULLET_LIST | MD_BULLET => is_ancestor_of_heading(node, heading),
        MD_LIST_MARKER_PREFIX
        | MD_INDENT_TOKEN_LIST
        | MD_QUOTE_INDENT_LIST
        | MD_NEWLINE
        | MD_QUOTE_PREFIX
        | MD_QUOTE_INDENT
        | MD_INDENT_TOKEN
        | MD_CONTINUATION_INDENT
        | MD_HTML_CONTENT => true,
        MD_HTML_BLOCK => MdHtmlBlock::cast(node.clone()).is_some_and(|block| is_html_comment(&block)),
        _ => false,
    }
}

fn is_ancestor_of_heading(node: &MarkdownSyntaxNode, heading: &AnyMdHeading) -> bool {
    heading.syntax().ancestors().any(|ancestor| ancestor == *node)
}

/// Whether an HTML block is a comment (`<!-- ... -->`), which markdownlint treats as
/// non-content when it precedes the document's title.
fn is_html_comment(block: &MdHtmlBlock) -> bool {
    block.content().is_ok_and(|content| {
        content.value_token().is_ok_and(|token| {
            let text = token.text_trimmed();
            text.starts_with("<!--") && text.ends_with("-->")
        })
    })
}
