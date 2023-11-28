use biome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_semantic::ReferencesExtensions;
use biome_js_syntax::{
    AnyJsExpression, AnyJsForInitializer, JsAssignmentExpression, JsAssignmentOperator,
    JsBinaryExpression, JsBinaryOperator, JsForStatement, JsIdentifierBinding,
    JsPostUpdateExpression, JsPostUpdateOperator, JsPreUpdateExpression, JsPreUpdateOperator,
    JsSyntaxKind, JsSyntaxToken, JsUnaryOperator, JsVariableDeclarator,
};
use biome_rowan::{declare_node_union, AstNode, AstSeparatedList};

use crate::{semantic_services::Semantic, utils::is_node_equal};

declare_rule! {
    /// This rule recommends a `for-of` loop when in a `for` loop, the index used to extract an item from the iterated array.
    ///
    ///
    /// Source: https://typescript-eslint.io/rules/prefer-for-of/
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// for (let i = 0; i < array.length; i++) {
    ///   console.log(array[i]);
    /// }
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// for (let i = 0; i < array.length; i++) {
    ///    console.log(i, array[i]);
    ///  }
    /// ```
    ///
    /// ```js
    /// for (let i = 0, j = 0; i < array.length; i++) {
    ///    console.log(i, array[i]);
    ///  }
    /// ```
    ///
    /// ```js
    /// for (let i = 1; i < array.length; i++) {
    ///    console.log(i, array[i]);
    ///  }
    /// ```
    ///
    pub(crate) UseForOf {
        version: "next",
        name: "useForOf",
        recommended: false,
    }
}

declare_node_union! {
    pub(crate) AnyIncrementableLike = JsPostUpdateExpression | JsPreUpdateExpression | JsAssignmentExpression
}

impl Rule for UseForOf {
    type Query = Semantic<JsForStatement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();
        let initializer = node.initializer()?;

        if !is_initializer_valid(&initializer)? {
            return None;
        }

        let declarators = initializer.as_js_variable_declaration()?.declarators();
        let initializer = declarators.first()?.ok()?;
        let initializer_id = initializer.id().ok()?;
        let test = node.test()?;
        let binding = initializer_id
            .as_any_js_binding()?
            .as_js_identifier_binding()?;

        if !is_test_valid(binding, &test)? {
            return None;
        }

        if !is_update_valid(binding, node.update()?)? {
            return None;
        }

        let body = node.body().ok()?;
        let block = body.as_js_block_statement();
        let references = get_initializer_references(block, binding, model);
        let array_right = test.as_js_binary_expression()?.right().ok()?;
        let array_used_in_for = array_right
            .as_js_static_member_expression()?
            .object()
            .ok()?;

        let index_only_used_with_array = |reference| {
            let array_in_use = reference_being_used_by_array(reference, &array_used_in_for)
                .is_some_and(|array_in_use| array_in_use);
            let is_delete =
                is_delete(reference, &array_used_in_for).is_some_and(|is_assignee| is_assignee);

            array_in_use && !is_delete
        };

        if references.iter().all(index_only_used_with_array) {
            Some(())
        } else {
            None
        }
    }

    fn diagnostic(node: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            node.query().range(),
            markup! {
                "Use "<Emphasis>"for-of"</Emphasis>" loop instead of a "<Emphasis>"for loop"</Emphasis>"."
            },
        ))
    }
}

/// Get initializer references taking scope to account (excluding initials `for` loop references)
fn get_initializer_references(
    block: Option<&biome_js_syntax::JsBlockStatement>,
    binding: &biome_js_syntax::JsIdentifierBinding,
    model: &biome_js_semantic::SemanticModel,
) -> Vec<AnyJsExpression> {
    let skip_references = {
        if block.is_none() {
            2
        } else {
            0
        }
    };

    binding
        .all_references(model)
        .skip(skip_references)
        .filter_map(|reference| {
            if let Some(block) = block {
                // If block is preset, we need to consider only references in this scope
                if block.range().start() != reference.scope().range().start() {
                    return None;
                }
            }

            AnyJsExpression::cast(reference.syntax().parent()?)
        })
        .collect()
}

