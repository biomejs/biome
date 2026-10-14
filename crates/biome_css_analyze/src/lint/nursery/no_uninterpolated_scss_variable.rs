use biome_analyze::{Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_css_syntax::{
    CssCustomIdentifier, CssCustomPropertyDelimiter, CssCustomPropertyValue, CssSyntaxNode,
    ScssInterpolatedString, ScssInterpolation, T,
};
use biome_diagnostics::Severity;
use biome_languages::CssFileSource;
use biome_rowan::{AstNode, TextRange, WalkEvent};
use biome_rule_options::no_uninterpolated_scss_variable::NoUninterpolatedScssVariableOptions;

declare_lint_rule! {
    /// Disallow Sass variables in custom property values without `#{}` interpolation.
    ///
    /// Sass doesn't evaluate the value of a custom property (a property whose name starts with
    /// `--`), except for the parts wrapped in `#{}`. A variable written there without
    /// interpolation is copied into the CSS as plain text, so the browser receives `$gap` instead
    /// of the variable's value. Sass reports no error.
    ///
    /// This applies wherever custom properties appear, including `@supports` and
    /// `@container style()` conditions. The rule only checks SCSS files.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```scss,expect_diagnostic
    /// $gap: 8px;
    ///
    /// :root {
    ///   --gap: $gap;
    /// }
    /// ```
    ///
    /// ```scss,expect_diagnostic
    /// @use "sass:math";
    ///
    /// :root {
    ///   --pi: math.$pi;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```scss
    /// $gap: 8px;
    ///
    /// :root {
    ///   --gap: #{$gap};
    ///   --label: "$gap";
    /// }
    ///
    /// a {
    ///   margin: $gap;
    /// }
    /// ```
    ///
    pub NoUninterpolatedScssVariable {
        version: "next",
        name: "noUninterpolatedScssVariable",
        language: "css",
        recommended: false,
        severity: Severity::Error,
    }
}

pub struct RuleState {
    namespace: Option<CssCustomIdentifier>,
    name: CssCustomIdentifier,
    range: TextRange,
}

impl Rule for NoUninterpolatedScssVariable {
    type Query = Ast<CssCustomPropertyValue>;
    type State = RuleState;
    type Signals = Box<[Self::State]>;
    type Options = NoUninterpolatedScssVariableOptions;

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
            let Some(dollar) = CssCustomPropertyDelimiter::cast(node) else {
                continue;
            };
            if !dollar.value().is_ok_and(|token| token.kind() == T![$]) {
                continue;
            }
            let Some(name) = dollar
                .syntax()
                .next_sibling()
                .and_then(CssCustomIdentifier::cast)
                .filter(|name| are_adjacent(dollar.syntax(), name.syntax()))
            else {
                continue;
            };
            let namespace = module_namespace(&dollar);
            let start = namespace
                .as_ref()
                .map_or(dollar.syntax(), |namespace| namespace.syntax())
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
        let variable = match &state.namespace {
            Some(namespace) => format!(
                "{}.${}",
                namespace.value_token().ok()?.text_trimmed(),
                name.text_trimmed()
            ),
            None => format!("${}", name.text_trimmed()),
        };
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "The Sass variable "<Emphasis>{variable}</Emphasis>" isn't interpolated, so Sass outputs it unchanged."
                },
            )
            .note(markup! {
                "Sass only evaluates "<Emphasis>"#{...}"</Emphasis>" inside custom property values, so the browser receives the text "<Emphasis>{variable}</Emphasis>" instead of the variable's value."
            })
            .note(markup! {
                "Wrap the variable in interpolation: "<Emphasis>"#{"{variable}"}"</Emphasis>"."
            }),
        )
    }
}

/// Returns `ns` when `dollar` is the `$` of a module variable written as `ns.$name`.
fn module_namespace(dollar: &CssCustomPropertyDelimiter) -> Option<CssCustomIdentifier> {
    let dot = dollar
        .syntax()
        .prev_sibling()
        .and_then(CssCustomPropertyDelimiter::cast)
        .filter(|dot| dot.value().is_ok_and(|token| token.kind() == T![.]))
        .filter(|dot| are_adjacent(dot.syntax(), dollar.syntax()))?;
    dot.syntax()
        .prev_sibling()
        .and_then(CssCustomIdentifier::cast)
        .filter(|namespace| are_adjacent(namespace.syntax(), dot.syntax()))
}

/// Whether `right` starts where `left` ends, with no whitespace or comments in between.
fn are_adjacent(left: &CssSyntaxNode, right: &CssSyntaxNode) -> bool {
    left.text_trimmed_range().end() == right.text_trimmed_range().start()
}
