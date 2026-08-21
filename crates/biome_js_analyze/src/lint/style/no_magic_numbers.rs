use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsBinding, AnyJsExpression, AnyJsFunction, AnyJsLiteralExpression, AnyJsMemberExpression,
    AnyJsRoot, AnyTsType, JsArrayAssignmentPatternElement, JsAssignmentExpression,
    JsAssignmentOperator,
    JsBigintLiteralExpression, JsBinaryExpression, JsBinaryOperator, JsCallExpression,
    JsComputedMemberAssignment, JsComputedMemberExpression, JsFormalParameter, JsInitializerClause,
    JsNumberLiteralExpression, JsObjectBindingPatternShorthandProperty, JsParenthesizedExpression,
    JsPropertyClassMember, JsPropertyObjectMember, JsStaticMemberAssignment, JsSyntaxNode,
    JsUnaryExpression, JsUnaryOperator, JsVariableDeclarator,
    JsxExpressionAttributeValue, JsxExpressionChild, TsAsExpression, TsEnumMemberList,
    TsIndexedAccessType, TsNonNullAssertionExpression, TsNumberLiteralType, TsPredicateReturnType,
    TsReturnTypeAnnotation, TsSatisfiesExpression, TsTypeAnnotation, TsTypeAssertionExpression,
    TsUnionTypeVariantList,
};
use biome_js_semantic::SemanticModel;
use biome_rowan::{AstNode, declare_node_union};
use biome_rule_options::no_magic_numbers::NoMagicNumbersOptions;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::{
    JsRuleAction,
    services::semantic::Semantic,
    utils::module_constant::{
        collision_free_module_constant_name_with_facts, extract_module_constant,
        extract_module_constant_with_reserved_names, is_module_constant_extractable_with_facts,
        module_constant_facts, module_constant_insertion_slot,
    },
};

declare_lint_rule! {
    /// Reports usage of "magic numbers" — numbers used directly instead of being assigned to named constants.
    ///
    /// Its goal is to improve code maintainability and readability by encouraging developers to extract such numbers into named constants, making their purpose explicit.
    ///
    /// It ignores:
    /// - non-magic values (like 0, 1, 2, 10, 24, 60, and their negative or bigint forms) found anywhere, including arithmetic expressions, fn calls etc.
    /// - Array indices
    /// - Enum values
    /// - Initial values in variable or class property declarations
    /// - Default values in function parameters or destructuring patterns
    /// - Arguments to JSON.stringify and parseInt (e.g., `JSON.stringify(22)`, `parseInt("123", 8)`)
    /// - Operands in bitwise operations (e.g., `a & 7`, `a | 7`)
    /// - Values in JSX expressions (e.g., `<div>{1}</div>`)
    /// - Object property values (e.g., `{ tax: 0.25 }`)
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// let total = price * 1.23; // Magic number for tax rate
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const TAX_RATE = 1.23;
    /// let total = price * TAX_RATE;
    /// ```
    ///
    /// ```ts
    /// const TAX_RATE = 1.23 as const;
    /// let total = price * TAX_RATE;
    /// ```
    pub NoMagicNumbers {
        version: "2.1.0",
        name: "noMagicNumbers",
        language: "ts",
        sources: &[RuleSource::Eslint("no-magic-numbers").inspired(), RuleSource::EslintTypeScript("no-magic-numbers").same()],
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoMagicNumbers {
    type Query = Semantic<JsOrTsNumericLiteral>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoMagicNumbersOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let literal_expression = ctx.query();

        if is_valid_number_in_relevant_context(literal_expression) {
            return None;
        }

        Some(())
    }

    fn diagnostic(context: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let literal_expression = context.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                literal_expression.range(),
                markup! {
                    "Magic number detected. Extract it to a constant with a meaningful name."
                },
            )
                .note(markup! {
                "Code is more readable and refactoring easier when special numbers are declared as constants as it makes their meaning explicit."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let numeric_literal = AnyJsNumericLiteral::cast(ctx.query().syntax().clone())?;
        if !is_actionable_numeric_literal(&numeric_literal) {
            return None;
        }
        let value = match numeric_literal.clone() {
            AnyJsNumericLiteral::JsNumberLiteralExpression(literal) => {
                AnyJsExpression::AnyJsLiteralExpression(
                    AnyJsLiteralExpression::JsNumberLiteralExpression(literal),
                )
            }
            AnyJsNumericLiteral::JsBigintLiteralExpression(literal) => {
                AnyJsExpression::AnyJsLiteralExpression(
                    AnyJsLiteralExpression::JsBigintLiteralExpression(literal),
                )
            }
        };
        let root = ctx.root();
        let (candidate_name, reserved_names, transfer_header) =
            coordinated_extraction(&root, ctx.model(), &numeric_literal)?;
        let (mutation, name) = if reserved_names.is_empty() && transfer_header {
            extract_module_constant(
                &root,
                ctx.model(),
                numeric_literal.syntax(),
                value,
                &candidate_name,
            )?
        } else {
            extract_module_constant_with_reserved_names(
                &root,
                ctx.model(),
                numeric_literal.syntax(),
                value,
                &candidate_name,
                &reserved_names,
                transfer_header,
            )?
        };

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Extract the magic number into "<Emphasis>{name}</Emphasis>"." }.to_owned(),
            mutation,
        ))
    }
}

