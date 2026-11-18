use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::{Binding, ReferencesExtensions, SemanticModel};
use biome_js_syntax::{
    AnyJsClass, AnyJsExpression, AnyJsFunction, AnyJsIdentifierReference, AnyJsLiteralExpression,
    AnyTsType, JsAssignmentOperator, JsBinaryOperator, JsBlockStatement, JsCallExpression,
    JsComputedMemberExpression, JsForStatement, JsGetterObjectMember, JsIdentifierExpression,
    JsLanguage, JsLogicalOperator, JsMethodObjectMember, JsParenthesizedExpression,
    JsPostUpdateOperator, JsPreUpdateOperator, JsReferenceIdentifier, JsSetterObjectMember,
    JsSyntaxToken, JsTemplateExpression, JsUnaryExpression, JsUnaryOperator, JsVariableDeclarator,
    JsVariableStatement, T, global_identifier,
};
use biome_rowan::{AstNode, AstSeparatedList, SyntaxKindSet, TextRange};
use biome_rule_options::use_iterator_zip::UseIteratorZipOptions;

use crate::{
    services::semantic::Semantic,
    utils::indexed_collection::{binding_type_annotation, is_known_non_indexed_collection_binding},
};

declare_lint_rule! {
    /// Prefer `Iterator.zip()` over reading several arrays with the same index.
    ///
    /// A `for` loop that stops at the end of the shortest array, and uses its index only to read the same position from each array, can be written as a `for...of` loop over `Iterator.zip()`.
    /// `Iterator.zip()` returns the elements at the same position together and stops at the end of the shortest array, so the loop needs neither an index nor a length check.
    ///
    /// The rule reports a `for` loop when all of the following are true:
    ///
    /// - The index is declared with `let`, starts at `0`, and increases by one after each iteration.
    /// - The loop stops at the end of the shortest of two or more arrays.
    ///   Each array must be a variable, not a property such as `object.names`.
    ///   The condition either compares the index to `Math.min()` of the array lengths, or compares the index to each length and joins the comparisons with `&&`.
    ///   The result of `Math.min()` can also be stored in a variable, as long as the variable is declared in the loop or in the statement right before it, and only the loop condition uses it.
    /// - The body is a block that uses the index and the arrays only to read `array[index]`, and it reads every array at least once.
    ///
    /// The rule ignores a loop when its body changes, calls, or deletes an array element, or when it uses the index or an array inside a nested function or class.
    ///
    /// `Iterator.zip()` only accepts values that can be looped over with `for...of`, and it doesn't accept strings.
    /// The rule therefore ignores a loop when it can tell that one of the variables holds something other than an array, such as a string, an object literal, a `Map`, or a `Set`.
    ///
    /// :::caution
    /// `Iterator.zip()` is new, and the JavaScript engine that runs your code may not support it yet.
    /// Before you enable this rule, check which engines support it on [Can I use](https://caniuse.com/mdn-javascript_builtins_iterator_zip) or in the [MDN compatibility table](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator/zip#browser_compatibility).
    /// :::
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// for (let index = 0; index < Math.min(names.length, scores.length); index++) {
    ///     save(names[index], scores[index]);
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// for (let index = 0; index < names.length && index < scores.length; index++) {
    ///     save(names[index], scores[index]);
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const length = Math.min(names.length, scores.length);
    /// for (let index = 0; index < length; index++) {
    ///     save(names[index], scores[index]);
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// for (const [name, score] of Iterator.zip([names, scores])) {
    ///     save(name, score);
    /// }
    ///
    /// // The index is used for something other than reading the arrays.
    /// for (let index = 0; index < Math.min(names.length, scores.length); index++) {
    ///     save(index, names[index], scores[index]);
    /// }
    ///
    /// // The loop changes one of the arrays.
    /// for (let index = 0; index < Math.min(names.length, scores.length); index++) {
    ///     names[index] = scores[index];
    /// }
    ///
    /// // The loop reads a single array.
    /// for (let index = 0; index < names.length; index++) {
    ///     save(names[index]);
    /// }
    ///
    /// // `Iterator.zip()` doesn't accept strings.
    /// const letters = "abc";
    /// for (let index = 0; index < Math.min(letters.length, scores.length); index++) {
    ///     save(letters[index], scores[index]);
    /// }
    /// ```
    ///
    pub UseIteratorZip {
        version: "2.6.3",
        name: "useIteratorZip",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("prefer-iterator-zip").same()],
        recommended: false,
        severity: Severity::Information,
    }
}

