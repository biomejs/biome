use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, AnyJsxTag, JsxExpressionChild, JsxText,
};
use biome_rowan::{AstNode, TextRange, TokenText, declare_node_union};
use biome_rule_options::no_react_native_raw_text::NoReactNativeRawTextOptions;

declare_lint_rule! {
    /// Disallow raw text outside `<Text>` components in React Native.
    ///
    /// In React Native, every string rendered in the UI must be wrapped in a `<Text>`
    /// component. Rendering text directly inside containers such as `<View>` throws at
    /// runtime on native platforms.
    ///
    /// By default, the following element names are treated as valid text containers:
    /// `Text`, `TSpan`, `StyledText`, and `Animated.Text`. Additional components can be
    /// whitelisted through the `skip` option.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <View>some text</View>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <View>{'some text'}</View>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// const text = 'some text';
    /// <View>{`${text}`}</View>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <View><Text>some text</Text></View>
    /// ```
    ///
    /// ```jsx
    /// <View><Text>{'some text'}</Text></View>
    /// ```
    ///
    /// ## Options
    ///
    /// ### `skip`
    ///
    /// An array of additional component names that are allowed to contain raw text.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "skip": ["Title"]
    ///     }
    /// }
    /// ```
    ///
    /// ```jsx,use_options
    /// const Title = ({ children }) => <Text>{children}</Text>;
    /// <Title>This is the title</Title>;
    /// ```
    ///
    pub NoReactNativeRawText {
        version: "2.4.13",
        name: "noReactNativeRawText",
        language: "jsx",
        sources: &[RuleSource::EslintReactNative("no-raw-text").same()],
        domains: &[RuleDomain::ReactNative],
        recommended: true,
        severity: Severity::Error,
    }
}

declare_node_union! {
    pub AnyRawTextNode = JsxText | JsxExpressionChild
}

pub enum RawTextKind {
    /// Non-empty raw text (JSX text or literal string expression).
    Text(TokenText),
    /// Whitespace-only raw text.
    Whitespace,
    /// Template literal whose first interpolation is an identifier.
    TemplateIdentifier(TokenText),
    /// Template literal without an identifier interpolation to display.
    TemplateLiteral,
}

pub struct RuleState {
    kind: RawTextKind,
    range: TextRange,
}

const DEFAULT_ALLOWED_ELEMENTS: &[&str] = &["Text", "TSpan", "StyledText", "Animated.Text"];

impl Rule for NoReactNativeRawText {
    type Query = Ast<AnyRawTextNode>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = NoReactNativeRawTextOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let kind = node.extract_text()?;

        if is_wrapped_in_allowed_element(node.syntax(), ctx.options()) {
            return None;
        }

        Some(RuleState {
            kind,
            range: node.range(),
        })
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = match &state.kind {
            RawTextKind::Text(value) => RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "Raw text ("{value.text()}") cannot be used outside of a "<Emphasis>"<Text>"</Emphasis>" tag."
                },
            ),
            RawTextKind::Whitespace => RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "Whitespace cannot be used outside of a "<Emphasis>"<Text>"</Emphasis>" tag."
                },
            ),
            RawTextKind::TemplateIdentifier(name) => RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "Raw text (TemplateLiteral: "{name.text()}") cannot be used outside of a "<Emphasis>"<Text>"</Emphasis>" tag."
                },
            ),
            RawTextKind::TemplateLiteral => RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "Raw text (TemplateLiteral) cannot be used outside of a "<Emphasis>"<Text>"</Emphasis>" tag."
                },
            ),
        };

        Some(
            diagnostic
                .note(markup! {
                    "In React Native, strings must be rendered within a "<Emphasis>"<Text>"</Emphasis>" component, otherwise the application throws at runtime."
                })
                .note(markup! {
                    "Wrap the text in a "<Emphasis>"<Text>"</Emphasis>" component, or add the containing component to the "<Emphasis>"skip"</Emphasis>" option."
                }),
        )
    }
}

impl AnyRawTextNode {
    /// Classifies the raw text carried by a JSX child node so the rule can
    /// report a targeted diagnostic.
    ///
    /// Returns `None` when the node carries nothing renderable. In particular,
    /// JSX text that only contains line breaks (optionally mixed with spaces)
    /// is stripped by React runtime: it represents formatting whitespace
    /// between elements rather than rendered content, so there is nothing for
    /// the rule to flag. Space-only text, on the other hand, is preserved at
    /// runtime and is still reported as [`RawTextKind::Whitespace`].
    fn extract_text(&self) -> Option<RawTextKind> {
        match self {
            Self::JsxText(text) => {
                let token = text.value_token().ok()?;
                let token_text = token.token_text_trimmed();
                let value = token_text.text();
                if has_only_line_breaks(value) {
                    return None;
                }
                if value.trim().is_empty() {
                    Some(RawTextKind::Whitespace)
                } else {
                    Some(RawTextKind::Text(token_text))
                }
            }
            Self::JsxExpressionChild(child) => {
                let expression = child.expression()?;
                match expression {
                    AnyJsExpression::AnyJsLiteralExpression(
                        AnyJsLiteralExpression::JsStringLiteralExpression(string),
                    ) => {
                        let inner = string.inner_string_text().ok()?;
                        if inner.text().trim().is_empty() {
                            Some(RawTextKind::Whitespace)
                        } else {
                            Some(RawTextKind::Text(inner))
                        }
                    }
                    AnyJsExpression::JsTemplateExpression(template) => {
                        if template.tag().is_some() {
                            return None;
                        }
                        let first_expression =
                            template.elements().into_iter().find_map(|element| {
                                element.as_js_template_element()?.expression().ok()
                            });

                        if let Some(AnyJsExpression::JsIdentifierExpression(identifier)) =
                            first_expression
                        {
                            let name = identifier.name().ok()?;
                            let token = name.value_token().ok()?;
                            Some(RawTextKind::TemplateIdentifier(token.token_text_trimmed()))
                        } else {
                            Some(RawTextKind::TemplateLiteral)
                        }
                    }
                    _ => None,
                }
            }
        }
    }
}

/// Strips ASCII spaces from `value` and returns `true` when only line-break
/// characters remain. Mirrors the upstream ESLint regex
/// `/^[\r\n\t\f\v]+$/.test(value.replace(/ /g, ''))`.
fn has_only_line_breaks(value: &str) -> bool {
    let stripped: String = value.chars().filter(|c| *c != ' ').collect();
    !stripped.is_empty()
        && stripped
            .chars()
            .all(|c| matches!(c, '\r' | '\n' | '\t' | '\u{000C}' | '\u{000B}'))
}

fn is_wrapped_in_allowed_element(
    node: &biome_js_syntax::JsSyntaxNode,
    options: &NoReactNativeRawTextOptions,
) -> bool {
    node.ancestors().filter_map(AnyJsxTag::cast).any(|tag| {
        DEFAULT_ALLOWED_ELEMENTS
            .iter()
            .any(|allowed| tag.matches_name(allowed))
            || options
                .skip
                .iter()
                .flat_map(|skip| skip.iter())
                .any(|allowed| tag.matches_name(allowed))
    })
}