fn coordinated_extraction(
    root: &AnyJsRoot,
    model: &SemanticModel,
    current: &AnyJsNumericLiteral,
) -> Option<(String, FxHashSet<String>, bool)> {
    let facts = module_constant_facts(root, model);
    let mut reserved_names = FxHashSet::default();
    let mut names_by_value = FxHashMap::default();

    for descendant in root.syntax().descendants() {
        let Some(numeric_literal) = AnyJsNumericLiteral::cast(descendant) else {
            continue;
        };
        let query = JsOrTsNumericLiteral::AnyJsNumericLiteral(numeric_literal.clone());
        if is_valid_number_in_relevant_context(&query)
            || !is_actionable_numeric_literal(&numeric_literal)
            || !is_module_constant_extractable_with_facts(root, numeric_literal.syntax(), &facts)
        {
            continue;
        }

        let value_key = numeric_literal.syntax().text_trimmed().to_string();
        if let Some(name) = names_by_value.get(&value_key).cloned() {
            if numeric_literal.syntax() == current.syntax() {
                reserved_names.remove(&name);
                return Some((
                    name,
                    reserved_names,
                    module_constant_insertion_slot(root, numeric_literal.syntax()) == Some(0),
                ));
            }
            continue;
        }

        let candidate_name = numeric_constant_name(&numeric_literal);
        let name = collision_free_module_constant_name_with_facts(
            model,
            numeric_literal.syntax(),
            &candidate_name,
            &reserved_names,
            &facts,
        );
        let insertion_slot = module_constant_insertion_slot(root, numeric_literal.syntax());
        let owns_header = insertion_slot == Some(0);
        names_by_value.insert(value_key, name.clone());

        if numeric_literal.syntax() == current.syntax() {
            return Some((name, reserved_names, owns_header));
        }

        reserved_names.insert(name);
    }

    None
}

declare_node_union! {
    pub AnyJsNumericLiteral = JsNumberLiteralExpression | JsBigintLiteralExpression
}

declare_node_union! {
    pub JsOrTsNumericLiteral = AnyJsNumericLiteral | TsNumberLiteralType
}

