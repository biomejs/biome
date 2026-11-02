use crate::CssRuleAction;
use crate::fonts::is_function_keyword;
use crate::utils::{
    is_custom_function, is_scss_built_in_function, scss_function_declarations,
    scss_function_names_equal,
};
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_factory::make;
use biome_css_syntax::{
    AnyCssCustomPropertyComponent, AnyCssRoot, AnyCssUrlValue, AnyCssValue, AnyScssExpression,
    AnyScssExpressionItem, AnyScssModuleMember, CssCustomIdentifier,
    CssCustomPropertyComponentList, CssCustomPropertyDelimiter, CssCustomPropertyFunction,
    CssCustomPropertyValue, CssIfFunction, CssIfSassTest, CssSyntaxNode, CssSyntaxToken,
    CssUrlFunction, ScssInterpolatedString, ScssInterpolation, ScssModuleMemberAccess,
    ScssVariable, T,
};
use biome_diagnostics::Severity;
use biome_languages::CssFileSource;
use biome_rowan::{
    AstNode, AstNodeList, AstSeparatedList, BatchMutationExt, TextRange, TextSize, WalkEvent,
};
use biome_rule_options::no_missing_scss_interpolation::NoMissingScssInterpolationOptions;

declare_lint_rule! {
    /// Disallow Sass variables and function calls in custom property values without `#{}` interpolation.
    ///
    /// Sass doesn't evaluate the value of a custom property (a property whose name starts with
    /// `--`), except for the parts wrapped in `#{}`. A variable or function call written there
    /// without interpolation is copied into the CSS as plain text, so the browser receives `$gap`
    /// instead of the variable's value, or `darken(#fff, 10%)` instead of a color. Sass reports no
    /// error.
    ///
    /// This applies wherever custom properties appear, including `@supports` and
    /// `@container style()` conditions. The rule only checks SCSS files.
    ///
    /// The rule reports these function calls:
    ///
    /// - module functions, such as `math.div()`;
    /// - Sass built-in functions, such as `darken()` and `map-get()`;
    /// - functions declared with `@function` in the same file, even when CSS has a function of the
    ///   same name, such as `rem()`;
    /// - `if()` calls with a `sass()` condition, such as `if(sass($flag): 1; else: 2)`.
    ///
    /// A reported call covers the variables inside it, so they aren't reported again. Other calls
    /// aren't reported, because they can be CSS functions or text that JavaScript reads, such as
    /// `spring(1, 100, 10)`. Variables inside them are still reported.
    ///
    /// Inside `url()`, only a variable at the start of the URL is reported, such as `url($path)`. A
    /// `$` elsewhere in a URL is treated as part of the address.
    ///
    /// The fix wraps a variable in `#{}`. It isn't offered for function calls, for a variable that
    /// starts a URL, such as `url($path)`, or for a variable in an `if()` condition. It also isn't
    /// offered for a variable inside a call that is neither a known CSS function nor reported by the
    /// rule, such as `bpx($size)`, because that call could be a Sass function from another file, and
    /// then the whole call needs `#{}`.
    ///
    /// :::note
    /// Sass built-in functions that share their name with a CSS function, such as `saturate()`,
    /// `invert()`, `min()` and `max()`, aren't reported, because the rule can't tell the two apart.
    /// Call them through their module, such as `math.max()`, so the rule can check them.
    ///
    /// Functions from other files that are called without a module name, after `@use ... as *` or
    /// `@import`, aren't reported either, because the rule only reads the current file.
    /// :::
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
    /// $gap: 8px;
    ///
    /// :root {
    ///   --gap: #{$gap};
    ///   --half: #{math.div(8px, 2)};
    ///   --label: "$gap";
    ///   --width: calc(100% - var(--gutter));
    /// }
    ///
    /// a {
    ///   margin: $gap;
    /// }
    /// ```
    ///
    pub NoMissingScssInterpolation {
        version: "2.6.5",
        name: "noMissingScssInterpolation",
        language: "css",
        recommended: false,
        severity: Severity::Error,
        sources: &[RuleSource::StylelintScss("dollar-variable-no-missing-interpolation").inspired()],
        fix_kind: FixKind::Unsafe,
    }
}

pub enum RuleState {
    Component(ComponentVariable),
    /// A variable the fix can't wrap, at the start of a URL as in `url($path)`, or in an `if()`
    /// condition as in `if(media(width > $a): 1)`.
    UnfixableVariable(TextRange),
    /// The name of a Sass call, including the `ns.` of a module function, or the `if` of an `if()`
    /// with a `sass()` condition.
    Function(TextRange),
}