/// Validates a for loop variable declarations.
///
/// The initializer must be declared with 0 and can't have multiple initializers.
///
/// # Returns
///
/// - `Some(true)` if the initializer is valid.
/// - `None` if the initializer is invalid (multiple initializers or not initialized with 0).
///
fn is_initializer_valid(initializer: &AnyJsForInitializer) -> Option<bool> {
    let initializer_declarations = initializer.as_js_variable_declaration()?.declarators();
    let initializer = initializer_declarations.first()?.ok()?;

    if initializer_declarations.len() > 1 || !is_zero_initialized(&initializer)? {
        return None;
    }

    Some(true)
}

/// Validates a for loop test expression.
///
/// The test expression must be declared using less than length of array (eg: i < array.length)
/// and reference to same variable declared in for initializer
///
/// # Returns
///
/// - `Some(true)` if the test expression is valid.
/// - `None` if the test expression is invalid (not using less than length or not the same variable name as the initializer).
///
fn is_test_valid(
    initializer_binding: &JsIdentifierBinding,
    test: &AnyJsExpression,
) -> Option<bool> {
    let test_binary_expression = test.as_js_binary_expression()?;
    let left = test_binary_expression.left().ok()?;
    let identifier_expression = left.as_js_identifier_expression()?;

    if initializer_binding.name_token().ok()?.text_trimmed()
        != identifier_expression
            .name()
            .ok()?
            .value_token()
            .ok()?
            .text_trimmed()
    {
        return None;
    }

    if !is_less_than_length_expression(test_binary_expression)? {
        return None;
    }

    Some(true)
}

/// Validates a for loop update/final expression.
///
/// The update/final must increment the variable by 1 and reference to same variable declared in for initializer
///
/// # Returns
///
/// - `Some(true)` if the  update/final expression is valid.
/// - `None` if the  update/final expression is invalid (not a increment or not the same variable name as the initializer).
///
fn is_update_valid(
    initializer_binding: &JsIdentifierBinding,
    update: AnyJsExpression,
) -> Option<bool> {
    let incrementable_like = AnyIncrementableLike::try_from(update).ok()?;

    if initializer_binding.name_token().ok()?.text_trimmed()
        != incrementable_like.get_name_token()?.text_trimmed()
    {
        return None;
    }

    if !incrementable_like.is_increment_by_one()? {
        return None;
    }

    Some(true)
}

fn reference_being_used_by_array(
    expression: &AnyJsExpression,
    array_used_in_for: &AnyJsExpression,
) -> Option<bool> {
    match expression.parent::<AnyJsExpression>()? {
        AnyJsExpression::JsComputedMemberExpression(computed_member) => Some(is_node_equal(
            computed_member.object().ok()?.syntax(),
            array_used_in_for.syntax(),
        )),
        _ => Some(false),
    }
}

fn is_delete(expression: &AnyJsExpression, array_used_in_for: &AnyJsExpression) -> Option<bool> {
    let parent = expression.parent::<AnyJsExpression>()?;

    match parent.parent::<AnyJsExpression>()? {
        AnyJsExpression::JsUnaryExpression(unary_expression) => {
            let is_delete = matches!(unary_expression.operator().ok()?, JsUnaryOperator::Delete);
            let argument = unary_expression.argument().ok()?;
            let argument = argument.as_js_computed_member_expression()?;
            let same_reference =
                is_node_equal(argument.object().ok()?.syntax(), array_used_in_for.syntax());

            Some(is_delete && same_reference)
        }
        _ => Some(false),
    }
}