/// Builds a stable uppercase name from the syntax surrounding a runtime number.
fn numeric_constant_name(numeric_literal: &AnyJsNumericLiteral) -> String {
    let literal_text = match numeric_literal {
        AnyJsNumericLiteral::JsNumberLiteralExpression(literal) => literal
            .value_token()
            .ok()
            .map(|token| token.text_trimmed().to_string()),
        AnyJsNumericLiteral::JsBigintLiteralExpression(literal) => literal
            .value_token()
            .ok()
            .map(|token| token.text_trimmed().to_string()),
    }
    .unwrap_or_else(|| "NUMBER".to_string());

    let mut variable = None;
    let mut property = None;
    let mut function = None;
    let mut call = None;

    for ancestor in numeric_literal.syntax().ancestors().skip(1) {
        if variable.is_none()
            && let Some(declarator) = JsVariableDeclarator::cast(ancestor.clone())
        {
            variable = declarator
                .id()
                .ok()
                .and_then(|pattern| pattern.as_any_js_binding().cloned())
                .and_then(|binding| binding_name(&binding));
        }

        if property.is_none() {
            property = property_name(&ancestor);
        }

        if function.is_none()
            && let Some(enclosing_function) = AnyJsFunction::cast(ancestor.clone())
        {
            function = function_name(&enclosing_function);
        }

        if call.is_none()
            && let Some(call_expression) = JsCallExpression::cast(ancestor.clone())
        {
            call = call_expression
                .callee()
                .ok()
                .and_then(|callee| callee.get_callee_member_name())
                .map(|token| token.text_trimmed().to_string());
        }

        if variable.is_some() && property.is_some() && function.is_some() && call.is_some() {
            break;
        }
    }

    let mut parts = Vec::new();
    for context in [variable, property, function, call].into_iter().flatten() {
        if let Some(normalized) = normalize_name_component(&context, true)
            && !parts.contains(&normalized)
        {
            parts.push(normalized);
        }
    }

    let value =
        normalize_name_component(&literal_text, false).unwrap_or_else(|| "NUMBER".to_string());
    if parts.is_empty() {
        format!("NUMBER_{value}")
    } else {
        parts.push(value);
        parts.join("_")
    }
}

fn binding_name(binding: &AnyJsBinding) -> Option<String> {
    binding
        .as_js_identifier_binding()?
        .name_token()
        .ok()
        .map(|token| token.text_trimmed().to_string())
}

fn function_name(function: &AnyJsFunction) -> Option<String> {
    let binding = match function {
        AnyJsFunction::JsFunctionDeclaration(function) => function.id().ok(),
        AnyJsFunction::JsFunctionExportDefaultDeclaration(function) => function.id(),
        AnyJsFunction::JsFunctionExpression(function) => function.id(),
        AnyJsFunction::JsArrowFunctionExpression(_) => None,
    }?;
    binding_name(&binding)
}

fn property_name(node: &JsSyntaxNode) -> Option<String> {
    if let Some(assignment) = JsAssignmentExpression::cast(node.clone()) {
        return property_name(&assignment.left().ok()?.into_syntax());
    }
    if let Some(property) = JsPropertyObjectMember::cast(node.clone()) {
        return property
            .name()
            .ok()?
            .name()
            .map(|name| name.to_string());
    }
    if let Some(property) = JsPropertyClassMember::cast(node.clone()) {
        return property.name().ok()?.name().map(|name| name.text().to_string());
    }
    if let Some(member) = AnyJsMemberExpression::cast(node.clone()) {
        return member.member_name().map(|name| name.text().to_string());
    }
    if let Some(member) = JsStaticMemberAssignment::cast(node.clone()) {
        return member
            .member()
            .ok()?
            .as_js_name()?
            .value_token()
            .ok()
            .map(|token| token.text_trimmed().to_string());
    }
    if let Some(member) = JsComputedMemberAssignment::cast(node.clone()) {
        return member
            .member()
            .ok()?
            .as_static_value()
            .map(|value| value.text().to_string());
    }
    None
}

fn normalize_name_component(text: &str, ensure_identifier_start: bool) -> Option<String> {
    let mut normalized = String::new();
    let mut previous_is_lowercase = false;

    for character in text.chars() {
        if character.is_ascii_alphanumeric() {
            if character.is_ascii_uppercase() && previous_is_lowercase {
                normalized.push('_');
            }
            normalized.push(character.to_ascii_uppercase());
            previous_is_lowercase = character.is_ascii_lowercase();
        } else {
            if !normalized.ends_with('_') {
                normalized.push('_');
            }
            previous_is_lowercase = false;
        }
    }

    let normalized = normalized.trim_matches('_');
    if normalized.is_empty() {
        return None;
    }

    // Numeric context names need a prefix before they can participate in a binding name.
    if ensure_identifier_start
        && normalized
        .as_bytes()
        .first()
        .is_some_and(|character| character.is_ascii_digit())
    {
        Some(format!("NUMBER_{normalized}"))
    } else {
        Some(normalized.to_string())
    }
}

