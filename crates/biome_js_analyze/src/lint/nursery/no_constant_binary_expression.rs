use biome_analyze::{
    context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic, RuleSource, RuleSourceKind,
};
use biome_console::fmt::{Display, Formatter};
use biome_console::markup;
use biome_js_semantic::{BindingExtensions, SemanticModel};
use biome_js_syntax::{
    AnyJsArrayElement, AnyJsCallArgument, AnyJsExpression, AnyJsLiteralExpression,
    AnyJsTemplateElement, JsAssignmentOperator, JsBinaryExpression, JsBinaryOperator,
    JsCallExpression, JsIdentifierExpression, JsLogicalExpression, JsLogicalOperator,
    JsReferenceIdentifier, JsUnaryOperator,
};
use biome_rowan::{declare_node_union, AstNode, AstNodeList, AstSeparatedList, SyntaxResult};

use crate::ast_utils::is_constant_condition;
use crate::globals::is_js_language_global;
use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow expressions where the operation doesn't affect the value
    ///
    /// Comparisons which will always evaluate to true or false and logical expressions
    /// (`||`, `&&`, `??`) which either always short-circuit or never short-circuit are both likely
    /// indications of programmer error.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const value1 = +x == null;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const value2 = condition ? x : {} || DEFAULT;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const value3 = !foo == null;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const value4 = new Boolean(foo) === true;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const objIsEmpty = someObj === {};
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const arrIsEmpty = someArr === [];
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const shortCircuit1 = condition1 && false && condition2;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const shortCircuit2 = condition1 || true || condition2;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const shortCircuit3 = condition1 ?? "non-nullish" ?? condition2;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const value1 = x == null;
    /// ```
    ///
    /// ```js
    /// const value2 = (condition ? x : {}) || DEFAULT;
    /// ```
    ///
    /// ```js
    /// const value3 = !(foo == null);
    /// ```
    ///
    /// ```js
    /// const value4 = Boolean(foo) === true;
    /// ```
    ///
    /// ```js
    /// const objIsEmpty = Object.keys(someObj).length === 0;
    /// ```
    ///
    /// ```js
    /// const arrIsEmpty = someArr.length === 0;
    /// ```
    ///
    pub NoConstantBinaryExpression {
        version: "next",
        name: "noConstantBinaryExpression",
        language: "js",
        recommended: false,
        sources: &[RuleSource::Eslint("no-constant-binary-expression")],
        source_kind: RuleSourceKind::SameLogic,
    }
}

declare_node_union! {
    pub Query = JsLogicalExpression | JsBinaryExpression
}

pub enum Property {
    Truthiness,
    Nullishness,
}

impl Display for Property {
    fn fmt(&self, fmt: &mut Formatter) -> std::io::Result<()> {
        match self {
            Self::Truthiness => fmt.write_str("truthiness"),
            Self::Nullishness => fmt.write_str("nullishness"),
        }
    }
}

pub enum Issue {
    /// A binary expression leads to a constant result, so it can be simplified.
    /// For example, `[] == true` is always true, while `[] === true` is always false.
    ConstantBinaryOperand {
        /// An operand in the expression that will always lead to the same result on comparison
        /// with  `==`, `!=`, `===`, or `!==` operator.
        operand: AnyJsExpression,
    },

    /// The left-hand of a logical expression leads to a constant result, so it can be simplified.
    /// For example, `{} ?? foo` will always return the left-hand value, while `null ?? foo` will
    /// always return the right-hand value.
    ConstantShortCircuit {
        /// Left-hand operand of the expression.
        left: AnyJsExpression,

        /// Right-hand operand of the expression.
        right: AnyJsExpression,

        /// Which property the expression will have constantly, truthiness or nullishness.
        property: Property,
    },

    /// A binary expression that always compare to a new object, so it can be simplified.
    /// For example, the result of `foo === []` is always false.
    AlwaysNew {
        /// An operand in the expression that will always construct a new object.
        operand: AnyJsExpression,
    },

