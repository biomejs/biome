use crate::services::semantic::Semantic;
use crate::{ast_utils, JsRuleAction};
use biome_analyze::context::RuleContext;
use biome_analyze::{declare_lint_rule, FixKind, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    global_identifier, AnyJsExpression, AnyJsLiteralExpression, AnyJsMemberExpression,
    JsCallExpression, JsSyntaxToken,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt};

declare_lint_rule! {
    /// Disallow `parseInt()` and `Number.parseInt()` in favor of binary, octal, and hexadecimal literals
    ///
    /// _JavaScript_ provides literal forms for binary, octal, and hexadecimal numbers.
    /// For example: `0b11`, `0o77`, and `0xff`.
    /// Using the literal forms enable static code analysis and avoid unnecessary computations.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// parseInt("111110111", 2);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// Number.parseInt("767", 8);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// Number.parseInt("-1f7", 16);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// parseInt(1);
    /// parseInt(1, 3);
    /// Number.parseInt(1);
    /// Number.parseInt(1, 3);
    ///
    /// 0b111110111 === 503;
    /// 0o767 === 503;
    /// 0x1F7 === 503;
    ///
    /// a[parseInt](1,2);
    ///
    /// parseInt(foo);
    /// parseInt(foo, 2);
    /// Number.parseInt(foo);
    /// Number.parseInt(foo, 2);
    /// ```
    pub UseNumericLiterals {
        version: "1.0.0",
        name: "useNumericLiterals",
        language: "js",
        sources: &[RuleSource::Eslint("prefer-numeric-literals")],
        recommended: false,
        severity: Severity::Warning,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseNumericLiterals {
    type Query = Semantic<JsCallExpression>;
    type State = CallInfo;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let expr = ctx.query();
        let model = ctx.model();
        CallInfo::try_from_expr(expr, model)
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! { "This call to "<Emphasis>{state.callee}</Emphasis>" can be replaced by "{state.radix.article()}" "{state.radix.description()}" literal." }
                .to_owned(),
        ).note(markup! {
            "Using a literal avoids unnecessary computations."
        }))
    }

    fn action(ctx: &RuleContext<Self>, call: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();
        let number = call.to_numeric_literal()?;
        let number = ast_utils::token_with_source_trivia(&number, node);
        mutation.replace_node_discard_trivia(
            AnyJsExpression::JsCallExpression(node.clone()),
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsNumberLiteralExpression(
                    make::js_number_literal_expression(number),
                ),
            ),
        );

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use the computed "{call.radix.description()}" literal instead." }.to_owned(),
            mutation,
        ))
    }
}

pub struct CallInfo {
    callee: &'static str,
    text: String,
    radix: Radix,
}

impl CallInfo {
    fn try_from_expr(expr: &JsCallExpression, model: &SemanticModel) -> Option<CallInfo> {
        let args = expr.arguments().ok()?.args();
        if args.len() != 2 {
            return None;
        }
        let [Some(text), Some(radix)] = expr.arguments().ok()?.get_arguments_by_index([0, 1])
        else {
            return None;
        };
        let text = text
            .as_any_js_expression()?
            .as_static_value()?
            .as_string_constant()?
            .to_string();
        let radix = radix
            .as_any_js_expression()?
            .as_any_js_literal_expression()?
            .as_js_number_literal_expression()?
            .as_number()?;
        let callee = get_callee(expr, model)?;
        Some(CallInfo {
            callee,
            text,
            radix: Radix::from_f64(radix)?,
        })
    }

    fn to_numeric_literal(&self) -> Option<JsSyntaxToken> {
        // `parseInt` ignores leading and trailing white-spaces
        let text_trimmed = self.text.trim();
        // Handle optional sign
        let (sign, text_trimmed) = if let Some(text) = text_trimmed.strip_prefix('-') {
            ("-", text)
        } else {
            ("", text_trimmed.strip_prefix('+').unwrap_or(text_trimmed))
        };
        i128::from_str_radix(text_trimmed, self.radix as u32).ok()?;
        let prefix = self.radix.prefix();
        let number = make::js_number_literal(format_args!("{sign}{prefix}{text_trimmed}"));
        Some(number)
    }
}

fn get_callee(expr: &JsCallExpression, model: &SemanticModel) -> Option<&'static str> {
    let callee = expr.callee().ok()?.omit_parentheses();
    if let Some((reference, name)) = global_identifier(&callee) {
        if name.text() == "parseInt" && model.binding(&reference).is_none() {
            return Some("parseInt()");
        }
        return None;
    }
    let callee = AnyJsMemberExpression::cast(callee.into_syntax())?;
    if callee.member_name()?.text() != "parseInt" {
        return None;
    }
    let object = callee.object().ok()?.omit_parentheses();
    let (reference, name) = global_identifier(&object)?;
    if name.text() == "Number" && model.binding(&reference).is_none() {
        return Some("Number.parseInt()");
    }
    None
}

#[derive(Copy, Clone)]
enum Radix {
    Binary = 2,
    Octal = 8,
    Hexadecimal = 16,
}

impl Radix {
    fn from_f64(v: f64) -> Option<Self> {
        Some(if v == 2.0 {
            Self::Binary
        } else if v == 8.0 {
            Self::Octal
        } else if v == 16.0 {
            Self::Hexadecimal
        } else {
            return None;
        })
    }

    const fn prefix(self) -> &'static str {
        match self {
            Self::Binary => "0b",
            Self::Octal => "0o",
            Self::Hexadecimal => "0x",
        }
    }

    const fn description(self) -> &'static str {
        match self {
            Self::Binary => "binary",
            Self::Octal => "octal",
            Self::Hexadecimal => "hexadecimal",
        }
    }

    const fn article(self) -> &'static str {
        if matches!(self, Self::Octal) {
            "an"
        } else {
            "a"
        }
    }
}
