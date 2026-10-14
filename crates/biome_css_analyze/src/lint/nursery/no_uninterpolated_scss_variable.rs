use crate::CssRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_factory::make;
use biome_css_syntax::{
    AnyCssCustomPropertyComponent, AnyCssValue, AnyScssExpression, AnyScssExpressionItem,
    AnyScssModuleMember, CssCustomIdentifier, CssCustomPropertyComponentList,
    CssCustomPropertyDelimiter, CssCustomPropertyValue, CssSyntaxNode, CssSyntaxToken,
    ScssInterpolatedString, ScssInterpolation, T,
};
use biome_diagnostics::Severity;
use biome_languages::CssFileSource;
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt, TextRange, WalkEvent};
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
        fix_kind: FixKind::Unsafe,
    }
}

pub struct RuleState {
    /// The `ns` and `.` of a module variable written as `ns.$name`.
    namespace: Option<(CssCustomIdentifier, CssCustomPropertyDelimiter)>,
    dollar: CssCustomPropertyDelimiter,
    name: CssCustomIdentifier,
}

impl RuleState {
    fn first_node(&self) -> &CssSyntaxNode {
        self.namespace
            .as_ref()
            .map_or(self.dollar.syntax(), |(namespace, _)| namespace.syntax())
    }

    fn range(&self) -> TextRange {
        TextRange::new(
            self.first_node().text_trimmed_range().start(),
            self.name.syntax().text_trimmed_range().end(),
        )
    }

    fn is_part(&self, node: &CssSyntaxNode) -> bool {
        node == self.dollar.syntax()
            || node == self.name.syntax()
            || self.namespace.as_ref().is_some_and(|(namespace, dot)| {
                node == namespace.syntax() || node == dot.syntax()
            })
    }
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
            signals.push(RuleState {
                namespace: module_namespace(&dollar),
                dollar,
                name,
            });
        }
        signals.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let variable = variable_text(state)?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range(),
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

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<CssRuleAction> {
        let list = state
            .dollar
            .syntax()
            .parent()
            .and_then(CssCustomPropertyComponentList::cast)?;
        let interpolation = AnyCssCustomPropertyComponent::ScssInterpolation(
            make_interpolation(state)?,
        );
        let mut inserted = false;
        let mut components = Vec::new();
        for component in list.iter() {
            if !state.is_part(component.syntax()) {
                components.push(component);
            } else if !inserted {
                components.push(interpolation.clone());
                inserted = true;
            }
        }

        let mut mutation = ctx.root().begin();
        mutation.replace_node_discard_trivia(
            list,
            make::css_custom_property_component_list(components),
        );
        Some(CssRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Wrap the variable in interpolation." }.to_owned(),
            mutation,
        ))
    }
}

fn variable_text(state: &RuleState) -> Option<String> {
    let name = state.name.value_token().ok()?;
    Some(match &state.namespace {
        Some((namespace, _)) => format!(
            "{}.${}",
            namespace.value_token().ok()?.text_trimmed(),
            name.text_trimmed()
        ),
        None => format!("${}", name.text_trimmed()),
    })
}

/// Builds `#{$name}` or `#{ns.$name}`, keeping the trivia that surrounded the variable.
fn make_interpolation(state: &RuleState) -> Option<ScssInterpolation> {
    let name_token = state.name.value_token().ok()?;
    let variable = make::scss_variable(
        without_trivia(&state.dollar.value().ok()?),
        make::css_identifier(without_trivia(&name_token)),
    );
    let value = match &state.namespace {
        Some((namespace, dot)) => {
            AnyCssValue::ScssModuleMemberAccess(make::scss_module_member_access(
                make::css_identifier(without_trivia(&namespace.value_token().ok()?)),
                without_trivia(&dot.value().ok()?),
                AnyScssModuleMember::ScssVariable(variable),
            ))
        }
        None => AnyCssValue::ScssVariable(variable),
    };
    let leading = state.first_node().first_token()?.leading_trivia();
    Some(make::scss_interpolation(
        CssSyntaxToken::new_detached(T![#], "#", [], [])
            .with_leading_trivia_pieces(leading.pieces()),
        CssSyntaxToken::new_detached(T!['{'], "{", [], []),
        AnyScssExpression::ScssExpression(make::scss_expression(
            make::scss_expression_item_list([AnyScssExpressionItem::AnyCssValue(value)]),
        )),
        CssSyntaxToken::new_detached(T!['}'], "}", [], [])
            .with_trailing_trivia_pieces(name_token.trailing_trivia().pieces()),
    ))
}

fn without_trivia(token: &CssSyntaxToken) -> CssSyntaxToken {
    token.with_leading_trivia([]).with_trailing_trivia([])
}

/// Returns `ns` and `.` when `dollar` is the `$` of a module variable written as `ns.$name`.
fn module_namespace(
    dollar: &CssCustomPropertyDelimiter,
) -> Option<(CssCustomIdentifier, CssCustomPropertyDelimiter)> {
    let dot = dollar
        .syntax()
        .prev_sibling()
        .and_then(CssCustomPropertyDelimiter::cast)
        .filter(|dot| dot.value().is_ok_and(|token| token.kind() == T![.]))
        .filter(|dot| are_adjacent(dot.syntax(), dollar.syntax()))?;
    let namespace = dot
        .syntax()
        .prev_sibling()
        .and_then(CssCustomIdentifier::cast)
        .filter(|namespace| are_adjacent(namespace.syntax(), dot.syntax()))?;
    Some((namespace, dot))
}

/// Whether `right` starts where `left` ends, with no whitespace or comments in between.
fn are_adjacent(left: &CssSyntaxNode, right: &CssSyntaxNode) -> bool {
    left.text_trimmed_range().end() == right.text_trimmed_range().start()
}