impl Rule for UseIteratorZip {
    type Query = Semantic<JsForStatement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseIteratorZipOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();

        let body = node.body().ok()?;
        let body = body.as_js_block_statement()?;

        let initializer = node.initializer()?;
        let declaration = initializer.as_js_variable_declaration()?;
        if !declaration.is_let() {
            return None;
        }
        let mut declarators = declaration.declarators().iter();
        let index = declarators.next()?.ok()?;
        let cached_bound = declarators.next();
        if declarators.next().is_some() {
            return None;
        }

        let index_id = index.id().ok()?;
        let index_binding = index_id.as_any_js_binding()?.as_js_identifier_binding()?;
        let index_name = index_binding.name_token().ok()?;
        let index_name = index_name.text_trimmed();
        if !is_number(&index.initializer()?.expression().ok()?, 0.0)
            || !is_increment_by_one(node.update()?, index_name)
        {
            return None;
        }

        let test = node.test()?.omit_parentheses();
        let arrays = match cached_bound {
            Some(cached_bound) => {
                cached_bound_arrays(&test, index_name, &cached_bound.ok()?, model)?
            }
            None => shortest_length_arrays(&test, index_name, model)
                .or_else(|| preceding_cached_bound_arrays(node, &test, index_name, model))?,
        };

        let mut inputs = Vec::<ZipInput>::with_capacity(arrays.len());
        for array in arrays {
            let name = array.value_token().ok()?;
            if inputs
                .iter()
                .any(|input| input.name.text_trimmed() == name.text_trimmed())
            {
                return None;
            }
            let binding = model.binding(&array);
            if binding.as_ref().is_some_and(|binding| {
                is_declared_in(binding, body) || is_known_non_zippable(binding, model)
            }) {
                return None;
            }
            inputs.push(ZipInput {
                name,
                binding,
                is_read: false,
            });
        }

        let index_binding = model.as_binding(index_binding);
        for reference in body
            .syntax()
            .descendants()
            .filter_map(AnyJsIdentifierReference::cast)
        {
            let name = reference.value_token().ok()?;
            let name = name.text_trimmed();
            if name != index_name && !inputs.iter().any(|input| input.name.text_trimmed() == name) {
                continue;
            }

            // The name can also belong to a different variable that is declared inside the body.
            let binding = model.binding(&reference);
            if binding.as_ref() != Some(&index_binding)
                && !inputs.iter().any(|input| input.is(name, binding.as_ref()))
            {
                continue;
            }

            let position = read_input_position(&reference, body, &index_binding, &inputs, model)?;
            inputs[position].is_read = true;
        }

        inputs.iter().all(|input| input.is_read).then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let range = TextRange::new(
            node.for_token().ok()?.text_trimmed_range().start(),
            node.r_paren_token().ok()?.text_trimmed_range().end(),
        );

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This "<Emphasis>"for"</Emphasis>" loop only uses its index to read the same position from several arrays."
                },
            )
            .note(markup! {
                <Emphasis>"Iterator.zip()"</Emphasis>" returns the elements at the same position together and stops at the end of the shortest array, so the loop needs neither an index nor a length check."
            })
            .note(markup! {
                "Use a "<Emphasis>"for...of"</Emphasis>" loop over "<Emphasis>"Iterator.zip()"</Emphasis>" instead."
            }),
        )
    }
}

/// An array whose length bounds the loop.
struct ZipInput {
    name: JsSyntaxToken,
    /// `None` when the array isn't declared in the file.
    binding: Option<Binding>,
    /// Whether the loop body reads `array[index]` at least once.
    is_read: bool,
}

impl ZipInput {
    /// Returns `true` if an identifier called `name` that resolves to `binding` refers to this array.
    fn is(&self, name: &str, binding: Option<&Binding>) -> bool {
        self.name.text_trimmed() == name && self.binding.as_ref() == binding
    }
}

