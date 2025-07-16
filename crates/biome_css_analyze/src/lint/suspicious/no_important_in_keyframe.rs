use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::{
    AnyCssDeclarationBlock, AnyCssKeyframesItem, CssDeclarationImportant, CssKeyframesBlock,
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
            let AnyCssKeyframesItem::CssKeyframesItem(keyframe_item) = item else {
                return None;
            };
            let AnyCssDeclarationBlock::CssDeclarationBlock(block_declaration) =
                keyframe_item.block().ok()?
            else {
                return None;
            };

            for any_colon_declaration in block_declaration.declarations() {
                if let Some(important) = any_colon_declaration
                    .as_css_declaration_with_semicolon()
                    .and_then(|decl| decl.declaration().ok()?.important())
                {
                    return Some(important);
                }
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