/// Checks if the given `numeric_literal` is not a magic number.
fn is_valid_number_in_relevant_context(numeric_literal: &JsOrTsNumericLiteral) -> bool {
    let syntax = numeric_literal.syntax();

    if let Some(js_literal) = AnyJsNumericLiteral::cast(syntax.clone()) {
        return is_allowed_number(&js_literal)
            || is_simple_initialize_value(&js_literal)
            || is_bitwise_operation_operand(&js_literal)
            || is_simple_initialize_value_with_big_int(&js_literal)
            || is_class_property_initial_value(&js_literal)
            || is_jsx_expression(&js_literal)
            || is_enum_value(&js_literal)
            || is_array_index(&js_literal)
            || is_default_value_in_fn_param_or_destruct_pattern(&js_literal)
            || is_object_property_value(&js_literal)
            || is_json_stringify_argument(&js_literal)
            || is_parse_int_radix_argument(&js_literal);
    }

    if let Some(ts_literal) = TsNumberLiteralType::cast(syntax.clone()) {
        return is_ts_numeric_literal_union_type(&ts_literal)
            || is_ts_numeric_literal_type_annotation(&ts_literal)
            || is_ts_numeric_literal_return_type(&ts_literal)
            || is_ts_numeric_literal_type_index(&ts_literal);
    }

    false
}

fn is_ts_const_assertion(numeric_literal: &AnyJsNumericLiteral) -> bool {
    numeric_literal.syntax().ancestors().any(|ancestor| {
        let ty = if let Some(expression) = TsAsExpression::cast(ancestor.clone()) {
            expression.ty().ok()
        } else {
            TsTypeAssertionExpression::cast(ancestor).and_then(|expression| expression.ty().ok())
        };
        ty.is_some_and(|ty| is_ts_const_type(&ty))
    })
}

fn is_actionable_numeric_literal(numeric_literal: &AnyJsNumericLiteral) -> bool {
    !is_ts_const_assertion(numeric_literal)
}

fn is_ts_const_type(ty: &AnyTsType) -> bool {
    let Some(reference) = ty.as_ts_reference_type() else {
        return false;
    };
    let Ok(name) = reference.name() else {
        return false;
    };
    let Some(identifier) = name.as_js_reference_identifier() else {
        return false;
    };
    let Ok(token) = identifier.value_token() else {
        return false;
    };
    token.text_trimmed() == "const"
}

const BITWISE_BINARY_OPERATORS: &[JsBinaryOperator] = &[
    JsBinaryOperator::BitwiseAnd,
    JsBinaryOperator::BitwiseOr,
    JsBinaryOperator::BitwiseXor,
    JsBinaryOperator::LeftShift,
    JsBinaryOperator::RightShift,
    JsBinaryOperator::UnsignedRightShift,
];
const BITWISE_ASSIGNMENT_OPERATORS: &[JsAssignmentOperator] = &[
    JsAssignmentOperator::BitwiseAndAssign,
    JsAssignmentOperator::BitwiseOrAssign,
    JsAssignmentOperator::BitwiseXorAssign,
    JsAssignmentOperator::LeftShiftAssign,
    JsAssignmentOperator::RightShiftAssign,
    JsAssignmentOperator::UnsignedRightShiftAssign,
];
const BITWISE_UNARY_OPERATORS: &[JsUnaryOperator] = &[JsUnaryOperator::BitwiseNot];

/// Checks if `numeric_literal` is used as an operand in a bitwise operation such as:
/// Bitwise binary expressions (&, |, ^),
/// Bitwise assignment expressions (&=, |=, ^=),
/// Bitwise unary expressions Not (~).
/// Example:  `let a = ~5;` where `5` is the value.
fn is_bitwise_operation_operand(numeric_literal: &AnyJsNumericLiteral) -> bool {
    is_any_operation_operand_in_js_expression(
        numeric_literal,
        BITWISE_BINARY_OPERATORS,
        BITWISE_UNARY_OPERATORS,
        BITWISE_ASSIGNMENT_OPERATORS,
    )
}

