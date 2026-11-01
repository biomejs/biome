use biome_analyze::shared::banner_comment::{has_banner, insert_banner, missing_banner_diagnostic};
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsRoot, JsSyntaxKind, JsSyntaxToken};
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::use_banner_comment::UseBannerCommentOptions;

use crate::JsRuleAction;

declare_lint_rule! {
    /// Enforce that every file starts with a configured banner comment.
    ///
    /// A banner comment is a comment placed at the very top of a file, commonly used to declare
    /// licensing, copyright, or authorship information. This rule reports files that don't start
    /// with the configured banner, and can insert it automatically.
    ///
    /// The banner must be the first comment of the file, after the shebang if any. Only block
    /// comments (`/* ... */`) are considered banners: a file that starts with a line comment
    /// (`// ...`) is reported. Comments whose lines start with `*`, such as `/** ... */`, are
    /// accepted too: the leading `*` characters and blank lines are ignored when comparing the
    /// comment with the configured banner.
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
    /// ```js,expect_diagnostic,use_options
    /// /* Copyright 1999 Someone Else */
    /// const a = 1;
    /// ```
    ///
    /// ```js,expect_diagnostic,use_options
    /// // Copyright 2026 Acme
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
    /// ```js,use_options
    /// /**
    ///  * Copyright 2026 Acme
    ///  */
    /// const a = 1;
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
    /// ```js,use_options
    /// /*
    /// Copyright 2026 Acme
    /// All rights reserved
    /// */
    /// const a = 1;
    /// ```
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

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let content = ctx.options().accepted_contents().first()?;
        let first_token = first_banner_token(ctx.query())?;
        let mut mutation = ctx.root().begin();
        insert_banner(&mut mutation, &first_token, content)?;

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Insert the required banner comment." }.to_owned(),
            mutation,
        ))
    }
}

/// Returns the first token of the file that can carry the banner, skipping the
/// byte order mark and the shebang.
fn first_banner_token(root: &AnyJsRoot) -> Option<JsSyntaxToken> {
    let mut token = root.syntax().first_token()?;
    while matches!(
        token.kind(),
        JsSyntaxKind::UNICODE_BOM | JsSyntaxKind::JS_SHEBANG
    ) {
        token = token.next_token()?;
    }
    Some(token)
}