/// A variable written as separate components, such as `$gap` or `ns.$gap`.
pub struct ComponentVariable {
    /// The `ns` and `.` of a module variable written as `ns.$name`.
    namespace: Option<(CssCustomIdentifier, CssCustomPropertyDelimiter)>,
    dollar: CssCustomPropertyDelimiter,
    name: CssCustomIdentifier,
}

impl Rule for NoMissingScssInterpolation {
    type Query = Ast<AnyCssRoot>;
    type State = RuleState;
    type Signals = Box<[Self::State]>;
    type Options = NoMissingScssInterpolationOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx.source_type::<CssFileSource>().is_scss() {
            return Box::default();
        }

        let root = ctx.query().syntax();
        let mut declared_functions = None;
        // The custom property value being checked. Values nested inside it, such as `if()`
        // branches, are checked as part of it.
        let mut value = None;
        let mut signals = Vec::new();
        let mut walk = root.preorder();
        while let Some(event) = walk.next() {
            let node = match event {
                WalkEvent::Enter(node) => node,
                WalkEvent::Leave(node) => {
                    if value.as_ref() == Some(&node) {
                        value = None;
                    }
                    continue;
                }
            };
            if value.is_none() {
                if CssCustomPropertyValue::can_cast(node.kind()) {
                    value = Some(node);
                }
                continue;
            }
            if ScssInterpolation::can_cast(node.kind())
                || ScssInterpolatedString::can_cast(node.kind())
            {
                walk.skip_subtree();
                continue;
            }
            if let Some(url) = CssUrlFunction::cast_ref(&node) {
                walk.skip_subtree();
                if let Some(range) = url_variable_range(&url) {
                    signals.push(RuleState::UnfixableVariable(range));
                }
                continue;
            }
            // An `if()` with a `sass()` condition is a Sass expression as a whole, so wrapping a
            // variable in one of its branches still leaves the `if()` unevaluated.
            if let Some(function) = CssIfFunction::cast_ref(&node).filter(has_sass_condition) {
                walk.skip_subtree();
                if let Ok(token) = function.if_token() {
                    signals.push(RuleState::Function(token.text_trimmed_range()));
                }
                continue;
            }
            if let Some(function) = CssCustomPropertyFunction::cast_ref(&node) {
                let Ok(name) = function.name() else {
                    continue;
                };
                let namespace = module_namespace(&function.clone().into());
                if namespace.is_none() {
                    let Ok(token) = name.value_token() else {
                        continue;
                    };
                    let declared = declared_functions.get_or_insert_with(|| {
                        scss_function_declarations(root).collect::<Vec<_>>()
                    });
                    if !is_sass_function(token.text_trimmed(), declared) {
                        continue;
                    }
                }
                // Calls and variables nested inside a reported call are part of the same mistake.
                walk.skip_subtree();
                let start = namespace
                    .as_ref()
                    .map_or(name.syntax(), |(namespace, _)| namespace.syntax())
                    .text_trimmed_range()
                    .start();
                signals.push(RuleState::Function(TextRange::new(
                    start,
                    name.syntax().text_trimmed_range().end(),
                )));
                continue;
            }
            // `if()` conditions are parsed as Sass expressions, which Sass doesn't evaluate in a
            // custom property either.
            if ScssVariable::can_cast(node.kind()) {
                let variable = node
                    .parent()
                    .filter(|parent| ScssModuleMemberAccess::can_cast(parent.kind()))
                    .unwrap_or(node);
                signals.push(RuleState::UnfixableVariable(variable.text_trimmed_range()));
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
                namespace: module_namespace(&dollar.clone().into()),
                dollar,
                name,
            }));
        }
        signals.into_boxed_slice()
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let range = match state {
            RuleState::Component(variable) => variable.range(),
            RuleState::UnfixableVariable(range) | RuleState::Function(range) => *range,
        };
        let root = ctx.query().syntax();
        let text = root
            .text_trimmed()
            .slice(range - root.text_trimmed_range().start())
            .into_text();
        if matches!(state, RuleState::Function(_)) {
            return Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    range,
                    markup! {
                        "The function "<Emphasis>{text.text()}"()"</Emphasis>" isn't interpolated, so Sass outputs it unchanged."
                    },
                )
                .note(markup! {
                    "Sass only evaluates "<Emphasis>"#{...}"</Emphasis>" inside custom property values, so the browser receives this call as plain text instead of its result."
                })
                .note(markup! {
                    "Wrap the call in "<Emphasis>"#{...}"</Emphasis>" so Sass evaluates it."
                }),
            );
        }
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "The Sass variable "<Emphasis>{text.text()}</Emphasis>" isn't interpolated, so Sass outputs it unchanged."
                },
            )
            .note(markup! {
                "Sass only evaluates "<Emphasis>"#{...}"</Emphasis>" inside custom property values, so the browser receives the text "<Emphasis>{text.text()}</Emphasis>" instead of the variable's value."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<CssRuleAction> {
        let RuleState::Component(state) = state else {
            return None;
        };
        if is_unknown_call_argument(&state.dollar) {
            return None;
        }
        let list = state
            .dollar
            .syntax()
            .parent()
            .and_then(CssCustomPropertyComponentList::cast)?;
        let interpolation =
            AnyCssCustomPropertyComponent::ScssInterpolation(make_interpolation(state)?);
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
            || self
                .namespace
                .as_ref()
                .is_some_and(|(namespace, dot)| node == namespace.syntax() || node == dot.syntax())
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
        AnyScssExpression::ScssExpression(make::scss_expression(make::scss_expression_item_list(
            [AnyScssExpressionItem::AnyCssValue(value)],
        ))),
        CssSyntaxToken::new_detached(T!['}'], "}", [], [])
            .with_trailing_trivia_pieces(name_token.trailing_trivia().pieces()),
    ))
}

