use crate::services::typed::Typed;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsAssignmentPattern, AnyJsExpression, AnyJsLiteralExpression, JsAssignmentExpression,
    JsAssignmentOperator, JsBinaryExpression, JsBinaryOperator, JsCallExpression,
    JsUnaryExpression, JsUnaryOperator, global_identifier,
};
use biome_js_type_info::{Literal, ResolvedTypeData, Type, TypeData};
use biome_rowan::{AstNode, AstNodeList, AstSeparatedList, TextRange, declare_node_union};
use biome_rule_options::no_useless_type_conversion::NoUselessTypeConversionOptions;

declare_lint_rule! {
    /// Disallow type conversions that do not change the type of an expression.
    ///
    /// This rule reports common conversion patterns when the converted expression
    /// is already known to have the target base type (AKA primitive type).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic,file=invalid-string.ts
    /// const text: string = "text";
    /// String(text);
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid-boolean.ts
    /// const value: boolean = true;
    /// !!value;
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid-assignment.ts
    /// let str = "text";
    /// str += "";
    /// ```
    ///
    /// ### Valid
    ///
    /// Genuine conversions are allowed.
    /// ```ts
    /// String(1);
    /// !!0;
    /// ```
    ///
    /// Unboxing boxed values is allowed.
    /// ```ts
    /// String(new String());
    /// ```
    pub NoUselessTypeConversion {
        version: "2.4.11",
        name: "noUselessTypeConversion",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintTypeScript("no-unnecessary-type-conversion").same()],
        domains: &[RuleDomain::Types],
        issue_number: Some("9752"),
    }
}

declare_node_union! {
    pub AnyPotentialUselessTypeConversion =
        JsAssignmentExpression
        | JsBinaryExpression
        | JsCallExpression
        | JsUnaryExpression
}

#[derive(Clone, Copy, Debug)]
enum PrimitiveKind {
    String,
    Number,
    Boolean,
    BigInt,
}

#[derive(Clone, Copy, Debug)]
enum ConversionKind {
    /// `String(value)`, `Number(value)`, `Boolean(value)`, or `BigInt(value)`.
    BuiltinCall,
    /// `value.toString()`.
    ToString,
    /// `value + ""` or `"" + value`.
    StringConcatenation,
    /// `value += ""`.
    StringAssignment,
    /// `+value`.
    UnaryPlus,
    /// `!!value`.
    DoubleNegation,
    /// `~~value`.
    DoubleBitwiseNot,
}

#[derive(Clone, Copy, Debug)]
pub struct RuleState {
    kind: ConversionKind,
    primitive: PrimitiveKind,
    range: TextRange,
}