    /// A strict binary expression that always compare two new objects, so it can be simplified.
    /// For example, the result of `[] != []` is always true.
    BothAlwaysNew,
}

impl Rule for NoConstantBinaryExpression {
    type Query = Semantic<Query>;
    type State = Issue;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let model = ctx.model();
        match ctx.query() {
            Query::JsLogicalExpression(expr) => {
                let operator = expr.operator().ok()?;
                let left = expr.left().ok()?;
                let right = expr.right().ok()?;

                match operator {
                    JsLogicalOperator::LogicalAnd | JsLogicalOperator::LogicalOr => {
                        if is_constant_condition(left.clone(), true, model).is_some() {
                            return Some(Issue::ConstantShortCircuit {
                                left,
                                right,
                                property: Property::Truthiness,
                            });
                        }
                    }
                    JsLogicalOperator::NullishCoalescing => {
                        if let Ok(true) = has_constant_nullishness(model, &left, false) {
                            return Some(Issue::ConstantShortCircuit {
                                left,
                                right,
                                property: Property::Nullishness,
                            });
                        }
                    }
                }
            }
            Query::JsBinaryExpression(expr) => {
                let operator = expr.operator().ok()?;
                let left = expr.left().ok()?;
                let right = expr.right().ok()?;

                if let Some(operand) =
                    find_binary_expression_constant_operand(model, operator, &left, &right)
                {
                    return Some(Issue::ConstantBinaryOperand {
                        operand: operand.clone(),
                    });
                }

                if let Some(operand) =
                    find_binary_expression_constant_operand(model, operator, &right, &left)
                {
                    return Some(Issue::ConstantBinaryOperand {
                        operand: operand.clone(),
                    });
                }

                match operator {
                    JsBinaryOperator::StrictEquality | JsBinaryOperator::StrictInequality => {
                        if let Ok(true) = is_always_new(model, &left) {
                            return Some(Issue::AlwaysNew { operand: left });
                        }
                        if let Ok(true) = is_always_new(model, &right) {
                            return Some(Issue::AlwaysNew { operand: right });
                        }
                    }
                    JsBinaryOperator::Equality | JsBinaryOperator::Inequality => {
                        if let (Ok(true), Ok(true)) =
                            (is_always_new(model, &left), is_always_new(model, &right))
                        {
                            return Some(Issue::BothAlwaysNew);
                        }
                    }
                    _ => {}
                }
            }
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(match state {
            Issue::ConstantBinaryOperand { .. } => RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! { "This binary expression leads to a constant result." },
            ),
            Issue::ConstantShortCircuit { left, property, .. } => RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! { "This logical expression can be simplified." },
            )
            .detail(
                left.range(),
                markup! { "This operand always evaluates to the same "{property}"." },
            ),
            Issue::AlwaysNew { operand } => RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! { "Unexpected comparison to newly constructed object." },
            )
            .detail(
                operand.range(),
                markup! { "This expression always constructs a new object."},
            ),
            Issue::BothAlwaysNew => RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! { "Unexpected comparison of two newly constructed objects." },
            ),
        })
    }
}

fn find_binary_expression_constant_operand<'a>(
    model: &SemanticModel,
    operator: JsBinaryOperator,
    a: &'a AnyJsExpression,
    b: &'a AnyJsExpression,
) -> Option<&'a AnyJsExpression> {
    match operator {
        JsBinaryOperator::Equality | JsBinaryOperator::Inequality => {
            if (is_null_or_undefined(model, a).ok()?
                && has_constant_nullishness(model, b, false).ok()?)
                || (is_static_boolean(model, a).ok()?
                    && has_constant_loose_boolean_comparison(model, b).ok()?)
            {
                return Some(b);
            }
        }
        JsBinaryOperator::StrictEquality | JsBinaryOperator::StrictInequality => {
            if (is_null_or_undefined(model, a).ok()?
                && has_constant_nullishness(model, b, false).ok()?)
                || (is_static_boolean(model, a).ok()?
                    && has_constant_strict_boolean_comparison(model, b).ok()?)
            {
                return Some(b);
            }
        }
        _ => {}
    }

    None
}

