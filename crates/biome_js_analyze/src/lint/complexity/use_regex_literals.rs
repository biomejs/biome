use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make::js_regex_literal_expression;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsLiteralExpression, JsCallArguments,
    JsComputedMemberExpression, JsNewOrCallExpression, JsSyntaxKind, JsSyntaxToken,
    global_identifier, static_value::StaticValue,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, SyntaxError, TextRange, TokenText};
use biome_rule_options::use_regex_literals::UseRegexLiteralsOptions;

use crate::{JsRuleAction, services::semantic::Semantic};

declare_lint_rule! {
    /// Enforce the use of the regular expression literals instead of the RegExp constructor if possible.
    ///
    /// There are two ways to create a regular expression:
    /// - Regular expression literals, e.g., `/abc/u`.
    /// - The RegExp constructor function, e.g., `new RegExp("abc", "u")` .
    ///
    /// The constructor function is particularly useful when you want to dynamically generate the pattern,
    /// because it takes string arguments.
    ///
    /// Using regular expression literals avoids some escaping required in a string literal,
    /// and are easier to analyze statically.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// new RegExp("abc", "u");
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// /abc/u;
    ///
    /// new RegExp("abc", flags);
    /// ```
    ///
    pub UseRegexLiterals {
        version: "1.3.0",
        name: "useRegexLiterals",
        language: "js",
        sources: &[RuleSource::Eslint("prefer-regex-literals").same()],
        recommended: true,
        severity: Severity::Warning,
        fix_kind: FixKind::Safe,
    }
}

pub struct UseRegexLiteralsState {
    pattern: StaticValue,
    flags: Option<StaticValue>,
    string_kind: StringKind,
}

impl Rule for UseRegexLiterals {
    type Query = Semantic<JsNewOrCallExpression>;
    type State = UseRegexLiteralsState;
    type Signals = Option<Self::State>;
    type Options = UseRegexLiteralsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();

        let (callee, arguments) = parse_node(node)?;
        if !is_regexp_object(callee, model) {
            return None;
        }

        let args = arguments.args();
        if args.len() > 2 {
            return None;
        }
        let mut args = args.iter();

        let pattern = args.next()?;
        let (pattern, string_kind) = extract_valid_pattern(pattern, model)?;

        let flags = match args.next() {
            Some(flags) => {
                let flags = extract_valid_flags(flags)?;
                Some(flags)
            }
            None => None,
        };
        Some(UseRegexLiteralsState {
            pattern,
            flags,
            string_kind,
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            markup! {
                "Use a regular expression literal instead of the "<Emphasis>"RegExp"</Emphasis>" constructor."
            },
        ).note(markup! {
            "Regular expression literals avoid some escaping required in a string literal, and are easier to analyze statically."
        }))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let prev = match ctx.query() {
            JsNewOrCallExpression::JsNewExpression(node) => AnyJsExpression::from(node.clone()),
            JsNewOrCallExpression::JsCallExpression(node) => AnyJsExpression::from(node.clone()),
        };

        let regex = create_regex(
            state.pattern.text(),
            state
                .flags
                .as_ref()
                .unwrap_or(&StaticValue::EmptyString(TextRange::empty(0.into()))),
            state.string_kind,
        );
        let token = JsSyntaxToken::new_detached(JsSyntaxKind::JS_REGEX_LITERAL, &regex, [], []);
        let next = AnyJsExpression::AnyJsLiteralExpression(AnyJsLiteralExpression::from(
            js_regex_literal_expression(token),
        ));
        let mut mutation = ctx.root().begin();
        mutation.replace_node(prev, next);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
               "Use a "<Emphasis>"literal notation"</Emphasis>" instead."
            }
            .to_owned(),
            mutation,
        ))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum StringKind {
    Literal,
    Template,
    RawTemplate,
}

fn extract_valid_pattern(
    pattern: Result<AnyJsCallArgument, SyntaxError>,
    model: &SemanticModel,
) -> Option<(StaticValue, StringKind)> {
    let Ok(AnyJsCallArgument::AnyJsExpression(expr)) = pattern else {
        return None;
    };
    let string_kind = if let Some(template_expr) = expr.as_js_template_expression() {
        // Only accept `String.raw()` or untagged template strings.
        if let Some(tag) = template_expr.tag() {
            let (object, member) = match tag.omit_parentheses() {
                AnyJsExpression::JsStaticMemberExpression(expr) => {
                    let object = expr.object().ok()?;
                    let member = expr.member().ok()?;
                    (object, member.value_token().ok()?.token_text_trimmed())
                }
                AnyJsExpression::JsComputedMemberExpression(expr) => {
                    let object = expr.object().ok()?;
                    let member = extract_inner_text(&expr)?;
                    (object, member)
                }
                _ => return None,
            };
            let (reference, name) = global_identifier(&object)?;
            if model.binding(&reference).is_some() || name.text() != "String" || member != "raw" {
                return None;
            }
            StringKind::RawTemplate
        } else {
            StringKind::Template
        }
    } else {
        StringKind::Literal
    };

    let pattern = expr.omit_parentheses().as_static_value()?;
    // A regex cannot contain a repetition without a quantifier.
    if matches!(pattern.as_string_constant()?, "*" | "+" | "?") {
        return None;
    }
    Some((pattern, string_kind))
}