impl Rule for NoUselessTypeConversion {
    type Query = Typed<AnyPotentialUselessTypeConversion>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = NoUselessTypeConversionOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        match ctx.query() {
            AnyPotentialUselessTypeConversion::JsCallExpression(call) => {
                run_builtin_call(ctx, call).or_else(|| run_to_string_call(ctx, call))
            }
            AnyPotentialUselessTypeConversion::JsUnaryExpression(unary) => {
                run_unary_expression(ctx, unary)
            }
            AnyPotentialUselessTypeConversion::JsBinaryExpression(binary) => {
                run_binary_expression(ctx, binary)
            }
            AnyPotentialUselessTypeConversion::JsAssignmentExpression(assignment) => {
                run_assignment_expression(ctx, assignment)
            }
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (article, name) = match (state.kind, state.primitive) {
            (ConversionKind::DoubleBitwiseNot, PrimitiveKind::Number) => {
                ("an integer-like number", "number")
            }
            (_, PrimitiveKind::String) => ("a string", "string"),
            (_, PrimitiveKind::Number) => ("a number", "number"),
            (_, PrimitiveKind::Boolean) => ("a boolean", "boolean"),
            (_, PrimitiveKind::BigInt) => ("a bigint", "bigint"),
        };

        let message = match state.kind {
            ConversionKind::BuiltinCall => match state.primitive {
                PrimitiveKind::String => "Avoid calling `String()` on a string value.",
                PrimitiveKind::Number => "Avoid calling `Number()` on a number value.",
                PrimitiveKind::Boolean => "Avoid calling `Boolean()` on a boolean value.",
                PrimitiveKind::BigInt => "Avoid calling `BigInt()` on a bigint value.",
            },
            ConversionKind::ToString => "Avoid calling `toString()` on a string value.",
            ConversionKind::StringConcatenation => {
                "Avoid concatenating a string value with an empty string."
            }
            ConversionKind::StringAssignment => {
                "Avoid appending an empty string to a string value."
            }
            ConversionKind::UnaryPlus => "Avoid applying unary `+` to a number value.",
            ConversionKind::DoubleNegation => "Avoid applying `!!` to a boolean value.",
            ConversionKind::DoubleBitwiseNot => match state.primitive {
                PrimitiveKind::BigInt => "Avoid applying `~~` to a bigint value.",
                _ => "Avoid applying `~~` to an integer value.",
            },
        };

        let note = match state.kind {
            ConversionKind::BuiltinCall | ConversionKind::ToString => {
                "Remove the conversion and use the value directly."
            }
            ConversionKind::StringConcatenation | ConversionKind::StringAssignment => {
                "This expression already evaluates to a string, so the empty string has no effect."
            }
            ConversionKind::UnaryPlus => {
                "This expression already evaluates to a number, so the unary operator has no effect."
            }
            ConversionKind::DoubleNegation => {
                "This expression already evaluates to a boolean, so the double negation has no effect."
            }
            ConversionKind::DoubleBitwiseNot => match state.primitive {
                PrimitiveKind::BigInt => {
                    "This expression already evaluates to a bigint, so the double bitwise NOT has no effect."
                }
                _ => {
                    "This expression already evaluates to an integer-like number, so the double bitwise NOT has no effect."
                }
            },
        };

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {{message}},
            )
            .note(markup! {
                "This expression already evaluates to "{article}"."
            })
            .note(markup! {{note}})
            .note(markup! {
                "Redundant conversions make it harder to see that the value already has the expected "{name}" type."
            }),
        )
    }
}

/// Detects calls to the global primitive conversion helpers when the argument
/// already has the target primitive type.
///
/// ## Examples
///
/// ```ts
/// const text: string = "text";
/// String(text);
/// ```
///
/// ```ts
/// const value: number = 1;
/// Number(value);
/// ```
fn run_builtin_call(
    ctx: &RuleContext<NoUselessTypeConversion>,
    node: &JsCallExpression,
) -> Option<RuleState> {
    let callee = node.callee().ok()?;
    let (reference, name) = global_identifier(&callee.clone().omit_parentheses())?;

    let primitive = match name.text() {
        "String" => PrimitiveKind::String,
        "Number" => PrimitiveKind::Number,
        "Boolean" => PrimitiveKind::Boolean,
        "BigInt" => PrimitiveKind::BigInt,
        _ => return None,
    };

    if ctx.has_binding(&reference) {
        return None;
    }

    let arguments = node.arguments().ok()?.args();
    if arguments.len() != 1 {
        return None;
    }

    let first_argument = arguments.first()?.ok()?;
    let argument = first_argument.as_any_js_expression()?;
    let ty = ctx.type_of_expression(argument);

    matches_primitive_type(&ty, primitive).then_some(RuleState {
        kind: ConversionKind::BuiltinCall,
        primitive,
        range: callee.range(),
    })
}

/// Detects `.toString()` calls on values that are already known to be strings.
///
/// ## Examples
///
/// ```ts
/// const text: string = "text";
/// text.toString();
/// ```
///
/// ```ts
/// declare const value: `foo-${string}`;
/// value.toString();
/// ```
fn run_to_string_call(
    ctx: &RuleContext<NoUselessTypeConversion>,
    node: &JsCallExpression,
) -> Option<RuleState> {
    let callee = node.callee().ok()?.omit_parentheses();
    let member_expression = callee.as_js_static_member_expression()?;

    let member = member_expression.member().ok()?;
    if member.value_token().ok()?.text_trimmed() != "toString" {
        return None;
    }

    if !node.arguments().ok()?.args().is_empty() {
        return None;
    }

    let object = member_expression.object().ok()?;
    let ty = ctx.type_of_expression(&object);

    matches_primitive_type(&ty, PrimitiveKind::String).then_some(RuleState {
        kind: ConversionKind::ToString,
        primitive: PrimitiveKind::String,
        range: member.range(),
    })
}