/// Checks if `numeric_literal` is used as an operand in any bitwise or arithmetic operation specified by
/// the provided lists of binary, unary, or assignment operators.
fn is_any_operation_operand_in_js_expression(
    numeric_literal: &AnyJsNumericLiteral,
    binary_operators: &[JsBinaryOperator],
    unary_operators: &[JsUnaryOperator],
    assignment_operators: &[JsAssignmentOperator],
) -> bool {
    get_sanitized_parent_node(numeric_literal.syntax()).is_some_and(|parent| {
        is_sanitized_operation_operand(
            binary_operators,
            unary_operators,
            assignment_operators,
            parent,
        )
    })
}

/// Checks if `numeric_literal` is simple initialization value,
/// Example:  `let foo = 2;` where `2` is the value.
fn is_simple_initialize_value(numeric_literal: &AnyJsNumericLiteral) -> bool {
    get_sanitized_parent_node(numeric_literal.syntax())
        .is_some_and(|parent| JsInitializerClause::can_cast(parent.kind()))
}

/// Checks if `numeric_literal` is BigInt initialization value,
/// Example:  `let bigintTwo: bigint = BigInt(123n);` where `123n` is the value.
fn is_simple_initialize_value_with_big_int(numeric_literal: &AnyJsNumericLiteral) -> bool {
    is_function_matching_name(numeric_literal, Some("BigInt"), None)
        && get_sanitized_parent_node(numeric_literal.syntax()).is_some_and(|parent| {
            parent.grand_parent().is_some_and(|grand_grand_parent| {
                grand_grand_parent.parent().is_some_and(|grand_x_3_parent| {
                    JsInitializerClause::can_cast(grand_x_3_parent.kind())
                })
            })
        })
}

/// Checks if `numeric_literal` is used within a JSX expression
/// Example:  `<div>{2222}</div>` where `2222` is the value.
fn is_jsx_expression(numeric_literal: &AnyJsNumericLiteral) -> bool {
    get_sanitized_parent_node(numeric_literal.syntax()).is_some_and(|parent| {
        JsxExpressionChild::can_cast(parent.kind())
            || JsxExpressionAttributeValue::can_cast(parent.kind())
    })
}

/// Checks if `numeric_literal` is used in an enum declaration
/// Example:  `enum E { A = 3, }` where `3` is the enum value.
fn is_enum_value(numeric_literal: &AnyJsNumericLiteral) -> bool {
    get_sanitized_parent_node(numeric_literal.syntax()).is_some_and(|parent| {
        parent
            .grand_parent()
            .is_some_and(|grand_grand_parent| TsEnumMemberList::can_cast(grand_grand_parent.kind()))
    })
}

/// Checks if `numeric_literal` is used as an index in an array access expression
/// Example:  `list[6] = "foo";` where `6` is the index.
fn is_array_index(numeric_literal: &AnyJsNumericLiteral) -> bool {
    get_sanitized_parent_node(numeric_literal.syntax()).is_some_and(|parent| {
        JsComputedMemberAssignment::can_cast(parent.kind())
            || JsComputedMemberExpression::can_cast(parent.kind())
    })
}

/// Checks if `numeric_literal` is used as a default value in destructuring patterns or function parameters,
/// such as in object/array destructuring or as a default parameter value in a function.
fn is_default_value_in_fn_param_or_destruct_pattern(numeric_literal: &AnyJsNumericLiteral) -> bool {
    get_sanitized_parent_node(numeric_literal.syntax()).is_some_and(|parent| {
        parent.parent().is_some_and(|grand_parent| {
            JsObjectBindingPatternShorthandProperty::can_cast(grand_parent.kind()) // e.g. const { tax = 0.25 } = accountancy;
                || JsFormalParameter::can_cast(grand_parent.kind()) // e.g. function mapParallel(concurrency = 3) {}
                || JsArrayAssignmentPatternElement::can_cast(grand_parent.kind()) // e.g. [head = 100] = []
        })
    })
}