fn without_trivia(token: &CssSyntaxToken) -> CssSyntaxToken {
    token.with_leading_trivia([]).with_trailing_trivia([])
}

/// Returns `ns` and `.` when `member` is the `$` of a module variable written as `ns.$name`, or
/// the call of a module function written as `ns.name()`.
fn module_namespace(
    member: &AnyCssCustomPropertyComponent,
) -> Option<(CssCustomIdentifier, CssCustomPropertyDelimiter)> {
    let dot = member
        .syntax()
        .prev_sibling()
        .and_then(CssCustomPropertyDelimiter::cast)
        .filter(|dot| dot.value().is_ok_and(|token| token.kind() == T![.]))
        .filter(|dot| are_adjacent(dot.syntax(), member.syntax()))?;
    let namespace = dot
        .syntax()
        .prev_sibling()
        .and_then(CssCustomIdentifier::cast)
        .filter(|namespace| are_adjacent(namespace.syntax(), dot.syntax()))?;
    Some((namespace, dot))
}

/// Whether a call to `name`, written without a module namespace, is a Sass built-in or one of the
/// `declared` function names. Built-ins whose names CSS also uses, such as `rgb` and `max`, count
/// as CSS, but a declared function shadows the CSS function of the same name, as `rem()` often does.
fn is_sass_function(name: &str, declared: &[CssSyntaxToken]) -> bool {
    !is_custom_function(name)
        && ((!is_function_keyword(name) && is_scss_built_in_function(name))
            || declared
                .iter()
                .any(|declared| scss_function_names_equal(declared.text_trimmed(), name)))
}

/// Whether a branch condition of `function` contains a `sass()` test, as in
/// `if(sass($flag): 1; else: 2)`. A test inside another `if()`, such as one nested in a `style()`
/// query, belongs to that `if()` instead.
fn has_sass_condition(function: &CssIfFunction) -> bool {
    function
        .css_if_branch_list()
        .iter()
        .filter_map(|branch| branch.ok()?.as_css_if_branch()?.condition().ok())
        .flat_map(|condition| condition.into_syntax().descendants())
        .filter_map(CssIfSassTest::cast)
        .any(|test| {
            test.syntax()
                .ancestors()
                .find_map(CssIfFunction::cast)
                .as_ref()
                == Some(function)
        })
}

/// Whether `dollar` is an argument of a call that is neither a known CSS function nor a function
/// the rule identifies as Sass, such as `bpx($a)`.
///
/// Calls identified as Sass are reported as a whole, so their arguments never reach this check.
fn is_unknown_call_argument(dollar: &CssCustomPropertyDelimiter) -> bool {
    dollar
        .syntax()
        .ancestors()
        .filter_map(CssCustomPropertyFunction::cast)
        .any(|function| {
            let Some(name) = function
                .name()
                .ok()
                .and_then(|name| name.value_token().ok())
            else {
                return false;
            };
            let name = name.text_trimmed();
            !(is_custom_function(name) || is_function_keyword(name))
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