// Check the expression will always give the same result when compared to a boolean value loosely.
fn has_constant_loose_boolean_comparison(
    model: &SemanticModel,
    node: &AnyJsExpression,
) -> SyntaxResult<bool> {
    Ok(match node.clone().omit_parentheses() {
        // always truthy
        AnyJsExpression::JsObjectExpression(_)
        | AnyJsExpression::JsClassExpression(_)
        | AnyJsExpression::JsArrowFunctionExpression(_)
        | AnyJsExpression::JsFunctionExpression(_) => true,

        // literals never change
        AnyJsExpression::AnyJsLiteralExpression(_) => true,

        // can have a custom .valueOf() or .toString() implementation
        AnyJsExpression::JsNewExpression(_) => false,

        AnyJsExpression::JsArrayExpression(expr) => {
            // always falsy when the array is empty
            expr.elements().is_empty()
                // single value array `[x]` can be `[0]` (falsy) or `[1]` (truthy)
                || expr.elements().elements().filter(|element| {
                    matches!(element.node(), Ok(AnyJsArrayElement::AnyJsExpression(_)))
                }).count() > 1
        }

        AnyJsExpression::JsUnaryExpression(expr) => match expr.operator()? {
            JsUnaryOperator::Void => true,   // always undefined
            JsUnaryOperator::Typeof => true, // always truthy
            JsUnaryOperator::LogicalNot => {
                is_constant_condition(expr.argument()?, true, model).is_some()
            }
            _ => false,
        },

        AnyJsExpression::JsCallExpression(expr) => is_constant_boolean_cast(model, &expr)?,

        AnyJsExpression::JsIdentifierExpression(expr) => is_undefined(model, &expr)?,

        AnyJsExpression::JsTemplateExpression(expr) => !expr
            .elements()
            .iter()
            .any(|element| matches!(element, AnyJsTemplateElement::JsTemplateElement(_))),

        AnyJsExpression::JsAssignmentExpression(expr) => match expr.operator()? {
            JsAssignmentOperator::Assign => {
                has_constant_loose_boolean_comparison(model, &expr.right()?)?
            }
            _ => false,
        },

        AnyJsExpression::JsSequenceExpression(expr) => {
            has_constant_loose_boolean_comparison(model, &expr.right()?)?
        }

        _ => false,
    })
}

// Check the expression will always give the same result when compared to a boolean value strictly.
fn has_constant_strict_boolean_comparison(
    model: &SemanticModel,
    node: &AnyJsExpression,
) -> SyntaxResult<bool> {
    Ok(match node.clone().omit_parentheses() {
        // not a boolean
        AnyJsExpression::JsObjectExpression(_)
        | AnyJsExpression::JsArrayExpression(_)
        | AnyJsExpression::JsArrowFunctionExpression(_)
        | AnyJsExpression::JsFunctionExpression(_)
        | AnyJsExpression::JsClassExpression(_)
        | AnyJsExpression::JsNewExpression(_)
        | AnyJsExpression::JsTemplateExpression(_)
        | AnyJsExpression::JsPreUpdateExpression(_)
        | AnyJsExpression::JsPostUpdateExpression(_) => true,

        // literals never change
        AnyJsExpression::AnyJsLiteralExpression(_) => true,

        AnyJsExpression::JsBinaryExpression(expr) => matches!(
            expr.operator()?,
            JsBinaryOperator::Plus
                | JsBinaryOperator::Minus
                | JsBinaryOperator::Times
                | JsBinaryOperator::Divide
                | JsBinaryOperator::Remainder
                | JsBinaryOperator::Exponent
                | JsBinaryOperator::BitwiseAnd
                | JsBinaryOperator::BitwiseOr
                | JsBinaryOperator::BitwiseXor
                | JsBinaryOperator::LeftShift
                | JsBinaryOperator::RightShift
                | JsBinaryOperator::UnsignedRightShift
        ),

        AnyJsExpression::JsUnaryExpression(expr) => match expr.operator()? {
            JsUnaryOperator::Delete => false,
            JsUnaryOperator::LogicalNot => {
                is_constant_condition(expr.argument()?, true, model).is_some()
            }
            _ => true,
        },

        AnyJsExpression::JsSequenceExpression(expr) => {
            has_constant_strict_boolean_comparison(model, &expr.right()?)?
        }

        AnyJsExpression::JsIdentifierExpression(expr) => is_undefined(model, &expr)?,

        AnyJsExpression::JsAssignmentExpression(expr) => match expr.operator()? {
            JsAssignmentOperator::Assign => {
                has_constant_strict_boolean_comparison(model, &expr.right()?)?
            }

            // TODO: Handle short-circuiting assignment operators
            JsAssignmentOperator::LogicalAndAssign
            | JsAssignmentOperator::LogicalOrAssign
            | JsAssignmentOperator::NullishCoalescingAssign => false,

            _ => true,
        },

        AnyJsExpression::JsCallExpression(expr) => match expr.callee()? {
            AnyJsExpression::JsIdentifierExpression(ident) => {
                let name = ident.name()?;

                ((name.has_name("String") || name.has_name("Number"))
                    && is_global_reference(model, &name))
                    || is_constant_boolean_cast(model, &expr)?
            }
            _ => false,
        },

        _ => false,
    })
}