/// Functions and classes, whose bodies can run after the loop iteration that created them.
const CAPTURING_KINDS: SyntaxKindSet<JsLanguage> = AnyJsFunction::KIND_SET
    .union(AnyJsClass::KIND_SET)
    .union(JsMethodObjectMember::KIND_SET)
    .union(JsGetterObjectMember::KIND_SET)
    .union(JsSetterObjectMember::KIND_SET);

/// Expressions that call their operand instead of reading its value.
const CALLING_KINDS: SyntaxKindSet<JsLanguage> =
    JsCallExpression::KIND_SET.union(JsTemplateExpression::KIND_SET);

/// Returns `true` if `expression` is the number literal `value`.
fn is_number(expression: &AnyJsExpression, value: f64) -> bool {
    matches!(
        expression.clone().omit_parentheses(),
        AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsNumberLiteralExpression(literal)
        ) if literal.as_number() == Some(value)
    )
}

/// Returns `true` if `expression` is a reference to the variable called `name`.
fn is_identifier_named(expression: &AnyJsExpression, name: &str) -> bool {
    expression
        .as_js_identifier_expression()
        .and_then(|identifier| identifier.name().ok())
        .is_some_and(|reference| reference.has_name(name))
}

/// Returns `true` if `update` adds one to the variable called `index_name`.
///
/// Accepts `index++`, `++index`, `index += 1`, and `index = index + 1`.
fn is_increment_by_one(update: AnyJsExpression, index_name: &str) -> bool {
    let target = match update.omit_parentheses() {
        AnyJsExpression::JsPostUpdateExpression(update) => {
            if update.operator() != Ok(JsPostUpdateOperator::Increment) {
                return false;
            }
            update.operand().ok()
        }
        AnyJsExpression::JsPreUpdateExpression(update) => {
            if update.operator() != Ok(JsPreUpdateOperator::Increment) {
                return false;
            }
            update.operand().ok()
        }
        AnyJsExpression::JsAssignmentExpression(assignment) => {
            let Ok(right) = assignment.right() else {
                return false;
            };
            let adds_one = match assignment.operator() {
                Ok(JsAssignmentOperator::AddAssign) => is_number(&right, 1.0),
                Ok(JsAssignmentOperator::Assign) => right
                    .omit_parentheses()
                    .as_js_binary_expression()
                    .is_some_and(|sum| {
                        sum.operator() == Ok(JsBinaryOperator::Plus)
                            && sum.left().is_ok_and(|left| {
                                is_identifier_named(&left.omit_parentheses(), index_name)
                            })
                            && sum.right().is_ok_and(|right| is_number(&right, 1.0))
                    }),
                _ => false,
            };
            if !adds_one {
                return false;
            }
            assignment
                .left()
                .ok()
                .and_then(|left| left.as_any_js_assignment().cloned())
        }
        _ => None,
    };

    target
        .as_ref()
        .and_then(|target| target.as_js_identifier_assignment())
        .and_then(|target| target.name_token().ok())
        .is_some_and(|name| name.text_trimmed() == index_name)
}

/// Returns the value that the index must stay below for `comparison` to be true.
///
/// Accepts `index < bound` and `bound > index`.
fn upper_bound(comparison: &AnyJsExpression, index_name: &str) -> Option<AnyJsExpression> {
    let comparison = comparison.as_js_binary_expression()?;
    let left = comparison.left().ok()?.omit_parentheses();
    let right = comparison.right().ok()?.omit_parentheses();

    match comparison.operator().ok()? {
        JsBinaryOperator::LessThan if is_identifier_named(&left, index_name) => Some(right),
        JsBinaryOperator::GreaterThan if is_identifier_named(&right, index_name) => Some(left),
        _ => None,
    }
}