/// Detects string concatenations that only use `+ ""` or `"" +` as a coercion.
///
/// ## Examples
///
/// ```ts
/// const text: string = "text";
/// text + "";
/// ```
///
/// ```ts
/// const text: string = "text";
/// "" + text;
/// ```
fn run_binary_expression(
    ctx: &RuleContext<NoUselessTypeConversion>,
    node: &JsBinaryExpression,
) -> Option<RuleState> {
    if node.operator().ok()? != JsBinaryOperator::Plus {
        return None;
    }

    let left = node.left().ok()?;
    let right = node.right().ok()?;

    // `"" + value` and `value + ""` are both common string coercion idioms.
    // Once one side is known to be the empty string, inspect the other side and
    // only report when type information already says it is a string.
    let expression = if is_empty_string_expression(&left) && !is_empty_string_expression(&right) {
        right
    } else if is_empty_string_expression(&right) && !is_empty_string_expression(&left) {
        left
    } else {
        return None;
    };

    let ty = ctx.type_of_expression(&expression);
    matches_primitive_type(&ty, PrimitiveKind::String).then_some(RuleState {
        kind: ConversionKind::StringConcatenation,
        primitive: PrimitiveKind::String,
        range: node.range(),
    })
}

/// Detects `+= ""` when the assignment target is already a string.
///
/// ## Examples
///
/// ```ts
/// let text: string = "text";
/// text += "";
/// ```
///
/// ```ts
/// let value: `foo-${string}` = "foo-bar";
/// value += "";
/// ```
fn run_assignment_expression(
    ctx: &RuleContext<NoUselessTypeConversion>,
    node: &JsAssignmentExpression,
) -> Option<RuleState> {
    if node.operator().ok()? != JsAssignmentOperator::AddAssign {
        return None;
    }

    if !is_empty_string_expression(&node.right().ok()?) {
        return None;
    }

    let left = node.left().ok()?;
    let ty = match left {
        AnyJsAssignmentPattern::AnyJsAssignment(assign) => {
            let identifier = assign.as_js_identifier_assignment()?;
            let name = identifier.name_token().ok()?;
            // `type_of_expression()` works on expressions, but the left-hand side
            // of `+=` is an assignment target. Resolve the named value instead so
            // we can ask for the variable's current type.
            ctx.type_of_named_value(node.range(), name.text_trimmed())
        }
        _ => return None,
    };

    matches_primitive_type(&ty, PrimitiveKind::String).then_some(RuleState {
        kind: ConversionKind::StringAssignment,
        primitive: PrimitiveKind::String,
        range: node.range(),
    })
}

/// Detects unary coercion idioms that are redundant for the current operand
/// type.
///
/// ## Examples
///
/// ```ts
/// const value: number = 1;
/// +value;
/// ```
///
/// ```ts
/// const flag: boolean = true;
/// !!flag;
/// ```
///
/// ```ts
/// const count = 1 as const;
/// ~~count;
/// ```
fn run_unary_expression(
    ctx: &RuleContext<NoUselessTypeConversion>,
    node: &JsUnaryExpression,
) -> Option<RuleState> {
    let argument = node.argument().ok()?;

    match node.operator().ok()? {
        JsUnaryOperator::Plus => {
            let ty = ctx.type_of_expression(&argument);
            matches_primitive_type(&ty, PrimitiveKind::Number).then_some(RuleState {
                kind: ConversionKind::UnaryPlus,
                primitive: PrimitiveKind::Number,
                range: node.range(),
            })
        }
        JsUnaryOperator::LogicalNot => {
            let nested = argument.as_js_unary_expression()?;
            if nested.operator().ok()? != JsUnaryOperator::LogicalNot {
                return None;
            }

            let nested_argument = nested.argument().ok()?;
            let ty = ctx.type_of_expression(&nested_argument);
            matches_primitive_type(&ty, PrimitiveKind::Boolean).then_some(RuleState {
                kind: ConversionKind::DoubleNegation,
                primitive: PrimitiveKind::Boolean,
                range: node.range(),
            })
        }
        JsUnaryOperator::BitwiseNot => {
            let nested = argument.as_js_unary_expression()?;
            if nested.operator().ok()? != JsUnaryOperator::BitwiseNot {
                return None;
            }

            let nested_argument = nested.argument().ok()?;
            let ty = ctx.type_of_expression(&nested_argument);
            let primitive = if matches_primitive_type(&ty, PrimitiveKind::BigInt) {
                PrimitiveKind::BigInt
            } else if is_integer_like_type(&ty) {
                PrimitiveKind::Number
            } else {
                return None;
            };

            Some(RuleState {
                kind: ConversionKind::DoubleBitwiseNot,
                primitive,
                range: node.range(),
            })
        }
        _ => None,
    }
}

