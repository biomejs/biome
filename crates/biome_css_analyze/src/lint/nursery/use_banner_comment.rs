use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::{CssLanguage, CssRoot};
use biome_rowan::{AstNode, BatchMutationExt, SyntaxToken, TriviaPieceKind};
use biome_rule_options::use_banner_comment::UseBannerCommentOptions;

use crate::CssRuleAction;

declare_lint_rule! {
    /// Enforce that every file starts with a configured banner comment.
    ///
    /// A banner comment is a `/* ... */` comment placed at the very top of a file,
    /// commonly used to declare licensing, copyright, or authorship information.
    /// This rule reports a diagnostic when a file does not begin with the configured
    /// banner, and can insert or replace it automatically.
    ///
    /// The rule does nothing when `content` is not configured.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///        "content": "Copyright 2026 Acme"
    ///     }
    /// }
    /// ```
    /// ```css,expect_diagnostic,use_options
    /// .a { color: red; }
    /// ```
    ///
    /// A banner that does not match the configured content is also reported:
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///        "content": "Copyright 2026 Acme"
    ///     }
    /// }
    /// ```
    /// ```css,expect_diagnostic,use_options
    /// /* Copyright 1999 Someone Else */
    /// .a { color: red; }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///        "content": "Copyright 2026 Acme"
    ///     }
    /// }
    /// ```
    /// ```css,use_options
    /// /* Copyright 2026 Acme */
    /// .a { color: red; }
    /// ```
    ///
    /// ## Comment styles
    ///
    /// CSS only has block comments (`/* ... */`), which are the comments this
    /// rule matches against. JSDoc-style comments (`/** ... */`) are accepted as
    /// well: their leading `*` margins and any blank lines are ignored when
    /// matching, so the following matches the configured banner
    /// `"Copyright 2026 Acme"`:
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///        "content": "Copyright 2026 Acme"
    ///     }
    /// }
    /// ```
    /// ```css,use_options
    /// /**
    ///  * Copyright 2026 Acme
    ///  */
    /// .a { color: red; }
    /// ```
    ///
    /// ## Options
    ///
    /// ### `content`
    ///
    /// The expected banner content. Accepts either a string (one canonical
    /// banner) or an array of strings (any one of which is acceptable).
    ///
    /// A multi-line banner is supported by embedding newlines in the string:
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///        "content": "Copyright 2026 Acme\nAll rights reserved"
    ///     }
    /// }
    /// ```
    /// ```css,use_options
    /// /*
    /// Copyright 2026 Acme
    /// All rights reserved
    /// */
    /// .a { color: red; }
    /// ```
    ///
    /// Or as an array to allow more than one acceptable banner:
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///        "content": ["Copyright 2026 Acme", "Copyright 2026 Acme Inc."]
    ///     }
    /// }
    /// ```
    /// ```css,use_options
    /// /* Copyright 2026 Acme Inc. */
    /// .a { color: red; }
    /// ```
    ///
    pub UseBannerComment {
        version: "next",
        name: "useBannerComment",
        language: "css",
        recommended: false,
        sources: &[RuleSource::EslintHeader("header").inspired()],
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseBannerComment {
    type Query = Ast<CssRoot>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseBannerCommentOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let options = ctx.options();
        let accepted = options.accepted_contents();
        if accepted.is_empty() {
            return None;
        }

        let first_token = ctx.query().syntax().first_token()?;
        let actual = extract_banner_block(&first_token);

        match actual {
            Some(body) if accepted.iter().any(|expected| banner_matches(&body, expected)) => None,
            _ => Some(()),
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let first_token = node.syntax().first_token()?;
        let range = first_token
            .leading_trivia()
            .pieces()
            .find(|p| p.is_comments())
            .map_or_else(|| first_token.text_range(), |c| c.text_range());

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "The file does not start with the required banner comment."
                },
            )
            .note(markup! {
                "A banner comment communicates licensing, copyright, or ownership information to anyone reading or auditing the file."
            })
            .note(markup! {
                "Add or update the leading "<Emphasis>"/* ... */"</Emphasis>" block to match the configured banner."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<CssRuleAction> {
        let canonical = ctx.options().accepted_contents().first()?.as_ref();
        let banner_block = build_banner_block(canonical);

        let node = ctx.query();
        let first_token = node.syntax().first_token()?;
        let new_first_token = replace_or_prepend_banner(&first_token, &banner_block);

        // Replace via the parent node so the text edit range covers the file's
        // leading trivia. Token-only replacement leaves the old trivia bytes
        // in the source because `SyntaxToken::text_range()` excludes trivia.
        let parent = first_token.parent()?;
        let new_parent = parent
            .clone()
            .replace_child(first_token.into(), new_first_token.into())?;

        let mut mutation = ctx.root().begin();
        mutation.replace_element_discard_trivia(parent.into(), new_parent.into());

        Some(CssRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Insert the required banner comment." }.to_owned(),
            mutation,
        ))
    }
}

/// Returns the inner text of the file's first leading `/* ... */` block comment
/// (the content between `/*` and `*/`), or `None` when there is no leading
/// comment. JSDoc-style `/** ... */` comments are block comments too, so they
/// are returned here; their `*` margins are normalized in [`banner_matches`].
fn extract_banner_block(token: &SyntaxToken<CssLanguage>) -> Option<String> {
    let comment = token.leading_trivia().pieces().find(|p| p.is_comments())?;
    let text = comment.text();
    let inner = text.strip_prefix("/*")?.strip_suffix("*/")?;
    Some(inner.to_string())
}

fn build_banner_block(canonical: &str) -> String {
    if canonical.contains('\n') {
        format!("/*\n{canonical}\n*/")
    } else {
        format!("/* {canonical} */")
    }
}

fn replace_or_prepend_banner(
    token: &SyntaxToken<CssLanguage>,
    banner_block: &str,
) -> SyntaxToken<CssLanguage> {
    let banner_kind = comment_kind(banner_block);
    let mut new_trivia: Vec<(TriviaPieceKind, String)> = vec![
        (banner_kind, banner_block.to_string()),
        (TriviaPieceKind::Newline, "\n".to_string()),
    ];

    let mut banner_dropped = false;
    for piece in token.leading_trivia().pieces() {
        if !banner_dropped && piece.text().starts_with("/*") {
            banner_dropped = true;
            continue;
        }
        new_trivia.push((piece.kind(), piece.text().to_string()));
    }

    token.with_leading_trivia(new_trivia.iter().map(|(k, t)| (*k, t.as_str())))
}

fn comment_kind(text: &str) -> TriviaPieceKind {
    if text.contains('\n') {
        TriviaPieceKind::MultiLineComment
    } else {
        TriviaPieceKind::SingleLineComment
    }
}

/// Returns whether `actual` (the text between `/*` and `*/`) matches the
/// configured `expected` banner content.
///
/// Matching is line-based and lenient: each line is trimmed, an optional leading
/// `*` margin (as used in JSDoc-style `/** ... */` comments) is removed, and
/// blank lines are ignored. As a result, `/* ... */` and `/** ... */` banners
/// match the same configured content. Kept in sync with the JS sibling rule.
fn banner_matches(actual: &str, expected: &str) -> bool {
    significant_banner_lines(actual).eq(significant_banner_lines(expected))
}

/// Yields the meaningful lines of a banner: each trimmed, with an optional
/// leading `*` margin removed, skipping blank lines. Borrows from the input,
/// so no allocation is performed.
fn significant_banner_lines(text: &str) -> impl Iterator<Item = &str> {
    text.lines().filter_map(|line| {
        let line = line.trim();
        let line = line.strip_prefix('*').unwrap_or(line).trim();
        (!line.is_empty()).then_some(line)
    })
}