/// Returns `array` if `expression` is `array.length`.
fn length_owner(expression: &AnyJsExpression) -> Option<JsReferenceIdentifier> {
    let member = expression.as_js_static_member_expression()?;
    if member.operator_token().ok()?.kind() != T![.]
        || member
            .member()
            .ok()?
            .as_js_name()?
            .value_token()
            .ok()?
            .text_trimmed()
            != "length"
    {
        return None;
    }

    member
        .object()
        .ok()?
        .omit_parentheses()
        .as_js_identifier_expression()?
        .name()
        .ok()
}

/// Returns the arrays whose lengths are passed to `Math.min()`.
///
/// Returns `None` unless `expression` is a call to the built-in `Math.min()` with at least
/// two arguments that are all of the form `array.length`.
fn math_min_arrays(
    expression: &AnyJsExpression,
    model: &SemanticModel,
) -> Option<Vec<JsReferenceIdentifier>> {
    let call = expression.as_js_call_expression()?;
    if call.optional_chain_token().is_some() {
        return None;
    }

    let callee = call.callee().ok()?.omit_parentheses();
    let callee = callee.as_js_static_member_expression()?;
    if callee.operator_token().ok()?.kind() != T![.]
        || callee
            .member()
            .ok()?
            .as_js_name()?
            .value_token()
            .ok()?
            .text_trimmed()
            != "min"
    {
        return None;
    }

    let object = callee.object().ok()?.omit_parentheses();
    let (reference, name) =
        global_identifier(&object.as_js_identifier_expression()?.clone().into())?;
    if name.text() != "Math" || model.binding(&reference).is_some() {
        return None;
    }

    let arguments = call.arguments().ok()?.args();
    if arguments.len() < 2 {
        return None;
    }
    arguments
        .iter()
        .map(|argument| {
            let argument = argument.ok()?.as_any_js_expression()?.clone();
            length_owner(&argument.omit_parentheses())
        })
        .collect()
}

/// Adds the array of every `index < array.length` comparison in `test` to `arrays`.
///
/// Returns `None` if `test` contains anything other than such comparisons joined with `&&`.
fn collect_compared_arrays(
    test: &AnyJsExpression,
    index_name: &str,
    arrays: &mut Vec<JsReferenceIdentifier>,
) -> Option<()> {
    let Some(logical) = test.as_js_logical_expression() else {
        arrays.push(length_owner(&upper_bound(test, index_name)?)?);
        return Some(());
    };
    if logical.operator().ok()? != JsLogicalOperator::LogicalAnd {
        return None;
    }

    collect_compared_arrays(&logical.left().ok()?.omit_parentheses(), index_name, arrays)?;
    collect_compared_arrays(
        &logical.right().ok()?.omit_parentheses(),
        index_name,
        arrays,
    )
}

/// Returns the arrays that the loop condition `test` compares the index against.
///
/// Accepts `index < Math.min(a.length, b.length)` and `index < a.length && index < b.length`.
fn shortest_length_arrays(
    test: &AnyJsExpression,
    index_name: &str,
    model: &SemanticModel,
) -> Option<Vec<JsReferenceIdentifier>> {
    if let Some(arrays) =
        upper_bound(test, index_name).and_then(|bound| math_min_arrays(&bound, model))
    {
        return Some(arrays);
    }

    // A single comparison bounds the loop by one array only.
    test.as_js_logical_expression()?;
    let mut arrays = Vec::new();
    collect_compared_arrays(test, index_name, &mut arrays)?;
    Some(arrays)
}

/// Returns the arrays whose shortest length is stored in the variable declared by `declarator`.
///
/// Returns `None` unless the loop condition `test` compares the index against that variable
/// and nothing else uses the variable.
fn cached_bound_arrays(
    test: &AnyJsExpression,
    index_name: &str,
    declarator: &JsVariableDeclarator,
    model: &SemanticModel,
) -> Option<Vec<JsReferenceIdentifier>> {
    let bound = upper_bound(test, index_name)?;
    let bound = bound.as_js_identifier_expression()?.name().ok()?;

    let id = declarator.id().ok()?;
    let mut references = id
        .as_any_js_binding()?
        .as_js_identifier_binding()?
        .all_references(model);
    if references.next()?.syntax() != *bound.syntax() || references.next().is_some() {
        return None;
    }

    let initializer = declarator.initializer()?.expression().ok()?;
    math_min_arrays(&initializer.omit_parentheses(), model)
}