/// Checks if `numeric_literal` is used as the initial value of a class property,
/// including both direct assignments (e.g. foo = 2) and assignments via unary expressions (e.g. bar = -3).
fn is_class_property_initial_value(numeric_literal: &AnyJsNumericLiteral) -> bool {
    get_sanitized_parent_node(numeric_literal.syntax()).is_some_and(|parent| {
        if JsUnaryExpression::can_cast(parent.kind()) {
            // e.g. class C { foo = -2; bar = +3; }
            return parent.grand_parent().is_some_and(|grand_grand_parent| {
                JsPropertyClassMember::can_cast(grand_grand_parent.kind())
            })
                // e.g. class C { foo = 2; static bar = 3; }
                || parent
                .parent()
                .is_some_and(|grand_parent| JsPropertyClassMember::can_cast(grand_parent.kind()));
        }
        // non unary e.g. Class C { foo = 2; #bar = 5; }
        if parent
            .parent()
            .is_some_and(|grand_parent| JsPropertyClassMember::can_cast(grand_parent.kind()))
        {
            return true;
        }

        false
    })
}

/// Returns true if `numeric_literal` is used as the value of an object property,
/// Example:  const with_tax = { tax: 0.25 };
fn is_object_property_value(numeric_literal: &AnyJsNumericLiteral) -> bool {
    get_sanitized_parent_node(numeric_literal.syntax())
        .is_some_and(|parent| JsPropertyObjectMember::can_cast(parent.kind()))
}

/// Checks if `numeric_literal` is used as a radix within parseInt() or Number.parseInt()
/// Example:  Number.parseInt("123", 10); radix is 10
fn is_parse_int_radix_argument(numeric_literal: &AnyJsNumericLiteral) -> bool {
    is_function_matching_name(numeric_literal, Some("parseInt"), Some("Number"))
        || is_function_matching_name(numeric_literal, Some("parseInt"), None)
}

/// Checks if `numeric_literal` is used as an argument of JSON.stringify()
/// Example:  JSON.stringify(22);
fn is_json_stringify_argument(numeric_literal: &AnyJsNumericLiteral) -> bool {
    is_function_matching_name(numeric_literal, Some("stringify"), Some("JSON"))
}

/// Checks if `numeric_literal` is used as an argument to a function call with
/// a specific `member_name` (e.g. `stringify`) and optionally `object_name` (e.g. `JSON`).
/// It traverses the AST to find the call expression and matches the callee's name and object as specified.
fn is_function_matching_name(
    numeric_literal: &AnyJsNumericLiteral,
    member_name: Option<&str>,
    object_name: Option<&str>,
) -> bool {
    get_sanitized_parent_node(numeric_literal.syntax())
        .and_then(|parent| parent.parent())
        .and_then(|grand_parent| grand_parent.parent())
        .and_then(|grand_grand_parent| {
            JsCallExpression::cast(grand_grand_parent).map(|call| call.callee())
        })
        .is_some_and(|callee| {
            let member_name = member_name.unwrap_or_default();

            object_name.is_some_and(|obj_name| {
                callee.clone().is_ok_and(|binding| {
                    binding.as_js_static_member_expression().is_some_and(
                        |static_member_expression| {
                            static_member_expression.object().is_ok_and(|obj| {
                                obj.omit_parentheses().to_trimmed_text().text().eq(obj_name)
                            }) && (member_name.is_empty()
                                || static_member_expression.member().is_ok_and(|member| {
                                    member.to_trimmed_text().text().eq(member_name)
                                }))
                        },
                    )
                })
            }) || callee.clone().is_ok_and(|callee| {
                let callee_name = callee.get_callee_member_name();

                callee_name
                    .clone()
                    .is_some_and(|name| name.token_text_trimmed().text().eq(member_name))
            })
        })
}

