use crate::CssRuleAction;
use crate::fonts::is_function_keyword;
use crate::utils::{is_custom_function, is_nested_custom_property_value};
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_factory::make;
use biome_css_syntax::{
    AnyCssCustomPropertyComponent, AnyCssUrlValue, AnyCssValue, AnyScssExpression,
    AnyScssExpressionItem, AnyScssModuleMember, CssCustomIdentifier,
    CssCustomPropertyComponentList, CssCustomPropertyDelimiter, CssCustomPropertyFunction,
    CssCustomPropertyValue, CssSyntaxNode, CssSyntaxToken, CssUrlFunction,
    ScssInterpolatedString, ScssInterpolation, ScssModuleMemberAccess, ScssVariable, T,
};
use biome_diagnostics::Severity;
use biome_languages::CssFileSource;
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt, TextRange, TextSize, WalkEvent};
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
    /// Inside `url()`, only a variable at the start of the URL is reported, such as `url($path)`. A
    /// `$` elsewhere in a URL is treated as part of the address.
    ///
    /// The fix wraps the variable in `#{}`. It isn't offered when the variable is an argument of a
    /// Sass function, such as `darken($base, 10%)`, because Sass would still output the call
    /// unchanged. Wrap the whole call in `#{}` instead. It isn't offered inside `url()` or in the
    /// conditions of `if()` either.
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

pub enum RuleState {
    Component(ComponentVariable),
    /// A variable the fix can't wrap, at the start of a URL as in `url($path)`, or in an `if()`
    /// condition as in `if(sass($flag): 1)`.
    Unfixable(TextRange),
}

/// A variable written as separate components, such as `$gap` or `ns.$gap`.
pub struct ComponentVariable {
    /// The `ns` and `.` of a module variable written as `ns.$name`.
    namespace: Option<(CssCustomIdentifier, CssCustomPropertyDelimiter)>,
    dollar: CssCustomPropertyDelimiter,
    name: CssCustomIdentifier,
}

impl Rule for NoUninterpolatedScssVariable {
    type Query = Ast<CssCustomPropertyValue>;
    type State = RuleState;
    type Signals = Box<[Self::State]>;
    type Options = NoUninterpolatedScssVariableOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let value = ctx.query().syntax();
        if !ctx.source_type::<CssFileSource>().is_scss() || is_nested_custom_property_value(value)
        {
            return Box::default();
        }

        let mut signals = Vec::new();
        let mut walk = value.preorder();
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
            if let Some(url) = CssUrlFunction::cast_ref(&node) {
                walk.skip_subtree();
                if let Some(range) = url_variable_range(&url) {
                    signals.push(RuleState::Unfixable(range));
                }
                continue;
            }
            // `if()` conditions are parsed as Sass expressions, which Sass doesn't evaluate in a
            // custom property either.
            if ScssVariable::can_cast(node.kind()) {
                let variable = node
                    .parent()
                    .filter(|parent| ScssModuleMemberAccess::can_cast(parent.kind()))
                    .unwrap_or(node);
                signals.push(RuleState::Unfixable(variable.text_trimmed_range()));
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
            signals.push(RuleState::Component(ComponentVariable {
                namespace: module_namespace(&dollar),
                dollar,
                name,
            }));
        }
        signals.into_boxed_slice()
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let range = match state {
            RuleState::Component(variable) => variable.range(),
            RuleState::Unfixable(range) => *range,
        };
        let value = ctx.query().syntax();
        let variable = value
            .text_trimmed()
            .slice(range - value.text_trimmed_range().start())
            .into_text();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "The Sass variable "<Emphasis>{variable.text()}</Emphasis>" isn't interpolated, so Sass outputs it unchanged."
                },
            )
            .note(markup! {
                "Sass only evaluates "<Emphasis>"#{...}"</Emphasis>" inside custom property values, so the browser receives the text "<Emphasis>{variable.text()}</Emphasis>" instead of the variable's value."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<CssRuleAction> {
        let RuleState::Component(state) = state else {
            return None;
        };
        if is_sass_call_argument(&state.dollar) {
            return None;
        }
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

impl ComponentVariable {
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

/// Builds `#{$name}` or `#{ns.$name}`, keeping the trivia that surrounded the variable.
fn make_interpolation(state: &ComponentVariable) -> Option<ScssInterpolation> {
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

/// Whether `dollar` is inside a call that Sass outputs unchanged, such as `darken($base, 10%)`
/// or `math.max($a, 1)`.
fn is_sass_call_argument(dollar: &CssCustomPropertyDelimiter) -> bool {
    dollar
        .syntax()
        .ancestors()
        .filter_map(CssCustomPropertyFunction::cast)
        .any(|function| {
            let is_module_call = function
                .syntax()
                .prev_sibling()
                .and_then(CssCustomPropertyDelimiter::cast)
                .is_some_and(|dot| {
                    dot.value().is_ok_and(|token| token.kind() == T![.])
                        && are_adjacent(dot.syntax(), function.syntax())
                });
            let Some(name) = function.name().ok().and_then(|name| name.value_token().ok()) else {
                return false;
            };
            let name = name.text_trimmed();
            is_module_call || !(is_custom_function(name) || is_function_keyword(name))
        })
}

/// Returns the range of the `$name` or `ns.$name` that starts the URL of `url`.
///
/// A `$` later in the URL, as in `url(https://example.com/a$b.svg)`, is part of the address.
fn url_variable_range(url: &CssUrlFunction) -> Option<TextRange> {
    let token = match url.value()? {
        AnyCssUrlValue::CssUrlValueRaw(raw) => raw.value_token().ok()?,
        AnyCssUrlValue::ScssInterpolatedUrlValue(value) => value
            .parts()
            .first()?
            .as_scss_url_text()?
            .value_token()
            .ok()?,
        _ => return None,
    };
    let text = token.text_trimmed();
    let namespace_len = identifier_len(text);
    let name = if namespace_len == 0 {
        text.strip_prefix('$')?
    } else {
        text[namespace_len..].strip_prefix(".$")?
    };
    let name_len = identifier_len(name);
    if name_len == 0 {
        return None;
    }
    let len = TextSize::try_from(text.len() - name.len() + name_len).ok()?;
    Some(TextRange::at(token.text_trimmed_range().start(), len))
}

/// Returns the byte length of the Sass identifier that starts `text`, which can't start with a
/// digit. Non-ASCII characters are identifier characters.
fn identifier_len(text: &str) -> usize {
    if text.starts_with(|c: char| c.is_ascii_digit()) {
        return 0;
    }
    text.find(|c: char| c.is_ascii() && !(c.is_ascii_alphanumeric() || matches!(c, '-' | '_')))
        .unwrap_or(text.len())
}

/// Whether `right` starts where `left` ends, with no whitespace or comments in between.
fn are_adjacent(left: &CssSyntaxNode, right: &CssSyntaxNode) -> bool {
    left.text_trimmed_range().end() == right.text_trimmed_range().start()
}