fn create_regex(pattern: &str, flags: &StaticValue, string_kind: StringKind) -> String {
    let flags = flags.text();
    let mut pattern_bytes = pattern.bytes().enumerate();
    let mut last_copied_inmdex = 0;
    // Reserve space for the pattern, its delimiters and its flags
    let mut new_pattern = String::with_capacity(pattern.len() + 2 + flags.len());
    new_pattern.push('/');
    while let Some((index, byte)) = pattern_bytes.next() {
        match byte {
            b'\n' => {
                new_pattern.push_str(&pattern[last_copied_inmdex..index]);
                new_pattern.push_str(r"\n");
                last_copied_inmdex = index + 1;
            }
            b'\\' if string_kind != StringKind::RawTemplate => {
                match pattern_bytes.next() {
                    Some((_, b'\\')) => {
                        // turn `\\` into `\`
                        new_pattern.push_str(&pattern[last_copied_inmdex..index]);
                        last_copied_inmdex = index + 1;
                    }
                    Some((
                        _,
                        b'/' | b'0'..b'7' | b'f' | b'n' | b'r' | b't' | b'u' | b'v' | b'x',
                    )) => {
                        // Keep escape sequence valid in both strings and regexes.
                    }
                    Some((_, b'b')) => {
                        // Backspace escape are not valid in regexes.
                        new_pattern.push_str(&pattern[last_copied_inmdex..index]);
                        // This could be represented more compactly with `\cH`.
                        // However `\x08` is more familiar.
                        new_pattern.push_str(r"\x08");
                        last_copied_inmdex = index + 2;
                    }
                    Some((_, b'\n')) if string_kind == StringKind::Literal => {
                        // String literal split over several lines.
                        new_pattern.push_str(&pattern[last_copied_inmdex..index]);
                        last_copied_inmdex = index + 2;
                    }
                    Some((_, b'\r')) if string_kind == StringKind::Literal => {
                        if matches!(pattern_bytes.next(), Some((_, b'\n'))) {
                            // String literal split over several lines.
                            new_pattern.push_str(&pattern[last_copied_inmdex..index]);
                            last_copied_inmdex = index + 3;
                        } else {
                            // Ignore `\` and preserve `\r`.
                            new_pattern.push_str(&pattern[last_copied_inmdex..index]);
                            last_copied_inmdex = index + 1;
                        }
                    }
                    _ => {
                        // Useless escaped character in strings are invalid escapes in regexes.
                        new_pattern.push_str(&pattern[last_copied_inmdex..index]);
                        last_copied_inmdex = index + 1;
                    }
                }
            }
            // Convert slash to "\/" to avoid parsing error in autofix.
            b'/' => {
                new_pattern.push_str(&pattern[last_copied_inmdex..index]);
                new_pattern.push_str(r"\/");
                last_copied_inmdex = index + 1;
            }
            _ => {}
        }
    }
    if pattern.is_empty() {
        // Emit `/(?:)/` instead of the invalid regex `//`.
        new_pattern.push_str("(?:)");
    } else {
        new_pattern.push_str(&pattern[last_copied_inmdex..]);
    }
    new_pattern.push('/');
    new_pattern.push_str(flags);
    new_pattern
}

fn is_regexp_object(expr: AnyJsExpression, model: &SemanticModel) -> bool {
    match global_identifier(&expr.omit_parentheses()) {
        Some((reference, name)) => match model.binding(&reference) {
            Some(_) if !reference.is_global_this() && !reference.has_name("window") => false,
            _ => name.text() == "RegExp",
        },
        None => false,
    }
}

fn parse_node(node: &JsNewOrCallExpression) -> Option<(AnyJsExpression, JsCallArguments)> {
    match node {
        JsNewOrCallExpression::JsNewExpression(node) => {
            let callee = node.callee().ok()?;
            let args = node.arguments()?;
            Some((callee, args))
        }
        JsNewOrCallExpression::JsCallExpression(node) => {
            let callee = node.callee().ok()?;
            let args = node.arguments().ok()?;
            Some((callee, args))
        }
    }
}

fn extract_valid_flags(flags: Result<AnyJsCallArgument, SyntaxError>) -> Option<StaticValue> {
    let Ok(AnyJsCallArgument::AnyJsExpression(flags)) = flags else {
        return None;
    };
    let flags = flags.omit_parentheses().as_static_value()?;
    // u flag (Unicode mode) and v flag (unicodeSets mode) cannot be combined.
    if matches!(flags.as_string_constant()?, "uv" | "vu") {
        return None;
    }
    Some(flags)
}

fn extract_inner_text(expr: &JsComputedMemberExpression) -> Option<TokenText> {
    expr.member()
        .ok()?
        .as_any_js_literal_expression()?
        .as_js_string_literal_expression()?
        .inner_string_text()
        .ok()
}