fn is_empty_string_expression(expression: &AnyJsExpression) -> bool {
    match expression.clone().omit_parentheses() {
        AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsStringLiteralExpression(string_literal),
        ) => string_literal
            .inner_string_text()
            .ok()
            .is_some_and(|text| text.is_empty()),
        AnyJsExpression::JsTemplateExpression(template_expression) => {
            template_expression.elements().len() == 0
        }
        _ => false,
    }
}

/// Returns true when the full type is already known to be the requested
/// primitive.
///
/// Union types are handled conservatively: every variant must match, otherwise
/// the conversion may still change runtime behavior for some inputs.
/// Generic types only match when their resolved constraint is precise enough to
/// prove the same property.
fn matches_primitive_type(ty: &Type, primitive: PrimitiveKind) -> bool {
    all_type_variants_match(ty, |current, raw| {
        primitive_matches_non_union_type(current, raw, primitive)
    })
}

fn primitive_matches_non_union_type(ty: &Type, raw: &TypeData, primitive: PrimitiveKind) -> bool {
    match primitive {
        PrimitiveKind::String => ty.is_string_or_string_literal(),
        PrimitiveKind::Number => ty.is_number_or_number_literal(),
        PrimitiveKind::Boolean => match raw {
            TypeData::Boolean => true,
            TypeData::Literal(literal) => matches!(literal.as_ref(), Literal::Boolean(_)),
            _ => false,
        },
        PrimitiveKind::BigInt => match raw {
            TypeData::BigInt => true,
            TypeData::Literal(literal) => matches!(literal.as_ref(), Literal::BigInt(_)),
            _ => false,
        },
    }
}

/// Returns true when every reachable variant keeps the same value after `~~`.
///
/// That is true for bigint values and for number literals that are already
/// integral. As with `matches_primitive_type()`, unions and constrained
/// generics only match when every resolved variant satisfies the check.
fn is_integer_like_type(ty: &Type) -> bool {
    all_type_variants_match(ty, |_current, raw| integer_like_matches_non_union_type(raw))
}

/// Evaluates `predicate` against every concrete variant reachable from `ty`.
///
/// The traversal expands unions and follows known generic constraints until it
/// reaches non-union, non-generic types. The check succeeds only when at least
/// one concrete variant is found and every variant satisfies `predicate`.
fn all_type_variants_match(ty: &Type, mut predicate: impl FnMut(&Type, &TypeData) -> bool) -> bool {
    let mut saw_variant = false;
    let mut pending = vec![ty.clone()];

    while let Some(current) = pending.pop() {
        if current.is_union() {
            let mut variants = current.flattened_union_variants().peekable();
            if variants.peek().is_none() {
                return false;
            }

            saw_variant = true;
            pending.extend(variants);
            continue;
        }

        let Some(raw) = current.resolved_data().map(ResolvedTypeData::as_raw_data) else {
            return false;
        };

        match raw {
            TypeData::Generic(generic) if generic.constraint.is_known() => {
                let Some(constraint) = current.resolve(&generic.constraint) else {
                    return false;
                };

                pending.push(constraint);
            }
            TypeData::Generic(_) => return false,
            _ if predicate(&current, raw) => saw_variant = true,
            _ => return false,
        }
    }

    saw_variant
}

fn integer_like_matches_non_union_type(raw: &TypeData) -> bool {
    match raw {
        TypeData::BigInt => true,
        TypeData::Literal(literal) => match literal.as_ref() {
            // `fract()` returns the fractional part of the numeric literal. It
            // is `0.0` exactly when the literal already represents an integer,
            // which means `~~` cannot truncate it any further.
            Literal::Number(value) => value.to_f64().is_some_and(|value| value.fract() == 0.0),
            Literal::BigInt(_) => true,
            _ => false,
        },
        _ => false,
    }
}
