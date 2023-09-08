use biome_analyze::context::RuleContext;
use biome_analyze::{declare_rule, Rule, RuleDiagnostic};
use rome_js_semantic::SemanticModel;
use rome_js_syntax::{
    global_identifier, AnyJsExpression, AnyJsMemberExpression, JsBinaryExpression, JsCaseClause,
    JsSwitchStatement, TextRange,
};
use rome_rowan::{declare_node_union, AstNode};

use crate::semantic_services::Semantic;

declare_rule! {
    /// Require calls to `isNaN()` when checking for `NaN`.
    ///
    /// In JavaScript, `NaN` is a special value of the `Number` type.
    /// It’s used to represent any of the "not-a-number" values represented by the double-precision 64-bit format as specified by the IEEE Standard for Binary Floating-Point Arithmetic.
    ///
    /// Because `NaN` is unique in JavaScript by not being equal to anything, including itself, the results of comparisons to `NaN` are confusing:
    /// - `NaN` === `NaN` or `NaN` == `NaN` evaluate to false
    /// - `NaN` !== `NaN` or `NaN` != `NaN` evaluate to true
    ///
    /// Therefore, use `Number.isNaN()` or global `isNaN()` functions to test whether a value is `NaN`.
    ///
    /// Note that `Number.isNaN()` and `isNaN()` [have not the same behavior](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/isNaN#description).
    /// When the argument to `isNaN()` is not a number, the value is first coerced to a number.
    /// `Number.isNaN()` does not perform this coercion.
    /// Therefore, it is a more reliable way to test whether a value is `NaN`.
    ///
    /// Source: [use-isnan](https://eslint.org/docs/latest/rules/use-isnan).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// 123 == NaN
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// 123 != NaN
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// switch(foo) { case (NaN): break; }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// Number.NaN == "abc"
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// if (Number.isNaN(123) !== true) {}
    ///
    /// foo(Number.NaN / 2)
    ///
    /// switch(foo) {}
    /// ```
    ///
    pub(crate) UseIsNan {
        version: "1.0.0",
        name: "useIsNan",
        recommended: true,
    }
}

declare_node_union! {
    pub(crate) UseIsNanQuery = JsBinaryExpression | JsCaseClause | JsSwitchStatement
}

enum Message {
    BinaryExpression,
    CaseClause,
    SwitchCase,
}

pub struct RuleState {
    range: TextRange,
    message_id: Message,
}

impl Message {
    fn as_str(&self) -> &str {
        match self {
			Self::BinaryExpression => "Use the Number.isNaN function to compare with NaN.",
			Self::CaseClause => "'case NaN' can never match. Use Number.isNaN before the switch.",
			Self::SwitchCase => "'switch(NaN)' can never match a case clause. Use Number.isNaN instead of the switch."
		}
    }
}

impl Rule for UseIsNan {
    type Query = Semantic<UseIsNanQuery>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();
        match node {
            UseIsNanQuery::JsBinaryExpression(bin_expr) => {
                if bin_expr.is_comparison_operator()
                    && (has_nan(bin_expr.left().ok()?, model)
                        || has_nan(bin_expr.right().ok()?, model))
                {
                    return Some(RuleState {
                        message_id: Message::BinaryExpression,
                        range: bin_expr.range(),
                    });
                }
            }
            UseIsNanQuery::JsCaseClause(case_clause) => {
                let test = case_clause.test().ok()?;
                let range = test.range();
                if has_nan(test, model) {
                    return Some(RuleState {
                        message_id: Message::CaseClause,
                        range,
                    });
                }
            }
            UseIsNanQuery::JsSwitchStatement(switch_stmt) => {
                let discriminant = switch_stmt.discriminant().ok()?;
                let range = discriminant.range();
                if has_nan(discriminant, model) {
                    return Some(RuleState {
                        message_id: Message::SwitchCase,
                        range,
                    });
                }
            }
        }
        None
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            state.range,
            state.message_id.as_str(),
        ))
    }
}

/// Checks whether an expression has `NaN`, `Number.NaN`, or `Number['NaN']`.
fn has_nan(expr: AnyJsExpression, model: &SemanticModel) -> bool {
    (|| {
        let expr = expr.omit_parentheses();
        let reference = if let Some((reference, name)) = global_identifier(&expr) {
            if name.text() != "NaN" {
                return None;
            }
            reference
        } else {
            let member_expr = AnyJsMemberExpression::cast_ref(expr.syntax())?;
            if member_expr.member_name()?.text() != "NaN" {
                return None;
            }
            let member_object = member_expr.object().ok()?.omit_parentheses();
            let (reference, name) = global_identifier(&member_object.omit_parentheses())?;
            if name.text() != "Number" {
                return None;
            }
            reference
        };
        model.binding(&reference).is_none().then_some(())
    })()
    .is_some()
}
