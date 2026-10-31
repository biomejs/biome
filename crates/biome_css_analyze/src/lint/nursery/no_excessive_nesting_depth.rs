use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::{
    AnyCssAtRule, AnyCssRoot, CssAtRule, CssFontFeatureValuesItem, CssKeyframesItem,
    CssMarginAtRule, CssNestedQualifiedRule, CssQualifiedRule, CssSyntaxKind, CssSyntaxToken,
    ScssElseClause,
};
use biome_rowan::{AstNode, TextRange, declare_node_union};
use biome_rule_options::no_excessive_nesting_depth::NoExcessiveNestingDepthOptions;

declare_lint_rule! {
    /// Limit how deeply rules and at-rules can be nested.
    ///
    /// Deeply nested blocks are hard to read, and each nested rule adds its parent's selector to its
    /// own, so the resulting selectors grow longer and more specific. Specific selectors are harder
    /// to override later.
    ///
    /// Every rule or at-rule that encloses a block adds one level of depth to the block, with these
    /// exceptions:
    ///
    /// - At-rules at the top of the file, such as an `@media` query wrapping a whole set of rules,
    ///   don't add a level.
    /// - An SCSS `@if`, together with its `@else if` and `@else` branches, adds a single level.
    /// - At-rules listed in [`ignoreAtRules`](#ignoreatrules) don't add a level. To ignore the
    ///   `@else if` and `@else` branches of an SCSS `@if`, list `else`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// `.e` is nested four levels deep, which is more than the default maximum of 3:
    ///
    /// ```css,expect_diagnostic
    /// .a {
    ///   .b {
    ///     .c {
    ///       .d {
    ///         .e {
    ///           color: red;
    ///         }
    ///       }
    ///     }
    ///   }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// .a {
    ///   .b {
    ///     .c {
    ///       .d {
    ///         color: red;
    ///       }
    ///     }
    ///   }
    /// }
    /// ```
    ///
    /// The `@media` query is at the top of the file, so it doesn't add a level:
    ///
    /// ```css
    /// @media (min-width: 600px) {
    ///   .a {
    ///     .b {
    ///       .c {
    ///         .d {
    ///           color: red;
    ///         }
    ///       }
    ///     }
    ///   }
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// ### `maxDepth`
    ///
    /// The maximum number of levels a rule or at-rule can be nested. Defaults to `3`.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "maxDepth": 1
    ///     }
    /// }
    /// ```
    ///
    /// ```css,expect_diagnostic,use_options
    /// .a {
    ///   .b {
    ///     .c {
    ///       color: red;
    ///     }
    ///   }
    /// }
    /// ```
    ///
    /// ### `ignoreAtRules`
    ///
    /// A list of at-rule names, without the `@`, that don't add a level. Names are
    /// case-insensitive. Defaults to an empty list.
    ///
    /// Rules nested inside an ignored at-rule are still counted from the outermost rule, so this
    /// option doesn't hide deep nesting inside media queries. Stylelint's `max-nesting-depth`
    /// doesn't report anything nested inside an ignored at-rule.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "ignoreAtRules": ["media"]
    ///     }
    /// }
    /// ```
    ///
    /// `.d` is nested three levels deep, because the `@media` query doesn't add a level:
    ///
    /// ```css,use_options
    /// .a {
    ///   .b {
    ///     @media (min-width: 600px) {
    ///       .c {
    ///         .d {
    ///           color: red;
    ///         }
    ///       }
    ///     }
    ///   }
    /// }
    /// ```
    ///
    /// `.e` is still nested four levels deep:
    ///
    /// ```css,expect_diagnostic,use_options
    /// .a {
    ///   .b {
    ///     @media (min-width: 600px) {
    ///       .c {
    ///         .d {
    ///           .e {
    ///             color: red;
    ///           }
    ///         }
    ///       }
    ///     }
    ///   }
    /// }
    /// ```
    ///
    pub NoExcessiveNestingDepth {
        version: "next",
        name: "noExcessiveNestingDepth",
        language: "css",
        recommended: false,
        sources: &[RuleSource::Stylelint("max-nesting-depth").inspired()],
    }
}

declare_node_union! {
    /// A statement that encloses a block and can contain nested rules or at-rules.
    pub AnyCssNestingStatement =
        CssQualifiedRule
        | CssNestedQualifiedRule
        | CssAtRule
        | CssKeyframesItem
        | CssMarginAtRule
        | CssFontFeatureValuesItem
}

pub struct ExcessiveNesting {
    range: TextRange,
    depth: usize,
}

impl Rule for NoExcessiveNestingDepth {
    type Query = Ast<AnyCssNestingStatement>;
    type State = ExcessiveNesting;
    type Signals = Option<Self::State>;
    type Options = NoExcessiveNestingDepthOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let statement = ctx.query();
        let options = ctx.options();

        let mut range = None;
        if let AnyCssNestingStatement::CssAtRule(at_rule) = statement {
            if !has_block(at_rule) {
                return None;
            }
            if is_ignored(at_rule, options) {
                range = Some(unignored_else_range(at_rule, options)?);
            }
        }

        let depth = nesting_depth(statement, options);

