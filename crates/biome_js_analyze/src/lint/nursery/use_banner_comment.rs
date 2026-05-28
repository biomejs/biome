use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsRoot, JsLanguage};
use biome_rowan::{AstNode, BatchMutationExt, SyntaxToken, TriviaPieceKind};
use biome_rule_options::use_banner_comment::UseBannerCommentOptions;

use crate::JsRuleAction;

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
    /// ```js,expect_diagnostic,use_options
    /// const a = 1;
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
    /// ```js,expect_diagnostic,use_options
    /// /* Copyright 1999 Someone Else */
    /// const a = 1;
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
    /// ```js,use_options
    /// /* Copyright 2026 Acme */
    /// const a = 1;
    /// ```
    ///
    /// ## Comment styles
    ///
    /// Only block comments (`/* ... */`) are recognized as banners. JSDoc-style
    /// comments (`/** ... */`) are accepted as well: their leading `*` margins
    /// and any blank lines are ignored when matching, so the following matches
    /// the configured banner `"Copyright 2026 Acme"`:
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///        "content": "Copyright 2026 Acme"
    ///     }
    /// }
    /// ```
    /// ```js,use_options
    /// /**
    ///  * Copyright 2026 Acme
    ///  */
    /// const a = 1;
    /// ```
    ///
    /// Line comments (`// ...`) are never treated as banners. A file whose first
    /// comment is a line comment, or that has no leading comment, is reported as
    /// missing the banner, and the fix inserts a `/* ... */` block:
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///        "content": "Copyright 2026 Acme"
    ///     }
    /// }
    /// ```
    /// ```js,expect_diagnostic,use_options
    /// // Copyright 2026 Acme
    /// const a = 1;
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
    /// ```js,use_options
    /// /*
    /// Copyright 2026 Acme
    /// All rights reserved
    /// */
    /// const a = 1;
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
    /// ```js,use_options
    /// /* Copyright 2026 Acme Inc. */
    /// const a = 1;
    /// ```
    ///
    pub UseBannerComment {
        version: "next",
        name: "useBannerComment",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintHeader("header").inspired()],
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseBannerComment {
    type Query = Ast<AnyJsRoot>;
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

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
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

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Insert the required banner comment." }.to_owned(),
            mutation,
        ))
    }
}

/// Returns the inner text of the file's first leading comment when it is a
/// `/* ... */` block (the content between `/*` and `*/`), or `None` when the
/// first comment is a line comment (`// ...`) or there is no leading comment.
///
/// Only block comments are recognized as banners. JSDoc-style `/** ... */`
/// comments are block comments too, so they are returned here; their `*`
/// margins are normalized later in [`banner_matches`].
fn extract_banner_block(token: &SyntaxToken<JsLanguage>) -> Option<String> {
    let comment = token.leading_trivia().pieces().find(|p| p.is_comments())?;
    let text = comment.text();
    let inner = text.strip_prefix("/*")?.strip_suffix("*/")?;
    Some(inner.to_string())
}

/// Builds the comment block for the canonical banner content.
/// Single-line content becomes `/* content */`; multi-line content is wrapped
/// with the `/*` and `*/` markers on their own lines.
fn build_banner_block(canonical: &str) -> String {
    if canonical.contains('\n') {
        format!("/*\n{canonical}\n*/")
    } else {
        format!("/* {canonical} */")
    }
}

/// Returns a new first token whose leading trivia starts with the expected
/// banner block followed by a newline. Any pre-existing leading `/* */` block
/// comment is dropped; other trivia (whitespace, newlines, line comments) are
/// kept.
fn replace_or_prepend_banner(
    token: &SyntaxToken<JsLanguage>,
    banner_block: &str,
) -> SyntaxToken<JsLanguage> {
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

/// Returns the appropriate `TriviaPieceKind` for a comment text:
/// `MultiLineComment` if it contains a line break, otherwise `SingleLineComment`.
fn comment_kind(text: &str) -> TriviaPieceKind {
    if text.contains('\n') {
        TriviaPieceKind::MultiLineComment
    } else {
        TriviaPieceKind::SingleLineComment
    }
}

/// Returns whether `actual` (the text between `/*` and `*/` of the file's
/// leading block comment) matches the configured `expected` banner content.
///
/// Matching is line-based and lenient: each line is trimmed, an optional leading
/// `*` margin (as used in JSDoc-style `/** ... */` comments) is removed, and
/// blank lines are ignored. As a result, `/* ... */` and `/** ... */` banners
/// match the same configured content.
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