/// Check the expression will always result a new object.
fn is_always_new(model: &SemanticModel, node: &AnyJsExpression) -> SyntaxResult<bool> {
    Ok(match node.clone().omit_parentheses() {
        AnyJsExpression::JsObjectExpression(_)
        | AnyJsExpression::JsArrayExpression(_)
        | AnyJsExpression::JsArrowFunctionExpression(_)
        | AnyJsExpression::JsFunctionExpression(_)
        | AnyJsExpression::JsClassExpression(_) => true,

        AnyJsExpression::JsNewExpression(expr) => match expr.callee()? {
            AnyJsExpression::JsIdentifierExpression(ident) => {
                let name = ident.name()?;
                is_js_language_global(name.name()?.text()) && is_global_reference(model, &name)
            }
            _ => false,
        },

        // Regex literals are objects
        AnyJsExpression::AnyJsLiteralExpression(expr) => {
            matches!(expr, AnyJsLiteralExpression::JsRegexLiteralExpression(_))
        }

        AnyJsExpression::JsConditionalExpression(expr) => {
            is_always_new(model, &expr.consequent()?)? && is_always_new(model, &expr.alternate()?)?
        }

        _ => false,
    })
}

/// Check the expression will result always true or always false.
fn is_static_boolean(model: &SemanticModel, node: &AnyJsExpression) -> SyntaxResult<bool> {
    Ok(match node.clone().omit_parentheses() {
        AnyJsExpression::AnyJsLiteralExpression(expr) => {
            matches!(expr, AnyJsLiteralExpression::JsBooleanLiteralExpression(_))
        }

        AnyJsExpression::JsCallExpression(expr) => is_constant_boolean_cast(model, &expr)?,

        AnyJsExpression::JsUnaryExpression(expr) => {
            expr.operator()? == JsUnaryOperator::LogicalNot
                && is_constant_condition(expr.argument()?, true, model).is_some()
        }

        _ => false,
    })
}

/// Check the call expression is `Boolean(x)` call and `x` is a constant condition.
fn is_constant_boolean_cast(model: &SemanticModel, node: &JsCallExpression) -> SyntaxResult<bool> {
    let AnyJsExpression::JsIdentifierExpression(ident) = node.callee()? else {
        return Ok(false);
    };

    let name = ident.name()?;

    Ok(
        // Boolean(expr) is constant where expr is constant
        name.has_name("Boolean") && is_global_reference(model, &name) && {
            match node.arguments()?.args().first() {
                Some(arg) => match arg? {
                    AnyJsCallArgument::AnyJsExpression(expr) => {
                        is_constant_condition(expr, true, model).is_some()
                    }
                    _ => false,
                },
                _ => true,
            }
        },
    )
}

