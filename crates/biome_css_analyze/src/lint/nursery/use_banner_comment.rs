use biome_analyze::shared::banner_comment::{has_banner, insert_banner, missing_banner_diagnostic};
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::{CssRoot, CssSyntaxKind, CssSyntaxToken};
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::use_banner_comment::UseBannerCommentOptions;

use crate::CssRuleAction;

declare_lint_rule! {
    /// Enforce that every file starts with a configured banner comment.
    ///
    /// A banner comment is a comment placed at the very top of a file, commonly used to declare
    /// licensing, copyright, or authorship information. This rule reports files that don't start
    /// with the configured banner, and can insert it automatically.
    ///
    /// The banner must be the first comment of the file. Comments whose lines start with `*`,
    /// such as `/** ... */`, are accepted too: the leading `*` characters and blank lines are
    /// ignored when comparing the comment with the configured banner.
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
    /// ```css,use_options
    /// /**
    ///  * Copyright 2026 Acme
    ///  */
    /// .a { color: red; }
    /// ```
    ///
    /// ## Options
    ///
    /// The following options are available
    ///
    /// ### `content`
    ///
    /// The banner that every file must start with. It accepts either a string, or an array of
    /// strings when more than one banner is acceptable. The fix inserts the first banner of the array.
    ///
    /// This option has no default value. Configure it explicitly to enable the rule.
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
        let accepted = ctx.options().accepted_contents();
        if accepted.is_empty() {
            return None;
        }
        let first_token = first_banner_token(ctx.query())?;
        (!has_banner(&first_token, accepted)).then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let first_token = first_banner_token(ctx.query())?;
        Some(missing_banner_diagnostic(rule_category!(), &first_token))
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<CssRuleAction> {
        let content = ctx.options().accepted_contents().first()?;
        let first_token = first_banner_token(ctx.query())?;
        let mut mutation = ctx.root().begin();
        insert_banner(&mut mutation, &first_token, content)?;

        Some(CssRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Insert the required banner comment." }.to_owned(),
            mutation,
        ))
    }
}

/// Returns the first token of the file that can carry the banner, skipping the byte order mark.
fn first_banner_token(root: &CssRoot) -> Option<CssSyntaxToken> {
    let token = root.syntax().first_token()?;
    if token.kind() == CssSyntaxKind::UNICODE_BOM {
        token.next_token()
    } else {
        Some(token)
    }
}
