use crate::fonts::is_function_keyword;
use crate::utils::is_custom_function;
use biome_analyze::{Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_css_syntax::{
    CssCustomIdentifier, CssCustomPropertyComponentList, CssCustomPropertyDelimiter,
    CssCustomPropertyFunction, CssCustomPropertyParenthesizedBlock, CssCustomPropertyValue,
    CssSyntaxNode, ScssInterpolatedString, ScssInterpolation, T,
};
use biome_diagnostics::Severity;
use biome_languages::CssFileSource;
use biome_rowan::{AstNode, TextRange, WalkEvent};
use biome_rule_options::no_uninterpolated_scss_function::NoUninterpolatedScssFunctionOptions;

declare_lint_rule! {
    /// Disallow Sass function calls in custom property values without `#{}` interpolation.
    ///
    /// Sass doesn't evaluate the value of a custom property (a property whose name starts with
    /// `--`), except for the parts wrapped in `#{}`. A Sass function called there without
    /// interpolation is copied into the CSS as plain text, so the browser receives
    /// `darken(#fff, 10%)` instead of a color. Sass reports no error.
    ///
    /// The rule reports module functions such as `math.div()`, and any other function that isn't
    /// a known CSS function. That includes Sass built-in functions, functions declared with
    /// `@function`, and functions made available by `@use` or `@import`. Known CSS functions, such
    /// as `calc()`, `var()` and `rgb()`, and custom functions whose names start with `--` are
    /// allowed.
    ///
    /// This applies wherever custom properties appear, including `@supports` and
    /// `@container style()` conditions. The rule only checks SCSS files.
    ///
    /// :::note
    /// Sass functions that share their name with a CSS function, such as `saturate()`, `invert()`,
    /// `min()` and `max()`, aren't reported, because the rule can't tell the two apart. Call them
    /// through their module, such as `math.max()`, so the rule can check them.
    ///
    /// A CSS function that Biome doesn't know yet is reported. Suppress the diagnostic for that
    /// call if this happens.
    /// :::
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```scss,expect_diagnostic
    /// :root {
    ///   --muted: darken(#fff, 10%);
    /// }
    /// ```
    ///
    /// ```scss,expect_diagnostic
    /// @use "sass:math";
    ///
    /// :root {
    ///   --half: math.div(8px, 2);
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```scss
    /// @use "sass:math";
    ///
    /// :root {
    ///   --half: #{math.div(8px, 2)};
    ///   --shadow: rgb(0 0 0 / 50%);
    ///   --width: calc(100% - var(--gutter));
    /// }
    /// ```
    ///
    pub NoUninterpolatedScssFunction {
        version: "next",
        name: "noUninterpolatedScssFunction",
        language: "css",
        recommended: false,
        severity: Severity::Warning,
    }
}

pub struct RuleState {
    namespace: Option<CssCustomIdentifier>,
    name: CssCustomIdentifier,
    range: TextRange,
}

impl Rule for NoUninterpolatedScssFunction {
    type Query = Ast<CssCustomPropertyValue>;
    type State = RuleState;
    type Signals = Box<[Self::State]>;
    type Options = NoUninterpolatedScssFunctionOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx.source_type::<CssFileSource>().is_scss() {
            return Box::default();
        }

        let mut signals = Vec::new();
        let mut walk = ctx.query().syntax().preorder();
        while let Some(event) = walk.next() {
            let WalkEvent::Enter(node) = event else {
                continue;
            };
            if ScssInterpolation::can_cast(node.kind())
                || ScssInterpolatedString::can_cast(node.kind())
            {
                walk.skip_subtree();
                continue;
            }
            let Some(function) = CssCustomPropertyFunction::cast(node) else {
                continue;
            };
            let Ok(name) = function.name() else {
                continue;
            };
            let namespace = module_namespace(&function);
            if namespace.is_none() {
                let Ok(token) = name.value_token() else {
                    continue;
                };
                let text = token.text_trimmed();
                // The body of `url()` is a URL, not a list of function calls.
                if text.eq_ignore_ascii_case("url") {
                    walk.skip_subtree();
                    continue;
                }
                // The arguments of an `if()` condition are condition syntax, such as
                // `selector(:has(a))` or `font-tech(color-COLRv1)`, not values.
                if is_if_condition(&function, text) {
                    walk.skip_subtree();
                    continue;
                }
                // CSS `if()` is parsed as its own node in regular values, so it isn't in the
                // list of known functions.
                if is_custom_function(text)
                    || is_function_keyword(text)
                    || text.eq_ignore_ascii_case("if")
                {
                    continue;
                }
            }
            // Calls nested inside a reported call are part of the same mistake.
            walk.skip_subtree();
            let start = namespace
                .as_ref()
                .map_or(name.syntax(), |namespace| namespace.syntax())
                .text_trimmed_range()
                .start();
            signals.push(RuleState {
                range: TextRange::new(start, name.syntax().text_trimmed_range().end()),
                namespace,
                name,
            });
        }
        signals.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let name = state.name.value_token().ok()?;
        let namespace = match &state.namespace {
            Some(namespace) => Some(namespace.value_token().ok()?),
            None => None,
        };
        let namespace = namespace.as_ref().map_or("", |token| token.text_trimmed());
        let dot = if namespace.is_empty() { "" } else { "." };
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "The function "<Emphasis>{namespace}{dot}{name.text_trimmed()}"()"</Emphasis>" isn't interpolated, so Sass outputs it unchanged."
                },
            )
            .note(markup! {
                "Sass only evaluates "<Emphasis>"#{...}"</Emphasis>" inside custom property values, so the browser receives this call as plain text instead of its result."
            })
            .note(markup! {
                "Wrap the call in "<Emphasis>"#{...}"</Emphasis>" so Sass evaluates it."
            }),
        )
    }
}

/// Whether `function` is a `style()`, `media()` or `supports()` condition of CSS `if()`.
///
/// Conditions can be grouped in parentheses, as in `if(not (style(--a: 1)): x; else: y)`, so
/// parenthesized blocks between the condition and `if()` are skipped.
fn is_if_condition(function: &CssCustomPropertyFunction, name: &str) -> bool {
    ["style", "media", "supports"]
        .iter()
        .any(|condition| name.eq_ignore_ascii_case(condition))
        && function
            .syntax()
            .ancestors()
            .skip(1)
            .find(|node| {
                !CssCustomPropertyComponentList::can_cast(node.kind())
                    && !CssCustomPropertyParenthesizedBlock::can_cast(node.kind())
            })
            .and_then(CssCustomPropertyFunction::cast)
            .and_then(|parent| parent.name().ok()?.value_token().ok())
            .is_some_and(|parent| parent.text_trimmed().eq_ignore_ascii_case("if"))
}

/// Returns `ns` when `function` is a module function written as `ns.name()`.
fn module_namespace(function: &CssCustomPropertyFunction) -> Option<CssCustomIdentifier> {
    let dot = function
        .syntax()
        .prev_sibling()
        .and_then(CssCustomPropertyDelimiter::cast)
        .filter(|dot| dot.value().is_ok_and(|token| token.kind() == T![.]))
        .filter(|dot| are_adjacent(dot.syntax(), function.syntax()))?;
    dot.syntax()
        .prev_sibling()
        .and_then(CssCustomIdentifier::cast)
        .filter(|namespace| are_adjacent(namespace.syntax(), dot.syntax()))
}

/// Whether `right` starts where `left` ends, with no whitespace or comments in between.
fn are_adjacent(left: &CssSyntaxNode, right: &CssSyntaxNode) -> bool {
    left.text_trimmed_range().end() == right.text_trimmed_range().start()
}