/// Check the expression has constant nullishness: always nullish or never nullish.
fn has_constant_nullishness(
    model: &SemanticModel,
    node: &AnyJsExpression,
    non_nullish: bool,
) -> SyntaxResult<bool> {
    if non_nullish && is_null_or_undefined(model, node)? {
        return Ok(false);
    }

    Ok(match node.clone().omit_parentheses() {
        // never nullish
        AnyJsExpression::JsObjectExpression(_)
        | AnyJsExpression::JsArrayExpression(_)
        | AnyJsExpression::JsArrowFunctionExpression(_)
        | AnyJsExpression::JsFunctionExpression(_)
        | AnyJsExpression::JsClassExpression(_)
        | AnyJsExpression::JsNewExpression(_)
        | AnyJsExpression::AnyJsLiteralExpression(_)
        | AnyJsExpression::JsTemplateExpression(_)
        | AnyJsExpression::JsPreUpdateExpression(_)
        | AnyJsExpression::JsPostUpdateExpression(_)
        | AnyJsExpression::JsBinaryExpression(_)
        | AnyJsExpression::JsUnaryExpression(_)
        | AnyJsExpression::JsInstanceofExpression(_)
        | AnyJsExpression::JsInExpression(_) => true,

        AnyJsExpression::JsCallExpression(expr) => match expr.callee()? {
            AnyJsExpression::JsIdentifierExpression(ident) => {
                let name = ident.name()?;

                (name.has_name("Boolean") || name.has_name("String") || name.has_name("Number"))
                    && model.scope(ident.syntax()).is_global_scope()
            }
            _ => false,
        },

        AnyJsExpression::JsLogicalExpression(expr) => {
            expr.operator()? == JsLogicalOperator::NullishCoalescing
                && has_constant_nullishness(model, &expr.right()?, true)?
        }

        AnyJsExpression::JsAssignmentExpression(expr) => match expr.operator()? {
            JsAssignmentOperator::Assign => {
                has_constant_nullishness(model, &expr.right()?, non_nullish)?
            }

            // TODO: Handle short-circuiting assignment operators
            JsAssignmentOperator::LogicalAndAssign
            | JsAssignmentOperator::LogicalOrAssign
            | JsAssignmentOperator::NullishCoalescingAssign => false,

            // always non-nullish
            _ => true,
        },

        AnyJsExpression::JsSequenceExpression(expr) => {
            has_constant_nullishness(model, &expr.right()?, non_nullish)?
        }

        AnyJsExpression::JsIdentifierExpression(expr) => is_undefined(model, &expr)?,

        _ => false,
    })
}

/// Check the expression always result a null or undefined value.
fn is_null_or_undefined(model: &SemanticModel, node: &AnyJsExpression) -> SyntaxResult<bool> {
    Ok(match node.clone().omit_parentheses() {
        AnyJsExpression::AnyJsLiteralExpression(expr) => {
            matches!(expr, AnyJsLiteralExpression::JsNullLiteralExpression(_))
        }

        AnyJsExpression::JsIdentifierExpression(expr) => is_undefined(model, &expr)?,

        AnyJsExpression::JsUnaryExpression(expr) => expr.operator()? == JsUnaryOperator::Void,

        _ => false,
    })
}

/// Check the identifier is a reference of the global `undefined` variable.
fn is_undefined(model: &SemanticModel, node: &JsIdentifierExpression) -> SyntaxResult<bool> {
    node.name()
        .map(|ident| ident.is_undefined() && is_global_reference(model, &ident))
}

/// Check the referenced variable is in the global scope.
fn is_global_reference(model: &SemanticModel, node: &JsReferenceIdentifier) -> bool {
    node.binding(model)
        .map_or(true, |b| b.scope().is_global_scope())
}