/// Like [cached_bound_arrays], for a variable declared in the statement right before the loop.
fn preceding_cached_bound_arrays(
    node: &JsForStatement,
    test: &AnyJsExpression,
    index_name: &str,
    model: &SemanticModel,
) -> Option<Vec<JsReferenceIdentifier>> {
    let statement = JsVariableStatement::cast(node.syntax().prev_sibling()?)?;
    let declaration = statement.declaration().ok()?;
    if !declaration.is_const() && !declaration.is_let() {
        return None;
    }

    let mut declarators = declaration.declarators().iter();
    let declarator = declarators.next()?.ok()?;
    if declarators.next().is_some() {
        return None;
    }
    cached_bound_arrays(test, index_name, &declarator, model)
}

/// Returns the position in `inputs` of the array that `reference` reads an element from.
///
/// `reference` is a use of the index or of one of the `inputs` inside the loop `body`.
/// Returns `None` if it is anything other than a part of `array[index]`, where the value of
/// the element is read during the loop iteration.
fn read_input_position(
    reference: &AnyJsIdentifierReference,
    body: &JsBlockStatement,
    index: &Binding,
    inputs: &[ZipInput],
    model: &SemanticModel,
) -> Option<usize> {
    let AnyJsIdentifierReference::JsReferenceIdentifier(reference) = reference else {
        return None;
    };
    if reference
        .syntax()
        .ancestors()
        .skip(1)
        .take_while(|ancestor| ancestor != body.syntax())
        .any(|ancestor| CAPTURING_KINDS.matches(ancestor.kind()))
    {
        return None;
    }

    let member = reference
        .parent::<JsIdentifierExpression>()?
        .syntax()
        .ancestors()
        .skip(1)
        .find(|ancestor| !JsParenthesizedExpression::can_cast(ancestor.kind()))
        .and_then(JsComputedMemberExpression::cast)?;
    if member.optional_chain_token().is_some() {
        return None;
    }

    let property = member.member().ok()?.omit_parentheses();
    let property = property.as_js_identifier_expression()?.name().ok()?;
    if model.binding(&property).as_ref() != Some(index) {
        return None;
    }

    let object = member.object().ok()?.omit_parentheses();
    let object = object.as_js_identifier_expression()?.name().ok()?;
    let object_name = object.value_token().ok()?;
    let object_binding = model.binding(&object);
    let position = inputs
        .iter()
        .position(|input| input.is(object_name.text_trimmed(), object_binding.as_ref()))?;

    is_value_read(member).then_some(position)
}

/// Returns `true` if the code only reads the value of `element`.
///
/// Returns `false` for `element()`, ``element`text` ``, and `delete element`.
fn is_value_read(element: JsComputedMemberExpression) -> bool {
    let Some(parent) = AnyJsExpression::from(element)
        .outer_expression()
        .and_then(|outer| outer.syntax().parent())
    else {
        return false;
    };

    !CALLING_KINDS.matches(parent.kind())
        && !JsUnaryExpression::cast(parent)
            .is_some_and(|unary| unary.operator() == Ok(JsUnaryOperator::Delete))
}

/// Returns `true` if the variable is declared inside the loop `body`.
///
/// A `var` declaration inside the body is visible in the loop condition and gives the
/// variable a new value on each iteration.
fn is_declared_in(binding: &Binding, body: &JsBlockStatement) -> bool {
    body.range()
        .contains_range(binding.syntax().text_trimmed_range())
}

/// Returns `true` if the variable declared by `binding` is known to hold something that
/// `Iterator.zip()` doesn't accept, or may not accept.
///
/// Besides values that aren't arrays or typed arrays, this includes variables with an object type
/// annotation such as `{ length: number }`, which also matches objects that can't be looped over.
fn is_known_non_zippable(binding: &Binding, model: &SemanticModel) -> bool {
    if is_known_non_indexed_collection_binding(binding, model) {
        return true;
    }
    binding_type_annotation(binding)
        .is_some_and(|annotation| matches!(annotation.ty(), Ok(AnyTsType::TsObjectType(_))))
}
