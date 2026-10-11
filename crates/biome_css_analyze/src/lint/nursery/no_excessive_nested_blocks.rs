use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, RuleSuppressions, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::{
    AnyCssRoot, CssAtRule, CssDeclarationBlock, CssDeclarationOrAtRuleBlock,
    CssDeclarationOrRuleBlock, CssDeclarationWithSemicolon, CssFontFeatureValuesBlock,
    CssKeyframesBlock, CssLanguage, CssPageAtRuleBlock, CssRelativeSelectorList, CssRuleBlock,
    CssSelectorList, ScssExpression, ScssIfAtRule, ScssVariableDeclaration,
};
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, SyntaxKindSet, TextRange, WalkEvent, declare_node_union};
use biome_rule_options::no_excessive_nested_blocks::NoExcessiveNestedBlocksOptions;

declare_lint_rule! {
    /// Enforce a maximum depth that blocks can be nested.
    ///
    /// Deeply nested styles are difficult to read and follow. Every level of nesting also makes
    /// the resulting selector longer, which makes the styles harder to override. This rule reports
    /// `{}` blocks nested beyond the configured limit.
    ///
    /// The block of a rule or at-rule written at the top level of the file isn't nested, so it
    /// doesn't count as a level. Wrapping styles in a top-level at-rule, such as `@media` or
    /// `@layer`, doesn't add a level either. Every other block adds one level.
    ///
    /// Only the outermost block exceeding the limit is reported: blocks nested inside it aren't
    /// reported again.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// .a {
    ///     .b {
    ///         .c {
    ///             .d {
    ///                 .e {
    ///                     .f {
    ///                         color: red;
    ///                     }
    ///                 }
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// .a {
    ///     .b {
    ///         .c {
    ///             .d {
    ///                 .e {
    ///                     color: red;
    ///                 }
    ///             }
    ///         }
    ///     }
    /// }
    ///
    /// @media (width >= 600px) {
    ///     .a {
    ///         .b {
    ///             .c {
    ///                 .d {
    ///                     .e {
    ///                         color: red;
    ///                     }
    ///                 }
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// ### max
    ///
    /// Default: 4
    ///
    /// The maximum block nesting depth allowed.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "max": 1
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```css,use_options,expect_diagnostic
    /// .a {
    ///     .b {
    ///         .c {
    ///             color: red;
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// #### Valid
    ///
    /// ```css,use_options
    /// .a {
    ///     .b {
    ///         color: red;
    ///     }
    /// }
    /// ```
    ///
    pub NoExcessiveNestedBlocks {
        version: "next",
        name: "noExcessiveNestedBlocks",
        language: "css",
        sources: &[RuleSource::Stylelint("max-nesting-depth").same()],
        severity: Severity::Warning,
        recommended: false,
    }
}

declare_node_union! {
    pub AnyCssNestingBlock =
        CssDeclarationBlock
        | CssDeclarationOrAtRuleBlock
        | CssDeclarationOrRuleBlock
        | CssFontFeatureValuesBlock
        | CssKeyframesBlock
        | CssPageAtRuleBlock
        | CssRuleBlock
}

impl Rule for NoExcessiveNestedBlocks {
    type Query = Ast<AnyCssNestingBlock>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoExcessiveNestedBlocksOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let block = ctx.query();

        // Blocks nested inside a reported one are suppressed and never reach this point, which
        // means a reported block is always exactly `max + 1` levels deep.
        let mut depth = 0usize;
        for ancestor in block.syntax().ancestors().skip(1) {
            if AnyCssNestingBlock::can_cast(ancestor.kind()) {
                depth += 1;
            } else if CssAtRule::cast_ref(&ancestor).is_some_and(|rule| is_top_level(&rule)) {
                // The block of a top-level at-rule was counted as a level, but it is free.
                depth = depth.saturating_sub(1);
            }
        }

        (depth > usize::from(ctx.options().max())).then_some(())
    }

    fn suppressed_nodes(
        ctx: &RuleContext<Self>,
        _state: &Self::State,
        suppressions: &mut RuleSuppressions<CssLanguage>,
    ) {
        // Every block nested inside the reported one is nested even deeper, so only the
        // outermost one is reported. The `@else` blocks of an `@if` sit beside its block at the
        // same depth, so the whole `@if` is walked to report the chain once.
        let block = ctx.query().syntax();
        let root = block
            .parent()
            .filter(|parent| ScssIfAtRule::can_cast(parent.kind()))
            .unwrap_or_else(|| block.clone());
        let mut preorder = root.preorder();

        while let Some(event) = preorder.next() {
            let WalkEvent::Enter(node) = event else {
                continue;
            };
            let kind = node.kind();

            if BLOCKLESS_KINDS.matches(kind) {
                preorder.skip_subtree();
            } else if AnyCssNestingBlock::can_cast(kind) {
                suppressions.suppress_node(node);
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let block = ctx.query();
        let max = ctx.options().max();
        let depth = usize::from(max) + 1;

        // Highlight what introduces the block: everything from the start of the rule or at-rule
        // up to the opening brace.
        let owner = block.syntax().parent()?;
        let owner = owner
            .parent()
            .filter(|parent| CssAtRule::can_cast(parent.kind()))
            .unwrap_or(owner);
        let opening_brace = block.syntax().first_token()?;
        let header_start = owner.text_trimmed_range().start();
        // A block with nothing before it, such as a keyframe that is missing its selector,
        // has no header, so the token before the brace belongs to the enclosing block.
        let range = opening_brace
            .prev_token()
            .map(|token| token.text_trimmed_range().end())
            .filter(|header_end| *header_end > header_start)
            .map_or_else(
                || opening_brace.text_trimmed_range(),
                |header_end| TextRange::new(header_start, header_end),
            );

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This block is nested too deeply."
                },
            )
            .note(markup! {
                "Blocks nested "{depth}" levels deep are hard to read and produce long selectors that are hard to override. The configured maximum is "{max}"."
            })
            .note(markup! {
                "Move the nested styles closer to the top level, or use a single class name instead of a chain of nested selectors."
            }),
        )
    }
}

/// Nodes that never contain an [AnyCssNestingBlock], which makes walking into them pointless.
const BLOCKLESS_KINDS: SyntaxKindSet<CssLanguage> = CssDeclarationWithSemicolon::KIND_SET
    .union(CssRelativeSelectorList::KIND_SET)
    .union(CssSelectorList::KIND_SET)
    .union(ScssExpression::KIND_SET)
    .union(ScssVariableDeclaration::KIND_SET);

/// Returns `true` if the at-rule is written at the top level of the file, outside any block.
fn is_top_level(rule: &CssAtRule) -> bool {
    rule.syntax()
        .grand_parent()
        .is_some_and(|root| AnyCssRoot::can_cast(root.kind()))
}