fn is_less_than_length_expression(binary_expression: &JsBinaryExpression) -> Option<bool> {
    let right = binary_expression.right().ok()?;
    let static_member_expression = right.as_js_static_member_expression()?;
    let object = static_member_expression.object().ok()?;
    let member = static_member_expression.member().ok()?;
    let member = member.as_js_name()?;
    let operator = binary_expression.operator().ok()?;

    Some(
        matches!(operator, JsBinaryOperator::LessThan)
            && member.value_token().ok()?.text() == "length"
            && !matches!(object.syntax().kind(), JsSyntaxKind::JS_THIS_EXPRESSION),
    )
}

fn is_zero_initialized(variable_declarator: &JsVariableDeclarator) -> Option<bool> {
    let expression = variable_declarator.initializer()?.expression().ok()?;
    let number_literal = expression
        .as_any_js_literal_expression()?
        .as_js_number_literal_expression()?;

    let value = number_literal.value_token().ok()?;
    let value = value.text_trimmed();

    Some(value == "0")
}

impl TryFrom<AnyJsExpression> for AnyIncrementableLike {
    type Error = ();

    fn try_from(value: AnyJsExpression) -> Result<Self, Self::Error> {
        match value {
            AnyJsExpression::JsAssignmentExpression(expression) => {
                Ok(AnyIncrementableLike::JsAssignmentExpression(expression))
            }
            AnyJsExpression::JsPostUpdateExpression(expression) => {
                Ok(AnyIncrementableLike::JsPostUpdateExpression(expression))
            }
            AnyJsExpression::JsPreUpdateExpression(expression) => {
                Ok(AnyIncrementableLike::JsPreUpdateExpression(expression))
            }
            _ => Err(()),
        }
    }
}

impl AnyIncrementableLike {
    ///Check whether the current expression is increment by one, like:
    ///
    /// JsPostUpdateExpression: i++
    /// JsPreUpdateExpression: ++i
    /// JsAssignmentExpression(Binary): i = i + 1
    /// JsAssignmentExpression(Shorthand): i += 1
    fn is_increment_by_one(&self) -> Option<bool> {
        match self {
            AnyIncrementableLike::JsPostUpdateExpression(expression) => Some(matches!(
                expression.operator().ok()?,
                JsPostUpdateOperator::Increment
            )),
            AnyIncrementableLike::JsPreUpdateExpression(expression) => Some(matches!(
                expression.operator().ok()?,
                JsPreUpdateOperator::Increment
            )),
            AnyIncrementableLike::JsAssignmentExpression(expression) => {
                let operator = expression.operator().ok()?;
                let right = expression.right().ok()?;

                if let Some(binary_expression) = JsBinaryExpression::cast_ref(right.syntax()) {
                    let binary_right = binary_expression.right().ok()?;
                    let number_literal = binary_right
                        .as_any_js_literal_expression()?
                        .as_js_number_literal_expression()?;

                    let binary_value = number_literal.value_token().ok()?;
                    let binary_value = binary_value.text_trimmed();

                    if matches!(binary_expression.operator().ok()?, JsBinaryOperator::Plus)
                        && binary_value == "1"
                    {
                        return Some(true);
                    }
                }

                let number_literal = right
                    .as_any_js_literal_expression()?
                    .as_js_number_literal_expression()?;

                let value = number_literal.value_token().ok()?;
                let value = value.text_trimmed();

                Some(matches!(operator, JsAssignmentOperator::AddAssign) && value == "1")
            }
        }
    }

    fn get_name_token(&self) -> Option<JsSyntaxToken> {
        match self {
            AnyIncrementableLike::JsPostUpdateExpression(expression) => expression
                .operand()
                .ok()?
                .as_js_identifier_assignment()?
                .name_token()
                .ok(),
            AnyIncrementableLike::JsPreUpdateExpression(expression) => expression
                .operand()
                .ok()?
                .as_js_identifier_assignment()?
                .name_token()
                .ok(),
            AnyIncrementableLike::JsAssignmentExpression(expression) => expression
                .left()
                .ok()?
                .as_any_js_assignment()?
                .as_js_identifier_assignment()?
                .name_token()
                .ok(),
        }
    }
}
