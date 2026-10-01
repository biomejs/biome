use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::{
    AnyCssDeclarationOrStatementBlock, AnyCssKeyframesItem, CssDeclarationImportant,
    CssDeclarationWithSemicolon, CssKeyframesBlock,
};
use biome_diagnostics::Severity;
use biome_rowan::AstNode;
use biome_rule_options::no_important_in_keyframe::NoImportantInKeyframeOptions;

declare_lint_rule! {
    /// Disallow invalid `!important` within keyframe declarations
    ///
    /// Using `!important` within keyframes declarations is completely ignored in some browsers.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// @keyframes foo {
    ///     from {
    ///       opacity: 0;
    ///     }
    ///     to {
    ///       opacity: 1 !important;
    ///     }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// @keyframes foo {
    ///     from {
    ///       opacity: 0;
    ///     }
    ///     to {
    ///       opacity: 1;
    ///     }
    /// }
    /// ```
    ///
    pub NoImportantInKeyframe {
        version: "1.8.0",
        name: "noImportantInKeyframe",
        language: "css",
        recommended: true,
        severity: Severity::Error,
        sources:&[RuleSource::Stylelint("keyframe-declaration-no-important").same()],
    }
}

impl Rule for NoImportantInKeyframe {
    type Query = Ast<CssKeyframesBlock>;
    type State = CssDeclarationImportant;
    type Signals = Option<Self::State>;
    type Options = NoImportantInKeyframeOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        for item in node.items() {
            let keyframe_item = match item {
                AnyCssKeyframesItem::CssKeyframesItem(keyframe_item) => keyframe_item,
                AnyCssKeyframesItem::ScssKeyframesVariableDeclaration(_) => continue,
                AnyCssKeyframesItem::CssBogusKeyframesItem(_) => return None,
            };
            let important = match keyframe_item.block().ok()? {
                AnyCssDeclarationOrStatementBlock::CssDeclarationBlock(block) => block
                    .declarations()
                    .into_iter()
                    .find_map(|item| find_important(item.as_css_declaration_with_semicolon())),
                // SCSS keyframe steps can also contain statements such as `@include`.
                AnyCssDeclarationOrStatementBlock::CssDeclarationOrAtRuleBlock(block) => block
                    .items()
                    .into_iter()
                    .find_map(|item| find_important(item.as_css_declaration_with_semicolon())),
                AnyCssDeclarationOrStatementBlock::CssBogusBlock(_) => return None,
            };

            if important.is_some() {
                return important;
            }
        }
        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, node: &Self::State) -> Option<RuleDiagnostic> {
        let span = node.range();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                span,
                markup! {
                    "Using "<Emphasis>"!important"</Emphasis>" within keyframes declaration is completely ignored in some browsers."
                },
            )
            .note(markup! {
                    "Consider removing useless "<Emphasis>"!important"</Emphasis>" declaration."
            }),
        )
    }
}

fn find_important(
    declaration: Option<&CssDeclarationWithSemicolon>,
) -> Option<CssDeclarationImportant> {
    declaration?.declaration().ok()?.important()
}