/// Checks if `numeric_literal` is used as an operand in any bitwise or arithmetic operation specified by
/// the provided lists of binary, unary, or assignment operators.
fn is_sanitized_operation_operand(
    binary_operators: &[JsBinaryOperator],
    unary_operators: &[JsUnaryOperator],
    assignment_operators: &[JsAssignmentOperator],
    parent: JsSyntaxNode,
) -> bool {
    // e.g. let a = 5 & 3; let a = 5 * 3;
    JsBinaryExpression::cast(parent.clone()).is_some_and(|expression| {
        expression
            .operator()
            .is_ok_and(|operator| binary_operators.contains(&operator.clone()))
    })
        // e.g. let a = ~5; let a = -5;
        || JsUnaryExpression::cast(parent.clone()).is_some_and(|expression| {
        expression
            .operator()
            .is_ok_and(|operator| unary_operators.contains(&operator.clone()))
    })
        // e.g. let a += 5; let a ^= 5;
        || JsAssignmentExpression::cast(parent).is_some_and(|expression| {
        expression
            .operator()
            .is_ok_and(|operator| assignment_operators.contains(&operator.clone()))
    })
}

fn is_ts_numeric_literal_union_type(numeric_literal: &TsNumberLiteralType) -> bool {
    get_sanitized_parent_node(numeric_literal.syntax())
        .is_some_and(|parent| TsUnionTypeVariantList::can_cast(parent.kind()))
}

fn is_ts_numeric_literal_type_index(numeric_literal: &TsNumberLiteralType) -> bool {
    get_sanitized_parent_node(numeric_literal.syntax())
        .is_some_and(|parent| TsIndexedAccessType::can_cast(parent.kind()))
}

/// Checks if the given `numeric_literal` is a numeric literal type annotation
/// Example: `const MyType: 100;` where `100` is the numeric literal type annotation.
fn is_ts_numeric_literal_type_annotation(numeric_literal: &TsNumberLiteralType) -> bool {
    get_sanitized_parent_node(numeric_literal.syntax())
        .is_some_and(|parent| TsTypeAnnotation::can_cast(parent.kind()))
}

/// Checks if the given `numeric_literal` is a numeric literal return type
/// Example: `function f(x: string): 100 { return 100 }` where `100` is the numeric literal return type.
fn is_ts_numeric_literal_return_type(numeric_literal: &TsNumberLiteralType) -> bool {
    get_sanitized_parent_node(numeric_literal.syntax())
        .is_some_and(|parent| TsReturnTypeAnnotation::can_cast(parent.kind()))
}

/// Returns the parent node of the given node, skipping over any unary plus/minus or parenthesized expression wrappers,
/// as well as all type-level wrappers (casts and `satisfies`).
/// It helps determine the true syntactic context of the node by ignoring these common, but semantically insignificant, wrappers.
fn get_sanitized_parent_node(node: &JsSyntaxNode) -> Option<JsSyntaxNode> {
    node.ancestors().skip(1).find(|parent| {
        if TsAsExpression::can_cast(parent.kind())
            || TsNonNullAssertionExpression::can_cast(parent.kind())
            || TsSatisfiesExpression::can_cast(parent.kind())
            || TsTypeAssertionExpression::can_cast(parent.kind())
            || TsPredicateReturnType::can_cast(parent.kind())
            || JsParenthesizedExpression::can_cast(parent.kind())
        {
            false
        } else if let Some(parent) = JsUnaryExpression::cast_ref(parent) {
            match parent.operator() {
                Err(_) => true,
                Ok(op) => !matches!(op, JsUnaryOperator::Plus | JsUnaryOperator::Minus),
            }
        } else {
            true
        }
    })
}

const ALWAYS_IGNORED_IN_ARITHMETIC_OPERATIONS: &[&str] = &[
    "-2", "-1", "0", "1", "2", "10", "24", "60", "-2n", "-1n", "0n", "1n", "2n",
];

fn is_allowed_number(numeric_literal: &AnyJsNumericLiteral) -> bool {
    match numeric_literal {
        AnyJsNumericLiteral::JsNumberLiteralExpression(expr) => expr.value_token(),
        AnyJsNumericLiteral::JsBigintLiteralExpression(expr) => expr.value_token(),
    }
    .is_ok_and(|token| ALWAYS_IGNORED_IN_ARITHMETIC_OPERATIONS.contains(&token.text_trimmed()))
}
