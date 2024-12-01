use biome_analyze::{
    context::RuleContext, declare_lint_rule, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make::js_regex_literal_expression;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    global_identifier, static_value::StaticValue, AnyJsCallArgument, AnyJsExpression,
    AnyJsLiteralExpression, JsCallArguments, JsComputedMemberExpression, JsNewOrCallExpression,
    JsSyntaxKind, JsSyntaxToken,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, SyntaxError, TokenText};

use crate::{services::semantic::Semantic, JsRuleAction};

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
        sources: &[RuleSource::Eslint("prefer-regex-literals")],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Safe,
    }
}

pub struct UseRegexLiteralsState {
    pattern: String,
    flags: Option<String>,
}

impl Rule for UseRegexLiterals {
    type Query = Semantic<JsNewOrCallExpression>;
    type State = UseRegexLiteralsState;
    type Signals = Option<Self::State>;
    type Options = ();

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
        let pattern = create_pattern(pattern, model)?;

        let flags = match args.next() {
            Some(flags) => {
                let flags = create_flags(flags)?;
                Some(flags)
            }
            None => None,
        };
        Some(UseRegexLiteralsState { pattern, flags })
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

        let token = JsSyntaxToken::new_detached(
            JsSyntaxKind::JS_REGEX_LITERAL,
            &format!(
                "/{}/{}",
                state.pattern,
                state.flags.as_deref().unwrap_or_default()
            ),
            [],
            [],
        );
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

fn create_pattern(
    pattern: Result<AnyJsCallArgument, SyntaxError>,
    model: &SemanticModel,
) -> Option<String> {
    let pattern = pattern.ok()?;
    let expr = pattern.as_any_js_expression()?;
    if let Some(expr) = expr.as_js_template_expression() {
        if let Some(tag) = expr.tag() {
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
        };
    };
    let pattern = extract_literal_string(pattern)?;
    let pattern = pattern.replace("\\\\", "\\");

    // Convert slash to "\/" to avoid parsing error in autofix.
    let pattern = pattern.replace('/', "\\/");

    // If pattern is empty, (?:) is used instead.
    if pattern.is_empty() {
        return Some("(?:)".to_string());
    }

    // A repetition without quantifiers is invalid.
    if pattern == "*" || pattern == "+" || pattern == "?" {
        return None;
    }
    Some(pattern)
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

fn create_flags(flags: Result<AnyJsCallArgument, SyntaxError>) -> Option<String> {
    let flags = flags.ok()?;
    let flags = extract_literal_string(flags)?;
    // u flag (Unicode mode) and v flag (unicodeSets mode) cannot be combined.
    if flags == "uv" || flags == "vu" {
        return None;
    }
    Some(flags)
}

fn extract_literal_string(from: AnyJsCallArgument) -> Option<String> {
    let AnyJsCallArgument::AnyJsExpression(expr) = from else {
        return None;
    };
    expr.omit_parentheses()
        .as_static_value()
        .and_then(|value| match value {
            StaticValue::String(_) => Some(value.text().to_string().replace('\n', "\\n")),
            StaticValue::EmptyString(_) => Some(String::new()),
            _ => None,
        })
}

fn extract_inner_text(expr: &JsComputedMemberExpression) -> Option<TokenText> {
    expr.member()
        .ok()?
        .as_any_js_literal_expression()?
        .as_js_string_literal_expression()?
        .inner_string_text()
        .ok()
}