        (depth > usize::from(options.max_depth())).then(|| ExcessiveNesting {
            range: range.unwrap_or_else(|| statement_range(statement)),
            depth,
        })
    }

    fn text_range(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<TextRange> {
        Some(state.range)
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let max_depth = ctx.options().max_depth();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "This block is nested "{state.depth}" levels deep, but the maximum is "{max_depth}"."
                },
            )
            .note(markup! {
                "Deeply nested blocks are hard to read, and each nested rule makes the resulting selector longer and more specific, which makes it harder to override."
            })
            .note(markup! {
                "Move this block to a shallower level, or split the nesting into separate rules."
            }),
        )
    }
}

/// Counts the enclosing statements that add a level to `statement`.
///
/// Top-level at-rules don't count, because wrapping a whole set of rules in a condition like
/// `@media` doesn't make any of them harder to read or more specific.
///
/// An `@else` clause is part of the `@if` at-rule in the syntax tree, but Sass users write it as
/// its own at-rule. A statement inside an `@else` or `@else if` branch is therefore counted once,
/// at the owning `@if`, under the name `else` for `ignoreAtRules`.
fn nesting_depth(
    statement: &AnyCssNestingStatement,
    options: &NoExcessiveNestingDepthOptions,
) -> usize {
    let mut depth = 0;
    let mut in_else_branch = false;
    for ancestor in statement.syntax().ancestors().skip(1) {
        if ScssElseClause::can_cast(ancestor.kind()) {
            in_else_branch = true;
            continue;
        }
        let Some(ancestor) = AnyCssNestingStatement::cast(ancestor) else {
            continue;
        };
        let adds_level = match &ancestor {
            AnyCssNestingStatement::CssAtRule(at_rule) => {
                let is_ignored = if std::mem::take(&mut in_else_branch) {
                    options.is_ignored_at_rule("else")
                } else {
                    is_ignored(at_rule, options)
                };
                let is_top_level = at_rule
                    .syntax()
                    .grand_parent()
                    .is_some_and(|root| AnyCssRoot::can_cast(root.kind()));
                !is_top_level && !is_ignored
            }
            _ => true,
        };
        if adds_level {
            depth += 1;
        }
    }
    depth
}

/// The range of the `@else` of an ignored `@if`, if its branches still need checking.
///
/// Sass users write `@else` as its own at-rule, so listing only `if` in `ignoreAtRules` doesn't
/// exempt the `@else` branches.
fn unignored_else_range(
    at_rule: &CssAtRule,
    options: &NoExcessiveNestingDepthOptions,
) -> Option<TextRange> {
    if options.is_ignored_at_rule("else") {
        return None;
    }
    let AnyCssAtRule::ScssIfAtRule(if_rule) = at_rule.rule().ok()? else {
        return None;
    };
    let else_clause = if_rule.else_clause()?;
    at_rule_name_range(else_clause.at_token().ok(), else_clause.else_token().ok())
}

fn is_ignored(at_rule: &CssAtRule, options: &NoExcessiveNestingDepthOptions) -> bool {
    at_rule
        .rule()
        .ok()
        .and_then(|rule| rule.syntax().first_token())
        .is_some_and(|name| options.is_ignored_at_rule(name.text_trimmed()))
}

/// Whether the at-rule ends with a block.
///
/// Checking only for a final `}` isn't enough: a blockless SCSS at-rule such as
/// `@extend .icon-#{$name}` can omit its `;` and end with the `}` of an interpolation.
fn has_block(at_rule: &CssAtRule) -> bool {
    at_rule
        .syntax()
        .last_token()
        .and_then(|token| token.parent())
        .is_some_and(|owner| {
            matches!(
                owner.kind(),
                CssSyntaxKind::CSS_DECLARATION_BLOCK
                    | CssSyntaxKind::CSS_DECLARATION_OR_AT_RULE_BLOCK
                    | CssSyntaxKind::CSS_DECLARATION_OR_RULE_BLOCK
                    | CssSyntaxKind::CSS_FONT_FEATURE_VALUES_BLOCK
                    | CssSyntaxKind::CSS_KEYFRAMES_BLOCK
                    | CssSyntaxKind::CSS_PAGE_AT_RULE_BLOCK
                    | CssSyntaxKind::CSS_RULE_BLOCK
            )
        })
}

/// The selector of a rule, or the `@` and name of an at-rule.
fn statement_range(statement: &AnyCssNestingStatement) -> TextRange {
    let range = match statement {
        AnyCssNestingStatement::CssQualifiedRule(rule) => Some(rule.prelude().range()),
        AnyCssNestingStatement::CssNestedQualifiedRule(rule) => Some(rule.prelude().range()),
        AnyCssNestingStatement::CssKeyframesItem(item) => Some(item.selectors().range()),
        AnyCssNestingStatement::CssAtRule(at_rule) => at_rule_name_range(
            at_rule.at_token().ok(),
            at_rule
                .rule()
                .ok()
                .and_then(|rule| rule.syntax().first_token()),
        ),
        AnyCssNestingStatement::CssMarginAtRule(at_rule) => {
            at_rule_name_range(at_rule.at_token().ok(), at_rule.name().ok())
        }
        AnyCssNestingStatement::CssFontFeatureValuesItem(item) => {
            at_rule_name_range(item.at_token().ok(), item.name().ok())
        }
    };
    range.unwrap_or_else(|| statement.range())
}

fn at_rule_name_range(
    at: Option<CssSyntaxToken>,
    name: Option<CssSyntaxToken>,
) -> Option<TextRange> {
    Some(at?.text_trimmed_range().cover(name?.text_trimmed_range()))
}
